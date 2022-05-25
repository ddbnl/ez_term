//! # Widget:
//! A module containing the base structs and traits for widgets"
//! functions allows starting the app based on a root layout.
use crossterm::style::{Color, StyledContent, Stylize};
use crossterm::event::{Event, KeyCode};
use std::io::{Error};
use crate::common::{self, ViewTree, KeyboardCallbackFunction, PixelMap,
                    GenericEzFunction, MouseCallbackFunction, EzContext, StateTree, KeyMap};
use crate::states::state::{EzState, Coordinates};
use crate::widgets::layout::{Layout};
use crate::widgets::label::{Label};
use crate::widgets::button::{Button};
use crate::widgets::canvas::{CanvasWidget};
use crate::widgets::checkbox::{Checkbox};
use crate::widgets::dropdown::{Dropdown};
use crate::widgets::radio_button::{RadioButton};
use crate::widgets::text_input::{TextInput};


/// Enum with variants representing Layouts and each widget type. A layout is not considered a
/// widget, so this enum gathers widgets and layouts in one place, as they do have methods in
/// common (e.g. both have positions, sizes, etc.). To access common methods, cast this enum
/// into a EzObject (trait for Layouts+Widgets) or EzWidget (Widgets only).
pub enum EzObjects {
    Layout(Layout),
    Label(Label),
    Button(Button),
    CanvasWidget(CanvasWidget),
    Checkbox(Checkbox),
    Dropdown(Dropdown),
    RadioButton(RadioButton),
    TextInput(TextInput),
}
impl EzObjects {

    /// Cast this enum to a generic [EzObject] trait object. As this trait is implemented by both
    /// [Layout] and [widget], it is safe to call on all variants.
    pub fn as_ez_object(&self) -> &dyn EzObject {
        match self {
            EzObjects::Label(i) => i,
            EzObjects::Button(i) => i,
            EzObjects::Layout(i) => i,
            EzObjects::CanvasWidget(i) => i,
            EzObjects::Checkbox(i) => i,
            EzObjects::Dropdown(i) => i,
            EzObjects::RadioButton(i) => i,
            EzObjects::TextInput(i) => i,
        }
    }

    /// Cast this enum to a mutable generic [EzObject] trait object. As this trait is implemented
    /// by both [Layout] and [widget], it is safe to call on all variants.
    pub fn as_ez_object_mut(&mut self) -> &mut dyn EzObject {
        match self {
            EzObjects::Layout(i) => i,
            EzObjects::Label(i) => i,
            EzObjects::Button(i) => i,
            EzObjects::CanvasWidget(i) => i,
            EzObjects::Checkbox(i) => i,
            EzObjects::Dropdown(i) => i,
            EzObjects::RadioButton(i) => i,
            EzObjects::TextInput(i) => i,
        }
    }

    /// Cast this enum to a generic [EzWidget] trait object. As this trait is implemented only
    /// [widget] and not [Layout], it is safe to call *only* on widgets..
    pub fn as_ez_widget(&self) -> &dyn EzWidget {
        match self {
            EzObjects::Label(i) => i,
            EzObjects::Button(i) => i,
            EzObjects::CanvasWidget(i) => i,
            EzObjects::Checkbox(i) => i,
            EzObjects::Dropdown(i) => i,
            EzObjects::RadioButton(i) => i,
            EzObjects::TextInput(i) => i,
            _ => panic!("Casted non-widget to IsWidget"),
        }
    }

    /// Cast this enum to a generic mutable [EzWidget] trait object. As this trait is implemented
    /// only [widget] and not [Layout], it is safe to call *only* on widgets..
    pub fn as_ez_widget_mut(&mut self) -> &mut dyn EzWidget {
        match self {
            EzObjects::Label(i) => i,
            EzObjects::Button(i) => i,
            EzObjects::CanvasWidget(i) => i,
            EzObjects::Checkbox(i) => i,
            EzObjects::Dropdown(i) => i,
            EzObjects::RadioButton(i) => i,
            EzObjects::TextInput(i) => i,
            _ => panic!("Casted non-widget to IsWidget"),
        }
    }

