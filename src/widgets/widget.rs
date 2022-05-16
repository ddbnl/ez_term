//! # Widget:
//! A module containing the base structs and traits for widgets"
//! functions allows starting the app based on a root layout.
use crossterm::style::{Color, StyledContent, Stylize};
use crossterm::event::{Event, KeyCode};
use std::io::{Error};
use std::collections::HashMap;
use crate::common::{self, ViewTree, WidgetTree, StateTree, Coordinates, KeyboardCallbackFunction, PixelMap, ValueChangeCallbackFunction};
use crate::widgets::widget_state::{WidgetState};
use crate::widgets::layout::{Layout};
use crate::widgets::canvas_widget::{CanvasWidget};
use crate::widgets::checkbox::{Checkbox};
use crate::widgets::dropdown::{Dropdown};
use crate::widgets::radio_button::{RadioButton};
use crate::widgets::text_input::{TextInput};
use crate::widgets::label::{Label};


/// Enum with variants representing Layouts and each widget type. A layout is not considered a
/// widget, so this enum gathers widgets and layouts in one place, as they do have methods in
/// common (e.g. both have positions, sizes, etc.). To access common methods, cast this enum
/// into a EzObject (trait for Layouts+Widgets) or EzWidget (Widgets only).
pub enum EzObjects {
    Layout(Layout),
    CanvasWidget(CanvasWidget),
    Checkbox(Checkbox),
    Dropdown(Dropdown),
    RadioButton(RadioButton),
    Label(Label),
    TextInput(TextInput),
}
impl EzObjects {

    /// Cast this enum to a generic [EzObject] trait object. As this trait is implemented by both
    /// [Layout] and [widget], it is safe to call on all variants.
    pub fn as_ez_object(&self) -> &dyn EzObject {
        match self {
            EzObjects::Layout(i) => i,
            EzObjects::CanvasWidget(i) => i,
            EzObjects::Checkbox(i) => i,
            EzObjects::Dropdown(i) => i,
            EzObjects::RadioButton(i) => i,
            EzObjects::Label(i) => i,
            EzObjects::TextInput(i) => i,
        }
    }

    /// Cast this enum to a mutable generic [EzObject] trait object. As this trait is implemented
    /// by both [Layout] and [widget], it is safe to call on all variants.
    pub fn as_ez_object_mut(&mut self) -> &mut dyn EzObject {
        match self {
            EzObjects::Layout(i) => i,
            EzObjects::CanvasWidget(i) => i,
            EzObjects::Checkbox(i) => i,
            EzObjects::Dropdown(i) => i,
            EzObjects::RadioButton(i) => i,
            EzObjects::Label(i) => i,
            EzObjects::TextInput(i) => i,
        }
    }

    /// Cast this enum to a generic [EzWidget] trait object. As this trait is implemented only
    /// [widget] and not [Layout], it is safe to call *only* on widgets..
    pub fn as_ez_widget(&self) -> &dyn EzWidget {
        match self {
            EzObjects::CanvasWidget(i) => i,
            EzObjects::Checkbox(i) => i,
            EzObjects::Dropdown(i) => i,
            EzObjects::RadioButton(i) => i,
            EzObjects::Label(i) => i,
            EzObjects::TextInput(i) => i,
            _ => panic!("Casted non-widget to IsWidget"),
        }
    }

    /// Cast this enum to a generic mutable [EzWidget] trait object. As this trait is implemented
    /// only [widget] and not [Layout], it is safe to call *only* on widgets..
    pub fn as_ez_widget_mut(&mut self) -> &mut dyn EzWidget {
        match self {
            EzObjects::CanvasWidget(i) => i,
            EzObjects::Checkbox(i) => i,
            EzObjects::Dropdown(i) => i,
            EzObjects::RadioButton(i) => i,
            EzObjects::Label(i) => i,
            EzObjects::TextInput(i) => i,
            _ => panic!("Casted non-widget to IsWidget"),
        }
    }

    /// Cast this as a layout ref, you must be sure you have one.
    pub fn as_layout(&self) -> &Layout {
        if let EzObjects::Layout(i) = self { i }
        else { panic!("wrong state.") }
    }

