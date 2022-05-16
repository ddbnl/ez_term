//! # Dropdown Widget
//! Widget which supports and arbitrary amount of possible values of which one can be chosen at any
//! time. The active value is always displayed, and when selected drops down all other possible
//! values for the user to select.
use std::io::{Error, ErrorKind};
use std::collections::HashMap;
use crossterm::event::{Event, KeyCode, MouseButton, MouseEventKind};
use crossterm::style::{Color};
use crate::common::{KeyboardCallbackFunction, GenericCallbackFunction, Coordinates,
                    StateTree, ViewTree, WidgetTree, PixelMap};
use crate::widgets::widget_state::{WidgetState, RedrawWidgetState, SelectableWidgetState};
use crate::widgets::widget::{EzWidget, Pixel, EzObject};
use crate::ez_parser::{load_bool_parameter, load_color_parameter};

pub struct Dropdown {

    /// ID of the widget, used to construct [path]
    pub id: String,

    /// Full path to this widget, e.g. "/root_layout/layout_2/THIS_ID"
    pub path: String,

    /// Horizontal position of this widget relative to its' parent [Layout]
    pub x: usize,

    /// Vertical position of this widget relative to its' parent [Layout]
    pub y: usize,

    /// Width of this widget
    pub width: usize,

    /// Absolute position of this layout on screen. Automatically propagated, do not set manually
    pub absolute_position: Coordinates,

    /// List of options this dropdown will display
    pub options: Vec<String>,

    /// Bool representing whether an empty value should be shown to choose from
    pub allow_none: bool,

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

    /// Global order number in which this widget will be selection when user presses down/up keys
    pub selection_order: usize,

    /// Optional function to call when this widget is right clicked, see
    /// [MouseCallbackFunction] for the callback fn type, or [set_bind_right_click] for
    /// examples.
    pub bound_right_mouse_click: Option<fn(pos: Coordinates)>,

    /// Optional function to call when the value of this widget changes, see
    /// [ValueChangeCallbackFunction] for the callback fn type, or [set_bind_on_value_change] for
    /// examples.
    pub bound_on_value_change: Option<GenericCallbackFunction>,

    /// A Key to callback function lookup used to store keybinds for this widget. See
    /// [KeyboardCallbackFunction] type for callback function signature.
    pub keymap: HashMap<KeyCode, KeyboardCallbackFunction>,

    /// Runtime state of this widget, see [DropdownState] and [WidgetState]
    pub state: DropdownState,
}

impl Default for Dropdown {
    fn default() -> Self {
        Dropdown {
            id: String::new(),
            path: String::new(),
            x: 0,
            y: 0,
            width: 0,
            absolute_position: (0, 0),
            options: Vec::new(),
            allow_none: true,
            border_horizontal_symbol: "━".to_string(),
            border_vertical_symbol: "│".to_string(),
            border_top_left_symbol: "┌".to_string(),
            border_top_right_symbol: "┐".to_string(),
            border_bottom_left_symbol: "└".to_string(),
            border_bottom_right_symbol: "┘".to_string(),
            border_foreground_color: Color::White,
            border_background_color: Color::Black,
            content_background_color: Color::Black,
            content_foreground_color: Color::White,
            selection_background_color: Color::Blue,
            selection_foreground_color: Color::Yellow,
            selection_order: 0,
            bound_right_mouse_click: None,
            bound_on_value_change: None,
            keymap: HashMap::new(),
            state: DropdownState{focussed: false, selected: false, dropped_down: false,
                dropped_down_selected_row:0, choice: String::new(), force_redraw: false},
        }
    }
}


/// [WidgetState] implementation.
#[derive(Clone)]
pub struct DropdownState {

    /// Bool representing whether this widget is currently focussed. If so, it gets the first
    /// chance to consume all events
    pub focussed: bool,

    /// Bool representing whether this widget is currently selected.
    pub selected: bool,

    /// The currently active choice of the dropdown.
    pub choice: String,