    /// Cast this as a layout ref, you must be sure you have one.
    pub fn as_layout(&self) -> &Layout {
        if let EzObjects::Layout(i) = self { i }
        else { panic!("wrong EzObject.") }
    }

    /// Cast this as a mutable layout ref, you must be sure you have one.
    pub fn as_layout_mut(&mut self) -> &mut Layout {
        if let EzObjects::Layout(i) = self { i }
        else { panic!("wrong EzObject.") }
    }
    /// Cast this as a Canvas widget ref, you must be sure you have one.
    pub fn as_canvas(&self) -> &CanvasWidget {
        if let EzObjects::CanvasWidget(i) = self { i }
        else { panic!("wrong EzObject.") }
    }

    /// Cast this as a mutable Canvas widget ref, you must be sure you have one.
    pub fn as_canvas_mut(&mut self) -> &mut CanvasWidget {
        if let EzObjects::CanvasWidget(i) = self { i }
        else { panic!("wrong EzObject.") }
    }

    /// Cast this state as a Label widget ref, you must be sure you have one.
    pub fn as_label(&self) -> &Label {
        if let EzObjects::Label(i) = self { i }
        else { panic!("wrong EzObject.") }
    }

    /// Cast this state as a mutable Label widget ref, you must be sure you have one.
    pub fn as_label_mut(&mut self) -> &mut Label {
        if let EzObjects::Label(i) = self { i }
        else { panic!("wrong EzObject.") }
    }

    /// Cast this state as a Label widget ref, you must be sure you have one.
    pub fn as_button(&self) -> &Button {
        if let EzObjects::Button(i) = self { i }
        else { panic!("wrong EzObject.") }
    }

    /// Cast this state as a mutable Label widget ref, you must be sure you have one.
    pub fn as_button_mut(&mut self) -> &mut Button {
        if let EzObjects::Button(i) = self { i }
        else { panic!("wrong EzObject.") }
    }

    /// Cast this as a Checkbox widget ref, you must be sure you have one.
    pub fn as_checkbox(&self) -> &Checkbox {
        if let EzObjects::Checkbox(i) = self { i }
        else { panic!("wrong EzObject.") }
    }

    /// Cast this as a mutable Checkbox widget ref, you must be sure you have one.
    pub fn as_checkbox_mut(&mut self) -> &mut Checkbox {
        if let EzObjects::Checkbox(i) = self { i }
        else { panic!("wrong EzObject.") }
    }

    /// Cast this state as a Dropdown widget ref, you must be sure you have one.
    pub fn as_dropdown(&self) -> &Dropdown {
        if let EzObjects::Dropdown(i) = self { i }
        else { panic!("wrong EzObject.") }
    }

    /// Cast this state as a mutable Dropdown widget ref, you must be sure you have one.
    pub fn as_dropdown_mut(&mut self) -> &mut Dropdown {
        if let EzObjects::Dropdown(i) = self { i }
        else { panic!("wrong EzObject.") }
    }

    /// Cast this state as a RadioButton widget ref, you must be sure you have one.
    pub fn as_radio_button(&self) -> &RadioButton {
        if let EzObjects::RadioButton(i) = self { i }
        else { panic!("wrong EzObject.") }
    }

    /// Cast this state as a mutable RadioButton widget ref, you must be sure you have one.
    pub fn as_radio_button_mut(&mut self) -> &mut RadioButton {
        if let EzObjects::RadioButton(i) = self { i }
        else { panic!("wrong EzObject.") }
    }

    /// Cast this state as a TextInput widget ref, you must be sure you have one.
    pub fn as_text_input(&self) -> &TextInput {
        if let EzObjects::TextInput(i) = self { i }
        else { panic!("wrong EzObject.") }
    }

    /// Cast this state as a mutable TextInput widget ref, you must be sure you have one.
    pub fn as_text_input_mut(&mut self) -> &mut TextInput {
        if let EzObjects::TextInput(i) = self { i }
        else { panic!("wrong EzObject.") }
    }
}


