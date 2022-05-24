//! # Radio button Widget
//! Widget which can only be turned on. It should be in a group of other radio buttons using the
//! same 'group' field value for all. The radio buttons in a group are mutually exlusive, so when
//! one is selected the others are deselected. Supports on_value_change callback, which is only
//! called for the radio button that became active.
use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use crossterm::event::{KeyCode};
use crate::common::{KeyboardCallbackFunction, Coordinates, PixelMap, GenericEzFunction,
                    MouseCallbackFunction, EzContext, StateTree, KeyMap};
use crate::states::radio_button_state::RadioButtonState;
use crate::states::state::{EzState, SelectableState, GenericState};
use crate::widgets::widget::{EzWidget, Pixel, EzObject, EzObjects};
use crate::ez_parser::{load_bool_parameter, load_color_parameter, load_halign_parameter,
                       load_valign_parameter, load_pos_hint_x_parameter, load_pos_hint_y_parameter};


pub struct RadioButton {

    /// ID of the widget, used to construct [path]
    pub id: String,

    /// Full path to this widget, e.g. "/root_layout/layout_2/THIS_ID"
    pub path: String,

    /// [Pixel.symbol] used when the Checkbox is active
    pub active_symbol: char,

    /// [Pixel.symbol] used when the Checkbox is not active
    pub inactive_symbol: char,

    /// Global order number in which this widget will be selection when user presses down/up keys
    pub selection_order: usize,

    /// Optional function to call when the value of this widget changes, see
    /// [ValueChangeCallbackFunction] for the callback fn type, or [set_bind_on_value_change] for
    /// examples.
    pub bound_on_value_change: Option<GenericEzFunction>,
    
    /// Optional function to call when this widget is selected via keyboard up/down or mouse hover,
    /// see [set_bind_on_select] for examples.
    pub bound_on_select: Option<fn(context: EzContext, mouse_position: Option<Coordinates>)>,

    /// Optional function to call when this widget is right clicked, see
    /// [MouseCallbackFunction] for the callback fn type, or [set_right_left_click] for
    /// examples.
    pub bound_on_deselect: Option<GenericEzFunction>,
    
    /// Optional function to call when this widget is left clicked, see
    /// [MouseCallbackFunction] for the callback fn type, or [set_bind_left_click] for
    /// examples.
    pub bound_right_mouse_click: Option<MouseCallbackFunction>,

    /// A Key to callback function lookup used to store keybinds for this widget. See
    /// [KeyboardCallbackFunction] type for callback function signature.
    pub keymap: KeyMap,

    /// Group this radio button belongs to. Set the same group value for a number of radio buttons
    /// to make them mutually exclusive.
    pub group: String,

    /// Runtime state of this widget, see [RadioButtonState] and [State]
    pub state: RadioButtonState,
}

impl Default for RadioButton {
    fn default() -> Self {
        RadioButton {
            id: "".to_string(),
            path: String::new(),
            group: String::new(),
            active_symbol: 'X',
            inactive_symbol: ' ',
            selection_order: 0,
            bound_on_value_change: None,
            bound_on_select: None,
            bound_on_deselect: None,
            bound_right_mouse_click: None,
            keymap: HashMap::new(),
            state: RadioButtonState::default(),
        }
    }
}


impl EzObject for RadioButton {

    fn load_ez_parameter(&mut self, parameter_name: String, parameter_value: String)
                         -> Result<(), Error> {
        match parameter_name.as_str() {
            "x" => self.state.x = parameter_value.trim().parse().unwrap(),
            "y" => self.state.y = parameter_value.trim().parse().unwrap(),
            "pos_hint_x" => self.state.set_pos_hint_x(
                load_pos_hint_x_parameter(parameter_value.trim()).unwrap()),
            "pos_hint_y" => self.state.set_pos_hint_y(
                load_pos_hint_y_parameter(parameter_value.trim()).unwrap()),
            "halign" =>
                self.state.halign =  load_halign_parameter(parameter_value.trim()).unwrap(),
            "valign" =>
                self.state.valign =  load_valign_parameter(parameter_value.trim()).unwrap(),
            "selection_order" => {
                let order = parameter_value.trim().parse().unwrap();
                if order == 0 {
                    return Err(Error::new(ErrorKind::InvalidData,
                                          "selection_order must be higher than 0."))
                }
                self.selection_order = order;
            },
            "group" => {
                let group = parameter_value.trim();
                if group.is_empty() {
                    return Err(Error::new(ErrorKind::InvalidData,
                                          "Radio button widget must have a group."))
                }
                self.group = group.to_string();
            },
            "active" => self.state.active = load_bool_parameter(parameter_value.trim()).unwrap(),
            "active_symbol" => self.active_symbol = parameter_value.chars().last().unwrap(),
            "inactive_symbol" => self.inactive_symbol = parameter_value.chars().last().unwrap(),
            "fg_color" =>
                self.state.content_foreground_color = load_color_parameter(parameter_value).unwrap(),
            "bg_color" =>
                self.state.content_background_color = load_color_parameter(parameter_value).unwrap(),
            "selection_fg_color" =>
                self.state.selection_foreground_color = load_color_parameter(parameter_value).unwrap(),
            "selection_bg_color" =>
                self.state.selection_background_color = load_color_parameter(parameter_value).unwrap(),
            _ => return Err(Error::new(ErrorKind::InvalidData,
                                format!("Invalid parameter name for radio button {}",
                                        parameter_name)))
        }
        Ok(())
    }

