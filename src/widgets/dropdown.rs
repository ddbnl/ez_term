//! # Dropdown Widget
//! Widget which supports and arbitrary amount of possible values of which one can be chosen at any
//! time. The active value is always displayed, and when selected drops down all other possible
//! values for the user to select.
use std::io::{Error, ErrorKind};

use crossterm::event::{Event, KeyCode, MouseButton, MouseEventKind};

use crate::parser::load_base_properties;
use crate::parser::load_common_properties::load_common_property;
use crate::run::definitions::{CallbackTree, Coordinates, Pixel, PixelMap, StateTree};
use crate::scheduler::scheduler::SchedulerFrontend;
use crate::states::definitions::{
    AutoScale, HorizontalAlignment, InfiniteSize, Padding, PosHint, SizeHint, StateCoordinates,
    StateSize, VerticalAlignment,
};
use crate::states::dropdown_state::{DropdownState, DroppedDownMenuState};
use crate::states::ez_state::{EzState, GenericState};
use crate::widgets::ez_object::{EzObject, EzObjects};
use crate::widgets::helper_functions::{add_border, add_padding};
use crate::{CallbackConfig, Context};

#[derive(Clone, Debug)]
pub struct Dropdown {
    /// ID of the widget, used to construct [path]
    pub id: String,

    /// Full path to this widget, e.g. "/root_layout/layout_2/THIS_ID"
    pub path: String,

    /// Runtime state of this widget, see [DropdownState] and [State]
    pub state: DropdownState,
}

impl Dropdown {
    pub fn new(id: String, path: String, scheduler: &mut SchedulerFrontend) -> Self {
        Dropdown {
            id,
            path: path.clone(),
            state: DropdownState::new(path, scheduler),
        }
    }

    pub fn from_state(
        id: String,
        path: String,
        _scheduler: &mut SchedulerFrontend,
        state: EzState,
    ) -> Self {
        Dropdown {
            id,
            path: path.clone(),
            state: state.as_dropdown().to_owned(),
        }
    }
}

