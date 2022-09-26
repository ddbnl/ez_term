use crate::run::definitions::{Coordinates, Pixel, PixelMap, Size, StateTree};
use crate::states::definitions::{
    ColorConfig, InfiniteSize, LayoutOrientation, ScrollingConfig, TableConfig,
};
use crate::widgets::ez_object::EzObject;
use crate::widgets::helper_functions::{align_content_horizontally, align_content_vertically};
use crate::widgets::layout::layout::Layout;
use crate::GenericState;

// Box mode implementations
impl Layout {
    /// Used by [get_contents] when the [LayoutMode] is set to [Box] and [LayoutOrientation] is
    /// set to [Horizontal]. Merges contents of sub layouts and/or widgets horizontally, using
    /// own [height] for each.
    pub fn get_table_mode_contents(&self, state_tree: &mut StateTree) -> PixelMap {
        let state = state_tree.get_mut(&self.get_path()).as_layout();

        let own_table_config = state.get_table_config().clone();
        let own_orientation = state.get_orientation().clone();
        if own_orientation == LayoutOrientation::Vertical
            || own_orientation == LayoutOrientation::Horizontal
        {
            panic!(
                "Error in layout: {}. When in table mode, orientation must be one of: \
            ‘lr-tb’, ‘tb-lr’, ‘rl-tb’, ‘tb-rl’, ‘lr-bt’, ‘bt-lr’, ‘rl-bt’ and ‘bt-rl’.",
                self.id
            )
        }
        if own_table_config.rows == 0 && own_table_config.cols == 0 {
            return PixelMap::new()
        }

        let own_effective_size = state.get_effective_size();
        let own_infinite_size = state.get_infinite_size().clone();
        let own_colors = state.get_color_config().clone();
        let own_scrolling = state.get_scrolling_config().clone();
        let own_fill = state.get_fill();
        let own_filler_symbol = state.get_filler_symbol();

        let (child_table, rows, cols) =
            self.get_table_model(&own_table_config, own_orientation, state_tree);
        if rows == 0 || cols == 0 {
            return PixelMap::new();
        }

        // Initial cell size. Can change if force_default is false, in which case it will scale to
        // size of largest child
        let default_height = if own_table_config.get_row_default_height() == 0 {
            own_effective_size.height / rows
        } else {
            own_table_config.get_row_default_height()
        };
        let default_width = if own_table_config.get_col_default_width() == 0 {
            own_effective_size.width / cols
        } else {
            own_table_config.get_col_default_width()
        };

        let content_list = self.get_table_mode_child_content(
            state_tree,
            &own_infinite_size,
            &own_scrolling,
            &own_effective_size,
        );

        let content = self.draw_table(
            &child_table,
            content_list,
            rows,
            cols,
            &own_table_config,
            default_width,
            default_height,
            &own_colors,
            state_tree,
            own_fill,
            own_filler_symbol,
        );

        let state = state_tree.get_mut(&self.get_path()).as_layout_mut();
        if state.get_auto_scale().get_auto_scale_width() {
            state.set_effective_width(content.len());
        }
        if state.get_auto_scale().get_auto_scale_height() {
            state.set_effective_height(content.iter().map(|x| x.len()).max().unwrap_or(0));
        }
        content
    }

    /// Calculate the amount of needed rows and columns based on the number of children
    fn get_rows_and_cols(
        &self,
        table_config: &TableConfig,
        state_tree: &mut StateTree,
    ) -> (usize, usize) {
        let children = self.get_children_in_view(state_tree);
        let (rows, cols);
        if table_config.get_rows() > 0 && table_config.get_cols() > 0 {
            rows = table_config.get_rows();
            cols = table_config.get_cols();
        } else if table_config.get_rows() > 0 {
            rows = table_config.get_rows();
            cols =
                (children.len() / rows) as usize + if children.len() % rows != 0 { 1 } else { 0 };
        } else {
            cols = table_config.get_cols();
            rows =
                (children.len() / cols) as usize + if children.len() % cols != 0 { 1 } else { 0 };
        }
        (rows, cols)
    }

