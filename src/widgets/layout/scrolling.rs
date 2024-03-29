use crate::run::definitions::{Pixel, PixelMap, StateTree};
use crate::scheduler::scheduler::SchedulerFrontend;
use crate::states::ez_state::GenericState;
use crate::widgets::ez_object::EzObject;
use crate::widgets::layout::layout::Layout;

// Scrolling implementations
impl Layout {
    /// Handle command by user to scroll down by increasing the scroll_view of y
    pub fn handle_scroll_down(
        &self,
        state_tree: &mut StateTree,
        scheduler: &mut SchedulerFrontend,
    ) {
        let state = state_tree.get_mut(&self.path).as_layout_mut();
        if !state.get_scrolling_config().get_scroll_y() {
            return;
        }
        let scroll_chunk = (state.get_effective_size().height as f64
            / state.get_scrolling_config().get_original_height() as f64)
            * 0.75;
        let new_view_start = f64::min(
            1.0,
            state.get_scrolling_config().get_scroll_start_y() + scroll_chunk,
        );
        state
            .get_scrolling_config_mut()
            .set_scroll_start_y(new_view_start);
        state.update(scheduler);
        self.propagate_absolute_positions(state_tree);
    }

    /// Handle command by user to scroll down by decreasing the scroll_view of y
    pub fn handle_scroll_up(&self, state_tree: &mut StateTree, scheduler: &mut SchedulerFrontend) {
        let state = state_tree.get_mut(&self.path).as_layout_mut();
        if !state.get_scrolling_config().get_scroll_y() {
            return;
        }
        let scroll_chunk = (state.get_effective_size().height as f64
            / state.get_scrolling_config().get_original_height() as f64)
            * 0.75;
        let new_view_start = if state.get_scrolling_config().get_scroll_start_y() > scroll_chunk {
            state.get_scrolling_config().get_scroll_start_y() - scroll_chunk
        } else {
            0.0
        };
        state
            .get_scrolling_config_mut()
            .set_scroll_start_y(new_view_start);
        state.update(scheduler);
        self.propagate_absolute_positions(state_tree);
    }

    /// Handle command by user to scroll down by increasing the scroll_view of x
    pub fn handle_scroll_right(
        &self,
        state_tree: &mut StateTree,
        scheduler: &mut SchedulerFrontend,
    ) {
        let state = state_tree.get_mut(&self.path).as_layout_mut();
        if !state.get_scrolling_config().get_scroll_x() {
            return;
        }
        let scroll_chunk = state.get_effective_size().width as f64
            / state.get_scrolling_config().get_original_width() as f64;
        let new_view_start = f64::min(
            1.0,
            state.get_scrolling_config().get_scroll_start_x() + scroll_chunk,
        );
        state
            .get_scrolling_config_mut()
            .set_scroll_start_x(new_view_start);
        state.update(scheduler);
        self.propagate_absolute_positions(state_tree);
    }

    /// Handle command by user to scroll down by decreasing the scroll_view of x
    pub fn handle_scroll_left(
        &self,
        state_tree: &mut StateTree,
        scheduler: &mut SchedulerFrontend,
    ) {
        let state = state_tree.get_mut(&self.path).as_layout_mut();
        if !state.get_scrolling_config().get_scroll_x() {
            return;
        }

        let scroll_chunk = state.get_effective_size().width as f64
            / state.get_scrolling_config().get_original_width() as f64;
        let new_view_start = if state.get_scrolling_config().get_scroll_start_x() > scroll_chunk {
            state.get_scrolling_config().get_scroll_start_x() - scroll_chunk
        } else {
            0.0
        };
        state
            .get_scrolling_config_mut()
            .set_scroll_start_x(new_view_start);
        state.update(scheduler);
        self.propagate_absolute_positions(state_tree);
    }

    /// Create a horizontal scrollbox out of this layout if its contents width exceed its own width
    pub fn create_horizontal_scroll_box(
        &self,
        state_tree: &mut StateTree,
        contents: PixelMap,
    ) -> PixelMap {
        let state = state_tree.get_mut(&self.get_path()).as_layout_mut();
        if !state.get_scrolling_config().get_scroll_x()
            || contents.len() <= state.get_effective_size().width
        {
            state.get_scrolling_config_mut().set_is_scrolling_x(false);
            return contents;
        }
        state
            .get_scrolling_config_mut()
            .set_original_width(contents.len());
        state.get_scrolling_config_mut().set_is_scrolling_x(true);
        let view_start = state
            .get_scrolling_config()
            .get_absolute_scroll_start_x(state.get_effective_size().width);
        let view_end = if contents.len() - view_start > state.get_effective_size().width {
            view_start + state.get_effective_size().width
        } else {
            contents.len()
        };
        self.propagate_absolute_positions(state_tree);
        self.create_horizontal_scrollbar(
            state_tree,
            contents[view_start..view_end].to_vec(),
            view_start,
        )
    }

