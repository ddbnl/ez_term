//! # Widget state:
//! A module containing the base structs and traits for widget states.
use crossterm::style::Color;
use crate::common::definitions::Coordinates;
use crate::scheduler::Scheduler;
use crate::states::canvas_state::{CanvasState};
use crate::states::label_state::{LabelState};
use crate::states::button_state::{ButtonState};
use crate::states::checkbox_state::{CheckboxState};
use crate::states::definitions::{AutoScale, BorderConfig, ColorConfig,
                                 HorizontalAlignment, HorizontalPositionHint, Padding, PosHint,
                                 Size, SizeHint, StateCoordinates, VerticalAlignment,
                                 VerticalPositionHint};
use crate::states::dropdown_state::{DropdownState, DroppedDownMenuState};
use crate::states::layout_state::LayoutState;
use crate::states::progress_bar_state::ProgressBarState;
use crate::states::radio_button_state::{RadioButtonState};
use crate::states::slider_state::SliderState;
use crate::states::text_input_state::{TextInputState};


/// Widget states are used to keep track of dynamic run time information of widgets, such as the
/// text of a label, or whether a checkbox is currently checked. All callbacks receive a mutable
/// ref to a StateTree which contains al widget states, so they can change widgets without a
/// mutable ref to the widget itself. Every frame the StateTree is compared to each widget to see
/// which widget has changed so it can be redrawn. The specific state struct for each widget type
/// is defined its' own module.
#[derive(Clone, Debug)]
pub enum EzState {
    Layout(LayoutState),
    Label(LabelState),
    Button(ButtonState),
    Canvas(CanvasState),
    Checkbox(CheckboxState),
    Dropdown(DropdownState),
    DroppedDownMenu(DroppedDownMenuState),
    RadioButton(RadioButtonState),
    TextInput(TextInputState),
    Slider(SliderState),
    ProgressBar(ProgressBarState)
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
            EzState::DroppedDownMenu(i) => i,
            EzState::RadioButton(i) => i,
            EzState::TextInput(i) => i,
            EzState::Canvas(i) => i,
            EzState::Slider(i) => i,
            EzState::ProgressBar(i) => i,
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
            EzState::DroppedDownMenu(i) => i,
            EzState::RadioButton(i) => i,
            EzState::TextInput(i) => i,
            EzState::Canvas(i) => i,
            EzState::Slider(i) => i,
            EzState::ProgressBar(i) => i,
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
        if let EzState::Canvas(i) = self { i }
        else { panic!("wrong state.") }
    }

    /// Cast this state as a mutable Canvas widget state ref, you must be sure you have one.
    pub fn as_canvas_mut(&mut self) -> &mut CanvasState {
        if let EzState::Canvas(i) = self { i }
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

    /// Cast this state as a Button widget state ref, you must be sure you have one.
    pub fn as_button(&self) -> &ButtonState {
        if let EzState::Button(i) = self { i }
        else { panic!("wrong state.") }
    }

    /// Cast this state as a mutable Button widget state ref, you must be sure you have one.
    pub fn as_button_mut(&mut self) -> &mut ButtonState {
        if let EzState::Button(i) = self { i }
        else { panic!("wrong state.") }
    }

    /// Cast this state as a Slider widget state ref, you must be sure you have one.
    pub fn as_slider(&self) -> &SliderState {
        if let EzState::Slider(i) = self { i }
        else { panic!("wrong state.") }
    }

    /// Cast this state as a mutable Slider widget state ref, you must be sure you have one.
    pub fn as_slider_mut(&mut self) -> &mut SliderState {
        if let EzState::Slider(i) = self { i }
        else { panic!("wrong state.") }
    }

    /// Cast this state as a Progress bar widget state ref, you must be sure you have one.
    pub fn as_progress_bar(&self) -> &ProgressBarState {
        if let EzState::ProgressBar(i) = self { i }
        else { panic!("wrong state.") }
    }

    /// Cast this state as a mutable Progress bar widget state ref, you must be sure you have one.
    pub fn as_progress_bar_mut(&mut self) -> &mut ProgressBarState {
        if let EzState::ProgressBar(i) = self { i }
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

    /// Cast this state as a dropped down menu modal state ref, you must be sure you have one.
    pub fn as_dropped_down_menu(&self) -> &DroppedDownMenuState {
        if let EzState::DroppedDownMenu(i) = self { i }
        else { panic!("wrong state.") }
    }

    /// Cast this state as a mutable dropped down menu modal state ref, you must be sure you have one.
    pub fn as_dropped_down_menu_mut(&mut self) -> &mut DroppedDownMenuState {
        if let EzState::DroppedDownMenu(i) = self { i }
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

    fn get_path(&self) -> &String;

    /// Set to None for passing an absolute width, or to a value between 0 and 1 to
    /// automatically scale width based on parent width
    fn set_size_hint(&mut self, _size_hint: SizeHint) { }

    /// Set to None for passing an absolute width, or to a value between 0 and 1 to
    /// automatically scale width based on parent width
    fn set_size_hint_x(&mut self, size_hint: Option<f64>) {
        self.set_size_hint(SizeHint::new(size_hint, self.get_size_hint().y));
    }

    /// Set to None for passing an absolute height, or to a value between 0 and 1 to
    /// automatically scale width based on parent height
    fn set_size_hint_y(&mut self, size_hint: Option<f64>) {

        self.set_size_hint(SizeHint::new(self.get_size_hint().x, size_hint));
    }

    /// If not None automatically scaled width based on parent width
    fn get_size_hint(&self) -> &SizeHint;

    /// Set to None to allow hardcoded or default positions. Set a pos hint to position relative
    /// to parent. A pos hint is a string and a float e.g:
    /// ("center_x", 0.9)
    /// When positioning the widget, "center_x" will be replaced with the middle x coordinate of
    /// parent Layout, and multiplied with the float.
    fn set_pos_hint(&mut self, _pos_hint: PosHint) { }

    /// Set to None to allow hardcoded or default positions. Set a pos hint to position relative
    /// to parent. A pos hint is a string and a float e.g:
    /// ("center_x", 0.9)
    /// When positioning the widget, "center_x" will be replaced with the middle x coordinate of
    /// parent Layout, and multiplied with the float.
    fn set_pos_hint_x(&mut self, pos_hint: Option<(HorizontalPositionHint, f64)>) {
        self.set_pos_hint(PosHint::new(pos_hint, self.get_pos_hint().y));
    }

    /// Set to None to allow hardcoded or default positions. Set a pos hint to position relative
    /// to parent. A pos hint is a string and a float e.g:
    /// ("top", 0.9)
    /// When positioning the widget, "top" will be replaced with the y coordinate of the top of the
    /// parent Layout, and multiplied by the float.
    fn set_pos_hint_y(&mut self, pos_hint: Option<(VerticalPositionHint, f64)>) {
        self.set_pos_hint(PosHint::new(self.get_pos_hint().x, pos_hint));
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

    /// Get mutable current [Size] of this object
    fn get_size_mut(&mut self) -> &mut Size;

    /// Get the effective amount of width and height within this object, taking off e.g. borders,
    /// padding, etc.
    fn get_effective_size(&self) -> Size {

        let mut size_copy = self.get_size().clone();
        let width_result: isize = size_copy.width as isize
            -if self.get_border_config().enabled {2} else {0}
            -self.get_padding().left as isize - self.get_padding().right as isize;
        let width = if width_result < 0 {0} else { width_result };
        let height_result: isize = size_copy.height as isize
            -if self.get_border_config().enabled {2} else {0}
            -self.get_padding().top as isize - self.get_padding().bottom as isize;
        let height = if height_result < 0 {0} else { height_result };
        size_copy.width = width as usize;
        size_copy.height = height as usize;
        size_copy
    }

    /// Set the how much width you want the actual content inside this widget to have. Width for
    /// e.g. border and padding will be added to this automatically.
    fn set_effective_width(&mut self, width: usize) {

        self.get_size_mut().width = width
            +if self.get_border_config().enabled {2} else {0}
            +self.get_padding().left + self.get_padding().right;
    }

    /// Set the how much height you want the actual content inside this widget to have. Height for
    /// e.g. border and padding will be added to this automatically.
    fn set_effective_height(&mut self, height: usize) {
        self.get_size_mut().height = height
            +if self.get_border_config().enabled {2} else {0}
            +self.get_padding().top + self.get_padding().bottom;
    }

    /// Get position relative to parent
    fn get_position(&self) -> &StateCoordinates;

    /// Get mut position relative to parent
    fn get_position_mut(&mut self) -> &mut StateCoordinates;

    /// Set x coordinate of [Position]
    fn set_x(&mut self, x: usize) {
        if x != self.get_position().x.get() {
            self.get_position_mut().x.set(x);
        }
    }

    /// Set y coordinate of [Position]
    fn set_y(&mut self, y: usize) {
        if y != self.get_position().y.get() {
            self.get_position_mut().y.set(y);
        }
    }

    /// Get position where the actual content of this widget starts relative to parent, taking out
    /// e.g. borders, padding, etc.
    fn get_effective_position(&self) -> Coordinates {
        Coordinates::new(
            self.get_position().x.get() +if self.get_border_config().enabled {1}
            else {0} + self.get_padding().left,
            self.get_position().y.get() +if self.get_border_config().enabled {1}
            else {0} + self.get_padding().top)
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
         self.get_absolute_position().x +if self.get_border_config().enabled {1} else {0}
             + self.get_padding().left,
         self.get_absolute_position().y +if self.get_border_config().enabled {1} else {0}
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
        self.set_padding(Padding::new(padding, current.bottom, current.left, current.right));
    }

    /// Set height of bottom padding
    fn set_padding_bottom(&mut self, padding: usize) {
        let current = self.get_padding().clone();
        self.set_padding(Padding::new(current.top, padding, current.left, current.right));
    }

    /// Set width of left padding
    fn set_padding_left(&mut self, padding: usize) {
        let current = self.get_padding().clone();
        self.set_padding(Padding::new(current.top, current.bottom, padding, current.right));
    }

    /// Set width of right padding
    fn set_padding_right(&mut self, padding: usize) {
        let current = self.get_padding().clone();
        self.set_padding(Padding::new(current.top, current.bottom, current.left, padding));
    }

    /// Pas a [BorderConfig] abject that will be used to draw the border if enabled
    fn set_border_config(&mut self, config: BorderConfig);

    /// Get the [state::BorderConfig] abject that will be used to draw the border if enabled
    fn get_border_config(&self) -> &BorderConfig;

    fn get_border_config_mut(&mut self) -> &mut BorderConfig;

    /// Set the [ColorConfig] abject that will be used to draw this widget
    fn set_color_config(&mut self, config: ColorConfig);

    /// Get a ref to the [ColorConfig] abject that will be used to draw this widget
    fn get_color_config(&self) -> &ColorConfig;

    /// Get a mut ref to the [ColorConfig] abject that will be used to draw this widget
    fn get_colors_config_mut(&mut self) -> &mut ColorConfig;

    /// Convenience function. Get a Foreground and Background color depending on the state of
    /// the widget (e.g. disabled, selected, etc.).
    fn get_context_colors(&self) -> (Color, Color) {

        let fg_color =
            if self.get_disabled() {self.get_color_config().disabled_foreground }
            else if self.get_selected() {self.get_color_config().selection_foreground }
            else {self.get_color_config().foreground };

        let bg_color =
            if self.get_disabled() {self.get_color_config().disabled_background }
            else if self.get_selected() {self.get_color_config().selection_background }
            else {self.get_color_config().background };

        (fg_color, bg_color)
    }

    /// Get the top left and bottom right corners of a widget in (X, Y) coordinate tuples.
    fn get_box(&self) -> (Coordinates, Coordinates) {
        let top_left = self.get_absolute_position();
        let bottom_right = Coordinates::new(
            top_left.x + self.get_size().width, top_left.y + self.get_size().height);
        (top_left, bottom_right)
    }

    /// Get all the coordinates of a box in list.
    fn get_box_coords(&self) -> Vec<Coordinates> {
        let mut coords = Vec::new();
        let (top_left, bottom_right) = self.get_box();
        for x in top_left.x..bottom_right.x + 1 {
            for y in top_left.y..bottom_right.y + 1 {
                coords.push(Coordinates::new(x, y));
            }
        }
        coords
    }

    /// Returns a bool representing whether two widgets overlap at any point.
    fn overlaps(&self, other_box: (Coordinates, Coordinates)) -> bool {
        let (l1, r1) = self.get_box();
        let (l2, r2) = other_box;
        // If one rectangle is on the left of the other there's no overlap
        if l1.x >= r2.x || l2.x >= r1.x { return false }
        // If one rectangle is above the other there's no overlap
        if l2.y >= r1.y || l1.y >= r2.y { return false }
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

    /// Returns a bool representing whether this widget can be select by keyboard or mouse. E.g.
    /// labels cannot be selected, but checkboxes can.
    fn is_selectable(&self) -> bool { false }

    /// Set the disabled field of a widget. Only implemented by interactive widgets (i.e. widgets
    /// that are selectable).
    fn set_disabled(&mut self, disabled: bool) { }

    /// Get the disabled field of a widget. Only implemented by interactive widgets (i.e. widgets
    /// that are selectable).
    fn get_disabled(&self) -> bool { false }

    /// Get the order in which this widget should be selected, represented by a usize number. E.g.
    /// if there is a '1' widget, a '2' widget, and this widget is '3', calling 'select_next_widget'
    /// will select 1, then 2, then this widget. Used for keyboard up and down keys.
    fn get_selection_order(&self) -> usize { 0 }

    /// Set the order in which this widget should be selected, represented by a usize number. E.g.
    /// if there is a '1' widget, a '2' widget, and this widget is '3', calling 'select_next_widget'
    /// will select 1, then 2, then this widget. Used for keyboard up and down keys.
    fn set_selection_order(&mut self, _order: usize) {}

    /// Set to true to force redraw the entire screen. The screen is still diffed before redrawing
    /// so this can be called efficiently. Nevertheless you want to call [set_changed] to redraw
    /// only the specific widget in most cases.
    fn set_force_redraw(&mut self, scheduler: &mut Scheduler) {
        scheduler.force_redraw();
    }

    fn set_selected(&mut self, _state: bool) {}

    fn get_selected(&self) -> bool { false }

    fn update(&self, scheduler: &mut Scheduler)  {
        scheduler.update_widget(self.get_path().to_string())
    }
}