    /// Cast this as a mutable layout ref, you must be sure you have one.
    pub fn as_layout_mut(&mut self) -> &mut Layout {
        if let EzObjects::Layout(i) = self { i }
        else { panic!("wrong state.") }
    }
    /// Cast this as a Canvas widget ref, you must be sure you have one.
    pub fn as_canvas(&self) -> &CanvasWidget {
        if let EzObjects::CanvasWidget(i) = self { i }
        else { panic!("wrong state.") }
    }

    /// Cast this as a mutable Canvas widget ref, you must be sure you have one.
    pub fn as_canvas_mut(&mut self) -> &mut CanvasWidget {
        if let EzObjects::CanvasWidget(i) = self { i }
        else { panic!("wrong state.") }
    }

    /// Cast this as a Checkbox widget ref, you must be sure you have one.
    pub fn as_checkbox(&self) -> &Checkbox {
        if let EzObjects::Checkbox(i) = self { i }
        else { panic!("wrong state.") }
    }

    /// Cast this as a mutable Checkbox widget ref, you must be sure you have one.
    pub fn as_checkbox_mut(&mut self) -> &mut Checkbox {
        if let EzObjects::Checkbox(i) = self { i }
        else { panic!("wrong state.") }
    }

    /// Cast this state as a Dropdown widget ref, you must be sure you have one.
    pub fn as_dropdown(&self) -> &Dropdown {
        if let EzObjects::Dropdown(i) = self { i }
        else { panic!("wrong state.") }
    }

    /// Cast this state as a mutable Dropdown widget ref, you must be sure you have one.
    pub fn as_dropdown_mut(&mut self) -> &mut Dropdown {
        if let EzObjects::Dropdown(i) = self { i }
        else { panic!("wrong state.") }
    }

    /// Cast this state as a Label widget ref, you must be sure you have one.
    pub fn as_label(&self) -> &Label {
        if let EzObjects::Label(i) = self { i }
        else { panic!("wrong state.") }
    }

    /// Cast this state as a mutable Label widget ref, you must be sure you have one.
    pub fn as_label_mut(&mut self) -> &mut Label {
        if let EzObjects::Label(i) = self { i }
        else { panic!("wrong state.") }
    }

    /// Cast this state as a RadioButton widget ref, you must be sure you have one.
    pub fn as_radio_button(&self) -> &RadioButton {
        if let EzObjects::RadioButton(i) = self { i }
        else { panic!("wrong state.") }
    }

    /// Cast this state as a mutable RadioButton widget ref, you must be sure you have one.
    pub fn as_radio_button_mut(&mut self) -> &mut RadioButton {
        if let EzObjects::RadioButton(i) = self { i }
        else { panic!("wrong state.") }
    }

    /// Cast this state as a TextInput widget ref, you must be sure you have one.
    pub fn as_text_input(&self) -> &TextInput {
        if let EzObjects::TextInput(i) = self { i }
        else { panic!("wrong state.") }
    }

    /// Cast this state as a mutable TextInput widget ref, you must be sure you have one.
    pub fn as_text_input_mut(&mut self) -> &mut TextInput {
        if let EzObjects::TextInput(i) = self { i }
        else { panic!("wrong state.") }
    }
}


/// Trait representing both widgets and layouts, implementing methods which are common to all UI
/// objects (such as size, position, etc.). If you don't know if an object is a Widget or a layout
/// (or don't care), cast the EzObjects enum into this type using 'common::cast_as_ez_object'.
pub trait EzObject {

    /// Accepts config lines from the ez_parser module and prepares them to be loaded by
    /// load_ez_parameter below.
    fn load_ez_config(&mut self, config: Vec<&str>) -> Result<(), Error> {
        for line in config {
            let parameter: Vec<&str> = line.split(':').collect();
            let parameter_name = parameter[0].to_string();
            let parameter_value = parameter[1].to_string();
            self.load_ez_parameter(parameter_name, parameter_value)?;
        }
        Ok(())
    }

    /// Load parameters for an object. Overloaded in each Widget/Layout module to load parameters
    /// specific to the respective widget definition.
    fn load_ez_parameter(&mut self, parameter_name: String, parameter_value: String)
                         -> Result<(), Error>;

    /// Set ID of the widget. IDs are used to create widgets paths. E.g.
    /// "/root_layout/sub_layout/widget_1".
    fn set_id(&mut self, id: String);