impl EzObject for Dropdown {
    fn load_ez_parameter(
        &mut self,
        parameter_name: String,
        parameter_value: String,
        scheduler: &mut SchedulerFrontend,
    ) -> Result<(), Error> {
        let consumed =
            load_common_property(&parameter_name, parameter_value.clone(), self, scheduler)?;
        if consumed {
            return Ok(());
        }
        match parameter_name.as_str() {
            "allow_none" => load_base_properties::load_bool_property(
                parameter_value.trim(),
                scheduler,
                self.path.clone(),
                &parameter_name,
                self.get_state_mut(),
            )?,
            "options" => {
                self.state.set_options(
                    parameter_value
                        .split(',')
                        .map(|x| x.trim().to_string())
                        .collect(),
                );
            }
            "choice" => load_base_properties::load_string_property(
                parameter_value.trim(),
                scheduler,
                self.path.clone(),
                &parameter_name,
                self.get_state_mut(),
            )?,
            _ => {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("Invalid parameter name for dropdown: {}", parameter_name),
                ))
            }
        }
        Ok(())
    }

    fn set_id(&mut self, id: &str) {
        self.id = id.to_string()
    }

    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn set_path(&mut self, id: &str) {
        self.id = id.to_string()
    }

    fn get_path(&self) -> String {
        self.path.clone()
    }

    fn get_state(&self) -> EzState {
        EzState::Dropdown(self.state.clone())
    }

    fn get_state_mut(&mut self) -> &mut dyn GenericState {
        &mut self.state
    }

    /// Content of this widget depends on whether it is currently dropped down or not. If not,
    /// then display a label with a border representing the currently selected value. If dropped
    /// down show a list of all options, with the currently selected one on top.
    fn get_contents(&self, state_tree: &mut StateTree) -> PixelMap {
        let state = state_tree.get_mut(&self.get_path()).as_dropdown_mut();
        // If dropped down get full content instead
        // Set a default value if user didn't give one
        if state.get_choice().is_empty() && !state.get_allow_none() {
            state.set_choice(
                &state
                    .get_options()
                    .first()
                    .expect("Dropdown widget must have at least one option")
                    .to_string(),
            );
        }
        // Create a bordered label representing currently active value
        let (fg_color, bg_color) = state.get_context_colors();
        let mut text = state.get_choice().chars().rev().collect::<String>();
        let mut contents = Vec::new();

        let write_width =
            if state.get_infinite_size().width || state.get_auto_scale().get_auto_scale_width() {
                state
                    .get_options()
                    .iter()
                    .map(|x| x.len())
                    .max()
                    .unwrap_or(0)
            } else {
                state.get_effective_size().width
            };

        for _ in 0..write_width {
            let mut new_y = Vec::new();
            if !text.is_empty() {
                new_y.push(Pixel::new(
                    text.pop().unwrap().to_string(),
                    fg_color,
                    bg_color,
                ));
            } else {
                new_y.push(Pixel::new(" ".to_string(), fg_color, bg_color));
            }
            contents.push(new_y);
        }
        if state.get_auto_scale().get_auto_scale_width() {
            state.set_effective_width(contents.len());
        }
        contents = add_border(
            contents,
            state.get_border_config(),
            state.get_color_config(),
        );
        state.set_effective_height(1);
        contents
    }

    /// Called when the widgets is not dropped down and enter/left mouse click occurs on it.
    fn on_press(
        &self,
        state_tree: &mut StateTree,
        callback_tree: &mut CallbackTree,
        scheduler: &mut SchedulerFrontend,
    ) -> bool {
        let consumed = self.on_press_callback(state_tree, callback_tree, scheduler);
        if consumed {
            return consumed;
        }
        let modal_id = format!("modal");
        let modal_path = format!("/root/modal");
        if state_tree.as_layout().has_modal() {
            return false;
        }

        let state = state_tree.get_mut(&self.get_path()).as_dropdown_mut();
        state.disabled.set(true);

        let position = StateCoordinates::new(
            state.get_absolute_position().usize_x(),
            state.get_absolute_position().usize_y(),
            format!("{}/position", modal_path),
            scheduler,
        );
        let new_modal_state = DroppedDownMenuState::create(
            modal_path.clone(),
            StateSize::new(
                state.get_size().get_width(),
                state.total_options() + 2,
                modal_path.clone(),
                scheduler,
            ),
            InfiniteSize::default(),
            AutoScale::new(false, false, modal_path.clone(), scheduler),
            state.get_options(),
            scheduler.new_bool_property(
                format!("{}/allow_none", modal_path).as_str(),
                state.get_allow_none(),
            ),
            SizeHint::new(None, None, modal_path.clone(), scheduler),
            position,
            Padding::new(0, 0, 0, 0, modal_path.clone(), scheduler),
            scheduler.new_horizontal_alignment_property(
                format!("{}/halign", modal_path).as_str(),
                HorizontalAlignment::Left,
            ),
            scheduler.new_vertical_alignment_property(
                format!("{}/valign", modal_path).as_str(),
                VerticalAlignment::Top,
            ),
            scheduler.new_bool_property(format!("{}/disabled", modal_path).as_str(), false),
            scheduler.new_usize_property(format!("{}/selection_order", modal_path).as_str(), 0),
            state.get_absolute_position(),
            PosHint::new(None, None, modal_path.clone(), scheduler),
            0,
            state.get_border_config().clone(),
            state.get_color_config().clone(),
            scheduler.new_string_property(
                format!("{}/choice", modal_path).as_str(),
                state.get_choice(),
            ),
            self.path.clone(),
        );
        let new_modal = DroppedDownMenu {
            id: modal_id,
            path: modal_path.clone(),
            state: new_modal_state,
        };
        let root_state = state_tree.as_layout_mut();
        root_state.update(scheduler);
        let new_states = root_state.open_modal(EzObjects::DroppedDownMenu(new_modal));
        for (path, new_state) in new_states {
            state_tree.add_node(path, new_state);
            scheduler.overwrite_callback_config(&modal_path, CallbackConfig::default());
        }
        true
    }

    fn on_hover(
        &self,
        state_tree: &mut StateTree,
        callback_tree: &mut CallbackTree,
        scheduler: &mut SchedulerFrontend,
        mouse_pos: Coordinates,
    ) -> bool {
        let consumed = self.on_hover_callback(state_tree, callback_tree, scheduler, mouse_pos);
        if consumed {
            return consumed;
        }
        scheduler.set_selected_widget(&self.path, Some(mouse_pos));
        true
    }

    fn get_clone(&self, scheduler: &mut SchedulerFrontend) -> EzObjects {
        let mut clone = self.clone();
        let mut new_state = DropdownState::new(self.path.clone(), scheduler);
        new_state.copy_state_values(self.get_state());
        clone.state = new_state;
        EzObjects::Dropdown(clone)
    }
}

