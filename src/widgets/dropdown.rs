//! # Dropdown Widget
//! Widget which supports and arbitrary amount of possible values of which one can be chosen at any
//! time. The active value is always displayed, and when selected drops down all other possible
//! values for the user to select.
use std::io::{Error, ErrorKind};
use crossterm::event::{Event, KeyCode, MouseButton, MouseEventKind};
use crate::{CallbackConfig, EzContext};
use crate::states::definitions::{StateSize, AutoScale, SizeHint, Padding, PosHint,
                                 StateCoordinates, HorizontalAlignment, VerticalAlignment};
use crate::states::dropdown_state::{DropdownState, DroppedDownMenuState};
use crate::states::ez_state::{EzState, GenericState};
use crate::widgets::ez_object::{self, EzObject};
use crate::parser::load_common_properties::load_common_property;
use crate::parser::load_base_properties::{load_ez_bool_property, load_ez_string_property};
use crate::property::ez_values::EzValues;
use crate::run::definitions::{CallbackTree, Coordinates, Pixel, PixelMap, StateTree};
use crate::scheduler::scheduler::Scheduler;
use crate::widgets::helper_functions::{add_border, add_padding};


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

    pub fn new(id: String, path: String, scheduler: &mut Scheduler) -> Self {

        Dropdown {
            id,
            path: path.clone(),
            state: DropdownState::new(path, scheduler),
        }
    }

    fn load_allow_none_property(&mut self, parameter_value: &str, scheduler: &mut Scheduler)
        -> Result<(), Error>{

        let path = self.path.clone();
        self.state.set_allow_none(load_ez_bool_property(
            parameter_value.trim(), scheduler, path.clone(),
            Box::new(move |state_tree: &mut StateTree, val: EzValues| {
                let state = state_tree.get_by_path_mut(&path)
                    .as_dropdown_mut();
                state.set_allow_none(*val.as_bool());
                path.clone()
            }))?);
        Ok(())
    }

    fn load_choice_property(&mut self, parameter_value: &str, scheduler: &mut Scheduler)
        -> Result<(), Error> {

        let path = self.path.clone();
        self.state.set_choice(load_ez_string_property(
            parameter_value.trim(), scheduler, path.clone(),
            Box::new(move |state_tree: &mut StateTree, val: EzValues| {
                let state = state_tree.get_by_path_mut(&path)
                    .as_dropdown_mut();
                state.set_choice(val.as_string().clone());
                path.clone()
            }))?);
        Ok(())
    }
}

impl EzObject for Dropdown {

    fn load_ez_parameter(&mut self, parameter_name: String, parameter_value: String,
                         scheduler: &mut Scheduler) -> Result<(), Error> {

        let consumed = load_common_property(
            &parameter_name, parameter_value.clone(),self, scheduler)?;
        if consumed { return Ok(()) }
        match parameter_name.as_str() {
            "allow_none" => self.load_allow_none_property(parameter_value.trim(),
                                                          scheduler)?,
            "options" => {
                self.state.options = parameter_value.split(',')
                    .map(|x| x.trim().to_string()).collect();
            },
            "choice" => self.load_choice_property(parameter_value.trim(), scheduler)?,
            _ => return Err(
                Error::new(ErrorKind::InvalidData,
                           format!("Invalid parameter name for dropdown: {}", parameter_name)))
        }
        Ok(())
    }

    fn set_id(&mut self, id: String) { self.id = id }

    fn get_id(&self) -> String { self.id.clone() }

    fn set_full_path(&mut self, path: String) { self.path = path }

    fn get_full_path(&self) -> String { self.path.clone() }

    fn get_state(&self) -> EzState { EzState::Dropdown(self.state.clone()) }

    fn get_state_mut(&mut self) -> &mut dyn GenericState { &mut self.state }

    /// Content of this widget depends on whether it is currently dropped down or not. If not,
    /// then display a label with a border representing the currently selected value. If dropped
    /// down show a list of all options, with the currently selected one on top.
    fn get_contents(&self, state_tree: &mut StateTree) -> PixelMap {

        let state =
            state_tree.get_by_path_mut(&self.get_full_path()).as_dropdown_mut();
        // If dropped down get full content instead
        // Set a default value if user didn't give one
        let mut active = state.choice.clone();
        if active.value.is_empty() && !state.allow_none.value {
            active.set(state.options.first()
                .expect("Dropdown widget must have at least one option").to_string());
        }
        // Create a bordered label representing currently active value
        let (fg_color, bg_color) = state.get_context_colors();
        let mut text = active.value.chars().rev().collect::<String>();
        let mut contents = Vec::new();

        let write_width = if state.get_size().infinite_width { text.len() }
                                else {state.get_effective_size().width };

        for _ in 0..write_width {
            let mut new_y = Vec::new();
            if !text.is_empty() {
                new_y.push(Pixel::new(text.pop().unwrap().to_string(),
                                                 fg_color, bg_color));
            } else {
                new_y.push(Pixel::new(" ".to_string(), fg_color,
                                                 bg_color));
            }
            contents.push(new_y);
        }
        contents = add_border(contents, state.get_border_config());
        state.set_effective_height(1);
        contents
    }


