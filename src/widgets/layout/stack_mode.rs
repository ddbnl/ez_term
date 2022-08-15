use crate::GenericState;
use crate::run::definitions::{Coordinates, Pixel, PixelMap, Size, StateTree};
use crate::states::definitions::{AutoScale, ColorConfig, InfiniteSize, LayoutOrientation,
                                 ScrollingConfig};
use crate::widgets::ez_object::EzObject;
use crate::widgets::layout::layout::Layout;

// Box mode implementations
impl Layout {
    /// Used by [get_contents] when the [LayoutMode] is set to [Box] and [LayoutOrientation] is
    /// set to [Horizontal]. Merges contents of sub layouts and/or widgets horizontally, using
    /// own [height] for each.
    pub fn get_stack_mode_contents(&self, state_tree: &mut StateTree) -> PixelMap {

        let state = state_tree.get_mut(&self.get_path()).as_layout();

        let own_orientation = state.get_orientation().clone();
        let own_auto_scaling = state.get_auto_scale().clone();
        if own_orientation == LayoutOrientation::Vertical ||
            own_orientation == LayoutOrientation::Horizontal {
            panic!("Error in layout: {}. When in table mode, orientation must be one of: \
            ‘lr-tb’, ‘tb-lr’, ‘rl-tb’, ‘tb-rl’, ‘lr-bt’, ‘bt-lr’, ‘rl-bt’ and ‘bt-rl’.",
                   self.id)
        }

        let own_effective_size = state.get_effective_size();
        let own_infinite_size = state.get_infinite_size().clone();
        let own_colors = state.get_color_config().clone();
        let own_scrolling = state.get_scrolling_config().clone();
        let own_fill = state.get_fill();
        let own_filler_symbol = state.get_filler_symbol();

        let content_list = self.get_stack_mode_child_content(
            state_tree, &own_infinite_size, &own_scrolling, &own_effective_size);
        self.get_orientated_content(own_orientation, state_tree, &content_list,
                                    &own_effective_size, &own_colors, &own_auto_scaling,
                                    own_fill, own_filler_symbol)
    }

    fn get_orientated_content(&self, orientation: LayoutOrientation, state_tree: &mut StateTree,
                              content_list: &[PixelMap], effective_size: &Size,
                              colors: &ColorConfig, auto_scaling: &AutoScale,
                              fill: bool, filler_symbol: String) -> PixelMap {

        match orientation {
            LayoutOrientation::LeftRightTopBottom =>
                self.get_left_right_top_bottom_content(state_tree, content_list, effective_size,
                                                       colors, auto_scaling, fill, filler_symbol),
            LayoutOrientation::LeftRightBottomTop =>
                self.get_left_right_bottom_top_content(state_tree, content_list, effective_size,
                                                       colors, auto_scaling, fill ,filler_symbol),
            LayoutOrientation::RightLeftTopBottom =>
                self.get_right_left_top_bottom_content(state_tree, content_list, effective_size,
                                                       colors, auto_scaling, fill, filler_symbol),
            LayoutOrientation::RightLeftBottomTop =>
                self.get_right_left_bottom_top_content(state_tree, content_list, effective_size,
                                                       colors, auto_scaling, fill, filler_symbol),
            LayoutOrientation::TopBottomLeftRight =>
                self.top_bottom_left_right_content(state_tree, content_list, effective_size,
                                                   colors, auto_scaling, fill, filler_symbol),
            LayoutOrientation::TopBottomRightLeft =>
                self.top_bottom_right_left_content(state_tree, content_list, effective_size,
                                                   colors, auto_scaling, fill, filler_symbol),
            LayoutOrientation::BottomTopLeftRight =>
                self.bottom_top_left_right_content(state_tree, content_list, effective_size,
                                                   colors, auto_scaling, fill, filler_symbol),
            LayoutOrientation::BottomTopRightLeft =>
                self.bottom_top_right_left_content(state_tree, content_list, effective_size,
                                                   colors, auto_scaling, fill, filler_symbol),
            _ => panic!("Invalid orientation for stack"),
        }
    }