impl Dropdown {
    /// Initialize an instance of this object using the passed config coming from [ez_parser]
    pub fn from_config(
        config: Vec<String>,
        id: String,
        path: String,
        scheduler: &mut SchedulerFrontend,
        file: String,
        line: usize,
    ) -> Self {
        let mut obj = Dropdown::new(id, path, scheduler);
        obj.load_ez_config(config, scheduler, file, line).unwrap();
        obj
    }
}

/// This is the menu displayed when the dropdown is actually dropped down. It is implemented as a
/// separate modal.
#[derive(Clone, Debug)]
pub struct DroppedDownMenu {
    /// ID of the widget, used to construct [path]
    pub id: String,

    /// Full path to this widget, e.g. "/root_layout/layout_2/THIS_ID"
    pub path: String,

    /// Runtime state of this widget, see [DropdownState] and [State]
    pub state: DroppedDownMenuState,
}

impl DroppedDownMenu {
    pub fn new(id: String, path: String, scheduler: &mut SchedulerFrontend) -> Self {
        DroppedDownMenu {
            id,
            path: path.clone(),
            state: DroppedDownMenuState::new(path, scheduler),
        }
    }
}

impl EzObject for DroppedDownMenu {
    fn load_ez_parameter(
        &mut self,
        _parameter_name: String,
        _parameter_value: String,
        _scheduler: &mut SchedulerFrontend,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn set_id(&mut self, id: &str) {
        self.id = id.to_string()
    }

    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn set_path(&mut self, path: &str) {
        self.path = path.to_string()
    }

    fn get_path(&self) -> String {
        self.path.clone()
    }

    fn get_state(&self) -> EzState {
        EzState::DroppedDownMenu(self.state.clone())
    }

    fn get_state_mut(&mut self) -> &mut dyn GenericState {
        &mut self.state
    }

    fn get_contents(&self, state_tree: &mut StateTree) -> PixelMap {
        let state = state_tree
            .get_mut(&self.get_path())
            .as_dropped_down_menu_mut();
        let mut options: Vec<String> = state
            .get_dropped_down_options()
            .iter()
            .map(|x| x.chars().rev().collect::<String>())
            .collect();

        let mut contents = Vec::new();
        for _ in 0..state.get_effective_size().width {
            let mut new_y = Vec::new();
            for y in 0..options.len() {
                let fg = if y == state.dropped_down_selected_row {
                    state.get_color_config().get_selection_fg_color()
                } else {
                    state.get_color_config().get_fg_color()
                };
                let bg = if y == state.dropped_down_selected_row {
                    state.get_color_config().get_selection_bg_color()
                } else {
                    state.get_color_config().get_bg_color()
                };
                if !options[y].is_empty() {
                    new_y.push(Pixel {
                        symbol: options[y].pop().unwrap().to_string(),
                        foreground_color: fg,
                        background_color: bg,
                        underline: false,
                    })
                } else {
                    new_y.push(Pixel::new(" ".to_string(), fg, bg))
                }
            }
            contents.push(new_y.clone());
        }
        let state = state_tree.get(&self.get_path()).as_dropped_down_menu();
        contents = add_padding(
            contents,
            state.get_padding(),
            state.colors.get_bg_color(),
            state.colors.get_fg_color(),
        );
        contents = add_border(
            contents,
            state.get_border_config(),
            state.get_color_config(),
        );
        contents
    }

    fn handle_event(
        &self,
        event: Event,
        state_tree: &mut StateTree,
        callback_tree: &mut CallbackTree,
        scheduler: &mut SchedulerFrontend,
    ) -> bool {
        let state = state_tree
            .get_mut(&self.get_path())
            .as_dropped_down_menu_mut();
        match event {
            Event::Key(key) => match key.code {
                KeyCode::Enter => {
                    self.handle_enter(state_tree, callback_tree, scheduler);
                    return true;
                }
                KeyCode::Down => {
                    self.handle_down(state);
                    return true;
                }
                KeyCode::Up => {
                    self.handle_up(state);
                    return true;
                }
                _ => (),
            },
            Event::Mouse(event) => {
                let mouse_position = Coordinates::new(event.column as usize, event.row as usize);
                if let MouseEventKind::Down(button) = event.kind {
                    if button == MouseButton::Left {
                        self.handle_left_click(
                            state_tree,
                            callback_tree,
                            scheduler,
                            mouse_position,
                        );
                        return true;
                    }
                } else if event.kind == MouseEventKind::Moved
                    && state.collides_effective(mouse_position)
                {
                    return self.handle_hover(state, mouse_position);
                }
            }
            _ => (),
        }
        false
    }

    fn get_clone(&self, scheduler: &mut SchedulerFrontend) -> EzObjects {
        let mut clone = self.clone();
        let mut new_state = DroppedDownMenuState::new(self.path.clone(), scheduler);
        new_state.copy_state_values(self.get_state());
        clone.state = new_state;
        EzObjects::DroppedDownMenu(clone)
    }
}

impl DroppedDownMenu {
    /// Called when this widget is already dropped down and enter is pressed
    pub fn handle_enter(
        &self,
        state_tree: &mut StateTree,
        callback_tree: &mut CallbackTree,
        scheduler: &mut SchedulerFrontend,
    ) {
        let selected = state_tree
            .get(&self.get_path())
            .as_dropped_down_menu()
            .dropped_down_selected_row;
        let choice = state_tree
            .get(&self.get_path())
            .as_dropped_down_menu()
            .get_dropped_down_options()[selected]
            .clone();
        let parent = state_tree
            .get(&self.get_path())
            .as_dropped_down_menu()
            .parent_path
            .clone();
        let state = state_tree.get_mut(&parent).as_dropdown_mut();
        state.set_choice(&choice);
        let state = state_tree.as_layout_mut();
        state.dismiss_modal(scheduler);
        state.update(scheduler);
        if let Some(ref mut i) = callback_tree.get_mut(&parent).obj.on_value_change {
            let context = Context::new(parent.clone(), state_tree, scheduler);
            i(context);
        }
    }

