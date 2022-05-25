//! # Dropdown Widget
//! Widget which supports and arbitrary amount of possible values of which one can be chosen at any
//! time. The active value is always displayed, and when selected drops down all other possible
//! values for the user to select.
use std::io::{Error, ErrorKind};
use crossterm::event::{Event, KeyCode, MouseButton, MouseEventKind};
use crate::common::{self, KeyboardCallbackFunction, GenericEzFunction, PixelMap,
                    MouseCallbackFunction, EzContext, StateTree, KeyMap};
use crate::states::dropdown_state::DropdownState;
use crate::states::state::{EzState, GenericState, SelectableState, Coordinates};
use crate::widgets::widget::{EzWidget, Pixel, EzObject};
use crate::ez_parser;


#[derive(Default)]
pub struct Dropdown {

    /// ID of the widget, used to construct [path]
    pub id: String,

    /// Full path to this widget, e.g. "/root_layout/layout_2/THIS_ID"
    pub path: String,

    /// Global order number in which this widget will be selection when user presses down/up keys
    pub selection_order: usize,
    
    /// Optional function to call when this widget is selected via keyboard up/down or mouse hover,
    /// see [set_bind_on_select] for examples.
    pub bound_on_select: Option<fn(context: EzContext, mouse_position: Option<Coordinates>)>,

    /// Optional function to call when this widget is right clicked, see
    /// [MouseCallbackFunction] for the callback fn type, or [set_right_left_click] for
    /// examples.
    pub bound_on_deselect: Option<GenericEzFunction>,
    
    /// Optional function to call when this widget is right clicked, see
    /// [MouseCallbackFunction] for the callback fn type, or [set_bind_right_click] for
    /// examples.
    pub bound_right_mouse_click: Option<MouseCallbackFunction>,

    /// Optional function to call when the value of this widget changes, see
    /// [ValueChangeCallbackFunction] for the callback fn type, or [set_bind_on_value_change] for
    /// examples.
    pub bound_on_value_change: Option<GenericEzFunction>,

    /// A Key to callback function lookup used to store keybinds for this widget. See
    /// [KeyboardCallbackFunction] type for callback function signature.
    pub keymap: KeyMap,

    /// Runtime state of this widget, see [DropdownState] and [State]
    pub state: DropdownState,
}


impl EzObject for Dropdown {

