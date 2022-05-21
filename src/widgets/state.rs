//! # Widget state:
//! A module containing the base structs and traits for widget states.
use crate::common::Coordinates;
use crate::widgets::canvas_widget::{CanvasState};
use crate::widgets::label::{LabelState};
use crate::widgets::button::{ButtonState};
use crate::widgets::checkbox::{CheckboxState};
use crate::widgets::dropdown::{ DropdownState};
use crate::widgets::layout::LayoutState;
use crate::widgets::radio_button::{RadioButtonState};
use crate::widgets::text_input::{TextInputState};


/// Widget states are used to keep track of dynamic run time information of widgets, such as the
/// text of a label, or whether a checkbox is currently checked. All callbacks receive a mutable
/// ref to a StateTree which contains al widget states, so they can change widgets without a
/// mutable ref to the widget itself. Every frame the StateTree is compared to each widget to see
/// which widget has changed so it can be redrawn. The specific state struct for each widget type
/// is defined its' own module.
pub enum State {
    Layout(LayoutState),
    Label(LabelState),
    Button(ButtonState),
    CanvasWidget(CanvasState),
    Checkbox(CheckboxState),
    Dropdown(DropdownState),
    RadioButton(RadioButtonState),
    TextInput(TextInputState),
}
impl State {

    /// Cast this enum to a generic widget state trait object, which contains methods for setting
    /// and getting fields common to all widget states. Can always be called safely.
    pub fn as_generic_state(&self) -> &dyn GenericState {
        match self {
            State::Layout(i) => i,
            State::Label(i) => i,
            State::Button(i) => i,
            State::Checkbox(i) => i,
            State::Dropdown(i) => i,
            State::RadioButton(i) => i,
            State::TextInput(i) => i,
            State::CanvasWidget(i) => i,
        }
    }

    /// Cast this enum to a mutable generic widget state trait object, which contains methods
    /// for setting and getting fields common to all widget states. Can always be called safely.
    pub fn as_generic_mut(&mut self) -> &mut dyn GenericState {
        match self {
            State::Layout(i) => i,
            State::Label(i) => i,
            State::Button(i) => i,
            State::Checkbox(i) => i,
            State::Dropdown(i) => i,
            State::RadioButton(i) => i,
            State::TextInput(i) => i,
            State::CanvasWidget(i) => i,
        }
    }

    /// Cast this enum to a selectable widget state trait object, which contains methods
    /// for managing the selection fields of a widget state. Not all widgets can be selected, so
    /// you have to be sure you are calling this method on one of the following:
    /// - CheckboxState
    /// - DropdownState
    /// - RadioButtonState
    /// - TextInputState
    pub fn as_selectable(&self) -> &dyn SelectableState {
        match self {
            State::Button(i) => i,
            State::Checkbox(i) => i,
            State::Dropdown(i) => i,
            State::RadioButton(i) => i,
            State::TextInput(i) => i,
            _ => panic!("Cannot be cast to selectable widget state")
        }
    }

    /// Cast this enum to a mutable selectable widget state trait object, which contains methods
    /// for managing the selection fields of a widget state. Not all widgets can be selected, so
    /// you have to be sure you are calling this method on one of the following state variants:
    /// - CheckboxState
    /// - DropdownState
    /// - RadioButtonState
    /// - TextInputState
    pub fn as_selectable_mut(&mut self) -> &mut dyn SelectableState {
        match self {
            State::Button(i) => i,
            State::Checkbox(i) => i,
            State::Dropdown(i) => i,
            State::RadioButton(i) => i,
            State::TextInput(i) => i,
            _ => panic!("Cannot be cast to selectable widget state")
        }
    }

    /// Cast this state as a Layout state ref, you must be sure you have one.
    pub fn as_layout(&self) -> &LayoutState {
        if let State::Layout(i) = self { i }
        else { panic!("wrong state.") }
    }

    /// Cast this state as a mutable Layout state ref, you must be sure you have one.
    pub fn as_layout_mut(&mut self) -> &mut LayoutState {
        if let State::Layout(i) = self { i }
        else { panic!("wrong state.") }
    }

    /// Cast this state as a Canvas widget state ref, you must be sure you have one.
    pub fn as_canvas(&self) -> &CanvasState {
        if let State::CanvasWidget(i) = self { i }
        else { panic!("wrong state.") }
    }