    /// Get the content of each child
    fn get_stack_mode_child_content(&self, state_tree: &mut StateTree, infinite_size: &InfiniteSize,
                                    scrolling_config: &ScrollingConfig, effective_size: &Size)
                                    -> Vec<PixelMap> {

        let mut content_list = Vec::new();
        for child in self.get_children() {
            let generic_child = child.as_ez_object();
            let state = state_tree
                .get_mut(&generic_child.get_path().clone()).as_generic_mut();

            if infinite_size.get_width() || scrolling_config.get_scroll_x() {
                state.get_infinite_size_mut().set_width(true);
            }
            if infinite_size.get_height() || scrolling_config.get_scroll_y() {
                state.get_infinite_size_mut().set_height(true);
            }

            // If autoscaling is enabled set child size to max width. It is then expected to scale
            // itself according to its' content
            if state.get_auto_scale().get_auto_scale_width() {
                state.get_size_mut().set_width(effective_size.width + 1);
            }
            if state.get_auto_scale().get_auto_scale_height() {
                state.get_size_mut().set_height(effective_size.height + 1);
            }

            let child_content = generic_child.get_contents(state_tree);
            if child_content.is_empty() { continue }  // handle empty widget
            let state =
                state_tree.get_mut(&generic_child.get_path()).as_generic_mut(); // re-borrow
            if state.get_infinite_size().width {
                state.get_size_mut().set_width(child_content.len())
            }
            if state.get_infinite_size().height {
                state.get_size_mut().set_height(child_content[0].len())
            }
            content_list.push(child_content);
        }
        content_list
    }

    fn get_left_right_top_bottom_content(
        &self, state_tree: &mut StateTree, content_list: &[PixelMap],
        effective_size: &Size, colors: &ColorConfig, auto_scaling: &AutoScale, filler: bool,
        filler_symbol: String) -> PixelMap {

        let mut merged_content = self.get_filler_content(
            effective_size, colors, filler, filler_symbol);

        let (mut largest_x, mut largest_y) = (0, 0);
        let mut pos = Coordinates::default();
        for (i, content) in content_list.iter().enumerate() {
            if content.len() > effective_size.width + 1 { break }
            if content.len() > effective_size.width - pos.x {
                if pos.y + largest_y >= effective_size.height { break }
                pos.y = largest_y + 1;
                let largest = content.iter().map(|x| x.len()).max().unwrap();
                if largest > effective_size.height - pos.y { break }
                pos.x = 0;
            }

            let state = state_tree.get_mut(
                &self.children[i].as_ez_object().get_path()).as_generic_mut();
            state.set_x(pos.x);
            state.set_y(pos.y);
            for child_x in 0..content.len() {
                for child_y in 0..content[child_x].len() {
                    if pos.y + child_y >= effective_size.height { continue }
                    if pos.x + child_x > largest_x { largest_x = pos.x + child_x }
                    if pos.y + child_y > largest_y { largest_y = pos.y + child_y }
                    merged_content[pos.x + child_x][pos.y + child_y] =
                        content[child_x][child_y].clone();
                }
            }
            pos.x += content.len();
        }

        if auto_scaling.get_auto_scale_width() {
            merged_content = merged_content[0..=largest_x].to_vec();
        }
        if auto_scaling.get_auto_scale_height() {
            merged_content = merged_content.iter()
                .map(|x| x[0..=largest_y].to_vec()).collect();
        }
        merged_content
    }

    fn get_left_right_bottom_top_content(
        &self, state_tree: &mut StateTree, content_list: &[PixelMap],
        effective_size: &Size, colors: &ColorConfig, auto_scaling: &AutoScale, filler: bool,
        filler_symbol: String) -> PixelMap {

        let mut merged_content = self.get_filler_content(
            effective_size, colors, filler, filler_symbol);

        let (mut largest_x, mut smallest_y) = (0, effective_size.height - 1);
        let mut pos = Coordinates::new(0, effective_size.height - 1);
        for (i, content) in content_list.iter().enumerate() {
            if content.len() > effective_size.width + 1 { break }
            if i == 0 || content.len() > effective_size.width - pos.x {
                let largest = content.iter().map(|x| x.len()).max().unwrap();
                if pos.y < largest { break }
                if pos.y == effective_size.height - 1 { pos.y -= largest - 1 }
                    else { pos.y -= largest };
                pos.x = 0;
            }

            let state = state_tree.get_mut(
                &self.children[i].as_ez_object().get_path()).as_generic_mut();
            state.set_x(pos.x);
            state.set_y(pos.y);
            for child_x in 0..content.len() {
                for child_y in 0..content[child_x].len() {
                    if pos.y + child_y > effective_size.height { continue }
                    if pos.x + child_x > largest_x { largest_x = pos.x + child_x }
                    if pos.y + child_y < smallest_y { smallest_y = pos.y + child_y }
                    merged_content[pos.x + child_x][pos.y + child_y] =
                        content[child_x][child_y].clone();
                }
            }
            pos.x += content.len();
        }
        if auto_scaling.get_auto_scale_width() {
            merged_content = merged_content[0..=largest_x].to_vec();
        }
        if auto_scaling.get_auto_scale_height() {
            merged_content = merged_content.iter()
                .map(|x| x[smallest_y..].to_vec()).collect();
        }
        merged_content
    }

