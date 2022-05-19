//! # Dropdown Widget
//! Widget which supports and arbitrary amount of possible values of which one can be chosen at any
//! time. The active value is always displayed, and when selected drops down all other possible
//! values for the user to select.
use std::io::{Error, ErrorKind};
use std::collections::HashMap;
use crossterm::event::{Event, KeyCode, MouseButton, MouseEventKind};
use crossterm::style::{Color};
use crate::common::{self, KeyboardCallbackFunction, GenericEzFunction, Coordinates, PixelMap, MouseCallbackFunction, EzContext, StateTree, KeyMap};
use crate::widgets::state::{State, GenericState, SelectableState};
use crate::widgets::widget::{EzWidget, Pixel, EzObject};
use crate::ez_parser::{load_bool_parameter, load_color_parameter, load_selection_order_parameter};

pub struct Dropdown {

    /// ID of the widget, used to construct [path]
    pub id: String,

    /// Full path to this widget, e.g. "/root_layout/layout_2/THIS_ID"
    pub path: String,

    /// Absolute position of this layout on screen. Automatically propagated, do not set manually
    pub absolute_position: Coordinates,

    /// The [Pixel.symbol] to use for the horizontal border if [border] is true
    pub border_horizontal_symbol: String,

    /// The [Pixel.symbol] to use for the vertical border if [border] is true
    pub border_vertical_symbol: String,

    /// The [Pixel.symbol] to use for the top left border if [border] is true
    pub border_top_left_symbol: String,

    /// The [Pixel.symbol] to use for the top left border if [border] is true
    pub border_top_right_symbol: String,

    /// The [Pixel.symbol] to use for the bottom left border if [border] is true
    pub border_bottom_left_symbol: String,

    /// The [Pixel.symbol] to use for the bottom right border if [border] is true
    pub border_bottom_right_symbol: String,

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

impl Default for Dropdown {
    fn default() -> Self {
        Dropdown {
            id: String::new(),
            path: String::new(),
            absolute_position: (0, 0),
            border_horizontal_symbol: "━".to_string(),
            border_vertical_symbol: "│".to_string(),
            border_top_left_symbol: "┌".to_string(),
            border_top_right_symbol: "┐".to_string(),
            border_bottom_left_symbol: "└".to_string(),
            border_bottom_right_symbol: "┘".to_string(),
            selection_order: 0,
            bound_on_select: None,
            bound_on_deselect: None,
            bound_right_mouse_click: None,
            bound_on_value_change: None,
            keymap: HashMap::new(),
            state: DropdownState::default(),
        }
    }
}


/// [State] implementation.
#[derive(Clone)]
pub struct DropdownState {

    /// Bool representing whether this widget is currently focussed. If so, it gets the first
    /// chance to consume all events
    pub focussed: bool,

    /// Bool representing whether this widget is currently selected.
    pub selected: bool,

    /// List of options this dropdown will display
    pub options: Vec<String>,

    /// Bool representing whether an empty value should be shown to choose from
    pub allow_none: bool,

    /// The currently active choice of the dropdown.
    pub choice: String,

    /// Bool representing whether this widget is currently dropped down or not
    pub dropped_down: bool,

    /// If dropped down, this represents which row of the dropdown is being hovered with the mouse,
    /// or has been selected with the keyboard using up/down.
    pub dropped_down_selected_row: usize,

    /// Horizontal position of this widget relative to its' parent [Layout]
    pub x: usize,

    /// Vertical position of this widget relative to its' parent [Layout]
    pub y: usize,

    /// Width of this widget
    pub width: usize,

    /// The[Pixel.foreground_color]  to use for the border if [border] is true
    pub border_foreground_color: Color,

    /// The [Pixel.background_color] to use for the border if [border] is true
    pub border_background_color: Color,

    /// The [Pixel.foreground_color] to use for this widgets' content
    pub content_foreground_color: Color,

    /// The [Pixel.background_color] to use for this widgets' content
    pub content_background_color: Color,

    /// The [Pixel.foreground_color] to use for this widgets' content when selected
    pub selection_foreground_color: Color,

    /// The [Pixel.background_color] to use for this widgets' content when selected
    pub selection_background_color: Color,

    /// Bool representing if state has changed. Triggers widget redraw.
    pub changed: bool,

