//! # Checkbox Widget
//! Widget which is either on or off and implements an on_value_change callback.
use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use crossterm::event::{KeyCode};
use crossterm::style::{Color};
use crate::common::{KeyboardCallbackFunction, Coordinates, StateTree, ViewTree, WidgetTree,
                    PixelMap, ValueChangeCallbackFunction};
use crate::widgets::widget::{EzWidget, Pixel, EzObject};
use crate::widgets::widget_state::{WidgetState, RedrawWidgetState, SelectableWidgetState};
use crate::ez_parser::{load_bool_parameter, load_color_parameter};

pub struct Checkbox {

    /// ID of the widget, used to construct [path]
    pub id: String,

    /// Full path to this widget, e.g. "/root_layout/layout_2/THIS_ID"
    pub path: String,

    /// Horizontal position of this widget relative to its' parent [Layout]
    pub x: usize,

    /// Vertical position of this widget relative to its' parent [Layout]
    pub y: usize,

    /// Absolute position of this layout on screen. Automatically propagated, do not set manually
    pub absolute_position: Coordinates,

    /// [Pixel.symbol] used when the Checkbox is active
    pub active_symbol: char,

    /// [Pixel.symbol] used when the Checkbox is not active
    pub inactive_symbol: char,

    /// The [Pixel.foreground_color] to use for this widgets' content
    pub content_foreground_color: Color,

    /// The [Pixel.background_color] to use for this widgets' content
    pub content_background_color: Color,

    /// The [Pixel.foreground_color] to use for this widgets' content when selected
    pub selection_foreground_color: Color,

    /// The [Pixel.background_color] to use for this widgets' content when selected
    pub selection_background_color: Color,

    /// Global order number in which this widget will be selection when user presses down/up keys
    pub selection_order: usize,

    /// Optional function to call when the value of this widget changes, see
    /// [ValueChangeCallbackFunction] for the callback fn type, or [set_bind_on_value_change] for
    /// examples.
    pub bound_on_value_change: Option<ValueChangeCallbackFunction>,

    /// Optional function to call when this widget is right clicked, see
    /// [MouseCallbackFunction] for the callback fn type, or [set_bind_right_click] for
    /// examples.
    pub bound_right_mouse_click: Option<fn(pos: Coordinates)>,

    /// A Key to callback function lookup used to store keybinds for this widget. See
    /// [KeyboardCallbackFunction] type for callback function signature.
    pub keymap: HashMap<KeyCode, KeyboardCallbackFunction>,

    /// Runtime state of this widget, see [CheckboxState] and [WidgetState]
    pub state: CheckboxState,
}

impl Default for Checkbox {
    fn default() -> Self {
        Checkbox {
            id: "".to_string(),
            path: String::new(),
            x: 0,
            y: 0,
            active_symbol: 'X',
            inactive_symbol: ' ',
            absolute_position: (0, 0),
            content_background_color: Color::Black,
            content_foreground_color: Color::White,
            selection_background_color: Color::Blue,
            selection_foreground_color: Color::Yellow,
            selection_order: 0,
            bound_on_value_change: None,
            bound_right_mouse_click: None,
            keymap: HashMap::new(),
            state: CheckboxState {active: false, selected: false, force_redraw: false},
        }
    }
}


/// [WidgetState] implementation.
#[derive(Clone)]
pub struct CheckboxState {
    /// Bool representing whether this widget is currently active (i.e. checkbox is checked)
    pub active: bool,

    /// Bool representing whether this widget is currently selected.
    pub selected: bool,

    /// If true this forces a global screen redraw on the next frame. Screen redraws are diffed
    /// so this can be called when needed without degrading performance. If only screen positions
    /// that fall within this widget must be redrawn, call [EzObject.redraw] instead.
    pub force_redraw: bool,
}
impl RedrawWidgetState for CheckboxState {
    fn set_force_redraw(&mut self, redraw: bool) { self.force_redraw = redraw }
    fn get_force_redraw(&self) -> bool { self.force_redraw }
}
impl SelectableWidgetState for CheckboxState {
    fn set_selected(&mut self, state: bool) { self.selected = state }
    fn get_selected(&self) -> bool { self.selected }
}


impl EzObject for Checkbox {