    fn get_right_left_top_bottom_content(
        &self, state_tree: &mut StateTree, content_list: &[PixelMap],
        effective_size: &Size, colors: &ColorConfig, auto_scaling: &AutoScale, filler: bool,
        filler_symbol: String) -> PixelMap {

        let mut merged_content = self.get_filler_content(
            effective_size, colors, filler, filler_symbol);

        let (mut smallest_x, mut largest_y) = (effective_size.width - 1, 0);
        let mut pos = Coordinates::new(effective_size.width - 1, 0);
        for (i, content) in content_list.iter().enumerate() {
            if content.len() > effective_size.width + 1 { break }
            if content.len() > pos.x {
                if pos.y + largest_y >= effective_size.height { break }
                pos.y = largest_y + 1;
                let largest = content.iter().map(|x| x.len()).max().unwrap();
                if largest > effective_size.height - pos.y { break }
                pos.x = effective_size.width - 1;
            }
            pos.x -= content.len();

            let state = state_tree.get_mut(
                &self.children[i].as_ez_object().get_path()).as_generic_mut();
            state.set_x(pos.x);
            state.set_y(pos.y);
            for child_x in 0..content.len() {
                for child_y in 0..content[child_x].len() {
                    if pos.y + child_y >= effective_size.height { continue }
                    if pos.x + child_x < smallest_x { smallest_x = pos.x + child_x }
                    if pos.y + child_y > largest_y { largest_y = pos.y + child_y }
                    merged_content[pos.x + child_x][pos.y + child_y] =
                        content[child_x][child_y].clone();
                }
            }
        }
        if auto_scaling.get_auto_scale_width() {
            merged_content = merged_content[smallest_x..].to_vec();
        }
        if auto_scaling.get_auto_scale_height() {
            merged_content = merged_content.iter()
                .map(|x| x[0..=largest_y].to_vec()).collect();
        }
        merged_content
    }

    fn get_right_left_bottom_top_content(
        &self, state_tree: &mut StateTree, content_list: &[PixelMap],
        effective_size: &Size, colors: &ColorConfig, auto_scaling: &AutoScale, filler: bool,
        filler_symbol: String) -> PixelMap {

        let mut merged_content = self.get_filler_content(
            effective_size, colors, filler, filler_symbol);

        let (mut smallest_x, mut smallest_y) =
            (effective_size.width - 1, effective_size.height - 1);
        let mut pos =
            Coordinates::new(effective_size.width - 1, effective_size.height - 1);
        for (i, content) in content_list.iter().enumerate() {
            if content.len() > effective_size.width + 1 { break }
            if i == 0 || content.len() > pos.x {
                let largest = content.iter().map(|x| x.len()).max().unwrap();
                if pos.y < largest { break }
                if pos.y == effective_size.height - 1 { pos.y -= largest - 1 }
                    else { pos.y -= largest };
                pos.x = effective_size.width - 1;
            }

            pos.x -= content.len();

            let state = state_tree.get_mut(
                &self.children[i].as_ez_object().get_path()).as_generic_mut();
            state.set_x(pos.x);
            state.set_y(pos.y);
            for child_x in 0..content.len() {
                for child_y in 0..content[child_x].len() {
                    if pos.y + child_y >= effective_size.height { continue }
                    if pos.x + child_x < smallest_x { smallest_x = pos.x + child_x }
                    if pos.y + child_y < smallest_y { smallest_y = pos.y + child_y }
                    merged_content[pos.x + child_x][pos.y + child_y] =
                        content[child_x][child_y].clone();
                }
            }
        }
        if auto_scaling.get_auto_scale_width() {
            merged_content = merged_content[smallest_x..].to_vec();
        }
        if auto_scaling.get_auto_scale_height() {
            merged_content = merged_content.iter()
                .map(|x| x[smallest_y..].to_vec()).collect();
        }
        merged_content
    }