/// Trait representing both widgets and layouts implementing methods which are common to all UI
/// objects (such as size, position, etc.). If you don't know if an object is a Widget or a Layout
/// (or don't care), cast the EzObjects enum into this type using [az_ez_object].
pub trait EzObject {

    /// Accepts config lines from the ez_parser module and prepares them to be loaded by
    /// load_ez_parameter below.
    fn load_ez_config(&mut self, config: Vec<&str>) -> Result<(), Error> {
        for line in config {
            let (parameter_name, parameter_value) = line.split_once(':').unwrap();
            self.load_ez_parameter(parameter_name.to_string(),
                                   parameter_value.to_string())?;
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

    /// Set a passed state as the current state.
    fn update_state(&mut self, new_state: &EzState);

    /// Get the State object belonging to this widget.
    fn get_state(&self) -> EzState;

    /// Redraw the widget on the screen. Using the view tree, only changed content is written to
    /// improve performance.
    fn redraw(&self, view_tree: &mut ViewTree, state_tree: &mut StateTree) {
        let state = state_tree.get(&self.get_full_path()).unwrap().as_generic();
        let mut pos = state.get_absolute_position();
        let content = self.get_contents(state_tree);
        common::write_to_screen(pos, content, view_tree);
    }

    /// Set the content for a widget manually. This is not implemented for most widgets, as they
    /// get their content from their state. E.g. a label gets content from its' current text.
    fn set_contents(&mut self, _contents: PixelMap) {
        panic!("Cannot manually set content color for this widget {}", self.get_id()); }

    /// Gets the visual content for this widget. Overloaded by each widget module. E.g. a label
    /// gets its' content from its' text, a checkbox from whether it has been checked, etc.
    fn get_contents(&self, state_tree: &mut StateTree) -> PixelMap;
}


/// Trait representing widgets only, implementing methods which are common to all widgets but
/// not layouts. You can cast an EzObjects enum into this trait using [as_ez_widget]
/// if you know for sure the [EzObject] you're dealing with is a widget and not a layout.
pub trait EzWidget: EzObject {

    /// Set the focus state of a widget. When a widget is focussed it alone consumes all events.
    fn set_focus(&mut self, _enabled: bool) {}

    /// Get the focus state of a widget. When a widget is focussed it alone consumes all events.
    fn get_focus(&self) -> bool { false }

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
    fn set_selection_order(&mut self, _order: usize) {
        panic!("Widget has no selection implementation: {}", self.get_id())
    }

    /// Get the key map belonging to a widget. Any keys bound to the widget are in here along with
    /// their callbacks. Key map should be used inside the "handle_event" method of a widget.
    fn get_key_map(&self) -> &KeyMap {
        panic!("Widget does not support keymap: {}", self.get_id())
    }

    /// Bind a new key to the widget and the callback it should activate. Focussed widgets have
    /// priority consuming events, next are global key binds, and then the selected widget.
    fn bind_key(&mut self, _key: KeyCode, _func: KeyboardCallbackFunction) {
    }

    /// Optionally consume an event that was passed to this widget. Return true if the event should
    /// be considered consumed. Simply consults the keymap by default, but can be overloaded for
    /// more complex circumstances.
    fn handle_event(&self, event: Event, context: EzContext) -> bool {
        if let Event::Key(key) = event {
            if self.get_key_map().contains_key(&key.code) {
                let func =
                    self.get_key_map().get(&key.code).unwrap();
                func(context, key.code);
                return true
            }
        }
        false
    }

    /// Set the callback for when the value of a widget changes.
    fn set_bind_on_value_change(&mut self, _func: GenericEzFunction) {
        panic!("Cannot set on value change bind for: {}", self.get_id())
    }

    /// Get the callback for when the value of a widget changes.
    fn get_bind_on_value_change(&self) -> Option<GenericEzFunction> {None}


    /// Call this function whenever the value of a widget is considered changed. E.g. when a
    /// checkbox is checked. This will active any bound on_value callbacks.
    fn on_value_change(&self, context: EzContext) {
        if let Some(i) = self.get_bind_on_value_change() {
            i(context);
        }
    }

    /// Set the callback for when enter is pressed when this widget is selected or on left click
    fn set_bind_on_press(&mut self, _func: GenericEzFunction) {
        panic!("Cannot set on press bind for: {}", self.get_id())
    }

    /// Get the callback for when enter is pressed when this widget is selected or on left click
    fn get_bind_on_press(&self) -> Option<GenericEzFunction> {None}

    /// This is called automatically by a global key/mouse bind on enter or left click.
    /// Will activate the 'on_keyboard_enter' callback.
    fn on_press(&self, context: EzContext) {
        match self.get_bind_on_press() {
            Some(i) => i(context),
            None => (),
        }
    }

    /// Set the callback for when this widget is left clicked.
    fn set_bind_left_click(&mut self, _func: MouseCallbackFunction) {
        panic!("Cannot set on left click bind for: {}", self.get_id())
    }

    /// Get the callback for when this widget is left clicked.
    fn get_bind_left_click(&self) -> Option<MouseCallbackFunction> {None}

    /// This is called automatically by a global mouse bind on the currently selected widget.
    /// Will activate the 'on_left_click' callback.
    fn on_left_click(&self, context: EzContext, position: Coordinates) {
        match self.get_bind_left_click() {
            Some(i) => {
                i(context, position);
            },
            None => (),
        }
    }

    /// Set the callback for when enter is pressed when this widget is selected.
    fn set_bind_keyboard_enter(&mut self, _func: GenericEzFunction) {
        panic!("Cannot set on keyboard enter bind for: {}", self.get_id())
    }

    /// Get the callback for when enter is pressed when this widget is selected.
    fn get_bind_keyboard_enter(&self) -> Option<GenericEzFunction> {None}

    /// This is called automatically by a global keybind on the currently selected widget.
    /// Will active the 'on_keyboard_enter' callback.
    fn on_keyboard_enter(&self, context: EzContext){
        match self.get_bind_keyboard_enter() {
            Some(i) => i(context),
            None => (),
        }
    }

    /// Set the callback for when this widget is right clicked.
    fn set_bind_right_click(&mut self, _func: MouseCallbackFunction) {
        panic!("Cannot set on right click bind for: {}", self.get_id())
    }

    /// Get the callback for when this widget is right clicked.
    fn get_bind_right_click(&self) -> Option<MouseCallbackFunction> {None}

    /// This is called automatically by a global mouse bind on the currently selected widget.
    /// Will active the 'on_right_click' callback.
    fn on_right_click(&self, context: EzContext, position: Coordinates) {
        match self.get_bind_right_click() {
            Some(i) => i(context, position),
            None => (),
        }
    }

    /// Set the callback for when this widget is selected.
    fn set_bind_on_select(&mut self, _func: fn(context: EzContext, mouse_pos: Option<Coordinates>)) {
        panic!("Cannot set on deselect bind for: {}", self.get_id())
    }

    /// Get the callback for when this widget is selected.
    fn get_bind_on_select(&self) -> Option<fn(context: EzContext, mouse_pos: Option<Coordinates>)> {
        None
    }

    /// This is called when the widget is selected.
    fn on_select(&self, context: EzContext, mouse_pos: Option<Coordinates>) {
        context.state_tree.get_mut(&context.widget_path).unwrap().as_selectable_mut()
            .set_selected(true);
        match self.get_bind_on_select() {
            Some(i) => i(context, mouse_pos),
            None => (),
        }
    }

    /// Set the callback for when this widget is deselected.
    fn set_bind_on_deselect(&mut self, _func: GenericEzFunction) {
        panic!("Cannot set on deselect bind for: {}", self.get_id())
    }

    /// Get the callback for when this widget is deselected.
    fn get_bind_on_deselect(&self) -> Option<GenericEzFunction> {None}

    /// This is called when the widget is deselected.
    fn on_deselect(&self, context: EzContext) {

        context.state_tree.get_mut(&context.widget_path).unwrap().as_selectable_mut()
            .set_selected(false);
        match self.get_bind_on_deselect() {
            Some(i) => i(context),
            None => (),
        }
    }
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