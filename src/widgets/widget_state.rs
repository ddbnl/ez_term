//! # Widget state:
//! A module containing the base structs and traits for widget states.
use crate::widgets::canvas_widget::{CanvasState};
use crate::widgets::checkbox::{CheckboxState};
use crate::widgets::dropdown::{ DropdownState};
use crate::widgets::radio_button::{RadioButtonState};
use crate::widgets::text_input::{TextInputState};
use crate::widgets::label::{LabelState};


/// Widget states are used to keep track of dynamic run time information of widgets, such as the
/// text of a label, or whether a checkbox is currently checked. All callbacks receive a mutable
/// ref to a StateTree which contains al widget states, so they can change widgets without a
/// mutable ref to the widget itself. Every frame the StateTree is compared to each widget to see
/// which widget has changed so it can be redrawn. The specific state struct for each widget type
/// is defined its' own module.
pub enum WidgetState {
    CanvasWidget(CanvasState),
    Checkbox(CheckboxState),
    Dropdown(DropdownState),
    RadioButton(RadioButtonState),
    Label(LabelState),
    TextInput(TextInputState),
}
impl WidgetState {

    /// Cast this enum to a redraw widget state trait object, which contains methods setting the
    /// redraw field of the widget state. All widget states implement forced redraw, so this method
    /// can always be called safely.
    pub fn as_redraw_state(&self) -> &dyn RedrawWidgetState {
        match self {
            WidgetState::Checkbox(i) => i,
            WidgetState::Dropdown(i) => i,
            WidgetState::RadioButton(i) => i,
            WidgetState::TextInput(i) => i,
            WidgetState::CanvasWidget(i) => i,
            WidgetState::Label(i) => i,
        }
    }

    /// Cast this enum to a mutable redraw widget state trait object, which contains methods
    /// setting the redraw field of the widget state. All widget states implement forced redraw,
    /// so this method can always be called safely.
    pub fn as_redraw_state_mut(&mut self) -> &mut dyn RedrawWidgetState {
        match self {
            WidgetState::Checkbox(i) => i,
            WidgetState::Dropdown(i) => i,
            WidgetState::RadioButton(i) => i,
            WidgetState::TextInput(i) => i,
            WidgetState::CanvasWidget(i) => i,
            WidgetState::Label(i) => i,
        }
    }

    /// Cast this enum to a selectable widget state trait object, which contains methods
    /// for managing the selection fields of a widget state. Not all widgets can be selected, so
    /// you have to be sure you are calling this method on one of the following:
    /// - CheckboxState
    /// - DropdownState
    /// - RadioButtonState
    /// - TextInputState
    pub fn as_selectable(&self) -> &dyn SelectableWidgetState {
        match self {
            WidgetState::Checkbox(i) => i,
            WidgetState::Dropdown(i) => i,
            WidgetState::RadioButton(i) => i,
            WidgetState::TextInput(i) => i,
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
    pub fn as_selectable_mut(&mut self) -> &mut dyn SelectableWidgetState {
        match self {
            WidgetState::Checkbox(i) => i,
            WidgetState::Dropdown(i) => i,
            WidgetState::RadioButton(i) => i,
            WidgetState::TextInput(i) => i,
            _ => panic!("Cannot be cast to selectable widget state")
        }
    }

    /// Cast this state as a Canvas widget state ref, you must be sure you have one.
    pub fn as_canvas(&self) -> &CanvasState {
        if let WidgetState::CanvasWidget(i) = self { i }
        else { panic!("wrong state.") }
    }

    /// Cast this state as a mutable Canvas widget state ref, you must be sure you have one.
    pub fn as_canvas_mut(&mut self) -> &mut CanvasState {
        if let WidgetState::CanvasWidget(i) = self { i }
        else { panic!("wrong state.") }
    }

    /// Cast this state as a Checkbox widget state ref, you must be sure you have one.
    pub fn as_checkbox(&self) -> &CheckboxState {
        if let WidgetState::Checkbox(i) = self { i }
        else { panic!("wrong state.") }
    }

    /// Cast this state as a mutable Checkbox widget state ref, you must be sure you have one.
    pub fn as_checkbox_mut(&mut self) -> &mut CheckboxState {
        if let WidgetState::Checkbox(i) = self { i }
        else { panic!("wrong state.") }
    }

    /// Cast this state as a Dropdown widget state ref, you must be sure you have one.
    pub fn as_dropdown(&self) -> &DropdownState {
        if let WidgetState::Dropdown(i) = self { i }
        else { panic!("wrong state.") }
    }

    /// Cast this state as a mutable Dropdown widget state ref, you must be sure you have one.
    pub fn as_dropdown_mut(&mut self) -> &mut DropdownState {
        if let WidgetState::Dropdown(i) = self { i }
        else { panic!("wrong state.") }
    }

    /// Cast this state as a Label widget state ref, you must be sure you have one.
    pub fn as_label(&self) -> &LabelState {
        if let WidgetState::Label(i) = self { i }
        else { panic!("wrong state.") }
    }

    /// Cast this state as a mutable Label widget state ref, you must be sure you have one.
    pub fn as_label_mut(&mut self) -> &mut LabelState {
        if let WidgetState::Label(i) = self { i }
        else { panic!("wrong state.") }
    }

    /// Cast this state as a RadioButton widget state ref, you must be sure you have one.
    pub fn as_radio_button(&self) -> &RadioButtonState {
        if let WidgetState::RadioButton(i) = self { i }
        else { panic!("wrong state.") }
    }

    /// Cast this state as a mutable RadioButton widget state ref, you must be sure you have one.
    pub fn as_radio_button_mut(&mut self) -> &mut RadioButtonState {
        if let WidgetState::RadioButton(i) = self { i }
        else { panic!("wrong state.") }
    }

    /// Cast this state as a TextInput widget state ref, you must be sure you have one.
    pub fn as_text_input(&self) -> &TextInputState {
        if let WidgetState::TextInput(i) = self { i }
        else { panic!("wrong state.") }
    }

    /// Cast this state as a mutable TextInput widget state ref, you must be sure you have one.
    pub fn as_text_input_mut(&mut self) -> &mut TextInputState {
        if let WidgetState::TextInput(i) = self { i }
        else { panic!("wrong state.") }
    }
}


/// WidgetState trait which contains methods for managing the forced redraw field of a widget state.
/// This is implemented for all widgets.
pub trait RedrawWidgetState {
    fn set_force_redraw(&mut self, state: bool);
    fn get_force_redraw(&self) -> bool;
}


/// WidgetState trait which contains methods for managed the selection fields of a state. Only
/// implemented by widgets that can be selected.
pub trait SelectableWidgetState {
    fn set_selected(&mut self, state: bool);
    fn get_selected(&self) -> bool;
}