    /// Cast this state as a mutable Canvas widget state ref, you must be sure you have one.
    pub fn as_canvas_mut(&mut self) -> &mut CanvasState {
        if let State::CanvasWidget(i) = self { i }
        else { panic!("wrong state.") }
    }

    /// Cast this state as a Label widget state ref, you must be sure you have one.
    pub fn as_label(&self) -> &LabelState {
        if let State::Label(i) = self { i }
        else { panic!("wrong state.") }
    }

    /// Cast this state as a mutable Label widget state ref, you must be sure you have one.
    pub fn as_label_mut(&mut self) -> &mut LabelState {
        if let State::Label(i) = self { i }
        else { panic!("wrong state.") }
    }

    /// Cast this state as a Label widget state ref, you must be sure you have one.
    pub fn as_button(&self) -> &ButtonState {
        if let State::Button(i) = self { i }
        else { panic!("wrong state.") }
    }

    /// Cast this state as a mutable Label widget state ref, you must be sure you have one.
    pub fn as_button_mut(&mut self) -> &mut ButtonState {
        if let State::Button(i) = self { i }
        else { panic!("wrong state.") }
    }

    /// Cast this state as a Checkbox widget state ref, you must be sure you have one.
    pub fn as_checkbox(&self) -> &CheckboxState {
        if let State::Checkbox(i) = self { i }
        else { panic!("wrong state.") }
    }

    /// Cast this state as a mutable Checkbox widget state ref, you must be sure you have one.
    pub fn as_checkbox_mut(&mut self) -> &mut CheckboxState {
        if let State::Checkbox(i) = self { i }
        else { panic!("wrong state.") }
    }

    /// Cast this state as a Dropdown widget state ref, you must be sure you have one.
    pub fn as_dropdown(&self) -> &DropdownState {
        if let State::Dropdown(i) = self { i }
        else { panic!("wrong state.") }
    }

    /// Cast this state as a mutable Dropdown widget state ref, you must be sure you have one.
    pub fn as_dropdown_mut(&mut self) -> &mut DropdownState {
        if let State::Dropdown(i) = self { i }
        else { panic!("wrong state.") }
    }

    /// Cast this state as a RadioButton widget state ref, you must be sure you have one.
    pub fn as_radio_button(&self) -> &RadioButtonState {
        if let State::RadioButton(i) = self { i }
        else { panic!("wrong state.") }
    }

    /// Cast this state as a mutable RadioButton widget state ref, you must be sure you have one.
    pub fn as_radio_button_mut(&mut self) -> &mut RadioButtonState {
        if let State::RadioButton(i) = self { i }
        else { panic!("wrong state.") }
    }

    /// Cast this state as a TextInput widget state ref, you must be sure you have one.
    pub fn as_text_input(&self) -> &TextInputState {
        if let State::TextInput(i) = self { i }
        else { panic!("wrong state.") }
    }

    /// Cast this state as a mutable TextInput widget state ref, you must be sure you have one.
    pub fn as_text_input_mut(&mut self) -> &mut TextInputState {
        if let State::TextInput(i) = self { i }
        else { panic!("wrong state.") }
    }
}


/// State trait which contains methods for managing fields common to all widget states.
pub trait GenericState {
    fn set_changed(&mut self, changed: bool);
    fn get_changed(&self) -> bool;
    fn set_size_hint_x(&mut self, size_hint: Option<f64>);
    fn get_size_hint_x(&self) -> Option<f64>;
    fn set_size_hint_y(&mut self, size_hint: Option<f64>);
    fn get_size_hint_y(&self) -> Option<f64>;
    fn set_width(&mut self, width: usize);
    fn get_width(&self) -> usize;
    fn set_height(&mut self, height: usize);
    fn get_height(&self) -> usize;
    fn set_position(&mut self, pos: Coordinates);
    fn get_position(&self) -> Coordinates;
    fn set_force_redraw(&mut self, state: bool);
    fn get_force_redraw(&self) -> bool;
}


/// State trait which contains methods for managed the selection fields of a state. Only
/// implemented by widgets that can be selected.
pub trait SelectableState {
    fn set_selected(&mut self, state: bool);
    fn get_selected(&self) -> bool;
}
