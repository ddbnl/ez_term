//! # Widget state:
//! A module containing the base structs and traits for widget states.
use crate::common::{Coordinates};
use crate::states::canvas_state::{CanvasState};
use crate::states::label_state::{LabelState};
use crate::states::button_state::{ButtonState};
use crate::states::checkbox_state::{CheckboxState};
use crate::states::dropdown_state::{ DropdownState};
use crate::states::layout_state::LayoutState;
use crate::states::radio_button_state::{RadioButtonState};
use crate::states::text_input_state::{TextInputState};


/// Widget states are used to keep track of dynamic run time information of widgets, such as the
/// text of a label, or whether a checkbox is currently checked. All callbacks receive a mutable
/// ref to a StateTree which contains al widget states, so they can change widgets without a
/// mutable ref to the widget itself. Every frame the StateTree is compared to each widget to see
/// which widget has changed so it can be redrawn. The specific state struct for each widget type
/// is defined its' own module.
pub enum EzState {
    Layout(LayoutState),
    Label(LabelState),
    Button(ButtonState),
    CanvasWidget(CanvasState),
    Checkbox(CheckboxState),
    Dropdown(DropdownState),
    RadioButton(RadioButtonState),
    TextInput(TextInputState),
}
impl EzState {

    /// Cast this enum to a generic widget state trait object, which contains methods for setting
    /// and getting fields common to all widget states. Can always be called safely.
    pub fn as_generic(&self) -> &dyn GenericState {
        match self {
            EzState::Layout(i) => i,
            EzState::Label(i) => i,
            EzState::Button(i) => i,
            EzState::Checkbox(i) => i,
            EzState::Dropdown(i) => i,
            EzState::RadioButton(i) => i,
            EzState::TextInput(i) => i,
            EzState::CanvasWidget(i) => i,
        }
    }

    /// Cast this enum to a mutable generic widget state trait object, which contains methods
    /// for setting and getting fields common to all widget states. Can always be called safely.
    pub fn as_generic_mut(&mut self) -> &mut dyn GenericState {
        match self {
            EzState::Layout(i) => i,
            EzState::Label(i) => i,
            EzState::Button(i) => i,
            EzState::Checkbox(i) => i,
            EzState::Dropdown(i) => i,
            EzState::RadioButton(i) => i,
            EzState::TextInput(i) => i,
            EzState::CanvasWidget(i) => i,
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
            EzState::Button(i) => i,
            EzState::Checkbox(i) => i,
            EzState::Dropdown(i) => i,
            EzState::RadioButton(i) => i,
            EzState::TextInput(i) => i,
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
            EzState::Button(i) => i,
            EzState::Checkbox(i) => i,
            EzState::Dropdown(i) => i,
            EzState::RadioButton(i) => i,
            EzState::TextInput(i) => i,
            _ => panic!("Cannot be cast to selectable widget state")
        }
    }

    /// Cast this state as a Layout state ref, you must be sure you have one.
    pub fn as_layout(&self) -> &LayoutState {
        if let EzState::Layout(i) = self { i }
        else { panic!("wrong state.") }
    }

    /// Cast this state as a mutable Layout state ref, you must be sure you have one.
    pub fn as_layout_mut(&mut self) -> &mut LayoutState {
        if let EzState::Layout(i) = self { i }
        else { panic!("wrong state.") }
    }

    /// Cast this state as a Canvas widget state ref, you must be sure you have one.
    pub fn as_canvas(&self) -> &CanvasState {
        if let EzState::CanvasWidget(i) = self { i }
        else { panic!("wrong state.") }
    }

    /// Cast this state as a mutable Canvas widget state ref, you must be sure you have one.
    pub fn as_canvas_mut(&mut self) -> &mut CanvasState {
        if let EzState::CanvasWidget(i) = self { i }
        else { panic!("wrong state.") }
    }

    /// Cast this state as a Label widget state ref, you must be sure you have one.
    pub fn as_label(&self) -> &LabelState {
        if let EzState::Label(i) = self { i }
        else { panic!("wrong state.") }
    }

    /// Cast this state as a mutable Label widget state ref, you must be sure you have one.
    pub fn as_label_mut(&mut self) -> &mut LabelState {
        if let EzState::Label(i) = self { i }
        else { panic!("wrong state.") }
    }

    /// Cast this state as a Label widget state ref, you must be sure you have one.
    pub fn as_button(&self) -> &ButtonState {
        if let EzState::Button(i) = self { i }
        else { panic!("wrong state.") }
    }

