//! # Widget state:
//! A module containing the base structs and traits for widget states.
use crossterm::style::Color;
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
    fn set_size_hint(&mut self, _size_hint: SizeHint) {}

    /// Set to None for passing an absolute width, or to a value between 0 and 1 to
    /// automatically scale width based on parent width
    fn set_size_hint_x(&mut self, size_hint: Option<f64>) {
        self.set_size_hint(SizeHint::new(size_hint, self.get_size_hint().y))
    }

    /// Set to None for passing an absolute height, or to a value between 0 and 1 to
    /// automatically scale width based on parent height
    fn set_size_hint_y(&mut self, size_hint: Option<f64>) {
        self.set_size_hint(SizeHint::new(self.get_size_hint().x, size_hint))
    }

    /// If not None automatically scaled width based on parent width
    fn get_size_hint(&self) -> &SizeHint;

    /// Set to None to allow hardcoded or default positions. Set a pos hint to position relative
    /// to parent. A pos hint is a string and a float e.g:
    /// ("center_x", 0.9)
    /// When positioning the widget, "center_x" will be replaced with the middle x coordinate of
    /// parent Layout, and multiplied with the float.
    fn set_pos_hint(&mut self, _pos_hint: PosHint) {}

    /// Set to None to allow hardcoded or default positions. Set a pos hint to position relative
    /// to parent. A pos hint is a string and a float e.g:
    /// ("center_x", 0.9)
    /// When positioning the widget, "center_x" will be replaced with the middle x coordinate of
    /// parent Layout, and multiplied with the float.
    fn set_pos_hint_x(&mut self, pos_hint: Option<(HorizontalPositionHint, f64)>) {
        self.set_pos_hint(PosHint::new(pos_hint, self.get_pos_hint().y))
    }

    /// Set to None to allow hardcoded or default positions. Set a pos hint to position relative
    /// to parent. A pos hint is a string and a float e.g:
    /// ("top", 0.9)
    /// When positioning the widget, "top" will be replaced with the y coordinate of the top of the
    /// parent Layout, and multiplied by the float.
    fn set_pos_hint_y(&mut self, pos_hint: Option<(VerticalPositionHint, f64)>) {
        self.set_pos_hint(PosHint::new(self.get_pos_hint().x, pos_hint))
    }

    /// If none widget uses hardcoded or default positions. If a pos hint the widget will be
    /// positioned relative to its' parent.
    fn get_pos_hint(&self) -> &PosHint;

    /// Set width autoscaling bool. If the widget supports it and turned on,
    /// automatically adjusts width to the actual width of its' content
    fn set_auto_scale(&mut self, _auto_scale: AutoScale);

    /// Get autoscaling config. If the widget supports it and turned on,
    /// automatically adjusts size to the actual size of its' content
    fn get_auto_scale(&self) -> &AutoScale;

    /// Set width autoscaling bool. If the widget supports it and turned on,
    /// automatically adjusts width to the actual width of its' content
    fn set_auto_scale_width(&mut self, auto_scale: bool) {
        self.set_auto_scale(AutoScale::new(auto_scale, self.get_auto_scale().height));
    }

    /// Set height autoscaling bool. If the widget supports it and turned on,
    /// automatically adjusts height to the actual height of its' content
    fn set_auto_scale_height(&mut self, auto_scale: bool) {
        self.set_auto_scale(AutoScale::new(self.get_auto_scale().width, auto_scale));
    }

    /// Hard code size, only does something when size_hint is off
    fn set_size(&mut self, size: Size);

    /// Get current [Size] of this object
    fn get_size(&self) -> &Size;

    /// Get the effective amount of width and height within this object, taking off e.g. borders,
    /// padding, etc.
    fn get_effective_size(&self) -> Size {

        let width_result: isize = self.get_size().width as isize
            -if self.has_border() {2} else {0}
            - self.get_padding().left as isize - self.get_padding().right as isize;
        let width = if width_result < 0 {0} else { width_result as usize};
        let height_result: isize = self.get_size().height as isize
            - if self.has_border() {2} else {0}
            - self.get_padding().top as isize - self.get_padding().bottom as isize;
        let height = if height_result < 0 {0} else { height_result as usize};
        Size::new(width, height)
    }

    /// Hard code width, only does something when size_hint_x is off
    fn set_width(&mut self, width: usize) {
        self.set_size(Size::new(width, self.get_size().height))
    }

    /// Set the how much width you want the actual content inside this widget to have. Width for
    /// e.g. border and padding will be added to this automatically.
    fn set_effective_width(&mut self, width: usize) {
        self.set_width(width +if self.has_border() {2} else {0}
            + self.get_padding().left + self.get_padding().right)
    }

    /// Hard code height, only does something when size_hint_y is off
    fn set_height(&mut self, height: usize) {
        self.set_size(Size::new(self.get_size().width, height))
    }

    /// Set the how much height you want the actual content inside this widget to have. Height for
    /// e.g. border and padding will be added to this automatically.
    fn set_effective_height(&mut self, height: usize) {
        self.set_height(height +if self.has_border() {2} else {0}
        + self.get_padding().top + self.get_padding().bottom)
    }

    /// Hard code position relative to parent, only works in float layout mode
    fn set_position(&mut self, pos: Coordinates);

    /// Set the x coordinate relative to parent, only works in float layout mode
    fn set_x(&mut self, x: usize) { self.set_position(Coordinates::new(x, self.get_position().y)) }

    /// Set the x coordinate relative to parent, only works in float layout mode
    fn set_y(&mut self, y: usize) { self.set_position(Coordinates::new(self.get_position().x, y)) }

    /// Get position relative to parent
    fn get_position(&self) -> Coordinates;

    /// Get position where the actual content of this widget starts relative to parent, taking out
    /// e.g. borders, padding, etc.
    fn get_effective_position(&self) -> Coordinates {
        Coordinates::new(
            self.get_position().x +if self.has_border() {1} else {0} + self.get_padding().left,
            self.get_position().y +if self.has_border() {1} else {0} + self.get_padding().top)
    }

    /// Set the absolute position of a widget, i.e. the position on screen rather than within its'
    /// parent widget. Should be set automatically through the "propagate_absolute_positions"
    /// function.
    fn set_absolute_position(&mut self, pos: Coordinates);

    /// Get the absolute position of a widget, i.e. the position on screen rather than within its'
    /// parent widget.
    fn get_absolute_position(&self) -> Coordinates;

    /// Get the absolute position of where the actual content of the widget starts, taking out
    /// e.g. border and padding
    fn get_effective_absolute_position(&self) -> Coordinates {
        Coordinates::new(
         self.get_absolute_position().x +if self.has_border() {1} else {0}
             + self.get_padding().left,
         self.get_absolute_position().y +if self.has_border() {1} else {0}
             + self.get_padding().top)
    }

    /// Set [HorizontalAlignment] of this widget.
    fn set_horizontal_alignment(&mut self, alignment: HorizontalAlignment);

    /// Get [HorizontalAlignment] of this widget
    fn get_horizontal_alignment(&self) -> HorizontalAlignment;

    /// Set [VerticalAlignment] of this widget
    fn set_vertical_alignment(&mut self, alignment: VerticalAlignment);

    /// Get [VerticalAlignment] of this widget
    fn get_vertical_alignment(&self) -> VerticalAlignment;

    /// Set [padding]
    fn set_padding(&mut self, padding: Padding);

    /// Get [padding]
    fn get_padding(&self) -> &Padding;

    /// Set height of top padding
    fn set_padding_top(&mut self, padding: usize) {
        let current = self.get_padding().clone();
        self.set_padding(Padding::new(padding, current.bottom, current.left, current.right))
    }

    /// Set height of bottom padding
    fn set_padding_bottom(&mut self, padding: usize) {
        let current = self.get_padding().clone();
        self.set_padding(Padding::new(current.top, padding, current.left, current.right))
    }

    /// Set width of left padding
    fn set_padding_left(&mut self, padding: usize) {
        let current = self.get_padding().clone();
        self.set_padding(Padding::new(current.top, current.bottom, padding, current.right))
    }

    /// Set width of right padding
    fn set_padding_right(&mut self, padding: usize) {
        let current = self.get_padding().clone();
        self.set_padding(Padding::new(current.top, current.bottom, current.left, padding))
    }

    /// Get a bool representing whether this object should have a surrounding border
    fn has_border(&self) -> bool;

    /// Set whether this object should have a surrounding border
    fn set_border(&mut self, enabled: bool);
    
    /// Pas a [BorderConfig] abject that will be used to draw the border if enabled
    fn set_border_config(&mut self, config: BorderConfig);

    /// Get the [state::BorderConfig] abject that will be used to draw the border if enabled
    fn get_border_config(&self) -> &BorderConfig;

    /// Set the [ColorConfig] abject that will be used to draw this widget
    fn set_colors(&mut self, config: ColorConfig);

    /// Get a ref to the [ColorConfig] abject that will be used to draw this widget
    fn get_colors(&self) -> &ColorConfig;

    /// Get a mut ref to the [ColorConfig] abject that will be used to draw this widget
    fn get_colors_mut(&mut self) -> &mut ColorConfig;

    /// Get the top left and bottom right corners of a widget in (X, Y) coordinate tuples.
    fn get_box(&self) -> (Coordinates, Coordinates) {
        let top_left = self.get_absolute_position();
        let top_right = Coordinates::new(
            top_left.x + self.get_size().width, top_left.y + self.get_size().height);
        (top_left, top_right)
    }

    /// Returns a bool representing whether two widgets overlap at any point.
    fn overlaps(&self, other_box: (Coordinates, Coordinates)) -> bool {
        let (l1, r1) = self.get_box();
        let (l2, r2) = other_box;
        // If one rectangle is on the left of the other there's no overlap
        if l1.x >= r2.x || l2.x >= r1.x { return false }
        // If one rectangle is above the other there's no overlap
        if r1.y >= l2.y || r2.y >= l1.y { return false }
        true
    }

    /// Returns a bool representing whether a single point collides with a widget.
    fn collides(&self, pos: Coordinates) -> bool {
        let starting_pos = self.get_effective_absolute_position();
        let end_pos = Coordinates::new(
            starting_pos.x + self.get_size().width - 1,
             starting_pos.y + self.get_size().height - 1);
        pos.x >= starting_pos.x && pos.x <= end_pos.x &&
            pos.y >= starting_pos.y && pos.y <= end_pos.y
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


#[derive(Clone, Copy, Debug)]
pub enum HorizontalAlignment {
    Left,
    Right,
    Center
}


#[derive(Clone, Copy, Debug)]
pub enum VerticalAlignment {
    Top,
    Bottom,
    Middle
}


#[derive(Clone, Copy, Debug)]
pub enum HorizontalPositionHint {
    Left,
    Right,
    Center
}


#[derive(Clone, Copy, Debug)]
pub enum VerticalPositionHint {
    Top,
    Bottom,
    Middle
}


/// Convenience wrapper around a size tuple.
#[derive(Copy, Clone, Default, Debug)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}
impl Size {
    pub fn new(width: usize, height: usize) -> Self { Size{width, height} }
}



/// Convenience wrapper around an XY tuple.
#[derive(Copy, Clone, Default, Debug)]
pub struct Coordinates {
    pub x: usize,
    pub y: usize,
}
impl Coordinates {
    pub fn new(x: usize, y: usize) -> Self {
        Coordinates{x, y}
    }
}


/// Convenience wrapper around an size_hint tuple.
#[derive(Copy, Clone, Debug)]
pub struct AutoScale {
    pub width: bool,
    pub height: bool,
}
impl AutoScale {
    pub fn new(width: bool, height: bool) -> Self { AutoScale{width, height} }
}
impl Default for AutoScale {
    fn default() -> Self { AutoScale{width: false, height: false }}
}


/// Convenience wrapper around an size_hint tuple.
#[derive(Copy, Clone, Debug)]
pub struct SizeHint {
    pub x: Option<f64>,
    pub y: Option<f64>,
}
impl SizeHint {
    pub fn new(x: Option<f64>, y: Option<f64>) -> Self { SizeHint{x, y} }
}
impl Default for SizeHint {
    fn default() -> Self { SizeHint{x: Some(1.0), y: Some(1.0) }}
}


/// Convenience wrapper around an pos_hint tuple.
#[derive(Copy, Clone, Debug)]
pub struct PosHint {
    pub x: Option<(HorizontalPositionHint, f64)>,
    pub y: Option<(VerticalPositionHint, f64)>,
}
impl PosHint {
    pub fn new(x: Option<(HorizontalPositionHint, f64)>,
               y: Option<(VerticalPositionHint, f64)>) -> Self {
        PosHint{x, y}
    }
}
impl Default for PosHint {
    fn default() -> Self { PosHint{x: Some((HorizontalPositionHint::Left, 1.0)),
                                    y: Some((VerticalPositionHint::Top, 1.0)) }}
}


/// Convenience wrapper around a border configuration
#[derive(Clone, Debug)]
pub struct BorderConfig {
    
    /// The [Pixel.symbol] to use for the horizontal border if [border] is true
    pub horizontal_symbol: String,
    
    /// The [Pixel.symbol] to use for the vertical border if [border] is true
    pub vertical_symbol: String,
    
    /// The [Pixel.symbol] to use for the top left border if [border] is true
    pub top_left_symbol: String,
    
    /// The [Pixel.symbol] to use for the top right border if [border] is true
    pub top_right_symbol: String,
    
    /// The [Pixel.symbol] to use for the bottom left border if [border] is true
    pub bottom_left_symbol: String,
    
    /// The [Pixel.symbol] to use for the bottom right border if [border] is true
    pub bottom_right_symbol: String,
    
    /// The [Pixel.foreground_color]  to use for the border if [border] is true
    pub fg_color: Color,
    
    /// The [Pixel.background_color] to use for the border if [border] is true
    pub bg_color: Color,
}
impl Default for BorderConfig {
    fn default() -> Self {
       BorderConfig {
           horizontal_symbol: "━".to_string(),
           vertical_symbol: "│".to_string(),
           top_left_symbol: "┌".to_string(),
           top_right_symbol: "┐".to_string(),
           bottom_left_symbol: "└".to_string(),
           bottom_right_symbol: "┘".to_string(),
           fg_color: Color::White,
           bg_color: Color::Black,
       } 
    }
}


#[derive(Clone, Debug)]
pub struct ColorConfig {

    /// The [Pixel.foreground_color] to use for this widgets' content
    pub foreground: Color,

    /// The [Pixel.background_color] to use for this widgets' content
    pub background: Color,

    /// The [Pixel.foreground_color] to use for this widgets' content when selected
    pub selection_foreground: Color,

    /// The [Pixel.background_color] to use for this widgets' content when selected
    pub selection_background: Color,

    /// The [Pixel.foreground_color] to use for this widgets' content when flashed
    pub flash_foreground: Color,

    /// The [Pixel.background_color] to use for this widgets' content when flashed
    pub flash_background: Color,

    /// The [Pixel.foreground_color] to use for filler pixels if [fill] is true
    pub filler_foreground: Color,

    /// The [Pixel.background_color] to use for filler pixels if [fill] is true
    pub filler_background: Color,

    /// The [Pixel.background_color] to use for this widgets' content when a position has been
    /// highlighted by the blinking cursor
    pub cursor: Color,

}
impl Default for ColorConfig {
    fn default() -> Self {
        ColorConfig {
            background: Color::Black,
            foreground: Color::White,
            selection_foreground: Color::Yellow,
            selection_background: Color::Blue,
            flash_foreground: Color::Yellow,
            flash_background: Color::White,
            filler_background: Color::Black,
            filler_foreground: Color::White,
            cursor: Color::DarkYellow,
        }
    }
}


#[derive(Clone, Copy, Default, Debug)]
pub struct Padding {
    pub top: usize,
    pub bottom: usize,
    pub left: usize,
    pub right: usize,
}
impl Padding {
    pub fn new(top: usize, bottom: usize, left: usize, right: usize) -> Padding{
        Padding { top, bottom, left, right }
    }
}
