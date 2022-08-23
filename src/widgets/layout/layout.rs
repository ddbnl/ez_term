//! # layout
//! Module implementing the layout struct.
use std::cmp::min;
use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use crossterm::event::{Event, KeyCode};
use crate::Context;
use crate::parser::load_base_properties;
use crate::parser::load_common_properties::load_common_property;
use crate::widgets::ez_object::{EzObject, EzObjects};
use crate::states::layout_state::LayoutState;
use crate::states::ez_state::{EzState, GenericState};
use crate::scheduler::scheduler::SchedulerFrontend;
use crate::states::definitions::{LayoutMode};
use crate::run::definitions::{CallbackTree, Coordinates, IsizeCoordinates, Pixel, PixelMap, StateTree};
use crate::widgets::helper_functions::{add_border, add_padding, reposition_with_pos_hint,
                                       resize_with_size_hint};


/// A layout is where widgets live. They implements methods for hardcoding widget placement or
/// placing them automatically in various ways.
#[derive(Clone, Debug)]
pub struct Layout {

    /// ID of the layout, used to construct [path]
    pub id: String,

    /// Full path to this layout, e.g. "/root_layout/layout_2/THIS_ID"
    pub path: String,

    /// List of children widgets and/or layouts
    pub children: Vec<EzObjects>,

    /// Child ID to index in [children] lookup. Used to get widgets by [id] and [path]
    pub child_lookup: HashMap<String, usize>,

    /// Runtime state of this layout, see [LayoutState] and [State]
    pub state: LayoutState,
}


impl Layout {
    pub fn new(id: String, path: String, scheduler: &mut SchedulerFrontend) -> Self {
        Layout {
            id,
            path: path.clone(),
            children: Vec::new(),
            child_lookup: HashMap::new(),
            state: LayoutState::new(path, scheduler),
        }
    }

    pub fn from_state(id: String, path: String, _scheduler: &mut SchedulerFrontend,
                      state: EzState) -> Self {
        Layout {
            id,
            path: path.clone(),
            children: Vec::new(),
            child_lookup: HashMap::new(),
            state: state.as_layout().to_owned(),
        }
    }
}


impl EzObject for Layout {

