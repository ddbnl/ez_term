use crate::run::definitions::{IsizeCoordinates, Pixel, PixelMap, StateTree};
use crate::scheduler::scheduler::SchedulerFrontend;
use crate::widgets::ez_object::EzObjects;
use crate::widgets::ez_object::EzObject;
use crate::states::ez_state::GenericState;
use crate::widgets::layout::layout::Layout;

// Tab mode implementations
impl Layout {

    pub fn handle_tab_left(&self, state_tree: &mut StateTree, scheduler: &mut SchedulerFrontend) {

        let mut next_button = false;
        for child in self.children.iter().rev() {
            if let EzObjects::Button(ref widget) = child {
                if next_button {
                    let state = state_tree.get_by_path_mut(&self.path)
                        .as_layout_mut();
                    state.set_selected_tab_header(widget.id.clone());
                    state.update(scheduler);
                    return
                } else if state_tree.get_by_path(&self.path).as_layout()
                    .get_selected_tab_header() == widget.id {
                    next_button = true;
                }
            }
        }
    }

    pub fn handle_tab_right(&self, state_tree: &mut StateTree, scheduler: &mut SchedulerFrontend) {

        let mut next_button = false;
        for child in self.children.iter() {
            if let EzObjects::Button(ref widget) = child {
                if next_button {
                    let state = state_tree.get_by_path_mut(&self.path)
                        .as_layout_mut();
                    state.set_selected_tab_header(widget.id.clone());
                    state.update(scheduler);
                    return
                } else if state_tree
                    .get_by_path(&self.path).as_layout().get_selected_tab_header() == widget.id {
                    next_button = true;
                }
            }
        }
    }

    pub fn get_tab_mode_contents(&self, state_tree: &mut StateTree) -> PixelMap {

        if self.children.is_empty() { return PixelMap::new() }
        let state = state_tree.get_by_path_mut(&self.path).as_layout_mut();
        let own_infinite_size = state.get_infinite_size().clone();
        let own_effective_size = state.get_effective_size();
        let own_pos = state.get_effective_absolute_position();
        let own_colors = state.get_color_config().clone();
        let selection = state.get_selected_tab_header().clone();
        if state.get_active_tab().is_empty() {
            if !self.children.is_empty() {
                state.set_active_tab(&self.children[0].as_ez_object().get_id());
            } else {
                return PixelMap::new()
            }
        }
        let active_tab = state.get_active_tab();

        let mut button_content = PixelMap::new();
        let mut tab_content = PixelMap::new();
        let mut pos_x: usize = 0;
        let mut selected_pos_x: usize = 0;
        let mut selected_width: usize = 0;
        for child in self.get_children() {
            if let EzObjects::Layout(i) = child {
                if i.get_id() != active_tab { continue }
                let child_state = state_tree
                    .get_by_path_mut(&child.as_ez_object().get_full_path()).as_generic_mut();
                child_state.set_effective_height(
                    if own_effective_size.height >= 3 { own_effective_size.height - 5} else {0});
                child_state.set_effective_width(
                    if own_effective_size.width >= 1 { own_effective_size.width - 1} else {0});
                child_state.get_position_mut().set_x(0);
                child_state.get_position_mut().set_y(3);
                child_state.set_absolute_position(IsizeCoordinates::new(
                    own_pos.x, own_pos.y + 3));
                tab_content = i.get_contents(state_tree);
            } else if let EzObjects::Button(i) = child {

                let child_state=
                    state_tree.get_by_path_mut(&i.path).as_button_mut();

                let tab_text = child_state.get_text();
                child_state.get_color_config_mut().set_fg_color(
                    if selection == i.id { own_colors.get_selection_fg_color() }
                    else if active_tab == tab_text {
                        own_colors.get_active_fg_color()
                    } else { own_colors.get_tab_fg_color() });
                child_state.get_color_config_mut().set_bg_color(
                    if selection == i.id { own_colors.get_selection_bg_color() }
                    else if active_tab == tab_text {
                        own_colors.get_active_bg_color()
                    } else { own_colors.get_tab_bg_color() });

                child_state.set_auto_scale_width(true);
                child_state.set_auto_scale_height(true);
                child_state.set_x(pos_x);
                child_state.set_y(0);
                let content = i.get_contents(state_tree);
                let child_state = state_tree
                    .get_by_path_mut(&i.path).as_button_mut();
                let mut custom_size = own_effective_size;
                custom_size.width -= 1;
                custom_size.height = 3;
                button_content = self.merge_horizontal_contents(
                    button_content, content,
                    custom_size.height, own_infinite_size.height,
                    own_colors.clone(),
                    child_state);
                child_state.set_absolute_position(
                    IsizeCoordinates::new(own_pos.x + pos_x as isize, own_pos.y + 1));

                if (!selection.is_empty() && selection == i.id) || (selection.is_empty() &&
                    active_tab == i.id.strip_suffix("_tab_header").unwrap()) {
                    selected_pos_x = pos_x;
                    selected_width = child_state.get_size().get_width();
                }

                pos_x = button_content.len();

            }
        }
        let fill_pixel = Pixel::new(" ".to_string(),
                                    own_colors.get_fg_color(),
                                    own_colors.get_bg_color());
        if own_effective_size.width < button_content.len()  {
            let mut difference;
            if own_effective_size.width <= selected_pos_x + selected_width {
                difference = (selected_pos_x + selected_width) - own_effective_size.width;
                if button_content.len() - difference > 3 {
                    difference += 3;
                }
            } else if selected_pos_x != 0 && button_content.len() > 3 {
                difference = 3;
            } else {
                difference = 0;
            }
            button_content = button_content[difference..].to_vec();
            for child in self.children.iter() {
                if let EzObjects::Button(button) = child {
                    let state = state_tree
                        .get_by_path_mut(&button.path).as_button_mut();
                    state.set_x(if state.get_position().get_x() >= difference
                    { state.get_position().get_x() - difference } else { 0 });
                    state.set_absolute_position(IsizeCoordinates::new(
                        if state.get_absolute_position().x >= difference as isize
                        { state.get_absolute_position().x - difference as isize } else { 0 },
                    state.get_absolute_position().y));
                }
            }
        }
        if button_content.len() > own_effective_size.width {
            button_content = button_content[..own_effective_size.width].to_vec();
        }
        while button_content.len() < own_effective_size.width {
            let row =  vec!(fill_pixel.clone(), fill_pixel.clone(), fill_pixel.clone());
            button_content.push(row);
        }

        let state = state_tree.get_by_path_mut(&self.path).as_layout_mut();
        self.merge_vertical_contents(
            button_content,
            tab_content,
            own_effective_size.width, own_infinite_size.width,
            own_colors, state)
    }
}