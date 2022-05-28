//! # Checkbox Widget
//! Widget which is either on or off and implements an on_value_change callback.
use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use crossterm::event::{KeyCode};
use crate::common;
use crate::widgets::widget::{EzWidget, Pixel, EzObject};
use crate::states::checkbox_state::CheckboxState;
use crate::states::state::{self, GenericState, SelectableState};
use crate::ez_parser;

pub struct Checkbox {

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
    pub bound_on_value_change: Option<common::GenericEzFunction>,
    
    /// Optional function to call when this widget is selected via keyboard up/down or mouse hover,
    /// see [set_bind_on_select] for examples.
    pub bound_on_select: Option<fn(context: common::EzContext, mouse_position: Option<state::Coordinates>)>,

    /// Optional function to call when this widget is right clicked, see
    /// [MouseCallbackFunction] for the callback fn type, or [set_right_left_click] for
    /// examples.
    pub bound_on_deselect: Option<common::GenericEzFunction>,
    
    /// Optional function to call when this widget is right clicked, see
    /// [MouseCallbackFunction] for the callback fn type, or [set_bind_right_click] for
    /// examples.
    pub bound_right_mouse_click: Option<common::MouseCallbackFunction>,

    /// A Key to callback function lookup used to store keybinds for this widget. See
    /// [KeyboardCallbackFunction] type for callback function signature.
    pub keymap: common::KeyMap,

    /// Runtime state of this widget, see [CheckboxState] and [State]
    pub state: CheckboxState,
}

impl Default for Checkbox {
    fn default() -> Self {
        Checkbox {
            id: "".to_string(),
            path: String::new(),
            active_symbol: 'X',
            inactive_symbol: ' ',
            selection_order: 0,
            bound_on_value_change: None,
            bound_on_select: None,
            bound_on_deselect: None,
            bound_right_mouse_click: None,
            keymap: HashMap::new(),
            state: CheckboxState::default(),
        }
    }
}


impl EzObject for Checkbox {

    fn load_ez_parameter(&mut self, parameter_name: String, parameter_value: String)
                         -> Result<(), Error> {

        match parameter_name.as_str() {
            "id" => self.set_id(parameter_value.trim().to_string()),
            "x" => self.state.set_x(parameter_value.trim().parse().unwrap()),
            "y" => self.state.set_y(parameter_value.trim().parse().unwrap()),
            "pos" => self.state.set_position(
                ez_parser::load_pos_parameter(parameter_value.trim()).unwrap()),
            "pos_hint" => self.state.set_pos_hint(
                ez_parser::load_full_pos_hint_parameter(parameter_value.trim()).unwrap()),
            "pos_hint_x" => self.state.set_pos_hint_x(
                ez_parser::load_pos_hint_x_parameter(parameter_value.trim()).unwrap()),
            "pos_hint_y" => self.state.set_pos_hint_y(
                ez_parser::load_pos_hint_y_parameter(parameter_value.trim()).unwrap()),
            "padding" => self.state.set_padding(ez_parser::load_full_padding_parameter(
                parameter_value.trim())?),
            "padding_x" => self.state.set_padding(ez_parser::load_padding_x_parameter(
                parameter_value.trim())?),
            "padding_y" => self.state.set_padding(ez_parser::load_padding_y_parameter(
                parameter_value.trim())?),
            "padding_top" => self.state.set_padding_top(parameter_value.trim().parse().unwrap()),
            "padding_bottom" => self.state.set_padding_bottom(parameter_value.trim().parse().unwrap()),
            "padding_left" => self.state.set_padding_left(parameter_value.trim().parse().unwrap()),
            "padding_right" => self.state.set_padding_right(parameter_value.trim().parse().unwrap()),
            "halign" =>
                self.state.halign =  ez_parser::load_halign_parameter(parameter_value.trim()).unwrap(),
            "valign" =>
                self.state.valign =  ez_parser::load_valign_parameter(parameter_value.trim()).unwrap(),
            "selection_order" => {
                let order = parameter_value.trim().parse().unwrap();
                if order == 0 {
                    return Err(Error::new(ErrorKind::InvalidData,
                                          "selection_order must be higher than 0."))
                }
                self.selection_order = order;
            },
            "active" =>
                self.state.active = ez_parser::load_bool_parameter(parameter_value.trim()).unwrap(),
            "active_symbol" => self.active_symbol = parameter_value.chars().last().unwrap(),
            "inactive_symbol" => self.inactive_symbol = parameter_value.chars().last().unwrap(),
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
            _ => return Err(Error::new(ErrorKind::InvalidData,
                                format!("Invalid parameter name for check box {}",
                                        parameter_name)))
        }
        Ok(())
    }