    fn load_ez_parameter(&mut self, parameter_name: String, parameter_value: String,
                         scheduler: &mut SchedulerFrontend) -> Result<(), Error> {

        let consumed = load_common_property(
            &parameter_name, parameter_value.clone(),self, scheduler)?;
        if consumed { return Ok(()) }
        match parameter_name.as_str() {
            "mode" => load_base_properties::load_layout_mode_property(
                parameter_value.trim(), scheduler, self.path.clone(),
                &parameter_name, self.get_state_mut())?,
            "orientation" => load_base_properties::load_layout_orientation_property(
                parameter_value.trim(), scheduler, self.path.clone(),
                &parameter_name, self.get_state_mut())?,
            "active_tab" => load_base_properties::load_string_property(
                parameter_value.trim(), scheduler, self.path.clone(),
                &parameter_name, self.get_state_mut())?,
            "tab_name" => load_base_properties::load_string_property(
                parameter_value.trim(), scheduler, self.path.clone(),
                &parameter_name, self.get_state_mut())?,
            "active_screen" => load_base_properties::load_string_property(
                parameter_value.trim(), scheduler, self.path.clone(),
                &parameter_name, self.get_state_mut())?,
            "view_size" => load_base_properties::load_usize_property(
                parameter_value.trim(), scheduler, self.path.clone(),
                &parameter_name, self.get_state_mut())?,
            "view_page" => load_base_properties::load_usize_property(
                parameter_value.trim(), scheduler, self.path.clone(),
                &parameter_name, self.get_state_mut())?,
            "scroll" => {
                let (x, y) = match parameter_value.split_once(',') {
                    Some((i, j)) => (i, j),
                    None => return Err(
                        Error::new(ErrorKind::InvalidData,
                                   format!("Invalid value for scroll: \"{}\". Required format \
                                   is \"scroll: true, false\"", parameter_value)))
                };
                load_base_properties::load_bool_property(
                    x.trim(), scheduler, self.path.clone(),
                    "scroll_x", self.get_state_mut())?;
                load_base_properties::load_bool_property(
                    y.trim(), scheduler, self.path.clone(),
                    "scroll_y", self.get_state_mut())?;
            }
            "rows" => load_base_properties::load_usize_property(
                parameter_value.trim(), scheduler, self.path.clone(),
                &parameter_name, self.get_state_mut())?,
            "cols" => load_base_properties::load_usize_property(
                parameter_value.trim(), scheduler, self.path.clone(),
                &parameter_name, self.get_state_mut())?,
            "row_default_height" => load_base_properties::load_usize_property(
                parameter_value.trim(), scheduler, self.path.clone(),
                &parameter_name, self.get_state_mut())?,
            "col_default_width" => load_base_properties::load_usize_property(
                parameter_value.trim(), scheduler, self.path.clone(),
                &parameter_name, self.get_state_mut())?,
            "force_default_row_height" => load_base_properties::load_bool_property(
                parameter_value.trim(), scheduler, self.path.clone(),
                &parameter_name, self.get_state_mut())?,
            "force_default_col_width" => load_base_properties::load_bool_property(
                parameter_value.trim(), scheduler, self.path.clone(),
                &parameter_name, self.get_state_mut())?,
            "scroll_x" => load_base_properties::load_bool_property(
                parameter_value.trim(), scheduler, self.path.clone(),
                &parameter_name, self.get_state_mut())?,
            "scroll_y" => load_base_properties::load_bool_property(
                parameter_value.trim(), scheduler, self.path.clone(),
                &parameter_name, self.get_state_mut())?,
            "scroll_start_x" => load_base_properties::load_f64_property(
                parameter_value.trim(), scheduler, self.path.clone(),
                &parameter_name, self.get_state_mut())?,
            "scroll_start_y" => load_base_properties::load_f64_property(
                parameter_value.trim(), scheduler, self.path.clone(),
                &parameter_name, self.get_state_mut())?,
            "can_drag" => load_base_properties::load_bool_property(
                parameter_value.trim(), scheduler, self.path.clone(),
                &parameter_name, self.get_state_mut())?,
            "fill" => load_base_properties::load_bool_property(
                parameter_value.trim(), scheduler, self.path.clone(),
                &parameter_name, self.get_state_mut())?,
            "filler_symbol" => load_base_properties::load_string_property(
                parameter_value.trim(), scheduler, self.path.clone(),
                &parameter_name, self.get_state_mut())?,
            _ => return Err(
                Error::new(ErrorKind::InvalidData,
                           format!("Invalid parameter name for layout {}",
                                   parameter_name)))
        }
        Ok(())
    }
    fn set_id(&mut self, id: &str) { self.id = id.to_string(); }

    fn get_id(&self) -> String { self.id.clone() }

    fn set_path(&mut self, id: &str) { self.id = id.to_string() }

    fn get_path(&self) -> String { self.path.clone() }

    fn get_state(&self) -> EzState { EzState::Layout(self.state.clone()) }

    fn get_state_mut(&mut self) -> &mut dyn GenericState { &mut self.state }

    fn get_contents(&self, state_tree: &mut StateTree) -> PixelMap {

        let mut merged_content = PixelMap::new();
        let mode = state_tree.get(&self.path).as_layout().get_mode().clone();

        self.set_child_sizes(state_tree);
        merged_content = match mode {
            LayoutMode::Box => self.get_box_mode_contents(state_tree),
            LayoutMode::Stack => self.get_stack_mode_contents(state_tree),
            LayoutMode::Table => self.get_table_mode_contents(state_tree),
            LayoutMode::Float => self.get_float_mode_contents(merged_content, state_tree),
            LayoutMode::Screen => self.get_screen_mode_contents(state_tree),
            LayoutMode::Tab => self.get_tab_mode_contents(state_tree),
        };

        merged_content = self.add_user_filler(state_tree, merged_content);
        merged_content = self.auto_scale_to_content(state_tree, merged_content);
        merged_content = self.add_empty_filler(state_tree, merged_content);
        merged_content = self.create_horizontal_scroll_box(state_tree, merged_content);
        merged_content = self.create_vertical_scroll_box(state_tree, merged_content);
        let state = state_tree.get(&self.get_path()).as_layout();

        if merged_content.is_empty() { return merged_content } // Empty widget
        // Put border around content if border if set
        if state.get_border_config().get_border() {
            merged_content = add_border(merged_content, state.get_border_config(),
                                        state.get_color_config());
        }
        // Put padding around content if set
        merged_content = add_padding(
            merged_content, state.get_padding(),
            state.get_color_config().get_bg_color(),
            state.get_color_config().get_fg_color());
        merged_content = self.get_modal_contents(state_tree, merged_content);

        self.propagate_absolute_positions(state_tree);
        merged_content
    }

