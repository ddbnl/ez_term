use crate::run::definitions::{Pixel, PixelMap, StateTree};
use crate::widgets::helper_functions::reposition_with_pos_hint;
use crate::states::ez_state::GenericState;
use crate::widgets::ez_object::EzObject;
use crate::widgets::layout::layout::Layout;

impl Layout {
    /// Used by [get_contents] when the [LayoutMode] is set to [Float]. Places each child in the
    /// XY coordinates defined by that child, relative to itself, and uses
    /// childs' [width] and [height].
    pub fn get_float_mode_contents(&self, mut content: PixelMap, state_tree: &mut StateTree)
                               -> PixelMap {
        let own_state = state_tree.get(&self.get_path()).as_layout();
        let own_height = own_state.get_effective_size().height;
        let own_width = own_state.get_effective_size().width;


        let (filler_symbol, filler_fg_color, filler_bg_color) =
            if own_state.get_fill() {
                (own_state.get_filler_symbol(), own_state.colors.get_filler_fg_color(),
                 own_state.colors.get_filler_bg_color())
            } else {
                (" ".to_string(), own_state.colors.get_fg_color(), own_state.colors.get_bg_color())
            };
        // Fill self with background first. Then overlay widgets.
        let filler = Pixel::new(filler_symbol, filler_fg_color,
                                filler_bg_color);

        for _ in 0..own_width {
            content.push(Vec::new());
            for _ in 0..own_height {
                content.last_mut().unwrap().push(filler.clone());
            }
        }
        for child in self.get_children() {
            if content.is_empty() { return content }  // No space left in widget

            let generic_child = child.as_ez_object();
            let state = state_tree.get_mut(
                &generic_child.get_path()).as_generic_mut();

            // If autoscaling is enabled set child size to max width. It is then expected to scale
            // itself according to its' content
            if state.get_auto_scale().get_auto_scale_width() {
                state.get_size_mut().set_width(own_width)
            }
            if state.get_auto_scale().get_auto_scale_height() {
                state.get_size_mut().set_height(own_height)
            }
            // Scale down child to remaining size in the case that the child is too large, rather
            // panicking.
            if state.get_size().get_height() > own_height {
                state.get_size_mut().set_height(own_height);
            }
            if state.get_size().get_width() > own_width {
                state.get_size_mut().set_width(own_width);
            }

            let child_content = generic_child.get_contents(state_tree);
            let state = state_tree.get_mut(
                &generic_child.get_path()).as_generic_mut(); // re-borrow
            reposition_with_pos_hint(own_width, own_height, state);
            let child_pos = state.get_position();
            for width in 0..child_content.len() {
                for height in 0..child_content[width].len() {
                    if child_pos.get_x() + width < content.len()
                        && child_pos.get_y() + height < content[child_pos.get_x() + width].len() {
                        content[child_pos.get_x() + width][child_pos.get_y() + height] =
                            child_content[width][height].clone();
                    }
                }
            }
        }
        content
    }
}