    fn load_ez_parameter(&mut self, parameter_name: String, parameter_value: String)
                         -> Result<(), Error> {

        match parameter_name.as_str() {
            "x" => self.state.set_x(parameter_value.trim().parse().unwrap()),
            "y" => self.state.set_y(parameter_value.trim().parse().unwrap()),
            "pos" => self.state.set_position(
                ez_parser::load_pos_parameter(parameter_value.trim()).unwrap()),
            "size_hint_x" => self.state.size_hint_x =
                ez_parser::load_size_hint_parameter(parameter_value.trim()).unwrap(),
            "pos_hint_x" => self.state.set_pos_hint_x(
                ez_parser::load_pos_hint_x_parameter(parameter_value.trim()).unwrap()),
            "pos_hint_y" => self.state.set_pos_hint_y(
                ez_parser::load_pos_hint_y_parameter(parameter_value.trim()).unwrap()),
            "width" => self.state.width = parameter_value.trim().parse().unwrap(),
            "padding_top" => self.state.padding_top = parameter_value.trim().parse().unwrap(),
            "padding_bottom" => self.state.padding_bottom = parameter_value.trim().parse().unwrap(),
            "padding_left" => self.state.padding_left = parameter_value.trim().parse().unwrap(),
            "padding_right" => self.state.padding_right = parameter_value.trim().parse().unwrap(),
            "halign" =>
                self.state.halign =  ez_parser::load_halign_parameter(parameter_value.trim()).unwrap(),
            "valign" =>
                self.state.valign =  ez_parser::load_valign_parameter(parameter_value.trim()).unwrap(),
            "selection_order" => {
                self.selection_order = ez_parser::load_selection_order_parameter(
                    parameter_value.as_str()).unwrap();
            },
            "border" => self.state.set_border(ez_parser::load_bool_parameter(parameter_value.trim())?),
            "border_horizontal_symbol" => self.state.border_config.horizontal_symbol =
                parameter_value.trim().to_string(),
            "border_vertical_symbol" => self.state.border_config.vertical_symbol =
                parameter_value.trim().to_string(),
            "border_top_right_symbol" => self.state.border_config.top_right_symbol =
                parameter_value.trim().to_string(),
            "border_top_left_symbol" => self.state.border_config.top_left_symbol =
                parameter_value.trim().to_string(),
            "border_bottom_left_symbol" => self.state.border_config.bottom_left_symbol =
                parameter_value.trim().to_string(),
            "border_bottom_right_symbol" => self.state.border_config.bottom_right_symbol =
                parameter_value.trim().to_string(),
            "border_fg_color" =>
                self.state.border_config.fg_color = ez_parser::load_color_parameter(parameter_value).unwrap(),
            "border_bg_color" =>
                self.state.border_config.bg_color = ez_parser::load_color_parameter(parameter_value).unwrap(),
            "fg_color" =>
                self.state.colors.foreground =
                    ez_parser::load_color_parameter(parameter_value).unwrap(),
            "bg_color" =>
                self.state.colors.background =
                    ez_parser::load_color_parameter(parameter_value).unwrap(),
            "selection_fg_color" =>
                self.state.colors.selection_foreground =
                    ez_parser::load_color_parameter(parameter_value).unwrap(),
            "selection_bg_color" =>
                self.state.colors.selection_background =
                    ez_parser::load_color_parameter(parameter_value).unwrap(),
            "allow_none" =>
                self.state.allow_none = ez_parser::load_bool_parameter(parameter_value.trim()).unwrap(),
            "options" => {
                self.state.options = parameter_value.split(',')
                    .map(|x| x.trim().to_string()).collect();
            },
            "active" => {
                self.state.choice = parameter_value.trim().to_string();
            }
            _ => return Err(Error::new(ErrorKind::InvalidData,
                                format!("Invalid parameter name for dropdown {}",
                                        parameter_name)))
        }
        Ok(())
    }

    fn set_id(&mut self, id: String) { self.id = id }

    fn get_id(&self) -> String { self.id.clone() }

    fn set_full_path(&mut self, path: String) { self.path = path }

    fn get_full_path(&self) -> String { self.path.clone() }

    fn update_state(&mut self, new_state: &EzState) {
        let state = new_state.as_dropdown();
        self.state = state.clone();
        self.state.changed = false;
        self.state.force_redraw = false;
    }

    fn get_state(&self) -> EzState { EzState::Dropdown(self.state.clone()) }

    /// Content of this widget depends on whether it is currently dropped down or not. If not,
    /// then display a label with a border representing the currently selected value. If dropped
    /// down show a list of all options, with the currently selected one on top.
    fn get_contents(&self, state_tree: &mut StateTree) -> PixelMap {

        let state =
            state_tree.get_mut(&self.get_full_path()).unwrap().as_dropdown_mut();
        // If dropped down get full content instead
        if state.dropped_down {
            return self.get_dropped_down_contents(state)
        }
        // Set a default value if user didn't give one
        let mut active = state.choice.clone();
        if active.is_empty() && !state.allow_none {
            active = state.options.first()
                .expect("Dropdown widget must have at least one option").to_string(); // todo move to validation
        }
        // Create a bordered label representing currently active value
        let fg_color = if state.selected {state.get_colors().selection_foreground }
        else {state.get_colors().foreground };
        let bg_color = if state.selected {state.get_colors().selection_background }
        else {state.get_colors().background };
        let mut text = active.chars().rev().collect::<String>();
        let mut contents = Vec::new();
        for _ in 0..state.get_effective_width() {
            let mut new_y = Vec::new();
            if !text.is_empty() {
                new_y.push(Pixel{symbol: text.pop().unwrap().to_string(),
                    foreground_color: fg_color, background_color: bg_color, underline: false})
            } else {
                new_y.push(Pixel{symbol: " ".to_string(), foreground_color: fg_color,
                    background_color: bg_color, underline: false})
            }
            contents.push(new_y);
        }
        contents = common::add_border(contents, state.get_border_config());
        contents
    }
}