    /// Get ID of the widget. IDs are used to create widgets paths. E.g.
    /// "/root_layout/sub_layout/widget_1".
    fn get_id(&self) -> String;


    /// Set full path to a widget. E.g. "/root_layout/sub_layout/widget_1". Call "get_by_path"
    /// method on the root layout and pass a full widget pass to retrieve a widget.
    fn set_full_path(&mut self, path: String);

    /// Get full path to a widget. E.g. "/root_layout/sub_layout/widget_1". Call "get_by_path"
    /// method on the root layout and pass a full widget pass to retrieve a widget.
    fn get_full_path(&self) -> String;

    /// Redraw the widget on the screen. Using the view tree, only changed content is written to
    /// improve performance.
    fn redraw(&mut self, view_tree: &mut ViewTree) {
        let pos = self.get_absolute_position();
        let content = self.get_contents();
        common::write_to_screen(pos, content, view_tree);
    }

    /// Set the content for a widget manually. This is not implemented for most widgets, as they
    /// get their content from their state. E.g. a label gets content from its' current text.
    fn set_contents(&mut self, _contents: PixelMap) {
        panic!("Cannot manually set content color for this widget {}", self.get_id()); }

    /// Gets the visual content for this widget. Overloaded by each widget module. E.g. a label
    /// gets its' content from its' text, a checkbox from whether it has been checked, etc.
    fn get_contents(&mut self) -> PixelMap;

    /// Manually set the width for a widget. Only allowed for some widgets, while others get their
    /// width from some other source.
    fn set_width(&mut self, _width: usize) {
        panic!("Manually setting width not allowed for this widget: {}", self.get_id()); }

    /// Get current width of a widget.
    fn get_width(&self) -> usize;

    /// Manually set the height for a widget. Only allowed for some widgets, while others get their
    /// height from some other source.
    fn set_height(&mut self, _height: usize) {
        panic!("Manually setting width not allowed for this widget: {}", self.get_id()); }


    /// Get current height of a widget.
    fn get_height(&self) -> usize;

    /// Get the top left and bottom right corners of a widget in (X, Y) coordinate tuples.
    fn get_box(&self) -> (Coordinates, Coordinates) {
        let top_left = self.get_absolute_position();
        let top_right = (top_left.0 + self.get_width(), top_left.1 + self.get_height());
        (top_left, top_right)
    }

    /// Returns a bool representing whether two widgets overlap at any point.
    fn overlaps(&self, other_box: (Coordinates, Coordinates)) -> bool {
        let (l1, r1) = self.get_box();
        let (l2, r2) = other_box;
        // If one rectangle is on the left of the other there's no overlap
        if l1.0 >= r2.0 || l2.0 >= r1.0 { return false }
        // If one rectangle is above the other there's no overlap
        if r1.1 >= l2.1 || r2.1 >= l1.1 { return false }
        true
    }

    /// Returns a bool representing whether a single point collides with a widget.
    fn collides(&self, pos: Coordinates) -> bool {
        let mut starting_pos = self.get_absolute_position();
        if self.has_border() {
            starting_pos = (starting_pos.0 + 1, starting_pos.1 + 1);
        }
        let end_pos = (starting_pos.0 + self.get_width() - 1,
                                  starting_pos.1 + self.get_height() - 1);
        pos.0 >= starting_pos.0 && pos.0 <= end_pos.0 &&
            pos.1 >= starting_pos.1 && pos.1 <= end_pos.1
    }

    /// Set the position of a widget. Can be done manually or automatically by a layout.
    fn set_position(&mut self, position: Coordinates);

    /// Get the position of a widget inside its' layout.
    fn get_position(&self) -> Coordinates;

    /// Set the absolute position of a widget, i.e. the position on screen rather than within its'
    /// parent widget. Should be set automatically through the "propagate_absolute_positions"
    /// function.
    fn set_absolute_position(&mut self, pos:Coordinates);

    /// Get the absolute position of a widget, i.e. the position on screen rather than within its'
    /// parent widget.
    fn get_absolute_position(&self) -> Coordinates;

    /// Set the symbol used to create the horizontal parts of the border for a widget.
    fn set_border_horizontal_symbol(&mut self, _symbol: String) {
        panic!("Cannot manually set border for this widget"); }

    /// Get the symbol used to create the horizontal parts of the border for a widget.
    fn get_border_horizontal_symbol(&self) -> String { "━".to_string() }

