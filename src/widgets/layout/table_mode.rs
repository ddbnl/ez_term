use crate::GenericState;
use crate::run::definitions::{Coordinates, Pixel, PixelMap, Size, StateTree};
use crate::states::definitions::{ColorConfig, LayoutOrientation, ScrollingConfig, StateSize, TableConfig};
use crate::widgets::helper_functions::{align_content_horizontally, align_content_vertically};
use crate::widgets::ez_object::EzObject;
use crate::widgets::layout::layout::Layout;

// Box mode implementations
impl Layout{

    /// Used by [get_contents] when the [LayoutMode] is set to [Box] and [LayoutOrientation] is
    /// set to [Horizontal]. Merges contents of sub layouts and/or widgets horizontally, using
    /// own [height] for each.
    pub fn get_table_mode_contents(&self, state_tree: &mut StateTree) -> PixelMap {

        let state = state_tree.get_by_path_mut(&self.get_full_path()).as_layout();

        let own_table_config = state.get_table_config().clone();
        let own_orientation = state.orientation.clone();
        if own_orientation == LayoutOrientation::Vertical ||
            own_orientation == LayoutOrientation::Horizontal {
            panic!("Error in layout: {}. When in table mode, orientation must be one of: \
            ‘lr-tb’, ‘tb-lr’, ‘rl-tb’, ‘tb-rl’, ‘lr-bt’, ‘bt-lr’, ‘rl-bt’ and ‘bt-rl’.",
            self.id)
        }
        if own_table_config.rows == 0 && own_table_config.columns == 0 {
            panic!("Error in layout: {}. When in table mode, rows or columns (or both) should be \
            constrained. Set the \"rows\" or \"cols\" property to a value above 0.", self.id);
        }

        let own_effective_size = state.get_effective_size();
        let own_size = state.get_size().clone();
        let own_colors = state.get_color_config().clone();
        let own_scrolling = state.get_scrolling_config().clone();

        let (child_table, rows, cols) =
            self.get_table_model(&own_table_config, own_orientation);

        // Initial cell size. Can change if force_default is false, in which case it will scale to
        // size of largest child
        let default_height =
            if own_table_config.default_height == 0 { own_effective_size.height / rows }
            else { own_table_config.default_height.value };
        let default_width =
            if own_table_config.default_width == 0 { own_effective_size.width / cols }
            else { own_table_config.default_width.value };

        let content_list = self.get_table_mode_child_content(
            state_tree, &own_size, &own_scrolling, &own_effective_size);

        self.draw_table(&child_table, content_list, rows, cols, &own_table_config, default_width,
                        default_height, &own_colors, state_tree)
    }

    /// Calculate the amount of needed rows and columns based on the number of children
    fn get_rows_and_cols(&self, table_config: &TableConfig) -> (usize, usize){

        let (rows, cols);
        if table_config.rows > 0 && table_config.columns > 0 {
            rows = table_config.rows.value;
            cols = table_config.columns.value;
        }
        else if table_config.rows > 0 {
            rows = table_config.rows.value;
            cols =  (self.children.len() / rows) as usize +
                if self.children.len() % rows != 0 { 1 } else { 0 };
        } else {
            cols = table_config.columns.value;
            rows =  (self.children.len() / cols) as usize +
                if self.children.len() % cols != 0 { 1 } else { 0 };
        }
        (rows, cols)
    }