    fn top_bottom_left_right_content(
        &self, state_tree: &mut StateTree, content_list: &[PixelMap],
        effective_size: &Size, colors: &ColorConfig, auto_scaling: &AutoScale, filler: bool,
        filler_symbol: String) -> PixelMap {

        let mut merged_content = self.get_filler_content(
            effective_size, colors, filler, filler_symbol);

        let (mut largest_x, mut largest_y) = (0, 0);
        let mut pos = Coordinates::default();
        for (i, content) in content_list.iter().enumerate() {
            let largest = content.iter().map(|x| x.len()).max().unwrap();
            if largest > effective_size.height + 1 { break }
            if largest > effective_size.height - pos.y {
                if pos.x + largest_x >= effective_size.width { break }
                pos.x = largest_x + 1;
                if content.len() > effective_size.width - pos.x { break }
                pos.y = 0;
            }

            let state = state_tree.get_mut(
                &self.children[i].as_ez_object().get_path()).as_generic_mut();
            state.set_x(pos.x);
            state.set_y(pos.y);
            for child_x in 0..content.len() {
                for child_y in 0..content[child_x].len() {
                    if pos.x + child_x >= effective_size.width { continue }
                    if pos.x + child_x > largest_x { largest_x = pos.x + child_x }
                    if pos.y + child_y > largest_y { largest_y = pos.y + child_y }
                    merged_content[pos.x + child_x][pos.y + child_y] =
                        content[child_x][child_y].clone();
                }
            }
            pos.y += largest;
        }

        if auto_scaling.get_auto_scale_width() {
            merged_content = merged_content[0..=largest_x].to_vec();
        }
        if auto_scaling.get_auto_scale_height() {
            merged_content = merged_content.iter()
                .map(|x| x[0..=largest_y].to_vec()).collect();
        }
        merged_content
    }

    fn top_bottom_right_left_content(
        &self, state_tree: &mut StateTree, content_list: &[PixelMap],
        effective_size: &Size, colors: &ColorConfig, auto_scaling: &AutoScale, filler: bool,
        filler_symbol: String) -> PixelMap {

        let mut merged_content = self.get_filler_content(
            effective_size, colors, filler, filler_symbol);

        let (mut smallest_x, mut largest_y) = (effective_size.width - 1, 0);
        let mut pos = Coordinates::new(effective_size.width - 1, 0);
        for (i, content) in content_list.iter().enumerate() {
            let largest = content.iter().map(|x| x.len()).max().unwrap();
            if largest > effective_size.height { break }
            if i == 0 || largest > effective_size.height - pos.y {
                if content.len() >= pos.x { break }
                pos.x -= content.len();
                pos.y = 0;
            }

            let state = state_tree.get_mut(
                &self.children[i].as_ez_object().get_path()).as_generic_mut();
            state.set_x(pos.x);
            state.set_y(pos.y);
            for child_x in 0..content.len() {
                for child_y in 0..content[child_x].len() {
                    if pos.x + child_x >= effective_size.width ||
                        pos.y + child_y >= effective_size.height { continue }
                    if pos.x + child_x < smallest_x { smallest_x = pos.x + child_x }
                    if pos.y + child_y > largest_y { largest_y = pos.y + child_y }
                    merged_content[pos.x + child_x][pos.y + child_y] =
                        content[child_x][child_y].clone();
                }
            }
            pos.y += largest;
        }

        if auto_scaling.get_auto_scale_width() {
            merged_content = merged_content[smallest_x..].to_vec();
        }
        if auto_scaling.get_auto_scale_height() {
            merged_content = merged_content.iter()
                .map(|x| x[0..=largest_y].to_vec()).collect();
        }
        merged_content
    }