    /// Set the symbol used to create the vertical parts of the border for a widget.
    fn set_border_vertical_symbol(&mut self, _symbol: String) {
        panic!("Cannot manually set border for this widget"); }

    /// Get the symbol used to create the vertical parts of the border for a widget.
    fn get_border_vertical_symbol(&self) -> String { "│".to_string() }

    /// Set the symbol used to create the bottom left part of the border for a widget.
    fn set_border_bottom_left_symbol(&mut self, _symbol: String) {
        panic!("Cannot manually set border for this widget"); }

    /// Get the symbol used to create the bottom left part of the border for a widget.
    fn get_border_bottom_left_symbol(&self) -> String { "└".to_string() }

    /// Set the symbol used to create the bottom right part of the border for a widget.
    fn set_border_bottom_right_symbol(&mut self, _symbol: String) {
        panic!("Cannot manually set border for this widget"); }

    /// Get the symbol used to create the bottom right part of the border for a widget.
    fn get_border_bottom_right_symbol(&self) -> String { "┘".to_string() }

    /// Set the symbol used to create the top left part of the border for a widget.
    fn set_border_top_left_symbol(&mut self, _symbol: String) {
        panic!("Cannot manually set border for this widget"); }

    /// Get the symbol used to create the top left part of the border for a widget.
    fn get_border_top_left_symbol(&self) -> String { "┌".to_string() }

    /// Set the symbol used to create the top right part of the border for a widget.
    fn set_border_top_right_symbol(&mut self, _symbol: String) {
        panic!("Cannot manually set border for this widget"); }

    /// Get the symbol used to create the top right part of the border for a widget.
    fn get_border_top_right_symbol(&self) -> String { "┐".to_string() }

    /// Set the foreground color used to create the border around a widget.
    fn set_border_foreground_color(&mut self, _color: Color) {
        panic!("Cannot manually set border for this widget"); }

    /// Get the foreground color used to create the border around a widget.
    fn get_border_foreground_color(&self) -> Color { Color::White }

    /// Set the background color used to create the border around a widget.
    fn set_border_background_color(&mut self, _color: Color) {
        panic!("Cannot manually set border for this widget"); }

    /// Get the background color used to create the border around a widget.
    fn get_border_background_color(&self) -> Color { Color::Black }

    /// Set whether a border will be painted around this widget.
    fn set_border(&mut self, _enabled: bool) { panic!("Widget has no border implementation") }

    /// Returns whether this widget has (or should get) a border.
    fn has_border(&self) -> bool { false }

    /// Add a border around the content of this widget.
    fn add_border(&self, mut content: PixelMap) -> PixelMap {
        // Create border elements
        let horizontal_border = Pixel{ symbol: self.get_border_horizontal_symbol(),
            background_color: self.get_border_background_color(),
            foreground_color: self.get_border_foreground_color(), underline: false};
        let vertical_border = Pixel{ symbol:self.get_border_vertical_symbol(),
            background_color: self.get_border_background_color(),
            foreground_color: self.get_border_foreground_color(), underline: false};
        let top_left_border = Pixel{ symbol:self.get_border_top_left_symbol(),
            background_color: self.get_border_background_color(),
            foreground_color: self.get_border_foreground_color(), underline: false};
        let top_right_border = Pixel{ symbol: self.get_border_top_right_symbol(),
            background_color: self.get_border_background_color(),
            foreground_color: self.get_border_foreground_color(), underline: false};
        let bottom_left_border = Pixel{ symbol: self.get_border_bottom_left_symbol(),
            background_color: self.get_border_background_color(),
            foreground_color: self.get_border_foreground_color(), underline: false};
        let bottom_right_border = Pixel{ symbol: self.get_border_bottom_right_symbol(),
            background_color: self.get_border_background_color(),
            foreground_color: self.get_border_foreground_color(), underline: false};
        // Create horizontal borders
        for x in 0..content.len() {
            let mut new_x = vec!(horizontal_border.clone());
            for y in &content[x] {
                new_x.push(y.clone());
            }
            new_x.push(horizontal_border.clone());
            content[x] = new_x
        }
        // Create left border
        let mut left_border = vec!(top_left_border);
        for _ in 0..self.get_height() {
            left_border.push(vertical_border.clone());
        }
        left_border.push(bottom_left_border);
        // Create right border
        let mut right_border = vec!(top_right_border);
        for _ in 0..self.get_height() {
            right_border.push(vertical_border.clone())
        }
        right_border.push(bottom_right_border);
        // Merge all borders around the content
        let mut new_content = vec!(left_border);
        for x in content {
            new_content.push(x);
        }
        new_content.push(right_border);
        new_content

    }
}