    fn set_id(&mut self, id: String) { self.id = id }

    fn get_id(&self) -> String { self.id.clone() }

    fn set_full_path(&mut self, path: String) { self.path = path }

    fn get_full_path(&self) -> String { self.path.clone() }

    fn update_state(&mut self, new_state: &EzState) {
        let state = new_state.as_radio_button();
        self.state = state.clone();
        self.state.changed = false;
        self.state.force_redraw = false;
    }


    fn get_state(&self) -> EzState { EzState::RadioButton(self.state.clone()) }

    fn get_contents(&self, state_tree: &mut StateTree) -> PixelMap {

        let state =
            state_tree.get_mut(&self.get_full_path()).unwrap().as_radio_button();
        let active_symbol = { if state.active {self.active_symbol}
                                    else {self.inactive_symbol} };
        let fg_color = if state.selected {state.selection_foreground_color}
        else {state.content_foreground_color};
        let bg_color = if state.selected {state.selection_background_color}
        else {state.content_background_color};
        vec!(
            vec!(Pixel {symbol: "(".to_string(), foreground_color: fg_color,
                background_color: bg_color, underline: false}),
            vec!(Pixel {symbol: " ".to_string(), foreground_color: fg_color,
                background_color: bg_color, underline: false}),
            vec!(Pixel {symbol: active_symbol.to_string(), foreground_color: fg_color,
                background_color: bg_color, underline: false}),
            vec!(Pixel {symbol: " ".to_string(), foreground_color: fg_color,
                background_color: bg_color, underline: false}),
            vec!(Pixel {symbol: ")".to_string(), foreground_color: fg_color,
                background_color: bg_color, underline: false}),
        )
    }
}

impl EzWidget for RadioButton {
    fn is_selectable(&self) -> bool { true}

    fn is_selected(&self) -> bool { self.state.selected }

    fn get_selection_order(&self) -> usize { self.selection_order }

    fn get_key_map(&self) -> &KeyMap {
       &self.keymap
    }

    fn bind_key(&mut self, key: KeyCode, func: KeyboardCallbackFunction) {
       self.keymap.insert(key, func);
    }

    fn set_bind_on_value_change(&mut self, func: GenericEzFunction) {
        self.bound_on_value_change = Some(func)
    }

    fn get_bind_on_value_change(&self) -> Option<GenericEzFunction> { self.bound_on_value_change }

    fn on_left_click(&self, context: EzContext, _position: Coordinates) {
        self.handle_press(context);
    }

    fn on_keyboard_enter(&self, context: EzContext) { self.handle_press(context); }

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

impl RadioButton {

    /// Initialize an instance of this object using the passed config coming from [ez_parser]
    pub fn from_config(config: Vec<&str>, id: String) -> Self {
        let mut obj = RadioButton::default();
        obj.set_id(id);
        obj.load_ez_config(config).unwrap();
        obj
    }
    /// Get the group this radio button belongs to. Radio buttons that share a group are
    /// mutually exclusive.
    fn get_group(&self) -> String { self.group.clone() }

    /// Function that handles this RadioButton being pressed (mouse clicked/keyboard entered).
    fn handle_press(&self, context: EzContext) {

        // Find all other radio buttons in same group and make them inactive (mutual exclusivity)
        for widget in context.widget_tree.values() {
            if let EzObjects::RadioButton(i) = widget {
                if i.get_group() == self.group && i.get_id() != self.get_id() {
                    let other_state =
                        context.state_tree.get_mut(&i.get_full_path()).unwrap()
                            .as_radio_button_mut();
                    other_state.set_active(false);
                }
            }
        }
        // Set entered radio button to active and select it
        let state = context.state_tree.
            get_mut(&self.get_full_path()).unwrap().as_radio_button_mut();
        if !self.state.active {
            self.toggle(state);
            state.set_selected(true);
            self.on_value_change(context);
        } else {
            return // Nothing to do
        }
    }

    /// Sets active value to its' opposite
    fn toggle(&self, state: &mut RadioButtonState) {
        if state.active {
            state.set_active(false);
        } else {
            state.set_active(true);
        }
    }
}