    /// Called when the widgets is not dropped down and enter/left mouse click occurs on it.
    fn on_press(&self, state_tree: &mut StateTree, _callback_tree: &mut CallbackTree,
                scheduler: &mut Scheduler) -> bool {

        let state = state_tree.get_by_path(&self.get_full_path())
            .as_dropdown();
        let modal_id = format!("{}_modal", self.get_id());
        let modal_path = format!("/modal/{}", modal_id);
        let position =
            StateCoordinates::new(
                state.get_absolute_position().x, state.get_absolute_position().y,
                format!("{}/position", modal_path), scheduler);
        let new_modal_state = DroppedDownMenuState {
            path: modal_path.clone(),
            size: StateSize::new(
                state.get_size().width.value, state.total_options() + 2,
                modal_path.clone(), scheduler),
            auto_scale: AutoScale::new(
                false, false, modal_path.clone(), scheduler),
            options: state.get_options(),
            allow_none: scheduler.new_bool_property(
                format!("{}/allow_none", modal_path).as_str(), state.allow_none.value),
            size_hint: SizeHint::new(None, None,
                                     modal_path.clone(), scheduler),
            position,
            padding: Padding::new(0, 0, 0, 0, modal_path.clone(),
                                  scheduler),
            halign: scheduler.new_horizontal_alignment_property(
                format!("{}/halign", modal_path).as_str(), HorizontalAlignment::Left),
            valign: scheduler.new_vertical_alignment_property(
                format!("{}/valign", modal_path).as_str(), VerticalAlignment::Top),
            disabled: scheduler.new_bool_property(
                format!("{}/disabled", modal_path).as_str(),false),
            selection_order: scheduler.new_usize_property(
                format!("{}/selection_order", modal_path).as_str(), 0),
            absolute_position: state.get_absolute_position(),
            pos_hint: PosHint::new(None, None, modal_path.clone(), scheduler),
            dropped_down_selected_row: 1,
            border_config: state.border_config.clone(),
            colors: state.colors.clone(),
            choice: scheduler.new_string_property(format!("{}/choice", modal_path).as_str(),
                                                  state.get_choice().value.clone()),
            parent_path: self.path.clone(),
        };
        let new_modal = DroppedDownMenu {
            id: modal_id,
            path: modal_path.clone(),
            state: new_modal_state,
        };
        let root_state = state_tree.get_by_path_mut("/root").as_layout_mut();
        let (_, extra_state_tree) = root_state
            .open_modal(ez_object::EzObjects::DroppedDownMenu(new_modal));
        root_state.update(scheduler);
        state_tree.extend(extra_state_tree);
        scheduler.set_callback_config(&modal_path, CallbackConfig::default());
        true
    }

    fn on_hover(&self, state_tree: &mut StateTree, callback_tree: &mut CallbackTree,
                scheduler: &mut Scheduler, mouse_pos: Coordinates) -> bool {

        scheduler.set_selected_widget(&self.path, Some(mouse_pos));
        self.on_hover_callback(state_tree, callback_tree, scheduler, mouse_pos);
        true
    }
}

impl Dropdown {

    /// Initialize an instance of this object using the passed config coming from [ez_parser]
    pub fn from_config(config: Vec<String>, id: String, path: String, scheduler: &mut Scheduler,
                       file: String, line: usize) -> Self {

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

    pub fn new(id: String, path: String, scheduler: &mut Scheduler) -> Self {

        DroppedDownMenu {
            id,
            path: path.clone(),
            state: DroppedDownMenuState::new(path, scheduler),
        }
    }
}

impl EzObject for DroppedDownMenu {

    fn load_ez_parameter(&mut self, _parameter_name: String, _parameter_value: String,
                         _scheduler: &mut Scheduler) -> Result<(), Error> { Ok(()) }

    fn set_id(&mut self, id: String) { self.id = id }

    fn get_id(&self) -> String { self.id.clone() }

    fn set_full_path(&mut self, path: String) { self.path = path }

    fn get_full_path(&self) -> String { self.path.clone() }

    fn get_state(&self) -> EzState { EzState::DroppedDownMenu(self.state.clone()) }

    fn get_state_mut(&mut self) -> &mut dyn GenericState { &mut self.state }