/// Trait representing both widgets only, implementing methods which are common to all widgets but
/// not layouts. You can cast a EzObjects enum into this trait using 'common::cast_as_ez_widget',
/// if you know for sure the object you're dealing with is a widget and not a layout.
pub trait EzWidget: EzObject {

    /// Set the focus state of a widget. When a widget is focussed it alone consumes all events.
    fn set_focus(&mut self, _enabled: bool) {}

    /// Get the focus state of a widget. When a widget is focussed it alone consumes all events.
    fn get_focus(&self) -> bool { false }

    /// Get the WidgetState object belonging to this widget.
    fn get_state(&self) -> WidgetState;

    /// Set the foreground color used for this widget.
    fn set_content_foreground_color(&mut self, _color: Color) {
        panic!("Cannot manually set content color for this widget {}", self.get_id()); }

    /// Get the foreground color used for this widget.
    fn get_content_foreground_color(&self) -> Color {
        panic!("Cannot manually get content color for this widget {}", self.get_id()); }

    /// Set the background color used for this widget.
    fn set_content_background_color(&mut self, _color: Color) {
        panic!("Cannot manually set content color for this widget {}", self.get_id()); }

    /// Get the background color used for this widget.
    fn get_content_background_color(&self) -> Color {
        panic!("Cannot manually get content color for this widget {}", self.get_id()); }

    /// Get the foreground color used for this widget when selected.
    fn set_selection_foreground_color(&mut self, _color: Color) {}

    /// Get the foreground color used for this widget when selected.
    fn get_selection_foreground_color(&self) -> Color { Color::Yellow}

    /// Get the background color used for this widget when selected.
    fn set_selection_background_color(&mut self, _color: Color) {}

    /// Get the background color used for this widget when selected.
    fn get_selection_background_color(&self) -> Color { Color::Blue}

    /// Get the key map belonging to a widget. Any keys bound to the widget are in here along with
    /// their callbacks. Key map should be used inside the "handle_event" method of a widget.
    fn get_key_map(&self) -> &HashMap<KeyCode, KeyboardCallbackFunction> {
        panic!("Widget does not support keymap") }

    /// Bind a new key to the widget and the callback it should activate. Focussed widgets have
    /// priority consuming events, next are global key binds, and then the selected widget.
    fn bind_key(&mut self, _key: KeyCode, _func: KeyboardCallbackFunction) {
    }

    /// Optionally consume an event that was passed to this widget. Return true if the event should
    /// be considered consumed. Simply consults the keymap by default, but can be overloaded for
    /// more complex circumstances.
    fn handle_event(&self, event: Event, view_tree: &mut ViewTree,
                    state_tree: & mut StateTree, widget_tree: &WidgetTree) -> bool {
        if let Event::Key(key) = event {
            if self.get_key_map().contains_key(&key.code) {
                let func =
                    self.get_key_map().get(&key.code).unwrap();
                func(self.get_full_path(), key.code, view_tree, state_tree, widget_tree);
                return true
            }
        }
        false
    }

    /// Returns a bool representing whether this widget can be select by keyboard or mouse. E.g.
    /// labels cannot be selected, but checkboxes can.
    fn is_selectable(&self) -> bool { false }

    /// Returns a bool representing whether this widget is currently selected.
    fn is_selected(&self) -> bool { false }

    /// Get the order in which this widget should be selected, represented by a usize number. E.g.
    /// if there is a '1' widget, a '2' widget, and this widget is '3', calling 'select_next_widget'
    /// will select 1, then 2, then this widget. Used for keyboard up and down keys.
    fn get_selection_order(&self) -> usize { 0 }

    /// Set the order in which this widget should be selected, represented by a usize number. E.g.
    /// if there is a '1' widget, a '2' widget, and this widget is '3', calling 'select_next_widget'
    /// will select 1, then 2, then this widget. Used for keyboard up and down keys.
    fn set_selection_order(&mut self, _order: usize) {}