    /// Called when this widget is already dropped down and up is pressed
    pub fn handle_up(&self, state: &mut DroppedDownMenuState) {
        if state.dropped_down_selected_row == 0 {
            state.set_dropped_down_selected_row(state.total_options() - 1);
        } else {
            state.set_dropped_down_selected_row(state.dropped_down_selected_row - 1);
        }
    }

    /// Called when this widget is already dropped down and down is pressed
    pub fn handle_down(&self, state: &mut DroppedDownMenuState) {
        if state.dropped_down_selected_row == state.total_options() - 1 {
            state.set_dropped_down_selected_row(0);
        } else {
            state.set_dropped_down_selected_row(state.dropped_down_selected_row + 1);
        }
    }

    /// Called when this widget is already dropped down and widget is left clicked
    pub fn handle_left_click(
        &self,
        state_tree: &mut StateTree,
        callback_tree: &mut CallbackTree,
        scheduler: &mut SchedulerFrontend,
        pos: Coordinates,
    ) {
        let state = state_tree.get(&self.get_path()).as_dropped_down_menu();
        let parent = state.parent_path.clone();
        if state.collides_effective(pos) {
            let clicked_row = pos.y - state.absolute_position.usize_y();
            // Check if not click on border
            if clicked_row <= state.get_effective_size().height {
                let choice = state.get_dropped_down_options()[clicked_row - 1].clone();
                let state = state_tree.get_mut(&parent).as_dropdown_mut();
                state.set_choice(&choice);
                state.set_disabled(false);
                scheduler.dismiss_modal(state_tree);
                scheduler.force_redraw();
                if let Some(ref mut i) = callback_tree.get_mut(&parent).obj.on_value_change {
                    let context = Context::new(parent, state_tree, scheduler);
                    i(context);
                }
            }
        } else {
            let state = state_tree.as_layout_mut();
            state.dismiss_modal(scheduler);
            let state = state_tree.get_mut(&parent).as_dropdown_mut();
            state.set_disabled(false);
        }
    }

    /// Called when this widget is already dropped down and widget is hovered
    pub fn handle_hover(&self, state: &mut DroppedDownMenuState, pos: Coordinates) -> bool {
        let hovered_row = pos.y - state.absolute_position.usize_y();
        // Check if not hover on border
        if hovered_row - 1 != state.dropped_down_selected_row
            && hovered_row <= state.get_dropped_down_options().len()
        {
            state.set_dropped_down_selected_row(hovered_row - 1);
            return true;
        }
        false
    }
}