    fn get_contents(&self, state_tree: &mut StateTree) -> PixelMap {

        let state = state_tree
            .get_by_path_mut(&self.get_full_path()).as_dropped_down_menu_mut();
        let mut options:Vec<String> = state.get_dropped_down_options().iter()
            .map(|x| x.chars().rev().collect::<String>()).collect();

        let mut contents = Vec::new();
        for _ in 0..state.get_effective_size().width {
            let mut new_y = Vec::new();
            for y in 0..options.len() {
                let fg = if y == state.dropped_down_selected_row
                {state.get_color_config().selection_foreground.value }
                else {state.get_color_config().foreground.value };
                let bg = if y == state.dropped_down_selected_row
                {state.get_color_config().selection_background.value }
                else {state.get_color_config().background.value };
                if !options[y].is_empty(){
                    new_y.push(Pixel{symbol: options[y].pop().unwrap().to_string(),
                        foreground_color: fg, background_color: bg, underline: false})
                } else {
                    new_y.push(Pixel::new(" ".to_string(), fg,
                                                     bg))
                }
            }
            contents.push(new_y.clone());

        }
        let state = state_tree
            .get_by_path(&self.get_full_path()).as_dropped_down_menu();
        contents = add_padding(
            contents, state.get_padding(), state.colors.background.value,
            state.colors.foreground.value);
        contents = add_border(contents, state.get_border_config());
        contents
    }

    fn handle_event(&self, event: Event, state_tree: &mut StateTree,
                    callback_tree: &mut CallbackTree, scheduler: &mut Scheduler) -> bool {

        let state = state_tree.get_by_path_mut(&self.get_full_path())
            .as_dropped_down_menu_mut();
        match event {
            Event::Key(key) => {
                match key.code {
                    KeyCode::Enter => {
                        self.handle_enter(
                            state_tree, callback_tree, scheduler);
                        return true
                    }
                    KeyCode::Down => {
                        self.handle_down(state);
                        return true
                    },
                    KeyCode::Up => {
                        self.handle_up(state);
                        return true
                    },
                    _ => ()
                }
            }
            Event::Mouse(event) => {
                let mouse_position = Coordinates::new(
                    event.column as usize,event.row as usize);
                if let MouseEventKind::Down(button) = event.kind {
                    if button == MouseButton::Left {
                        self.handle_left_click(state_tree, callback_tree,
                                               scheduler,mouse_position);
                        return true
                    }
                } else if event.kind == MouseEventKind::Moved &&
                    state.collides(mouse_position) {
                    return self.handle_hover(state, mouse_position)
                }
            },
            _ => ()
        }
        false
    }
}

impl DroppedDownMenu {

    /// Called when this widget is already dropped down and enter is pressed
    pub fn handle_enter(&self, state_tree: &mut StateTree, callback_tree: &mut CallbackTree,
                        scheduler: &mut Scheduler) {

        let selected = state_tree.get_by_path(&self.get_full_path())
            .as_dropped_down_menu().dropped_down_selected_row;
        let choice = state_tree
            .get_by_path(&self.get_full_path()).as_dropped_down_menu()
            .get_dropped_down_options()[selected].clone();
        let parent = state_tree.get_by_path(&self.get_full_path())
            .as_dropped_down_menu().parent_path.clone();
        let state = state_tree.get_by_path_mut(&parent).as_dropdown_mut();
        state.set_choice(choice);
        state.update(scheduler);
        let state = state_tree.get_by_path_mut("/root").as_layout_mut();
        state.dismiss_modal(scheduler);
        state.update(scheduler);
        if let Some(ref mut i) =
        callback_tree.get_by_path_mut(&parent).on_value_change {
            let context = EzContext::new(parent.clone(), state_tree, scheduler);
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
    pub fn handle_left_click(&self, state_tree: &mut StateTree, callback_tree: &mut CallbackTree,
                             scheduler: &mut Scheduler, pos: Coordinates) {

        let state = state_tree.get_by_path(&self.get_full_path())
            .as_dropped_down_menu();
        let parent = state.parent_path.clone();
        if state.collides(pos) {
            let clicked_row = pos.y - state.absolute_position.y;
            // Check if not click on border
            if clicked_row != 0 && clicked_row <= state.get_effective_size().height - 2 {
                let choice = state.get_dropped_down_options()[clicked_row - 1]
                    .clone();
                let state = state_tree.get_by_path_mut(&parent).as_dropdown_mut();
                state.set_choice(choice);
                state.update(scheduler);
                let state = state_tree.get_by_path_mut("/root").as_layout_mut();
                state.dismiss_modal(scheduler);
                state.update(scheduler);
                if let Some(ref mut i) = callback_tree
                    .get_by_path_mut(&parent).on_value_change {
                    let context = EzContext::new(parent, state_tree, scheduler);
                    i(context);
                }
            }
        } else {
            let state = state_tree.get_by_path_mut("/root").as_layout_mut();
            state.dismiss_modal(scheduler);
            state.update(scheduler);
        }
    }

    /// Called when this widget is already dropped down and widget is hovered
    pub fn handle_hover(&self, state: &mut DroppedDownMenuState,
                        pos: Coordinates) -> bool {
        let hovered_row = pos.y - state.absolute_position.y;
        // Check if not hover on border
        if hovered_row != 0 && hovered_row - 1 != state.dropped_down_selected_row &&
            hovered_row <= state.get_dropped_down_options().len() {
            state.set_dropped_down_selected_row(hovered_row - 1);
            return true
        }
        false
    }
}
