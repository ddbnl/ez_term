use crate::GenericState;
use crate::run::definitions::{Coordinates, Pixel, PixelMap, StateTree};
use crate::states::definitions::{ColorConfig, LayoutOrientation};
use crate::widgets::helper_functions::{align_content_horizontally, align_content_vertically};
use crate::widgets::ez_object::EzObject;
use crate::widgets::layout::layout::Layout;

// Box mode implementations
impl Layout{

    /// Returns [get_box_mode_horizontal_orientation_contents] or
    /// [get_box_mode_vertical_orientation_contents] depending on orientation
    pub fn get_box_mode_contents(&self, state_tree: &mut StateTree) -> PixelMap {

        match state_tree.get_by_path(&self.path).as_layout().get_orientation() {
            LayoutOrientation::Horizontal => {
                self.get_box_mode_horizontal_orientation_contents(state_tree)
            },
            LayoutOrientation::Vertical => {
                self.get_box_mode_vertical_orientation_contents(state_tree)
            },
            _ => panic!("Error in layout: {}, mode \"Box\" requires orientation \
                        \"Horizontal\" or \"Vertical\"", self.id),
        }
    }

    /// Used by [get_contents] when the [LayoutMode] is set to [Box] and [LayoutOrientation] is
    /// set to [Horizontal]. Merges contents of sub layouts and/or widgets horizontally, using
    /// own [height] for each.
    pub fn get_box_mode_horizontal_orientation_contents(&self, state_tree: &mut StateTree)
        -> PixelMap {

        let state = state_tree.get_by_path_mut(&self.get_full_path()).as_layout();
        let own_effective_size = state.get_effective_size();
        let own_infinite_size = state.get_infinite_size().clone();
        let own_colors = state.get_color_config().clone();
        let own_scrolling = state.get_scrolling_config().clone();

        let mut position = Coordinates::new(0, 0);
        let mut content_list = Vec::new();
        for child in self.get_children() {

            let generic_child = child.as_ez_object();
            let state = state_tree
                .get_by_path_mut(&generic_child.get_full_path().clone()).as_generic_mut();

            if own_infinite_size.width || own_scrolling.get_enable_x() {
                state.get_infinite_size_mut().set_width(true);
            }
            if own_infinite_size.height || own_scrolling.get_enable_y() {
                state.get_infinite_size_mut().set_height(true);
            }

            let width_left =
                if !own_scrolling.get_enable_x() && !own_infinite_size.width &&
                    !state.get_infinite_size().width && own_effective_size.width >= position.x
                    {own_effective_size.width - position.x} else {0};
            // If autoscaling is enabled set child size to max width. It is then expected to scale
            // itself according to its' content
            if state.get_auto_scale().get_auto_scale_width() {
                state.get_size_mut().set_width(width_left)
            }
            if state.get_auto_scale().get_auto_scale_height() {
                state.get_size_mut().set_height(own_effective_size.height)
            }
            // Scale down child to remaining size in the case that the child is too large, rather
            // panicking.
            if !own_scrolling.get_enable_x() && !own_infinite_size.width &&
                state.get_size().get_width() > width_left {
                state.get_size_mut().set_width(width_left);
            }
            if !own_scrolling.get_enable_y() && !own_infinite_size.height &&
                state.get_size().get_height() > own_effective_size.height {
                state.get_size_mut().set_height(own_effective_size.height);
            }

            state.set_x(position.x);
            state.set_y(position.y);
            let child_content = generic_child.get_contents(state_tree);
            if child_content.is_empty() { continue }  // handle empty widget
            let state = state_tree.get_by_path_mut(&generic_child.get_full_path())
                .as_generic_mut(); // re-borrow
            if state.get_infinite_size().width {
                state.get_size_mut().set_width(child_content.len())
            }
            if state.get_infinite_size().height {
                state.get_size_mut().set_height(child_content[0].len())
            }

            position.x += child_content.len();
            content_list.push(child_content);
        }

        self.scale_to_largest_child(&content_list, state_tree);
        let own_effective_size = state_tree.get_by_path_mut(&self.get_full_path())
            .as_layout().get_effective_size();
        let mut merged_content = PixelMap::new();
        for (i, content) in content_list.into_iter().enumerate() {
            merged_content = self.merge_horizontal_contents(
                merged_content, content,
                own_effective_size.height,
                own_infinite_size.height,own_colors.clone(),
                state_tree.get_by_path_mut(
                    &self.children.get(i).unwrap().as_ez_object()
                    .get_full_path()).as_generic_mut());
        }
        merged_content
    }

