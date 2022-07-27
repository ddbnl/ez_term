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
        let own_state = state_tree.get_by_path(&self.get_full_path()).as_layout();
        let own_height = own_state.get_effective_size().height;
        let own_width = own_state.get_effective_size().width;


        // Fill self with background first. Then overlay widgets.
        let filler = Pixel::new(own_state.get_filler_symbol().value.clone(),
                                own_state.get_color_config().filler_foreground.value,
                                own_state.get_color_config().filler_background.value);
        for _ in 0..own_width {
            content.push(Vec::new());
            for _ in 0..own_height {
                if own_state.get_fill().value {
                    content.last_mut().unwrap().push(filler.clone());
                } else {
                    content.last_mut().unwrap().push(
                        Pixel::new(
                            " ".to_string(),
                            own_state.get_color_config().foreground.value,
                            own_state.get_color_config().background.value));
                }
            }
        }
        for child in self.get_children() {
            if content.is_empty() { return content }  // No space left in widget

            let generic_child = child.as_ez_object();
            let state = state_tree.get_by_path_mut(
                &generic_child.get_full_path()).as_generic_mut();

            // If autoscaling is enabled set child size to max width. It is then expected to scale
            // itself according to its' content
            if state.get_auto_scale().width.value {
                state.get_size_mut().width.set(own_width)
            }
            if state.get_auto_scale().height.value {
                state.get_size_mut().height.set(own_height)
            }
            // Scale down child to remaining size in the case that the child is too large, rather
            // panicking.
            if state.get_size().height > own_height {
                state.get_size_mut().height.set(own_height);
            }
            if state.get_size().width > own_width {
                state.get_size_mut().width.set(own_width);
            }

            let child_content = generic_child.get_contents(state_tree);
            let state = state_tree.get_by_path_mut(
                &generic_child.get_full_path()).as_generic_mut(); // re-borrow
            reposition_with_pos_hint(own_width, own_height, state);
            let child_pos = state.get_position();
            for width in 0..child_content.len() {
                for height in 0..child_content[width].len() {
                    if child_pos.x.get() + width < content.len()
                        && child_pos.y.get() + height < content[child_pos.x.get() + width].len() {
                        content[child_pos.x.get() + width][child_pos.y.get() + height] =
                            child_content[width][height].clone();
                    }
                }
            }
        }
        content
    }
}