    /// Cast this state as a mutable Label widget state ref, you must be sure you have one.
    pub fn as_button_mut(&mut self) -> &mut ButtonState {
        if let EzState::Button(i) = self { i }
        else { panic!("wrong state.") }
    }

    /// Cast this state as a Checkbox widget state ref, you must be sure you have one.
    pub fn as_checkbox(&self) -> &CheckboxState {
        if let EzState::Checkbox(i) = self { i }
        else { panic!("wrong state.") }
    }

    /// Cast this state as a mutable Checkbox widget state ref, you must be sure you have one.
    pub fn as_checkbox_mut(&mut self) -> &mut CheckboxState {
        if let EzState::Checkbox(i) = self { i }
        else { panic!("wrong state.") }
    }

    /// Cast this state as a Dropdown widget state ref, you must be sure you have one.
    pub fn as_dropdown(&self) -> &DropdownState {
        if let EzState::Dropdown(i) = self { i }
        else { panic!("wrong state.") }
    }

    /// Cast this state as a mutable Dropdown widget state ref, you must be sure you have one.
    pub fn as_dropdown_mut(&mut self) -> &mut DropdownState {
        if let EzState::Dropdown(i) = self { i }
        else { panic!("wrong state.") }
    }

    /// Cast this state as a RadioButton widget state ref, you must be sure you have one.
    pub fn as_radio_button(&self) -> &RadioButtonState {
        if let EzState::RadioButton(i) = self { i }
        else { panic!("wrong state.") }
    }

    /// Cast this state as a mutable RadioButton widget state ref, you must be sure you have one.
    pub fn as_radio_button_mut(&mut self) -> &mut RadioButtonState {
        if let EzState::RadioButton(i) = self { i }
        else { panic!("wrong state.") }
    }

    /// Cast this state as a TextInput widget state ref, you must be sure you have one.
    pub fn as_text_input(&self) -> &TextInputState {
        if let EzState::TextInput(i) = self { i }
        else { panic!("wrong state.") }
    }

    /// Cast this state as a mutable TextInput widget state ref, you must be sure you have one.
    pub fn as_text_input_mut(&mut self) -> &mut TextInputState {
        if let EzState::TextInput(i) = self { i }
        else { panic!("wrong state.") }
    }
}


/// State trait which contains methods for managing fields common to all widget states.
pub trait GenericState {
    /// Set to true whenever state changes to redraw widget next frame
    fn set_changed(&mut self, changed: bool);

    /// Widget is redrawn next frame if this returns true
    fn get_changed(&self) -> bool;

    /// Set to None for passing an absolute width, or to a value between 0 and 1 to
    /// automatically scale width based on parent width
    fn set_size_hint_x(&mut self, _size_hint: Option<f64>) {}

    /// If not None automatically scaled width based on parent width
    fn get_size_hint_x(&self) -> Option<f64>;

    /// Set to None for passing an absolute height, or to a value between 0 and 1 to
    /// automatically scale width based on parent height
    fn set_size_hint_y(&mut self, _size_hint: Option<f64>) {}

    /// If not None automatically scaled height based on parent height
    fn get_size_hint_y(&self) -> Option<f64>;

    /// Set to None to allow hardcoded or default positions. Set a pos hint to position relative
    /// to parent. A pos hint is a string and a float e.g:
    /// ("center_x", 0.9)
    /// When positioning the widget, "center_x" will be replaced with the middle x coordinate of
    /// parent Layout, and multiplied with the float.
    fn set_pos_hint_x(&mut self, _pos_hint: Option<(HorizontalPositionHint, f64)>) {}

    /// If none widget uses hardcoded or default positions. If a pos hint the widget will be
    /// positioned relative to its' parent.
    fn get_pos_hint_x(&self) -> &Option<(HorizontalPositionHint, f64)>;

    /// Set to None to allow hardcoded or default positions. Set a pos hint to position relative
    /// to parent. A pos hint is a string and a float e.g:
    /// ("top", 0.9)
    /// When positioning the widget, "top" will be replaced with the y coordinate of the top of the
    /// parent Layout, and multiplied by the float.
    fn set_pos_hint_y(&mut self, _pos_hint: Option<(VerticalPositionHint, f64)>) {}

    /// If none widget uses hardcoded or default positions. If a pos hint the widget will be
    /// positioned relative to its' parent.
    fn get_pos_hint_y(&self) -> &Option<(VerticalPositionHint, f64)>;

    /// Set width autoscaling bool. If the widget supports it and turned on,
    /// automatically adjusts width to the actual width of its' content
    fn set_auto_scale_width(&mut self, _auto_scale: bool) {
        panic!("Auto scaling not supported for this widget")
    }