    fn handle_event(&self, event: Event, state_tree: &mut StateTree,
                    callback_tree: &mut CallbackTree, scheduler: &mut SchedulerFrontend) -> bool {

        if let Event::Key(key) = event {
            if callback_tree.get(&self.path).obj.keymap
                .contains(key.code, key.modifiers) {
                let func =
                    callback_tree.get_mut(&self.path).obj.keymap
                        .get_mut(key.code, key.modifiers).unwrap();
                let context = Context::new(
                    self.get_path(), state_tree, scheduler);
                let consumed = func(context, key.code, key.modifiers);
                if consumed { return true }
            }
            if key.code == KeyCode::PageUp {
                self.handle_scroll_up(state_tree, scheduler);
                return true
            } else if key.code == KeyCode::PageDown {
                self.handle_scroll_down(state_tree, scheduler);
                return true
            } else if key.code == KeyCode::Left {
                let state = state_tree.get_mut(&self.get_path())
                    .as_layout_mut();
                if state.get_mode() == &LayoutMode::Tab {
                    self.handle_tab_left(state_tree, scheduler);
                } else {
                    self.handle_scroll_left(state_tree, scheduler);
                }
                return true
            } else if key.code == KeyCode::Right {
                let state = state_tree.get_mut(&self.get_path())
                    .as_layout_mut();
                if state.get_mode() == &LayoutMode::Tab {
                    self.handle_tab_right(state_tree, scheduler);
                } else {
                    self.handle_scroll_right(state_tree, scheduler);
                }
                return true
            }
        }
        false
    }

    /// Implement user keyboard enter to select a new tab after it's already selected.
    fn on_keyboard_enter(&self, state_tree: &mut StateTree, callback_tree: &mut CallbackTree,
                         scheduler: &mut SchedulerFrontend) -> bool {

        if self.on_keyboard_enter_callback(state_tree, callback_tree, scheduler) { return true }
        let state = state_tree.get_mut(&self.path).as_layout_mut();
        if !state.get_selected_tab_header().is_empty() {
            let tab_name = state.get_selected_tab_header()
                .strip_suffix("_tab_header").unwrap().to_string();
            let tab_path = self.resolve_tab_name(&tab_name, state_tree);
            let state = state_tree.get_mut(&self.path).as_layout_mut();
            state.set_active_tab(&tab_path);
            state.update(scheduler);
            return true
        }
        false
    }