    /// Create a vertical scrollbox out of this layout if its contents width exceed its own width
    pub fn create_vertical_scroll_box(
        &self,
        state_tree: &mut StateTree,
        contents: PixelMap,
    ) -> PixelMap {
        let state = state_tree.get_mut(&self.get_path()).as_layout_mut();
        let largest = contents.iter().map(|x| x.len()).max().unwrap_or(0);
        if !state.get_scrolling_config().get_scroll_y()
            || largest <= state.get_effective_size().height
        {
            state.get_scrolling_config_mut().set_is_scrolling_y(false);
            return contents;
        }
        state
            .get_scrolling_config_mut()
            .set_original_height(largest);
        state.get_scrolling_config_mut().set_is_scrolling_y(true);
        let view_start = state
            .get_scrolling_config()
            .get_absolute_scroll_start_y(state.get_effective_size().height);
        let view_end = if largest - view_start > state.get_effective_size().height {
            view_start + state.get_effective_size().height
        } else {
            largest
        };
        let scrolled_contents: PixelMap = contents
            .iter()
            .map(|x| x[view_start..view_end].to_vec())
            .collect();
        self.propagate_absolute_positions(state_tree);
        self.create_vertical_scrollbar(state_tree, scrolled_contents, view_start)
    }

    /// Create a scrolling bar for a horizontal scrollbox
    fn create_horizontal_scrollbar(
        &self,
        state_tree: &mut StateTree,
        mut contents: PixelMap,
        view_start: usize,
    ) -> PixelMap {
        let state = state_tree.get(&self.get_path()).as_layout();
        let (fg_color, _) = state.get_context_colors();
        let bg_color = state.get_color_config().get_bg_color();

        let (scrollbar_size, scrollbar_pos) = self.get_horizontal_scrollbar_parameters(
            state.get_scrolling_config().get_original_width(),
            state.get_effective_size().width,
            view_start,
        );

        for (i, x) in contents.iter_mut().enumerate() {
            let symbol = if i >= scrollbar_pos && i <= scrollbar_pos + scrollbar_size {
                "▀".to_string()
            } else {
                " ".to_string()
            };
            x.push(Pixel::new(symbol, fg_color, bg_color))
        }
        contents
    }

    /// Create a scrolling bar for a vertical scrollbox
    fn create_vertical_scrollbar(
        &self,
        state_tree: &mut StateTree,
        mut contents: PixelMap,
        view_start: usize,
    ) -> PixelMap {
        let mut scrollbar = Vec::new();
        let state = state_tree.get(&self.get_path()).as_layout();
        let (fg_color, _) = state.get_context_colors();
        let bg_color = state.get_color_config().get_bg_color();

        let (scrollbar_size, scrollbar_pos) = self.get_vertical_scrollbar_parameters(
            state.get_scrolling_config().get_original_height(),
            state.get_effective_size().height,
            view_start,
        );

        for x in 0..state.get_effective_size().height {
            let symbol = if x >= scrollbar_pos && x <= scrollbar_pos + scrollbar_size {
                "▐".to_string()
            } else {
                " ".to_string()
            };
            scrollbar.push(Pixel::new(symbol, fg_color, bg_color))
        }
        contents.push(scrollbar);
        contents
    }

    pub fn get_horizontal_scrollbar_parameters(
        &self,
        content_width: usize,
        widget_width: usize,
        view_start: usize,
    ) -> (usize, usize) {
        let scrollbar_ratio = content_width as f32 / widget_width as f32;
        let scrollbar_size = (widget_width as f32 / scrollbar_ratio) as usize;
        let mut scrollbar_pos = if view_start != 0 {
            (view_start as f32 / scrollbar_ratio).round() as usize
        } else {
            0
        };
        if scrollbar_pos == 0 && view_start != 0 {
            scrollbar_pos = 1
        }
        (scrollbar_size, scrollbar_pos)
    }

    pub fn get_vertical_scrollbar_parameters(
        &self,
        content_height: usize,
        widget_height: usize,
        view_start: usize,
    ) -> (usize, usize) {
        let scrollbar_ratio = content_height as f32 / widget_height as f32;
        let scrollbar_size = (widget_height as f32 / scrollbar_ratio) as usize;
        let mut scrollbar_pos = if view_start != 0 {
            (view_start as f32 / scrollbar_ratio).round() as usize
        } else {
            0
        };
        if scrollbar_pos == 0 && view_start != 0 {
            scrollbar_pos = 1
        };
        (scrollbar_size, scrollbar_pos)
    }
}