    fn set_id(&mut self, id: String) { self.id = id }

    fn get_id(&self) -> String { self.id.clone() }

    fn set_full_path(&mut self, path: String) {
        self.path = path
    }

    fn get_full_path(&self) -> String {
        self.path.clone()
    }

    fn update_state(&mut self, new_state: &state::EzState) {
        let state = new_state.as_checkbox();
        self.state = state.clone();
        self.state.changed = false;
        self.state.force_redraw = false;
    }

    fn get_state(&self) -> state::EzState { state::EzState::Checkbox(self.state.clone()) }

    fn get_contents(&self, state_tree: &mut common::StateTree) -> common::PixelMap {

        let state = state_tree.get(&self.get_full_path()).unwrap().as_checkbox();
        let active_symbol = { if state.active {self.active_symbol}
                              else {self.inactive_symbol} };
        let fg_color = if state.selected {state.get_colors().selection_foreground }
        else {state.get_colors().foreground };
        let bg_color = if state.selected {state.get_colors().selection_background }
        else {state.get_colors().background };
        let mut contents = vec!(
            vec!(Pixel {symbol: "[".to_string(), foreground_color: fg_color,
                background_color: bg_color, underline: false}),
            vec!(Pixel {symbol: " ".to_string(), foreground_color: fg_color,
                background_color: bg_color, underline: false}),
            vec!(Pixel { symbol: active_symbol.to_string(), foreground_color: fg_color,
                background_color: bg_color, underline: false}),
            vec!(Pixel { symbol: " ".to_string(), foreground_color: fg_color,
                background_color: bg_color, underline: false}),
            vec!(Pixel { symbol: "]".to_string(), foreground_color: fg_color,
                background_color: bg_color, underline: false}),
        );
        if state.has_border() {
            contents = common::add_border(contents, state.get_border_config());
        }
        let parent_colors = state_tree.get(self.get_full_path()
            .rsplit_once('/').unwrap().0).unwrap().as_generic().get_colors();
        contents = common::add_padding(
            contents, state.get_padding(),parent_colors.background,
            parent_colors.foreground);
        contents
    }

}

impl EzWidget for Checkbox {

    fn is_selectable(&self) -> bool { true }

    fn is_selected(&self) -> bool { self.state.selected }

    fn get_selection_order(&self) -> usize { self.selection_order }

    fn get_key_map(&self) -> &common::KeyMap { &self.keymap }

    fn bind_key(&mut self, key: KeyCode, func: common::KeyboardCallbackFunction) {
        self.keymap.insert(key, func);
    }

    fn set_bind_on_value_change(&mut self, func: common::GenericEzFunction) {
        self.bound_on_value_change = Some(func)
    }

    fn get_bind_on_value_change(&self) -> Option<common::GenericEzFunction> { self.bound_on_value_change }

    fn on_left_click(&self, context: common::EzContext, _position: state::Coordinates) {
        let state = context.state_tree.get_mut(&self.get_full_path()).unwrap()
            .as_checkbox_mut();
        self.toggle(state);
        state.set_selected(true);
        self.on_value_change(context);
    }

    fn on_keyboard_enter(&self, context: common::EzContext) {

        let state = context.state_tree.get_mut(&self.get_full_path()).unwrap()
            .as_checkbox_mut();
        self.toggle(state);
        state.set_selected(true);
        self.on_value_change(context);
    }
    fn set_bind_right_click(&mut self, func: common::MouseCallbackFunction) {
        self.bound_right_mouse_click = Some(func)
    }

    fn get_bind_right_click(&self) -> Option<common::MouseCallbackFunction> { self.bound_right_mouse_click }

    fn set_bind_on_select(&mut self, func: fn(common::EzContext, Option<state::Coordinates>)) {
        self.bound_on_select = Some(func);
    }

    fn get_bind_on_select(&self) -> Option<fn(common::EzContext, Option<state::Coordinates>)> {
        self.bound_on_select
    }

}

impl Checkbox {

    /// Initialize an instance of this object using the passed config coming from [ez_parser]
    pub fn from_config(config: Vec<&str>) -> Self {
        let mut obj = Checkbox::default();
        obj.load_ez_config(config).unwrap();
        obj
    }

    /// Sets [active] to true if false, and false if true
    fn toggle(&self, state: &mut CheckboxState) {
        if state.get_active() {
            state.set_active(false);
        } else {
            state.set_active(true);
        }
    }
}