    // Implement clicking under are above the scrollbar to move it down or up respectively.
    fn on_left_mouse_click(&self, state_tree: &mut StateTree, callback_tree: &mut CallbackTree,
                           scheduler: &mut SchedulerFrontend, mouse_pos: Coordinates) -> bool {

        if self.on_left_mouse_click_callback(state_tree, callback_tree, scheduler, mouse_pos) {
            return true
        }
        let state = state_tree.get_mut(&self.path).as_layout_mut();

        let v_edge = if state.get_border_config().get_border()
        { state.get_effective_size().height + 1 }
        else { state.get_effective_size().height };
        if state.get_scrolling_config().get_is_scrolling_x() && mouse_pos.y == v_edge {

            let (scrollbar_size, scrollbar_pos) =
                self.get_horizontal_scrollbar_parameters(
                state.get_scrolling_config().get_original_width(),
                state.get_effective_size().width,
                state.get_scrolling_config()
                    .get_absolute_scroll_start_x(state.get_effective_size().width));

            if mouse_pos.x < scrollbar_pos {
                self.handle_scroll_left(state_tree, scheduler);
                return true
            } else if mouse_pos.x > scrollbar_pos + scrollbar_size {
                self.handle_scroll_right(state_tree, scheduler);
                return true
            }
        }

        let h_edge = if state.get_border_config().get_border()
            { state.get_effective_size().width + 1 }
            else { state.get_effective_size().width };
        if state.get_scrolling_config().get_is_scrolling_y() &&
            mouse_pos.x == h_edge {
            let (scrollbar_size, scrollbar_pos) = self.get_vertical_scrollbar_parameters(
                state.get_scrolling_config().get_original_height(),
                state.get_effective_size().height,
                state.get_scrolling_config()
                    .get_absolute_scroll_start_y(state.get_effective_size().height));

            if mouse_pos.y < scrollbar_pos {
                self.handle_scroll_up(state_tree, scheduler);
                return true
            } else if mouse_pos.y > scrollbar_pos + scrollbar_size {
                self.handle_scroll_down(state_tree, scheduler);
                return true
            }
        }
        false
    }

    /// Implement clicking on the scrollbar and dragging it down or up.
    fn on_drag(&self, state_tree: &mut StateTree, callback_tree: &mut CallbackTree,
               scheduler: &mut SchedulerFrontend, previous_pos: Option<Coordinates>,
               mouse_pos: Coordinates) -> bool {

        let mut consumed =
            self.on_drag_callback(state_tree, callback_tree, scheduler, previous_pos, mouse_pos);

        let state = state_tree.get_mut(&self.path).as_layout_mut();

        let offset = if state.get_border_config().get_border() { 2 } else { 1 };

        if state.get_scrolling_config().get_is_scrolling_x() &&
                mouse_pos.y + offset == state.get_size().get_height() {
            consumed = self.handle_scroll_drag_x(state, previous_pos, mouse_pos);

        } else if state.get_scrolling_config().get_is_scrolling_y() &&
                mouse_pos.x + offset == state.get_size().get_width() {
            self.handle_scroll_drag_y(state, previous_pos, mouse_pos);

        } else if self.path.starts_with("/root/modal") && state.can_drag.value {
            let abs_mouse_pos = Coordinates::new(
                state.get_absolute_position().x as usize + mouse_pos.x,
                state.get_absolute_position().y as usize + mouse_pos.y
            );
            if previous_pos.is_none() {
                for child in self.children.iter() {
                    if state_tree.get(&child.as_ez_object().get_path())
                        .as_generic().collides(abs_mouse_pos) {
                        return false;
                    }
                }
                return true
            }
            state.set_pos_hint_x(None);
            state.set_pos_hint_y(None);
            let diff_x: isize = mouse_pos.x as isize - previous_pos.unwrap().x as isize;
            let diff_y: isize = mouse_pos.y as isize - previous_pos.unwrap().y as isize;
            let pos = state.get_position().clone();
            let size = state.size.clone();
            let root_size =
                state_tree.as_generic().get_size().clone();
            let state = state_tree.get_mut(&self.path).as_layout_mut();
            state.set_x(
                if diff_x < 0 && diff_x.abs() > pos.x.value as isize {
                    0
                } else if diff_x > 0 && pos.x.value + diff_x as usize >
                        (root_size.get_width() - size.get_width()) {
                    root_size.get_width() - size.get_width()
                } else {
                    (pos.x.value as isize + diff_x) as usize
                }
            );
            state.set_y(
                if diff_y < 0 && diff_y.abs() > pos.y.value as isize {
                    0
                } else if diff_y > 0 && pos.y.value + diff_y as usize >
                        (root_size.get_height() - size.get_height()) {
                    root_size.get_height() - size.get_height()
                } else {
                    (pos.y.value as isize + diff_y) as usize
                }
            )
        }
        let state = state_tree.get_mut(&self.path).as_layout_mut();
        state.update(scheduler);
        self.propagate_absolute_positions(state_tree);
        consumed
    }