    /// Create a model of the order in which children should be added, by creating a table of
    /// child index. For example, a table layout with 9 children and orientation 'rl-tb' will return
    /// 2 1 0
    /// 5 4 3
    /// 8 7 6
    fn get_table_model(&self, table_config: &TableConfig, orientation: LayoutOrientation)
                       -> (Vec<Vec<usize>>, usize, usize) {

        let (rows, cols) = self.get_rows_and_cols(table_config);
        let mut children_table = vec![vec![0; rows]; cols];

        match orientation {
            LayoutOrientation::LeftRightTopBottom => {
                let (mut x, mut y) = (0, 0);
                for i in 0..self.children.len() {
                    if x == cols {
                        x = 0;
                        y += 1;
                    }
                    children_table[x][y] = i;
                    x += 1;
                }
            }
            LayoutOrientation::LeftRightBottomTop => {
                let (mut x, mut y) = (0, rows-1);
                for i in 0..self.children.len() {
                    if x == cols {
                        x = 0;
                        y -= 1;
                    }
                    children_table[x][y] = i;
                    x += 1;
                }
            }
            LayoutOrientation::RightLeftTopBottom => {
                let (mut x, mut y) = (cols-1, 0);
                for i in 0..self.children.len() {
                    children_table[x][y] = i;
                    if i == self.children.len() - 1 { break }
                    if x == 0 {
                        x = cols-1;
                        y += 1;
                    } else {
                        x -= 1;
                    }
                }
            }
            LayoutOrientation::RightLeftBottomTop => {
                let (mut x, mut y) = (cols-1, rows-1);
                for i in 0..self.children.len() {
                    children_table[x][y] = i;
                    if i == self.children.len() - 1 { break }
                    if x == 0 {
                        x = cols-1;
                        y -= 1;
                    } else {
                        x -= 1;
                    }
                }
            }
            LayoutOrientation::TopBottomLeftRight => {
                let (mut x, mut y) = (0, 0);
                for i in 0..self.children.len() {
                    if y == rows {
                        y = 0;
                        x += 1;
                    }
                    children_table[x][y] = i;
                    y += 1;
                }
            }
            LayoutOrientation::TopBottomRightLeft => {
                let (mut x, mut y) = (cols-1, 0);
                for i in 0..self.children.len() {
                    if y == rows {
                        y = 0;
                        x -= 1;
                    }
                    children_table[x][y] = i;
                    y += 1;
                }
            }
            LayoutOrientation::BottomTopLeftRight => {
                let (mut x, mut y) = (0, rows-1);
                for i in 0..self.children.len() {
                    children_table[x][y] = i;
                    if i == self.children.len() - 1 { break }
                    if y == 0 {
                        y = rows-1;
                        x += 1;
                    } else {
                        y -= 1;
                    }
                }
            }
            LayoutOrientation::BottomTopRightLeft => {
                let (mut x, mut y) = (cols-1, rows-1);
                for i in 0..self.children.len() {
                    children_table[x][y] = i;
                    if i == self.children.len() - 1 { break }
                    if y == 0 {
                        y = rows-1;
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
    fn get_row_heights(&self, table_config: &TableConfig, rows: usize, cols: usize,
                       child_table: &Vec<Vec<usize>>, content_list: &Vec<PixelMap>,
                       default_height: usize) -> Vec<usize> {

        let mut row_heights = Vec::new();
        if !table_config.force_default_height.value {
            for y in 0..rows {
                let mut largest = 0;
                for x in 0..cols {
                    if x < child_table.len() && y < child_table[x].len() {
                        let height =
                            content_list.get(child_table[x][y]).unwrap().iter()
                                .map(|x| x.len()).max().unwrap();
                        if height > largest { largest = height };
                    }
                }
                if table_config.default_height > 0 &&
                    largest < table_config.default_height.value {
                    largest = table_config.default_height.value
                }
                row_heights.push(largest);
            }
        } else {
            for _ in 0..rows { row_heights.push(default_height) }
        }
        row_heights
    }

    /// Calculate how wide each column should be. if force_default is true, use the default.
    /// Otherwise find the widest child.
    fn get_col_widths(&self, table_config: &TableConfig, rows: usize, cols: usize,
                      child_table: &Vec<Vec<usize>>, content_list: &[PixelMap],
                      default_width: usize) -> Vec<usize> {

        let mut col_widths = Vec::new();
        if !table_config.force_default_width.value {
            for x in 0..cols {
                let mut largest = 0;
                for y in 0..rows {
                    if x < child_table.len() && y < child_table[x].len() {
                        let width = content_list.get(child_table[x][y]).unwrap().len();
                        if width > largest { largest = width };
                    }
                }
                if table_config.default_width > 0 &&
                    largest < table_config.default_width.value {
                    largest = table_config.default_width.value
                }
                col_widths.push(largest);

            }
        } else {
            for _ in 0..cols { col_widths.push(default_width) }
        }
        col_widths
    }

    /// Get the content of each child, so we can draw it in a table after.
    fn get_table_mode_child_content(&self, state_tree: &mut StateTree, size: &StateSize,
                                    scrolling_config: &ScrollingConfig, effective_size: &Size)
                                    -> Vec<PixelMap> {

        let mut content_list = Vec::new();
        for child in self.get_children() {

            let generic_child = child.as_ez_object();
            let state = state_tree
                .get_by_path_mut(&generic_child.get_full_path().clone()).as_generic_mut();

            if size.infinite_width || scrolling_config.enable_x.value {
                state.get_size_mut().infinite_width = true;
            }
            if size.infinite_height || scrolling_config.enable_y.value {
                state.get_size_mut().infinite_height = true;
            }

            // If autoscaling is enabled set child size to max width. It is then expected to scale
            // itself according to its' content
            if state.get_auto_scale().width.value {
                state.get_size_mut().width.set(effective_size.width)
            }
            if state.get_auto_scale().height.value {
                state.get_size_mut().height.set(effective_size.height)
            }

            let child_content = generic_child.get_contents(state_tree);
            if child_content.is_empty() { continue }  // handle empty widget
            let state = state_tree.get_by_path_mut(&generic_child.get_full_path())
                .as_generic_mut(); // re-borrow
            if state.get_size().infinite_width {
                state.get_size_mut().width.set(child_content.len())
            }
            if state.get_size().infinite_height {
                state.get_size_mut().height.set(child_content[0].len())
            }
            content_list.push(child_content);
        }
        content_list
    }

    /// Draw a list of child content in a table using the child_table as a model for the
    /// orientation.
    fn draw_table(&self, child_table: &Vec<Vec<usize>>, mut content_list: Vec<PixelMap>,
                  rows: usize, cols:usize, table_config: &TableConfig, default_width: usize,
                  default_height: usize, colors: &ColorConfig, state_tree: &mut StateTree)
        -> PixelMap {

        let row_heights = self.get_row_heights(
            table_config, rows, cols, child_table, &content_list, default_height);
        let col_widths = self.get_col_widths(
            table_config, rows, cols, child_table, &content_list, default_width);
        let total_height: usize = row_heights.iter().sum();
        let total_width: usize = col_widths.iter().sum();
        let mut content = vec!(
            vec!(Pixel::new(" ".to_string(), colors.foreground.value,
                            colors.background.value); total_height); total_width);

        let mut write_pos = Coordinates::new(0, 0);
        for x in 0..cols {
            write_pos.y = 0;
            for y in 0..rows {
                if x >= child_table.len() || y >= child_table[x].len() { continue }
                let state = state_tree.get_by_path_mut(
                    &self.children.get(child_table[x][y]).unwrap().as_ez_object()
                        .get_full_path()).as_generic_mut();
                let child_content =
                    std::mem::take(&mut content_list[child_table[x][y]]);

                let (child_content, offset) = align_content_horizontally(
                    child_content,
                    state.get_horizontal_alignment().value, row_heights[y],
                    colors.foreground.value,colors.background.value);
                state.get_position_mut().x.set(write_pos.x + offset);

                let (child_content, offset) = align_content_vertically(
                    child_content,
                    state.get_vertical_alignment().value, row_heights[y],
                    colors.foreground.value,colors.background.value);
                state.get_position_mut().y.set(write_pos.y + offset);

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