    fn bottom_top_left_right_content(
        &self, state_tree: &mut StateTree, content_list: &[PixelMap],
        effective_size: &Size, colors: &ColorConfig, auto_scaling: &AutoScale, filler: bool,
        filler_symbol: String) -> PixelMap {

        let mut merged_content = self.get_filler_content(
            effective_size, colors, filler, filler_symbol);

        let (mut largest_x, mut smallest_y) = (0, effective_size.height - 1);
        let mut pos = Coordinates::new(0,effective_size.height - 1);
        for (i, content) in content_list.iter().enumerate() {
            let largest = content.iter().map(|x| x.len()).max().unwrap();
            if largest > pos.y + 1 {
                if content.len() > effective_size.width - pos.x { break }
                pos.x = content.len();
                pos.y = effective_size.height - 1;
            }
            if pos.y == effective_size.height - 1 {
                pos.y -= largest - 1
            } else {
                pos.y -= largest
            };

            let state = state_tree.get_mut(
                &self.children[i].as_ez_object().get_path()).as_generic_mut();
            state.set_x(pos.x);
            state.set_y(pos.y);
            for child_x in 0..content.len() {
                for child_y in 0..content[child_x].len() {
                    if pos.x + child_x >= effective_size.width ||
                        pos.y + child_y >= effective_size.height { continue }
                    if pos.x + child_x > largest_x { largest_x = pos.x + child_x }
                    if pos.y + child_y < smallest_y { smallest_y = pos.y + child_y }
                    merged_content[pos.x + child_x][pos.y + child_y] =
                        content[child_x][child_y].clone();
                }
            }
        }

        if auto_scaling.get_auto_scale_width() {
            merged_content = merged_content[0..=largest_x].to_vec();
        }
        if auto_scaling.get_auto_scale_height() {
            merged_content = merged_content.iter()
                .map(|x| x[smallest_y..].to_vec()).collect();
        }
        merged_content
    }
    fn bottom_top_right_left_content(
        &self, state_tree: &mut StateTree, content_list: &[PixelMap],
        effective_size: &Size, colors: &ColorConfig, auto_scaling: &AutoScale, filler: bool,
        filler_symbol: String) -> PixelMap {

        let mut merged_content = self.get_filler_content(
            effective_size, colors, filler, filler_symbol);

        let (mut smallest_x, mut smallest_y) = (effective_size.width - 1,
                                                effective_size.height - 1);
        let mut pos = Coordinates::new(effective_size.width - 1,
                                                  effective_size.height - 1);
        for (i, content) in content_list.iter().enumerate() {
            let largest = content.iter().map(|x| x.len()).max().unwrap();
            if largest > effective_size.height { break }
            if i == 0 || largest > pos.y + 1 {
                if content.len() >= pos.x { break }
                pos.x -= content.len();
                pos.y = effective_size.height - 1;
            }
            if pos.y == effective_size.height - 1 {
                pos.y -= largest - 1
            } else {
                pos.y -= largest
            };

            let state = state_tree.get_mut(
                &self.children[i].as_ez_object().get_path()).as_generic_mut();
            state.set_x(pos.x);
            state.set_y(pos.y);
            for child_x in 0..content.len() {
                for child_y in 0..content[child_x].len() {
                    if pos.x + child_x >= effective_size.width ||
                        pos.y + child_y >= effective_size.height { continue }
                    if pos.x + child_x < smallest_x { smallest_x = pos.x + child_x }
                    if pos.y + child_y < smallest_y { smallest_y = pos.y + child_y }
                    merged_content[pos.x + child_x][pos.y + child_y] =
                        content[child_x][child_y].clone();
                }
            }
        }

        if auto_scaling.get_auto_scale_width() {
            merged_content = merged_content[smallest_x..].to_vec();
        }
        if auto_scaling.get_auto_scale_height() {
            merged_content = merged_content.iter()
                .map(|x| x[smallest_y..].to_vec()).collect();
        }
        merged_content
    }

    fn get_filler_content(&self, effective_size: &Size, colors: &ColorConfig, fill: bool,
        filler_symbol: String) -> PixelMap {

        let (symbol, fg_color, bg_color) =
            if fill {
                (filler_symbol, colors.get_filler_fg_color(),
                 colors.get_filler_bg_color())
            } else {
                (" ".to_string(), colors.get_fg_color(), colors.get_bg_color())
            };

        let merged_content = vec!(
            vec!(Pixel::new(symbol, fg_color, bg_color);
                 effective_size.height); effective_size.width);

        merged_content

    }
}