    fn load_ez_parameter(&mut self, parameter_name: String, parameter_value: String)
                         -> Result<(), Error> {

        match parameter_name.as_str() {
            "x" => self.x = parameter_value.trim().parse().unwrap(),
            "y" => self.y = parameter_value.trim().parse().unwrap(),
            "selectionOrder" => {
                let order = parameter_value.trim().parse().unwrap();
                if order == 0 {
                    return Err(Error::new(ErrorKind::InvalidData,
                                          "selectionOrder must be higher than 0."))
                }
                self.selection_order = order;
            },
            "active" => self.state.active = load_bool_parameter(parameter_value.trim()).unwrap(),
            "activeSymbol" => self.active_symbol = parameter_value.chars().last().unwrap(),
            "inactiveSymbol" => self.inactive_symbol = parameter_value.chars().last().unwrap(),
            "contentForegroundColor" =>
                self.content_foreground_color = load_color_parameter(parameter_value).unwrap(),
            "contentBackgroundColor" =>
                self.content_background_color = load_color_parameter(parameter_value).unwrap(),
            "selectionForegroundColor" =>
                self.selection_foreground_color = load_color_parameter(parameter_value).unwrap(),
            "selectionBackgroundColor" =>
                self.selection_background_color = load_color_parameter(parameter_value).unwrap(),
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

    fn get_contents(&mut self) -> PixelMap {

        let active_symbol = { if self.state.active {self.active_symbol}
                              else {self.inactive_symbol} };
        let fg_color = if self.state.selected {self.get_selection_foreground_color()}
        else {self.get_content_foreground_color()};
        let bg_color = if self.state.selected {self.get_selection_background_color()}
        else {self.get_content_background_color()};
        vec!(
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
        )
    }

    fn get_width(&self) -> usize { 5 }

    fn get_height(&self) -> usize { 1 }

    fn set_position(&mut self, position: Coordinates) {
        self.x = position.0;
        self.y = position.1;
    }

    fn get_position(&self) -> Coordinates { (self.x, self.y) }

    fn set_absolute_position(&mut self, pos: Coordinates) { self.absolute_position = pos }

    fn get_absolute_position(&self) -> Coordinates { self.absolute_position }
}

impl EzWidget for Checkbox {

    fn get_state(&self) -> WidgetState { WidgetState::Checkbox(self.state.clone()) }

    fn set_content_foreground_color(&mut self, color: Color) { self.content_foreground_color = color }

    fn get_content_foreground_color(&self) -> Color { self.content_foreground_color }

    fn set_content_background_color(&mut self, color: Color) { self.content_background_color = color }

    fn get_content_background_color(&self) -> Color { self.content_background_color }

    fn set_selection_foreground_color(&mut self, color: Color) {
        self.selection_foreground_color = color }

    fn get_selection_foreground_color(&self) -> Color { self.selection_foreground_color }

    fn set_selection_background_color(&mut self, color: Color) {
        self.selection_background_color = color }

    fn get_selection_background_color(&self) -> Color { self.selection_background_color }

    fn get_key_map(&self) -> &HashMap<KeyCode, KeyboardCallbackFunction> { &self.keymap }

    fn bind_key(&mut self, key: KeyCode, func: KeyboardCallbackFunction) {
        self.keymap.insert(key, func);
    }

    fn is_selectable(&self) -> bool { true }

    fn is_selected(&self) -> bool { self.state.selected }

    fn get_selection_order(&self) -> usize { self.selection_order }

    fn set_bind_on_value_change(&mut self, func: ValueChangeCallbackFunction) {
        self.bound_on_value_change = Some(func)
    }

    fn get_bind_on_value_change(&self) -> Option<ValueChangeCallbackFunction> {
        self.bound_on_value_change
    }

    fn on_keyboard_enter(&self, view_tree: &mut ViewTree, state_tree: &mut StateTree,
                         widget_tree: &WidgetTree) {

        let state = state_tree.get_mut(&self.get_full_path()).unwrap()
            .as_checkbox_mut();
        self.toggle(state);
        state.selected = true;
        self.on_value_change(self.get_full_path(), view_tree, state_tree, widget_tree);
    }

    fn on_left_click(&self, _position: Coordinates, view_tree: &mut ViewTree,
                     state_tree: &mut StateTree, widget_tree: &WidgetTree) {
        let state = state_tree.get_mut(&self.get_full_path()).unwrap()
            .as_checkbox_mut();
        self.toggle(state);
        state.selected = true;
        self.on_value_change(self.get_full_path(), view_tree, state_tree, widget_tree);
    }

    fn set_bind_right_click(&mut self, func: fn(Coordinates)) {
        self.bound_right_mouse_click = Some(func)
    }

    fn get_bind_right_click(&self) -> Option<fn(Coordinates)> { self.bound_right_mouse_click }

    fn state_changed(&self, other_state: &WidgetState) -> bool {
        let state = other_state.as_checkbox();
        if state.selected != self.state.selected { return true };
        if state.active != self.state.active { return true };
        false
    }

    fn update_state(&mut self, new_state: &WidgetState) {
        let state = new_state.as_checkbox();
        self.state = state.clone();
        self.state.force_redraw = false;
    }
}

impl Checkbox {

    /// Initialize an instance of this object using the passed config coming from [ez_parser]
    pub fn from_config(config: Vec<&str>, id: String) -> Self {
        let mut obj = Checkbox::default();
        obj.set_id(id);
        obj.load_ez_config(config).unwrap();
        obj
    }

    /// Gets [active]
    fn get_active(&self) -> bool { self.state.active }

    /// Sets [active]
    fn set_active(&mut self, active: bool) { self.state.active = active }

    /// Sets [active] to true if false, and false if true
    fn toggle(&self, state: &mut CheckboxState) {
        if state.active {
            state.active = false;
        } else {
            state.active = true;
        }
    }
}