    /// Call this function whenever the value of a widget is considered changed. E.g. when a
    /// checkbox is checked. This will active any bound on_value callbacks.
    fn on_value_change(&self, widget_path: String, view_tree: &mut ViewTree,
                       state_tree: &mut StateTree, widget_tree: &WidgetTree) {
        if let Some(i) = self.get_bind_on_value_change() {
            i(widget_path, view_tree, state_tree, widget_tree);
        }
    }

    /// Set the callback for when the value of a widget changes.
    fn set_bind_on_value_change(&mut self, _func: ValueChangeCallbackFunction) {}

    /// Get the callback for when the value of a widget changes.
    fn get_bind_on_value_change(&self) -> Option<ValueChangeCallbackFunction> {None}

    /// Set the callback for when enter is pressed when this widget is selected.
    fn set_bind_keyboard_enter(&mut self, _func: fn()) {}

    /// Get the callback for when enter is pressed when this widget is selected.
    fn get_bind_keyboard_enter(&self) -> Option<fn()> {None}

    /// This is called automatically by a global keybind on the currently selected widget.
    /// Will active the 'on_keyboard_enter' callback.
    fn on_keyboard_enter(&self, _view_tree: &mut ViewTree, _state_tree: &mut StateTree,
                         _widget_tree: &WidgetTree) {
        match self.get_bind_keyboard_enter() {
            Some(i) => i(),
            None => (),
        }
    }

    /// This is called automatically by a global mouse bind on the currently selected widget.
    /// Will active the 'on_left_click' callback.
    fn on_left_click(&self, position: Coordinates, _view_tree: &mut ViewTree,
                     _state_tree: &mut StateTree, _widget_tree: &WidgetTree) {
        match self.get_bind_left_click() {
            Some(i) => i(position),
            None => (),
        }
    }
    /// Set the callback for when this widget is left clicked.
    fn set_bind_left_click(&mut self, _func: fn(pos: Coordinates)) {}

    /// Get the callback for when this widget is left clicked.
    fn get_bind_left_click(&self) -> Option<fn(pos: Coordinates)> {None}

    /// This is called automatically by a global mouse bind on the currently selected widget.
    /// Will active the 'on_right_click' callback.
    fn on_right_click(&self, position: Coordinates, _view_tree: &mut ViewTree,
                      _state_tree: &mut StateTree, _widget_tree: &WidgetTree) {
        match self.get_bind_right_click() {
            Some(i) => i(position),
            None => (),
        }
    }

    /// Set the callback for when this widget is right clicked.
    fn set_bind_right_click(&mut self, _func: fn(pos: Coordinates)) {}

    /// Get the callback for when this widget is right clicked.
    fn get_bind_right_click(&self) -> Option<fn(pos: Coordinates)> {None}

    /// Returns a bool representing whether the state of this widget has changed by comparing a
    /// passed state to its' current state.
    fn state_changed(&self, other_state: &WidgetState) -> bool;

    /// Set a passed state as the current state.
    fn update_state(&mut self, new_state: &WidgetState);

}

/// Struct representing a single X,Y position on the screen. It has a symbol, colors, and other
/// properties governing how the position will look on screen.
#[derive(Clone)]
pub struct Pixel {

    /// Symbol drawn on screen.
    pub symbol: String,

    /// Foreground color in crossterm::style::color
    pub foreground_color: Color,

    /// Background color in crossterm::style::color
    pub background_color: Color,

    /// Whether symbol should be underlined
    pub underline: bool
}
impl Pixel {
    /// Turn into a crossterm StyledContent which can be drawn on screen.
    pub fn get_pixel(&self) -> StyledContent<String> {
        let mut pixel = self.symbol.clone()
            .with(self.foreground_color)
            .on(self.background_color);
        if self.underline {
            pixel = pixel.underlined();
        }
        pixel
    }

    /// Create a pixel from a symbol with default values.
    pub fn from_symbol(symbol: String) -> Self {
        let mut pixel = Pixel::default();
        pixel.symbol = symbol;
        pixel
    }
}
impl Default for Pixel {
    fn default() -> Self {
       Pixel{
           symbol: " ".to_string(),
           foreground_color: Color::White,
           background_color: Color::Blue,
           underline: false
       }
    }
}