    /// Bool representing whether this widget is currently dropped down or not
    pub dropped_down: bool,

    /// If dropped down, this represents which row of the dropdown is being hovered with the mouse,
    /// or has been selected with the keyboard using up/down.
    pub dropped_down_selected_row: usize,

    /// If true this forces a global screen redraw on the next frame. Screen redraws are diffed
    /// so this can be called when needed without degrading performance. If only screen positions
    /// that fall within this widget must be redrawn, call [EzObject.redraw] instead.
    pub force_redraw: bool,
}
impl RedrawWidgetState for DropdownState {
    fn set_force_redraw(&mut self, redraw: bool) { self.force_redraw = redraw }
    fn get_force_redraw(&self) -> bool { self.force_redraw }
}
impl SelectableWidgetState for DropdownState {
    fn set_selected(&mut self, state: bool) { self.selected = state }
    fn get_selected(&self) -> bool { self.selected }
}


impl EzObject for Dropdown {

    fn load_ez_parameter(&mut self, parameter_name: String, parameter_value: String)
                         -> Result<(), Error> {
        match parameter_name.as_str() {
            "x" => self.x = parameter_value.trim().parse().unwrap(),
            "y" => self.y = parameter_value.trim().parse().unwrap(),
            "width" => self.width = parameter_value.trim().parse().unwrap(),
            "selectionOrder" => {
                let order = parameter_value.trim().parse().unwrap();
                if order == 0 {
                    return Err(Error::new(ErrorKind::InvalidData,
                                          "selectionOrder must be higher than 0."))
                }
                self.selection_order = order;
            },
            "contentForegroundColor" =>
                self.content_foreground_color = load_color_parameter(parameter_value).unwrap(),
            "contentBackgroundColor" =>
                self.content_background_color = load_color_parameter(parameter_value).unwrap(),
            "selectionForegroundColor" =>
                self.selection_foreground_color = load_color_parameter(parameter_value).unwrap(),
            "selectionBackgroundColor" =>
                self.selection_background_color = load_color_parameter(parameter_value).unwrap(),
            "allowNone" =>
                self.allow_none = load_bool_parameter(parameter_value.trim()).unwrap(),
            "options" => {
                self.options = parameter_value.split(',')
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

    /// Content of this widget depends on whether it is currently dropped down or not. If not,
    /// then display a label with a border representing the currently selected value. If dropped
    /// down show a list of all options, with the currently selected one on top.
    fn get_contents(&mut self) -> PixelMap {

        // If dropped down get full content instead
        if self.state.dropped_down {
            return self.get_dropped_down_contents()
        }
        // Set a default value if user didn't give one
        let mut active = self.state.choice.clone();
        if active.is_empty() && !self.allow_none {
            active = self.options.first()
                .expect("Dropdown widget must have at least one option").to_string(); // todo move to validation
        }
        // Create a bordered label representing currently active value
        let fg_color = if self.state.selected {self.get_selection_foreground_color()}
        else {self.get_content_foreground_color()};
        let bg_color = if self.state.selected {self.get_selection_background_color()}
        else {self.get_content_background_color()};
        let mut text = active.chars().rev().collect::<String>();
        let mut contents = Vec::new();
        for _ in 0..self.get_width() {
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
        contents = self.add_border(contents);
        contents
    }

    fn get_width(&self) -> usize { self.width }

    fn get_height(&self) -> usize {
        if self.state.dropped_down { self.total_options() }
        else { 1 }
    }

    fn set_position(&mut self, position: Coordinates) {
        self.x = position.0;
        self.y = position.1;
    }

    fn get_position(&self) -> Coordinates { (self.x, self.y) }

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
        self.border_top_right_symbol = symbol }
    fn get_border_top_right_symbol(&self) -> String { self.border_top_right_symbol.clone() }

    fn set_border_foreground_color(&mut self, color: Color) { self.border_foreground_color = color }

    fn get_border_foreground_color(&self) -> Color { self.border_foreground_color }

    fn set_border_background_color(&mut self, color: Color) { self.border_background_color = color }

    fn get_border_background_color(&self) -> Color { self.border_background_color }

    fn has_border(&self) -> bool { true }

}

impl EzWidget for Dropdown {
    fn set_focus(&mut self, enabled: bool) { self.state.focussed = enabled }

    fn get_focus(&self) -> bool { self.state.focussed }

    fn get_state(&self) -> WidgetState {
        WidgetState::Dropdown(self.state.clone())
    }

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

    fn get_key_map(&self) -> &HashMap<KeyCode, KeyboardCallbackFunction> {
       &self.keymap
    }

    fn bind_key(&mut self, key: KeyCode, func: KeyboardCallbackFunction) {
       self.keymap.insert(key, func);
    }

    fn handle_event(&self, event: Event, view_tree: &mut ViewTree, state_tree: &mut StateTree,
                    widget_tree: &WidgetTree) -> bool {
        let state = state_tree.get_mut(&self.get_full_path()).unwrap()
            .as_dropdown_mut();
        match event {
            Event::Key(key) => {
                match key.code {
                    KeyCode::Enter => {
                        self.handle_enter(view_tree, state_tree, widget_tree);
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
                        self.handle_left_click(mouse_position, view_tree, state_tree,
                        widget_tree);
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

    fn is_selectable(&self) -> bool { true }

    fn is_selected(&self) -> bool { self.state.selected }

    fn get_selection_order(&self) -> usize { self.selection_order }

    fn set_bind_on_value_change(&mut self, func: GenericCallbackFunction) {
        self.bound_on_value_change = Some(func)
    }

    fn get_bind_on_value_change(&self) -> Option<GenericCallbackFunction> {
        self.bound_on_value_change
    }

    fn on_keyboard_enter(&self, _widget_path: String, _view_tree: &mut ViewTree,
                         state_tree: &mut StateTree, widget_tree: &WidgetTree) {
        self.on_press(state_tree, widget_tree);
    }

    fn on_left_click(&self, _position: Coordinates, _view_tree: &mut ViewTree,
                     state_tree: &mut StateTree, widget_tree: &WidgetTree) {
        self.on_press(state_tree, widget_tree);
    }

    fn set_bind_right_click(&mut self, func: fn(Coordinates)) {
        self.bound_right_mouse_click = Some(func)
    }

    fn get_bind_right_click(&self) -> Option<fn(Coordinates)> { self.bound_right_mouse_click }

    fn state_changed(&self, other_state: &WidgetState) -> bool {
        let state = other_state.as_dropdown();
        if state.selected != self.state.selected { return true }
        if state.focussed != self.state.focussed { return true }
        if state.dropped_down != self.state.dropped_down { return true }
        if state.dropped_down_selected_row != self.state.dropped_down_selected_row { return true }
        if state.choice != self.state.choice { return true }
        false
    }

    fn update_state(&mut self, new_state: &WidgetState) {
        let state = new_state.as_dropdown();
        self.state = state.clone();
        self.state.force_redraw = false;
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
    fn handle_enter(&self, view_tree: &mut ViewTree, state_tree: &mut StateTree,
                    widget_tree: &WidgetTree) {
        let state = state_tree.get_mut(
            &self.get_full_path()).unwrap().as_dropdown_mut();
        state.selected = true;
        state.choice = self.get_dropped_down_options()
            [self.state.dropped_down_selected_row].clone();
        self.exit_focus(view_tree, state_tree, widget_tree);
    }

    /// Called when this widget is already dropped down and up is pressed
    fn handle_up(&self, state: &mut DropdownState) {

        if state.dropped_down_selected_row == 0 {
            state.dropped_down_selected_row = self.total_options() - 1;
        } else {
            state.dropped_down_selected_row -= 1;
        }
    }

    /// Called when this widget is already dropped down and down is pressed
    fn handle_down(&self, state: &mut DropdownState) {
        if state.dropped_down_selected_row == self.total_options() - 1 {
            state.dropped_down_selected_row = 0;
        } else {
            state.dropped_down_selected_row += 1;
        }
    }

    /// Called when this widget is already dropped down and widget is left clicked
    fn handle_left_click(&self, pos: Coordinates, view_tree: &mut ViewTree,
                         state_tree: &mut StateTree, widget_tree: &WidgetTree) {

        let state = state_tree.get_mut(
            &self.get_full_path()).unwrap().as_dropdown_mut();
        if self.collides(pos) {
            let clicked_row = pos.1 - self.absolute_position.1;
            // Check if not click on border
            if clicked_row != 0 && clicked_row != self.get_height() + 2 {
                state.selected = true;
                state.choice = self.get_dropped_down_options()[clicked_row - 1]
                    .clone();
            }
        } else {
            // Click outside widget
            state.selected = false;
        }
        self.exit_focus(view_tree, state_tree, widget_tree);
    }

    /// Called when this widget is already dropped down and widget is hovered
    fn handle_hover(&self, state: &mut DropdownState, pos: Coordinates) {
        let hovered_row = pos.1 - self.absolute_position.1;
        // Check if not hover on border
        if hovered_row -1 != state.dropped_down_selected_row &&
            hovered_row != 0 && hovered_row != self.get_height() + 2 { // +2 border
            state.dropped_down_selected_row = hovered_row - 1;
        }
    }

    /// Called when widget leaves dropdown mode. Forces a screen redraw because dropping down may
    /// have overwritten other widgets.
    fn exit_focus(&self, view_tree: &mut ViewTree,
                  state_tree: &mut StateTree, widget_tree: &WidgetTree) {
        let state = state_tree.get_mut(
            &self.get_full_path()).unwrap().as_dropdown_mut();
        state.focussed = false;
        state.dropped_down = false;
        state.force_redraw = true;
        if state.choice != self.state.choice {
            self.on_value_change(self.get_full_path(), view_tree, state_tree,
            widget_tree);
        }
    }

    /// Called when the widgets is not dropped down and enter/left mouse click occurs on it.
    fn on_press(&self, state_tree: &mut StateTree, _widget_tree: &WidgetTree) {
        let state = state_tree.get_mut(&self.get_full_path()).unwrap()
            .as_dropdown_mut();
        state.dropped_down_selected_row = 1;
        state.dropped_down = true;
        state.focussed = true;
        state.selected = true;
    }

    /// Represents the total amount of options, including the empty option if it was allowed
    fn total_options(&self) -> usize { self.options.len() + if self.allow_none {1} else {0} }

    /// Get an ordered list of options, including the empty option if it was allowed. Order is:
    /// - Active choice
    /// - Empty (if allowed)
    /// - Rest of the options in user defined order
    fn get_dropped_down_options(&self) -> Vec<String> {
        let mut options = vec!(self.state.choice.clone());
        if self.allow_none && !self.state.choice.is_empty() {
            options.push("".to_string())
        }
        for option in self.options.iter()
            .filter(|x| x.to_string() != self.state.choice) {
            options.push(option.to_string());
        }
        options
    }

    /// Return a PixelMap of this widgets' content in dropped down mode. I.e. a menu of options
    /// for the user to choose from.
    fn get_dropped_down_contents(&mut self) -> PixelMap {

        let mut options:Vec<String> = self.get_dropped_down_options().iter()
            .map(|x| x.chars().rev().collect::<String>()).collect();

        let mut contents = Vec::new();
        for _ in 0..self.get_width() {
            let mut new_y = Vec::new();
            for y in 0..options.len() {
                let fg = if y == self.state.dropped_down_selected_row
                {self.selection_foreground_color} else {self.content_foreground_color};
                let bg = if y == self.state.dropped_down_selected_row
                {self.selection_background_color} else {self.content_background_color};
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
        contents = self.add_border(contents);
        contents
    }
}