    /// Create a model of the order in which children should be added, by creating a table of
    /// child index. For example, a table layout with 9 children and orientation 'rl-tb' will return
    /// 2 1 0
    /// 5 4 3
    /// 8 7 6
    fn get_table_model(
        &self,
        table_config: &TableConfig,
        orientation: LayoutOrientation,
        state_tree: &mut StateTree,
    ) -> (Vec<Vec<usize>>, usize, usize) {
        let (rows, cols) = self.get_rows_and_cols(table_config, state_tree);
        let children = self.get_children_in_view(state_tree);
        let mut children_table = vec![vec![0; rows]; cols];

        match orientation {
            LayoutOrientation::LeftRightTopBottom => {
                let (mut x, mut y) = (0, 0);
                for i in 0..children.len() {
                    if x == cols {
                        x = 0;
                        y += 1;
                    }
                    children_table[x][y] = i;
                    x += 1;
                }
            }
            LayoutOrientation::LeftRightBottomTop => {
                let (mut x, mut y) = (0, rows - 1);
                for i in 0..children.len() {
                    if x == cols {
                        x = 0;
                        y -= 1;
                    }
                    children_table[x][y] = i;
                    x += 1;
                }
            }
            LayoutOrientation::RightLeftTopBottom => {
                let (mut x, mut y) = (cols - 1, 0);
                for i in 0..children.len() {
                    children_table[x][y] = i;
                    if i == children.len() - 1 {
                        break;
                    }
                    if x == 0 {
                        x = cols - 1;
                        y += 1;
                    } else {
                        x -= 1;
                    }
                }
            }
            LayoutOrientation::RightLeftBottomTop => {
                let (mut x, mut y) = (cols - 1, rows - 1);
                for i in 0..children.len() {
                    children_table[x][y] = i;
                    if i == children.len() - 1 {
                        break;
                    }
                    if x == 0 {
                        x = cols - 1;
                        y -= 1;
                    } else {
                        x -= 1;
                    }
                }
            }
            LayoutOrientation::TopBottomLeftRight => {
                let (mut x, mut y) = (0, 0);
                for i in 0..children.len() {
                    if y == rows {
                        y = 0;
                        x += 1;
                    }
                    children_table[x][y] = i;
                    y += 1;
                }
            }
            LayoutOrientation::TopBottomRightLeft => {
                let (mut x, mut y) = (cols - 1, 0);
                for i in 0..children.len() {
                    if y == rows {
                        y = 0;
                        x -= 1;
                    }
                    children_table[x][y] = i;
                    y += 1;
                }
            }
            LayoutOrientation::BottomTopLeftRight => {
                let (mut x, mut y) = (0, rows - 1);
                for i in 0..children.len() {
                    children_table[x][y] = i;
                    if i == children.len() - 1 {
                        break;
                    }
                    if y == 0 {
                        y = rows - 1;
                        x += 1;
                    } else {
                        y -= 1;
                    }
                }
            }
            LayoutOrientation::BottomTopRightLeft => {
                let (mut x, mut y) = (cols - 1, rows - 1);
                for i in 0..children.len() {
                    children_table[x][y] = i;
                    if i == children.len() - 1 {
                        break;
                    }
                    if y == 0 {
                        y = rows - 1;
                        x -= 1;
                    } else {
                        y -= 1;
                    }
                }
            }
            _ => (),
        }
        (children_table, rows, cols)
    }

    /// Calculate how high each row should be. if force_default is true, use the default. Otherwise
    /// find the highest child.
    fn get_row_heights(
        &self,
        table_config: &TableConfig,
        rows: usize,
        cols: usize,
        child_table: &Vec<Vec<usize>>,
        content_list: &Vec<PixelMap>,
        default_height: usize,
    ) -> Vec<usize> {
        let mut row_heights = Vec::new();
        if !table_config.get_force_default_row_height() {
            for y in 0..rows {
                let mut largest = 0;
                for x in 0..cols {
                    if x < child_table.len() && y < child_table[x].len() {
                        let height = content_list
                            .get(child_table[x][y])
                            .unwrap()
                            .iter()
                            .map(|x| x.len())
                            .max()
                            .unwrap_or(0);
                        if height > largest {
                            largest = height
                        };
                    }
                }
                if table_config.get_row_default_height() > 0
                    && largest < table_config.get_row_default_height()
                {
                    largest = table_config.get_row_default_height()
                }
                row_heights.push(largest);
            }
        } else {
            for _ in 0..rows {
                row_heights.push(default_height)
            }
        }
        row_heights
    }

    /// Calculate how wide each column should be. if force_default is true, use the default.
    /// Otherwise find the widest child.
    fn get_col_widths(
        &self,
        table_config: &TableConfig,
        rows: usize,
        cols: usize,
        child_table: &Vec<Vec<usize>>,
        content_list: &[PixelMap],
        default_width: usize,
    ) -> Vec<usize> {
        let mut col_widths = Vec::new();
        if !table_config.get_force_default_col_width() {
            for x in 0..cols {
                let mut largest = 0;
                for y in 0..rows {
                    if x < child_table.len() && y < child_table[x].len() {
                        let width = content_list.get(child_table[x][y]).unwrap().len();
                        if width > largest {
                            largest = width
                        };
                    }
                }
                if table_config.get_col_default_width() > 0
                    && largest < table_config.get_col_default_width()
                {
                    largest = table_config.get_col_default_width()
                }
                col_widths.push(largest);
            }
        } else {
            for _ in 0..cols {
                col_widths.push(default_width)
            }
        }
        col_widths
    }