    /// If true this forces a global screen redraw on the next frame. Screen redraws are diffed
    /// so this can be called when needed without degrading performance. If only screen positions
    /// that fall within this widget must be redrawn, call [EzObject.redraw] instead.
    pub force_redraw: bool,
}
impl Default for DropdownState {
    fn default() -> Self {
       DropdownState {
           x: 0,
           y: 0,
           width: 0,
           focussed: false,
           selected: false,
           options: Vec::new(),
           allow_none: true,
           dropped_down: false,
           dropped_down_selected_row:0,
           choice: String::new(),
           border_foreground_color: Color::White,
           border_background_color: Color::Black,
           content_background_color: Color::Black,
           content_foreground_color: Color::White,
           selection_background_color: Color::Blue,
           selection_foreground_color: Color::Yellow,
           changed: false,
           force_redraw: false
       }
    }
}
impl GenericState for DropdownState {

    fn set_changed(&mut self, changed: bool) { self.changed = changed }

    fn get_changed(&self) -> bool { self.changed }

    fn set_width(&mut self, width: usize) { self.width = width; self.changed = true }

    fn get_width(&self) -> usize { self.width }

    fn set_height(&mut self, _height: usize) {
        panic!("Cannot set height directly for dropdown state")
    }

    fn get_height(&self) -> usize {
        if self.dropped_down { self.total_options() }
        else { 1 }
    }

    fn set_position(&mut self, position: Coordinates) {
        self.x = position.0;
        self.y = position.1;
        self.changed = true;
    }

    fn get_position(&self) -> Coordinates { (self.x, self.y) }

    fn set_force_redraw(&mut self, redraw: bool) {
        self.force_redraw = redraw;
        self.changed = true;
    }

    fn get_force_redraw(&self) -> bool { self.force_redraw }
}
impl SelectableState for DropdownState {
    fn set_selected(&mut self, state: bool) {
        self.selected = state;
        self.changed = true;
    }
    fn get_selected(&self) -> bool { self.selected }
}
impl DropdownState {

    pub fn set_choice(&mut self, choice: String) {
        self.choice = choice.clone();
        self.changed = true;
    }

    pub fn get_choice(&self) -> String {
        self.choice.clone()
    }

    pub fn set_options(&mut self, options: Vec<String>) {
        self.options = options
    }

    pub fn get_options(&self) -> Vec<String> {
        self.options.clone()
    }

    pub fn set_focussed(&mut self, allow_none: bool) {
        self.focussed = allow_none;
        self.changed = true;
    }

    pub fn get_focussed(&self) -> bool { self.focussed }

    pub fn set_allow_none(&mut self, allow_none: bool) {
        self.allow_none = allow_none;
        self.changed = true;
    }

    pub fn get_allow_none(&self) -> bool {
        self.allow_none
    }

    pub fn set_dropped_down(&mut self, dropped_down: bool) {
        self.dropped_down = dropped_down;
        self.changed = true;
    }

    pub fn get_dropped_down(&self) -> bool {
        self.dropped_down
    }

    pub fn set_dropped_down_selected_row(&mut self, dropped_down_selected_row: usize) {
        self.dropped_down_selected_row = dropped_down_selected_row;
        self.changed = true;
    }

    pub fn get_dropped_down_selected_row(&self) -> usize {
        self.dropped_down_selected_row
    }

    pub fn set_border_foreground_color(&mut self, color: Color) {
        self.border_foreground_color = color;
        self.changed = true;
    }

    pub fn get_border_foreground_color(&self) -> Color {
        self.border_foreground_color
    }

    pub fn set_border_background_color(&mut self, color: Color) {
        self.border_background_color = color;
        self.changed = true;
    }

    pub fn get_border_background_color(&self) -> Color {
        self.border_background_color
    }

    pub fn set_content_foreground_color(&mut self, color: Color) {
        self.content_foreground_color = color;
        self.changed = true;
    }

    pub fn get_content_foreground_color(&self) -> Color {
        self.content_foreground_color
    }

    pub fn set_content_background_color(&mut self, color: Color) {
        self.content_background_color = color;
        self.changed = true;
    }

    pub fn get_content_background_color(&self) -> Color {
        self.content_background_color
    }

    pub fn set_selection_foreground_color(&mut self, color: Color) {
        self.selection_foreground_color = color;
        self.changed = true;
    }