    /// Used by [get_contents] when the [LayoutMode] is set to [Box] and [LayoutOrientation] is
    /// set to [Vertical]. Merges contents of sub layouts and/or widgets vertically.
    pub fn get_box_mode_vertical_orientation_contents(&self, state_tree: &mut StateTree) -> PixelMap {

        // Some clones as we will need to borrow from state tree again later

        let state = state_tree.get_by_path_mut(&self.get_full_path()).as_layout();
        let own_effective_size = state.get_effective_size();
        let own_infinite_size = state.get_infinite_size().clone();
        let own_colors = state.get_color_config().clone();
        let own_scrolling = state.get_scrolling_config().clone();

        let mut position = Coordinates::new(0, 0);
        let mut content_list = Vec::new();
        for child in self.get_children() {

            let generic_child = child.as_ez_object();
            let child_state =
                state_tree.get_by_path_mut(&generic_child.get_full_path()).as_generic_mut();

            // If we're scrolling on an axis then the child can be infinite size on that axis
            if own_infinite_size.width || own_scrolling.get_enable_x() {
                child_state.get_infinite_size_mut().set_width(true);
            }
            if own_infinite_size.height || own_scrolling.get_enable_y() {
                child_state.get_infinite_size_mut().set_height(true);
            }

            // Determine how much height we have left to give the child. Can be 0 if we're scrolling
            // as we use [size.get_infinite_height()]
            let height_left =
                if !own_scrolling.get_enable_y() && !own_infinite_size.height &&
                    own_effective_size.height >= position.y &&
                    !child_state.get_infinite_size().height
                {own_effective_size.height - position.y } else { 0 };
            // If autoscaling is enabled set child size to max width. It is then expected to scale
            // itself according to its' content
            if child_state.get_auto_scale().get_auto_scale_width() {
                child_state.get_size_mut().set_width(own_effective_size.width)
            }
            if child_state.get_auto_scale().get_auto_scale_height() {
                child_state.get_size_mut().set_height(height_left)
            }
            // Scale down child to remaining size in the case that the child is too large, rather
            // panicking.
            if !own_scrolling.get_enable_x() && !own_infinite_size.width &&
                !child_state.get_infinite_size().width &&
                child_state.get_size().get_width() > own_effective_size.width {
                child_state.get_size_mut().set_width(own_effective_size.width);
            }
            if !own_scrolling.get_enable_y() && !own_infinite_size.height &&
                !child_state.get_infinite_size().height &&
                child_state.get_size().get_height() > height_left {
                child_state.get_size_mut().set_height(height_left);
            }

            child_state.set_x(position.x);
            child_state.set_y(position.y);
            let child_content = generic_child.get_contents(state_tree);
            let state = state_tree.get_by_path_mut(&generic_child.get_full_path())
                .as_generic_mut(); // re-borrow
            if state.get_infinite_size().width {
                state.get_size_mut().set_width(child_content.len())
            }
            if state.get_infinite_size().height {
                state.get_size_mut().set_height(
                    if !child_content.is_empty() { child_content[0].len() } else { 0 })
            }
            position.y += if !child_content.is_empty() {child_content[0].len()} else {0};
            content_list.push(child_content);
        }
        self.scale_to_largest_child(&content_list, state_tree);
        let own_effective_size = state_tree.get_by_path_mut(&self.get_full_path())
            .as_layout().get_effective_size();
        let mut merged_content = PixelMap::new();
        for (i, content) in content_list.into_iter().enumerate() {
            merged_content = self.merge_vertical_contents(
                merged_content, content,
                own_effective_size.width, own_infinite_size.width,
                own_colors.clone(),
                state_tree.get_by_path_mut(
                    &self.children.get(i).unwrap().as_ez_object()
                    .get_full_path()).as_generic_mut());
        }
        merged_content
    }

    /// Take a [PixelMap] and merge it horizontally with another [PixelMap]
    pub fn merge_horizontal_contents(&self, mut merged_content: PixelMap, mut new: PixelMap,
                                     parent_height: usize, parent_infinite_height: bool,
                                     parent_colors: ColorConfig, state: &mut dyn GenericState)
                                     -> PixelMap {

        if !parent_infinite_height && parent_height > new[0].len() {
            let offset;
            (new, offset) = align_content_vertically(
                new, state.get_vertical_alignment(), parent_height,
                parent_colors.get_fg_color(),
                parent_colors.get_bg_color());
            state.set_y(state.get_position().get_y() + offset);
        }

        for x in 0..new.len() {
            merged_content.push(new[x].clone());
        }
        merged_content
    }

    /// Take a [PixelMap] and merge it vertically with another [PixelMap]
    pub fn merge_vertical_contents(
        &self, mut merged_content: PixelMap, mut new: PixelMap, parent_width: usize,
        parent_infinite_width: bool, parent_colors: ColorConfig, state: &mut dyn GenericState)
        -> PixelMap {

        if new.is_empty() {
            return merged_content
        }

        let offset;
        if parent_width > new.len() && !parent_infinite_width {
            (new, offset) = align_content_horizontally(
                new, state.get_horizontal_alignment(), parent_width,
                parent_colors.get_fg_color(),
                parent_colors.get_bg_color());
            state.set_x(state.get_position().get_x() + offset);
        }

        let write_width = if !state.get_infinite_size().width { parent_width }
                              else { new.len() };
        for x in 0..write_width {
            for y in 0..new[0].len() {
                if x >= merged_content.len() {
                    merged_content.push(Vec::new());
                }
                if x < new.len() && y < new[x].len() {
                    merged_content[x].push(new[x][y].clone())
                } else {
                    merged_content[x].push(Pixel { symbol: " ".to_string(),
                        foreground_color: parent_colors.get_fg_color(),
                        background_color: parent_colors.get_bg_color(),
                        underline: false});
                }
            }
        }

        merged_content
    }
}