    fn on_scroll_up(&self, state_tree: &mut StateTree, callback_tree: &mut CallbackTree,
                    scheduler: &mut SchedulerFrontend) -> bool {

        if self.on_scroll_up_callback(state_tree, callback_tree, scheduler) { return true }
        let state = state_tree.get_mut(&self.path).as_layout_mut();
        if state.get_scrolling_config().get_is_scrolling_y() {
            self.handle_scroll_up(state_tree, scheduler);
            return true
        } else if state.get_scrolling_config().get_is_scrolling_x() {
            self.handle_scroll_left(state_tree, scheduler);
            return true
        }
        false
    }

    fn on_scroll_down(&self, state_tree: &mut StateTree, callback_tree: &mut CallbackTree,
                      scheduler: &mut SchedulerFrontend) -> bool {

        if self.on_scroll_down_callback(state_tree, callback_tree, scheduler) { return true }
        let state = state_tree.get_mut(&self.path).as_layout_mut();
        if state.get_scrolling_config().get_is_scrolling_y() {
            self.handle_scroll_down(state_tree, scheduler);
            return true
        } else if state.get_scrolling_config().get_is_scrolling_x() {
            self.handle_scroll_right(state_tree, scheduler);
            return true
        }
    false
    }

    fn on_select(&self, state_tree: &mut StateTree, callback_tree: &mut CallbackTree,
                 scheduler: &mut SchedulerFrontend, mouse_pos: Option<Coordinates>) -> bool {

        if self.on_select_callback(state_tree, callback_tree, scheduler, mouse_pos) { return true }
        let state = state_tree.get_mut(&self.path).as_layout_mut();
        if state.mode.value == LayoutMode::Tab {
            for child in self.children.iter() {
                if let EzObjects::Button(i) = child {
                    state.set_selected_tab_header(i.id.clone());
                    state.update(scheduler);
                    return true
                }
            }
        }

        true
    }

    fn on_deselect(&self, state_tree: &mut StateTree, callback_tree: &mut CallbackTree,
                   scheduler: &mut SchedulerFrontend) -> bool {

        if self.on_deselect_callback(state_tree, callback_tree, scheduler) { return true }
        let state = state_tree.get_mut(&self.path).as_layout_mut();
        state.set_selected_tab_header("".to_string());
        state.update(scheduler);
        true
    }

    fn get_clone(&self, scheduler: &mut SchedulerFrontend) -> EzObjects {

        let children: Vec<EzObjects> = self.children.iter()
            .map(|x| x.as_ez_object().get_clone(scheduler)).collect();
        let mut clone = self.clone();
        let mut new_state = LayoutState::new(self.path.clone(), scheduler);
        new_state.copy_state_values(self.get_state());
        clone.state = new_state;
        clone.children = children;
        EzObjects::Layout(clone)
    }
}

impl Layout {
    /// Initialize an instance of this object using the passed config coming from [ez_parser]
    pub fn from_config(config: Vec<String>, id: String, path: String, scheduler: &mut SchedulerFrontend,
                       file: String, line: usize) -> Self {

        let mut obj = Layout::new(id, path, scheduler);
        obj.load_ez_config(config, scheduler, file, line).unwrap();
        obj
    }