    pub fn get_selection_foreground_color(&self) -> Color {
        self.selection_foreground_color
    }

    pub fn set_selection_background_color(&mut self, color: Color) {
        self.selection_background_color = color;
        self.changed = true;
    }

    pub fn get_selection_background_color(&self) -> Color {
        self.selection_background_color
    }

    /// Return the total amount of options in this dropdown including the empty option if it is
    /// allowed.
    fn total_options(&self) -> usize {
        self.options.len() + if self.allow_none {1} else {0}
    }

}


impl EzObject for Dropdown {

    fn load_ez_parameter(&mut self, parameter_name: String, parameter_value: String)
                         -> Result<(), Error> {
        match parameter_name.as_str() {
            "x" => self.state.x = parameter_value.trim().parse().unwrap(),
            "y" => self.state.y = parameter_value.trim().parse().unwrap(),
            "width" => self.state.width = parameter_value.trim().parse().unwrap(),
            "selectionOrder" => {
                self.selection_order = load_selection_order_parameter(
                    parameter_value.as_str()).unwrap();
            },
            "contentForegroundColor" =>
                self.state.content_foreground_color =
                    load_color_parameter(parameter_value).unwrap(),
            "contentBackgroundColor" =>
                self.state.content_background_color =
                    load_color_parameter(parameter_value).unwrap(),
            "selectionForegroundColor" =>
                self.state.selection_foreground_color =
                    load_color_parameter(parameter_value).unwrap(),
            "selectionBackgroundColor" =>
                self.state.selection_background_color =
                    load_color_parameter(parameter_value).unwrap(),
            "allowNone" =>
                self.state.allow_none = load_bool_parameter(parameter_value.trim()).unwrap(),
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

    fn update_state(&mut self, new_state: &State) {
        let state = new_state.as_dropdown();
        self.state = state.clone();
        self.state.changed = false;
        self.state.force_redraw = false;
    }

    fn get_state(&self) -> State { State::Dropdown(self.state.clone()) }
    /// Content of this widget depends on whether it is currently dropped down or not. If not,
    /// then display a label with a border representing the currently selected value. If dropped
    /// down show a list of all options, with the currently selected one on top.

    fn get_contents(&self, _state_tree: &mut StateTree) -> PixelMap {

        // If dropped down get full content instead
        if self.state.dropped_down {
            return self.get_dropped_down_contents()
        }
        // Set a default value if user didn't give one
        let mut active = self.state.choice.clone();
        if active.is_empty() && !self.state.allow_none {
            active = self.state.options.first()
                .expect("Dropdown widget must have at least one option").to_string(); // todo move to validation
        }
        // Create a bordered label representing currently active value
        let fg_color = if self.state.selected {self.state.selection_foreground_color}
        else {self.state.content_foreground_color};
        let bg_color = if self.state.selected {self.state.selection_background_color}
        else {self.state.content_background_color};
        let mut text = active.chars().rev().collect::<String>();
        let mut contents = Vec::new();
        for _ in 0..self.state.get_width() {
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
        contents = common::add_border(contents,
                                      self.border_horizontal_symbol.clone(),
                                      self.border_vertical_symbol.clone(),
                                      self.border_top_left_symbol.clone(),
                                      self.border_top_right_symbol.clone(),
                                      self.border_bottom_left_symbol.clone(),
                                      self.border_bottom_right_symbol.clone(),
                                      self.state.border_background_color,
                                      self.state.border_foreground_color);
        contents
    }

    fn set_absolute_position(&mut self, pos: Coordinates) { self.absolute_position = pos }

    fn get_absolute_position(&self) -> Coordinates { self.absolute_position }

    fn set_border_horizontal_symbol(&mut self, symbol: String) {
        self.border_horizontal_symbol = symbol }

    fn get_border_horizontal_symbol(&self) -> String { self.border_horizontal_symbol.clone() }

    fn set_border_vertical_symbol(&mut self, symbol: String) {
        self.border_vertical_symbol = symbol }

    fn get_border_vertical_symbol(&self) -> String { self.border_vertical_symbol.clone() }

    fn set_border_bottom_left_symbol(&mut self, symbol: String) {
        self.border_bottom_left_symbol = symbol }

    fn get_border_bottom_left_symbol(&self) -> String { self.border_bottom_left_symbol.clone() }

    fn set_border_bottom_right_symbol(&mut self, symbol: String) {
        self.border_bottom_right_symbol = symbol }
    fn get_border_bottom_right_symbol(&self) -> String { self.border_bottom_right_symbol.clone() }

    fn set_border_top_left_symbol(&mut self, symbol: String) {
        self.border_top_left_symbol = symbol }
    fn get_border_top_left_symbol(&self) -> String { self.border_top_left_symbol.clone() }

    fn set_border_top_right_symbol(&mut self, symbol: String) {
        self.border_top_right_symbol = symbol
    }

    fn get_border_top_right_symbol(&self) -> String { self.border_top_right_symbol.clone() }

    fn has_border(&self) -> bool { true }

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
                let mouse_position = (event.column as usize, event.row as usize);
                if let MouseEventKind::Down(button) = event.kind {
                    if button == MouseButton::Left {
                        self.handle_left_click(context, mouse_position);
                        return true
                    }
                } else if event.kind == MouseEventKind::Moved && self.collides(mouse_position) {
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

    fn get_bind_on_value_change(&self) -> Option<GenericEzFunction> {
        self.bound_on_value_change
    }
    
    fn set_bind_on_select(&mut self, func: fn(EzContext, Option<Coordinates>)) {
        self.bound_on_select = Some(func);
    }

    fn get_bind_on_select(&self) -> Option<fn(EzContext, Option<Coordinates>)> {
        self.bound_on_select
    }
    
    fn on_left_click(&self, context: EzContext, _position: Coordinates) { self.on_press(context); }

    fn on_keyboard_enter(&self, context: EzContext) { self.on_press(context); }

    fn set_bind_right_click(&mut self, func: MouseCallbackFunction) {
        self.bound_right_mouse_click = Some(func)
    }

    fn get_bind_right_click(&self) -> Option<MouseCallbackFunction> { self.bound_right_mouse_click }

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
        state.set_choice(self.get_dropped_down_options()
            [self.state.dropped_down_selected_row].clone());
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
        if self.collides(pos) {
            let clicked_row = pos.1 - self.absolute_position.1;
            // Check if not click on border
            if clicked_row != 0 && clicked_row != self.state.get_height() + 2 {
                state.set_selected(true);
                state.set_choice(self.get_dropped_down_options()[clicked_row - 1]
                    .clone());
            }
        } else {
            // Click outside widget
            state.set_selected(false);
        }
        self.exit_focus(context);
    }

    /// Called when this widget is already dropped down and widget is hovered
    fn handle_hover(&self, state: &mut DropdownState, pos: Coordinates) {
        let hovered_row = pos.1 - self.absolute_position.1;
        // Check if not hover on border
        if hovered_row -1 != state.dropped_down_selected_row &&
            hovered_row != 0 && hovered_row != self.state.get_height() + 2 { // +2 border
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
    fn get_dropped_down_options(&self) -> Vec<String> {
        let mut options = vec!(self.state.choice.clone());
        if self.state.allow_none && !self.state.choice.is_empty() {
            options.push("".to_string())
        }
        for option in self.state.options.iter()
            .filter(|x| x.to_string() != self.state.choice) {
            options.push(option.to_string());
        }
        options
    }

    /// Return a PixelMap of this widgets' content in dropped down mode. I.e. a menu of options
    /// for the user to choose from.
    fn get_dropped_down_contents(&self) -> PixelMap {

        let mut options:Vec<String> = self.get_dropped_down_options().iter()
            .map(|x| x.chars().rev().collect::<String>()).collect();

        let mut contents = Vec::new();
        for _ in 0..self.state.get_width() {
            let mut new_y = Vec::new();
            for y in 0..options.len() {
                let fg = if y == self.state.dropped_down_selected_row
                {self.state.selection_foreground_color} else {self.state.content_foreground_color};
                let bg = if y == self.state.dropped_down_selected_row
                {self.state.selection_background_color} else {self.state.content_background_color};
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
        contents = common::add_border(contents,
                                      self.border_horizontal_symbol.clone(),
                                      self.border_vertical_symbol.clone(),
                                      self.border_top_left_symbol.clone(),
                                      self.border_top_right_symbol.clone(),
                                      self.border_bottom_left_symbol.clone(),
                                      self.border_bottom_right_symbol.clone(),
                                      self.state.border_background_color,
                                      self.state.border_foreground_color);
        contents
    }
}