    /// Get width autoscaling bool. If the widget supports it and turned on,
    /// automatically adjusts width to the actual width of its' content
    fn get_auto_scale_width(&self) -> bool { false }

    /// Set height autoscaling bool. If the widget supports it and turned on,
    /// automatically adjusts height to the actual height of its' content
    fn set_auto_scale_height(&mut self, _auto_scale: bool) {
        panic!("Auto scaling not supported for this widget")
    }

    /// Get height autoscaling bool. If the widget supports it and turned on,
    /// automatically adjusts height to the actual height of its' content
    fn get_auto_scale_height(&self) -> bool { false }

    /// Hard code width, only does something when size_hint_x is off
    fn set_width(&mut self, width: usize);

    /// Get width, only does something when size_hint_x is off
    fn get_width(&self) -> usize;

    /// Set the how much width you want the actual content inside this widget to have. Width for
    /// e.g. border and padding will be added to this automatically.
    fn set_effective_width(&mut self, width: usize) { self.set_width(width) }

    /// Get the effective amount of width within the widgets, taking off e.g. borders, padding, etc.
    fn get_effective_width(&self) -> usize { self.get_width() }

    /// Hard code height, only does something when size_hint_y is off
    fn set_height(&mut self, height: usize);

    /// Get height, only does something when size_hint_y is off
    fn get_height(&self) -> usize;

    /// Set the how much height you want the actual content inside this widget to have. Height for
    /// e.g. border and padding will be added to this automatically.
    fn set_effective_height(&mut self, height: usize) { self.set_height(height) }

    /// Get the effective amount of height within the widgets, taking off e.g. borders, padding, etc.
    fn get_effective_height(&self) -> usize { self.get_height() }

    /// Hard code position relative to parent, only does something in float layout mode
    fn set_position(&mut self, pos: Coordinates);

    /// Get position relative to parent
    fn get_position(&self) -> Coordinates;

    /// Get position where the actual content of this widget starts relative to parent, taking out
    /// e.g. borders, padding, etc.
    fn get_effective_position(&self) -> Coordinates { self.get_position() }

    /// Set the absolute position of a widget, i.e. the position on screen rather than within its'
    /// parent widget. Should be set automatically through the "propagate_absolute_positions"
    /// function.
    fn set_absolute_position(&mut self, pos:Coordinates);

    /// Get the absolute position of a widget, i.e. the position on screen rather than within its'
    /// parent widget.
    fn get_absolute_position(&self) -> Coordinates;

    /// Get the absolute position of where the actual content of the widget starts, taking out
    /// e.g. border and padding
    fn get_effective_absolute_position(&self) -> Coordinates { self.get_absolute_position() }

    /// Set [HorizontalAlignment] of this widget.
    fn set_horizontal_alignment(&mut self, alignment: HorizontalAlignment);

    /// Get [HorizontalAlignment] of this widget
    fn get_horizontal_alignment(&self) -> HorizontalAlignment;

    /// Set [VerticalAlignment] of this widget
    fn set_vertical_alignment(&mut self, alignment: VerticalAlignment);

    /// Get [VerticalAlignment] of this widget
    fn get_vertical_alignment(&self) -> VerticalAlignment;


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
        let starting_pos = self.get_effective_absolute_position();
        let end_pos =
            (starting_pos.0 + self.get_width() - 1,
             starting_pos.1 + self.get_height() - 1);
        pos.0 >= starting_pos.0 && pos.0 <= end_pos.0 &&
            pos.1 >= starting_pos.1 && pos.1 <= end_pos.1
    }
    /// Set to true to force redraw the entire screen. The screen is still diffed before redrawing
    /// so this can be called efficiently. Nevertheless you want to call [set_changed] to redraw
    /// only the specific widget in most cases.
    fn set_force_redraw(&mut self, state: bool);

    /// Redraws the entire screen if set to true.
    fn get_force_redraw(&self) -> bool;
}


/// State trait which contains methods for managed the selection fields of a state. Only
/// implemented by widgets that can be selected.
pub trait SelectableState {
    fn set_selected(&mut self, state: bool);
    fn get_selected(&self) -> bool;
}


#[derive(Clone, Copy)]
pub enum HorizontalAlignment {
    Left,
    Right,
    Center
}

#[derive(Clone, Copy)]
pub enum VerticalAlignment {
    Top,
    Bottom,
    Middle
}

#[derive(Clone, Copy)]
pub enum HorizontalPositionHint {
    Left,
    Right,
    Center
}

#[derive(Clone, Copy)]
pub enum VerticalPositionHint {
    Top,
    Bottom,
    Middle
}