    fn handle_scroll_drag_x(&self, state: &mut LayoutState, previous_pos: Option<Coordinates>,
                            mouse_pos: Coordinates) -> bool {

        let scroll_start_x = state.get_scrolling_config()
            .get_absolute_scroll_start_x(state.get_effective_size().width);
        let (scrollbar_size, scrollbar_pos) =
            self.get_horizontal_scrollbar_parameters(
                state.get_scrolling_config().get_original_width(),
                state.get_effective_size().width, scroll_start_x);

        if previous_pos.is_none() {
            return if mouse_pos.x >= scrollbar_pos && mouse_pos.x <= scrollbar_pos + scrollbar_size {
                true
            } else {
                false
            }
        }

        let absolute_diff = (mouse_pos.x as isize)
            .abs_diff(previous_pos.unwrap().x as isize);
        let absolute_scroll = 1.0 / ((absolute_diff * state.get_effective_size().width) as f64);
        if previous_pos.unwrap().x > mouse_pos.x && absolute_scroll >
            state.get_scrolling_config().get_scroll_start_x() {
            state.get_scrolling_config_mut().set_scroll_start_x(0.0);
        } else if previous_pos.unwrap().x < mouse_pos.x &&
            state.get_scrolling_config().get_scroll_start_x() + absolute_scroll > 1.0 {
            state.get_scrolling_config_mut().set_scroll_start_x(1.0)
        } else {
            let new_view_start= if previous_pos.unwrap().x > mouse_pos.x {
                state.get_scrolling_config().get_scroll_start_x() - absolute_scroll
            } else {
                state.get_scrolling_config().get_scroll_start_x() + absolute_scroll
            };
            state.get_scrolling_config_mut().set_scroll_start_x(new_view_start);
        }
        true
    }
    fn handle_scroll_drag_y(&self, state: &mut LayoutState, previous_pos: Option<Coordinates>,
                            mouse_pos: Coordinates) -> bool {

        let scroll_start_y = state.get_scrolling_config()
            .get_absolute_scroll_start_y(state.get_effective_size().height);
        let (scrollbar_size, scrollbar_pos) =
            self.get_vertical_scrollbar_parameters(
                state.get_scrolling_config().get_original_height(),
                state.get_effective_size().height,scroll_start_y);

        if previous_pos.is_none() {
            return if mouse_pos.y >= scrollbar_pos && mouse_pos.y <= scrollbar_pos + scrollbar_size {
                true
            } else {
                false
            }
        }

        let absolute_diff = (mouse_pos.y as isize)
            .abs_diff(previous_pos.unwrap().y as isize);
        let absolute_scroll = 1.0 / ((absolute_diff * state.get_effective_size().height) as f64);
        if previous_pos.unwrap().y > mouse_pos.y && absolute_scroll >
            state.get_scrolling_config().get_scroll_start_x() {
            state.get_scrolling_config_mut().set_scroll_start_y(0.0);
        } else if previous_pos.unwrap().y < mouse_pos.y &&
            state.get_scrolling_config().get_scroll_start_y() + absolute_scroll > 1.0 {
            state.get_scrolling_config_mut().set_scroll_start_y(1.0)
        } else {
            let new_view_start = if previous_pos.unwrap().y > mouse_pos.y {
                state.get_scrolling_config().get_scroll_start_y() - absolute_scroll
            } else {
                state.get_scrolling_config().get_scroll_start_y() + absolute_scroll
            };
            state.get_scrolling_config_mut().set_scroll_start_y(new_view_start);
        }
        true
    }

    /// Scale size down to the size of the actual content of the layout.
    fn auto_scale_to_content(&self, state_tree: &mut StateTree, contents: PixelMap) -> PixelMap {

        let state = state_tree.get_mut(&self.get_path())
            .as_layout_mut();
        if state.get_auto_scale().get_auto_scale_width() {
            let auto_scale_width = contents.len();
            if auto_scale_width < state.get_effective_size().width {
                state.set_effective_width(auto_scale_width);
            }
        }
        if state.get_auto_scale().get_auto_scale_height() {
            let auto_scale_height = contents.iter()
                .map(|x| x.len()).max().unwrap_or(0);
            if auto_scale_height < state.get_effective_size().height {
                state.set_effective_height(auto_scale_height);
            }
        }
        contents
    }

