use crate::run::definitions::{PixelMap, StateTree};
use crate::widgets::ez_object::EzObject;
use crate::widgets::layout::layout::Layout;

// Screen mode implementations
impl Layout{

    pub fn get_screen_mode_contents(&self, state_tree: &mut StateTree) -> PixelMap {

        let mut active_screen = state_tree.get_by_path(&self.path)
            .as_layout().get_active_screen().value.clone();
        if active_screen.is_empty() && !self.children.is_empty() {
            active_screen = self.children.first().unwrap().as_layout().get_id();
            state_tree.get_by_path_mut(&self.path).as_layout_mut()
                .set_active_screen(&active_screen);
        }
        self.get_child(&active_screen).unwrap().as_layout().get_contents(state_tree)
    }
}