impl EzWidget for Dropdown {
    fn set_focus(&mut self, enabled: bool) { self.state.focussed = enabled }

    fn get_focus(&self) -> bool { self.state.focussed }

    fn is_selectable(&self) -> bool { true }

    fn is_selected(&self) -> bool { self.state.selected }

    fn get_selection_order(&self) -> usize { self.selection_order }

    fn get_key_map(&self) -> &KeyMap {
       &self.keymap
    }

    fn bind_key(&mut self, key: KeyCode, func: KeyboardCallbackFunction) {
       self.keymap.insert(key, func);
    }

    fn handle_event(&self, event: Event, context: EzContext) -> bool {
        let state = context.state_tree.get_mut(&self.get_full_path()).unwrap()
            .as_dropdown_mut();
        match event {
            Event::Key(key) => {
                match key.code {
                    KeyCode::Enter => {
                        self.handle_enter(context);
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
                let mouse_position = Coordinates::new(event.column as usize, 
                                                                 event.row as usize);
                if let MouseEventKind::Down(button) = event.kind {
                    if button == MouseButton::Left {
                        self.handle_left_click(context, mouse_position);
                        return true
                    }
                } else if event.kind == MouseEventKind::Moved &&
                        state.collides(mouse_position) {
                    self.handle_hover(state, mouse_position);
                    return true
                }
            },
            _ => ()
        }
        false
    }

    fn set_bind_on_value_change(&mut self, func: GenericEzFunction) {
        self.bound_on_value_change = Some(func)
    }

    fn get_bind_on_value_change(&self) -> Option<GenericEzFunction> { self.bound_on_value_change }

    fn on_left_click(&self, context: EzContext, _position: Coordinates) { self.on_press(context); }

    fn on_keyboard_enter(&self, context: EzContext) { self.on_press(context); }

    fn set_bind_right_click(&mut self, func: MouseCallbackFunction) {
        self.bound_right_mouse_click = Some(func)
    }

    fn get_bind_right_click(&self) -> Option<MouseCallbackFunction> { self.bound_right_mouse_click }

    fn set_bind_on_select(&mut self, func: fn(EzContext, Option<Coordinates>)) {
        self.bound_on_select = Some(func);
    }

    fn get_bind_on_select(&self) -> Option<fn(EzContext, Option<Coordinates>)> {
        self.bound_on_select
    }

}

impl Dropdown {

    /// Initialize an instance of this object using the passed config coming from [ez_parser]
    pub fn from_config(config: Vec<&str>, id: String) -> Self {
        let mut obj = Dropdown::default();
        obj.set_id(id);
        obj.load_ez_config(config).unwrap();
        obj
    }

    /// Called when this widget is already dropped down and enter is pressed
    fn handle_enter(&self, context: EzContext) {
        let state = context.state_tree.get_mut(
            &self.get_full_path()).unwrap().as_dropdown_mut();
        state.set_selected(true);
        let choice = self.get_dropped_down_options(state)
            [state.dropped_down_selected_row].clone();
        state.set_choice(choice);
        self.exit_focus(context);
    }

    /// Called when this widget is already dropped down and up is pressed
    fn handle_up(&self, state: &mut DropdownState) {

        if state.dropped_down_selected_row == 0 {
            state.set_dropped_down_selected_row(self.state.total_options() - 1);
        } else {
            state.set_dropped_down_selected_row(self.state.dropped_down_selected_row - 1);
        }
    }

    /// Called when this widget is already dropped down and down is pressed
    fn handle_down(&self, state: &mut DropdownState) {
        if state.dropped_down_selected_row == self.state.total_options() - 1 {
            state.set_dropped_down_selected_row(0);
        } else {
            state.set_dropped_down_selected_row(self.state.dropped_down_selected_row + 1);
        }
    }

    /// Called when this widget is already dropped down and widget is left clicked
    fn handle_left_click(&self, context: EzContext, pos: Coordinates) {

        let state = context.state_tree.get_mut(
            &self.get_full_path()).unwrap().as_dropdown_mut();
        if state.collides(pos) {
            let clicked_row = pos.y - state.absolute_position.y;
            // Check if not click on border
            if clicked_row != 0 && clicked_row <= self.state.get_height() {
                let choice = self.get_dropped_down_options(state)[clicked_row - 1]
                    .clone();
                state.set_choice(choice);
            }
        } else {
            // Click outside widget
            state.set_selected(false);
        }
        self.exit_focus(context);
    }

    /// Called when this widget is already dropped down and widget is hovered
    fn handle_hover(&self, state: &mut DropdownState, pos: Coordinates) {
        let hovered_row = pos.y - state.absolute_position.y;
        // Check if not hover on border
        if hovered_row - 1 != state.dropped_down_selected_row &&
        hovered_row != 0 && hovered_row != self.state.get_height() + 1 {
            state.set_dropped_down_selected_row(hovered_row - 1);
        }
    }

    /// Called when widget leaves dropdown mode. Forces a screen redraw because dropping down may
    /// have overwritten other widgets.
    fn exit_focus(&self, context: EzContext) {
        let state = context.state_tree.get_mut(
            &self.get_full_path()).unwrap().as_dropdown_mut();
        state.set_focussed(false);
        state.set_dropped_down(false);
        state.set_force_redraw(true);
        if state.choice != self.state.choice {
            self.on_value_change(context);
        }
    }

    /// Called when the widgets is not dropped down and enter/left mouse click occurs on it.
    fn on_press(&self, context: EzContext) {
        let state = context.state_tree.get_mut(&self.get_full_path()).unwrap()
            .as_dropdown_mut();
        state.set_dropped_down_selected_row(1);
        state.set_dropped_down(true);
        state.set_focussed(true);
        state.set_selected(true);
    }

    /// Get an ordered list of options, including the empty option if it was allowed. Order is:
    /// - Active choice
    /// - Empty (if allowed)
    /// - Rest of the options in user defined order
    fn get_dropped_down_options(&self, state: &mut DropdownState) -> Vec<String> {
        let mut options = vec!(state.choice.clone());
        if state.allow_none && !state.choice.is_empty() {
            options.push("".to_string())
        }
        for option in state.options.iter()
            .filter(|x| x.to_string() != state.choice) {
            options.push(option.to_string());
        }
        options
    }

    /// Return a PixelMap of this widgets' content in dropped down mode. I.e. a menu of options
    /// for the user to choose from.
    fn get_dropped_down_contents(&self, state: &mut DropdownState) -> PixelMap {

        let mut options:Vec<String> = self.get_dropped_down_options(state).iter()
            .map(|x| x.chars().rev().collect::<String>()).collect();

        let mut contents = Vec::new();
        for _ in 0..state.get_width() - 2{
            let mut new_y = Vec::new();
            for y in 0..options.len() {
                let fg = if y == state.dropped_down_selected_row
                {state.get_colors().selection_foreground }
                else {state.get_colors().foreground };
                let bg = if y == state.dropped_down_selected_row
                {state.get_colors().selection_background }
                else {state.get_colors().background };
                if !options[y].is_empty(){
                    new_y.push(Pixel{symbol: options[y].pop().unwrap().to_string(),
                        foreground_color: fg, background_color: bg, underline: false})
                } else {
                    new_y.push(Pixel{symbol: " ".to_string(), foreground_color: fg,
                        background_color: bg, underline: false})
                }
            }
            contents.push(new_y.clone());

        }
        contents = common::add_border(contents, state.get_border_config());
        contents = common::add_padding(contents, state.padding_top,
                                       state.padding_bottom,
                                       state.padding_left, state.padding_right,
                                       state.get_colors().background,
                                       state.get_colors().foreground);
        contents
    }
}