    /// Get the content of each child, so we can draw it in a table after.
    fn get_table_mode_child_content(
        &self,
        state_tree: &mut StateTree,
        infinite_size: &InfiniteSize,
        scrolling_config: &ScrollingConfig,
        effective_size: &Size,
    ) -> Vec<PixelMap> {
        let mut content_list = Vec::new();
        for child in self.get_children_in_view(state_tree) {
            let generic_child = child.as_ez_object();
            let state = state_tree
                .get_mut(&generic_child.get_path().clone())
                .as_generic_mut();

            if infinite_size.get_width() || scrolling_config.get_scroll_x() {
                state.get_infinite_size_mut().set_width(true);
            }
            if infinite_size.get_height() || scrolling_config.get_scroll_y() {
                state.get_infinite_size_mut().set_height(true);
            }

            // If autoscaling is enabled set child size to max width. It is then expected to scale
            // itself according to its' content
            if state.get_auto_scale().get_auto_scale_width() {
                state.get_size_mut().set_width(effective_size.width)
            }
            if state.get_auto_scale().get_auto_scale_height() {
                state.get_size_mut().set_height(effective_size.height)
            }

            let child_content = generic_child.get_contents(state_tree);
            let state = state_tree
                .get_mut(&generic_child.get_path())
                .as_generic_mut(); // re-borrow

            state.get_size_mut().set_width(child_content.len());
            state
                .get_size_mut()
                .set_height(child_content.iter().map(|x| x.len()).max().unwrap_or(0));
            content_list.push(child_content);
        }
        content_list
    }

    /// Draw a list of child content in a table using the child_table as a model for the
    /// orientation.
    fn draw_table(
        &self,
        child_table: &Vec<Vec<usize>>,
        mut content_list: Vec<PixelMap>,
        rows: usize,
        cols: usize,
        table_config: &TableConfig,
        default_width: usize,
        default_height: usize,
        colors: &ColorConfig,
        state_tree: &mut StateTree,
        fill: bool,
        filler_symbol: String,
    ) -> PixelMap {
        let (symbol, fg_color, bg_color) = if fill {
            (
                filler_symbol.clone(),
                colors.get_filler_fg_color(),
                colors.get_filler_bg_color(),
            )
        } else {
            (
                " ".to_string(),
                colors.get_fg_color(),
                colors.get_bg_color(),
            )
        };

        let row_heights = self.get_row_heights(
            table_config,
            rows,
            cols,
            child_table,
            &content_list,
            default_height,
        );
        let col_widths = self.get_col_widths(
            table_config,
            rows,
            cols,
            child_table,
            &content_list,
            default_width,
        );
        let total_height: usize = row_heights.iter().sum();
        let total_width: usize = col_widths.iter().sum();
        let mut content =
            vec![vec!(Pixel::new(symbol, fg_color, bg_color); total_height); total_width];

        let mut write_pos = Coordinates::new(0, 0);
        let children = self.get_children_in_view(state_tree);
        for x in 0..cols {
            write_pos.y = 0;
            for y in 0..rows {
                if x >= child_table.len() || y >= child_table[x].len() ||
                    child_table[x][y] >= children.len() {
                    continue;
                }
                let state = state_tree
                    .get_mut(
                        &children
                            .get(child_table[x][y])
                            .unwrap()
                            .as_ez_object()
                            .get_path(),
                    )
                    .as_generic_mut();
                let child_content = std::mem::take(&mut content_list[child_table[x][y]]);

                let (child_content, offset) = align_content_horizontally(
                    child_content,
                    state.get_halign(),
                    col_widths[x],
                    filler_symbol.clone(),
                    fg_color,
                    bg_color,
                );
                state.get_position_mut().set_x(write_pos.x + offset);

                let (child_content, offset) = align_content_vertically(
                    child_content,
                    state.get_valign(),
                    row_heights[y],
                    filler_symbol.clone(),
                    fg_color,
                    bg_color,
                );
                state.get_position_mut().set_y(write_pos.y + offset);

                for child_x in 0..col_widths[x] {
                    for child_y in 0..row_heights[y] {
                        if child_x < child_content.len() && child_y < child_content[child_x].len() {
                            content[write_pos.x + child_x][write_pos.y + child_y] =
                                child_content[child_x][child_y].clone();
                        }
                    }
                }
                write_pos.y += row_heights[y];
            }
            write_pos.x += col_widths[x];
        }
        content
    }
}