    /// Overwrite a PixelMap of current own content with the content of the active modal. Modals
    /// overlap all content.
    fn get_modal_contents(&self, state_tree: &mut StateTree, mut contents: PixelMap) -> PixelMap {

        if !state_tree.get(&self.get_path()).as_layout().has_modal() {
            return contents
        }

        // Size modal
        let parent_size = state_tree.get(&self.get_path()).as_layout()
            .get_size().clone();
        let modal = state_tree.get(&self.get_path()).as_layout()
            .get_modal().clone();
        let state = &mut state_tree
            .get_mut(&modal.as_ez_object().get_path()).obj;
        resize_with_size_hint(state, parent_size.get_width(),
                              parent_size.get_height());
        reposition_with_pos_hint(
            parent_size.get_width(), parent_size.get_height(),
            state.as_generic_mut());
        let x = state.as_generic().get_position().get_x();
        let y = state.as_generic().get_position().get_y();
        state.as_generic_mut().set_absolute_position(IsizeCoordinates::new(
            x as isize, y as isize));

        //Get contents
        let modal_content;
        if let EzObjects::Layout(ref i) = modal {
            modal_content = i.get_contents(state_tree);
            i.propagate_absolute_positions(state_tree);
        } else {
            modal_content = modal.as_ez_object().get_contents(state_tree);
        }

        // Overwrite own content with modal (modal is always on top)
        let state = state_tree
            .get_mut(&modal.as_ez_object().get_path()).as_generic();
        let start_pos = state.get_position();
        for x in 0..modal_content.len() {
            for y in 0..modal_content[x].len() {
                let write_pos = Coordinates::new(start_pos.get_x() + x,
                                                 start_pos.get_y() + y);
                if write_pos.x < parent_size.get_width() &&
                    write_pos.y < parent_size.get_height() {
                    contents[write_pos.x][write_pos.y] = modal_content[x][y].clone();
                }
            }
        }
        contents
    }
    /// Fill any empty positions with [Pixel] from [get_filler]
    pub fn add_user_filler(&self, state_tree: &mut StateTree, mut contents: PixelMap) -> PixelMap {

        let state = state_tree.get_mut(&self.get_path())
            .as_layout_mut();
        if !state.get_fill() { return contents }

        let filler = Pixel::new(state.get_filler_symbol(),
                                state.get_color_config().get_filler_fg_color(),
                                state.get_color_config().get_filler_bg_color());

        for x in 0..contents.len() {
            for y in contents[x].iter_mut() {
                if y.symbol.is_empty() || y.symbol == " " {
                    y.symbol = filler.symbol.clone();
                    y.foreground_color = filler.foreground_color;
                    y.background_color = filler.background_color;
                }
            }
            while contents[x].len() < (state.get_effective_size().height) {
                contents[x].push(filler.clone());
            }
        }
        while contents.len() < state.get_effective_size().width {
            let mut new_x = Vec::new();
            for _ in 0..state.get_effective_size().height {
                new_x.push(filler.clone());
            }
            contents.push(new_x);
        }
        contents
    }

    /// Fill any empty positions with empty [Pixel]. Used to fill full size of the layout in case
    /// the user did not define a custom filler.
    pub fn add_empty_filler(&self, state_tree: &mut StateTree, mut contents: PixelMap) -> PixelMap {

        let state = state_tree.get_mut(&self.get_path())
            .as_layout_mut();
        while contents.len() < state.get_effective_size().width {
            contents.push(Vec::new());
        }
        let largest = contents.iter().map(|x| x.len()).max().unwrap_or(0);
        for x in contents.iter_mut() {
            while x.len() < state.get_effective_size().height || x.len() < largest {
                x.push(Pixel::new(
                    " ".to_string(), state.get_color_config().get_fg_color(),
                    state.get_color_config().get_bg_color()));
            }
        }
        contents
    }

    /// Get layout children that are in view based on view_size and view_page properties.
    pub fn get_children_in_view(&self, state_tree: &mut StateTree) -> &[EzObjects] {

        if self.children.is_empty() { return  &[] }

        let state = state_tree.get_mut(&self.path).as_layout_mut();
        if state.get_view_size() == 0 { return &self.children }
        if state.get_view_page() == 0 { state.set_view_page(1) }

        let max_pages = (self.children.len() / state.get_view_size())
            + if self.children.len() % state.get_view_size() > 0 {1} else {0};
        if state.get_view_page() > max_pages { state.set_view_page(max_pages) }

        let child_start = state.get_view_size() * (state.get_view_page() - 1);
        let child_end = min(child_start + state.get_view_size(), self.children.len());

        &self.children[child_start..child_end]

    }
}
