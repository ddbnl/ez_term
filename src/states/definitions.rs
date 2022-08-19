use std::collections::HashMap;
use crossterm::event::{KeyCode, KeyModifiers};
use crossterm::style::Color;
use crate::property::ez_property::EzProperty;
use crate::scheduler::definitions::{GenericFunction, KeyboardCallbackFunction, MouseCallbackFunction,
                                    MouseDragCallbackFunction, OptionalMouseCallbackFunction};
use crate::scheduler::scheduler::{SchedulerFrontend};
use crate::scheduler::scheduler_funcs::clean_up_property;


/// The mode determining how widgets are placed in a [layout]. Default is box.
#[derive(Clone, PartialEq, Debug)]
pub enum LayoutMode {

    /// Widgets are placed next to each other or under one another depending on orientation.
    /// In horizontal orientation widgets always use the full height of the layout, and in
    /// vertical position they always use the full with.
    Box,

    /// Widgets are stacked in the primary orientation until no more space is left, then along
    /// the secondary orientation. For example with orientation "left-right, top-bottom", widgets
    /// are placed from left to right until there is no more width left, then the next widgets
    /// will be placed below that, also left-to-right.
    Stack,

    /// Widgets are placed evenly in a number of rows and columns. It is recommended to either set
    /// the amount of rows or columns manually, otherwise widgets will be placed in a single row
    /// or column.
    Table,

    /// Widgets are placed according to [PositionHint] or in their hardcoded XY positions.
    Float,

    /// This layout can only contain other layouts and only the root widget may be a Screen layout.
    /// Only the contents of the active screen will be shown. Active screen is controlled through
    /// [LayoutState.active_screen].
    Screen,

    /// This layout can only contain other layouts and presents those as tabs. A tab header will
    /// automatically be added for each child layout, so the user can switch between tabs. The tab
    /// header will display the [id] of the child layout.
    Tab,
}


/// Used with Box, stack and table mode [layout]. Default is horizontal for box,
/// or TopBottomLeftRight for the other modes.
#[derive(Clone, PartialEq, Debug)]
pub enum LayoutOrientation {
    Horizontal,
    Vertical,
    LeftRightTopBottom,
    TopBottomLeftRight,
    RightLeftTopBottom,
    TopBottomRightLeft,
    LeftRightBottomTop,
    BottomTopLeftRight,
    RightLeftBottomTop,
    BottomTopRightLeft,
}


/// Property determining how content is placed horizontally in a layout; default is left.
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum HorizontalAlignment {
    Left,
    Right,
    Center
}


/// Property determining how content is placed vertically in a layout; default is top.
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum VerticalAlignment {
    Top,
    Bottom,
    Middle
}

pub type VerticalPosHint = Option<(VerticalAlignment, f64)>;
pub type HorizontalPosHint = Option<(HorizontalAlignment, f64)>;

/// Composite object containing properties for table mode layout. If you want to bind a callback to
/// any of the properties, access them directly first.
#[derive(PartialEq, Clone, Debug)]
pub struct TableConfig {

    /// Maximum amount of rows. Usually you want to set either the maximum amount of rows or the
    /// maximum amount of columns, and let the other one grow with the amount of content
    pub rows: EzProperty<usize>,

    /// Maximum amount of columns. Usually you want to set either the maximum amount of rows or the
    /// maximum amount of columns, and let the other one grow with the amount of content
    pub cols: EzProperty<usize>,

    /// Default height of rows. If kept at 0, it will be set to the height of the parent divided by
    /// the amount of rows. If force_default_height is false, widgets are allowed to be larger
    /// than default_height, in which case each row will grow to its' largest widget.
    pub row_default_height: EzProperty<usize>,

    /// Default width of columns. If kept at 0, it will be set to the width of the parent divided by
    /// the amount of columns. If force_default_width is false, widgets are allowed to be larger
    /// than default_width, in which case each column will grow to its' largest widget.
    pub col_default_width: EzProperty<usize>,

    /// Each row will be exactly default_height. If default_height is 0, it will be set to the
    /// height of the parent divided by the amount of rows.
    pub force_default_row_height: EzProperty<bool>,

    /// Each column will be exactly default_width. If default_width is 0, it will be set to the
    /// width of the parent divided by the amount of columns.
    pub force_default_col_width: EzProperty<bool>,
}
impl TableConfig {

    pub fn new(name: String, scheduler: &mut SchedulerFrontend) -> Self {

        let rows_property = scheduler.new_usize_property(
            format!("{}/rows", name).as_str(), 0);
        let columns_property = scheduler.new_usize_property(
            format!("{}/cols", name).as_str(), 4);

        let default_height_property = scheduler.new_usize_property(
            format!("{}/row_default_height", name).as_str(), 0);
        let default_width_property = scheduler.new_usize_property(
            format!("{}/col_default_width", name).as_str(), 0);

        let force_default_height_property = scheduler.new_bool_property(
            format!("{}/force_default_row_height", name).as_str(),false);
        let force_default_width_property = scheduler.new_bool_property(
            format!("{}/force_default_col_width", name).as_str(),false);

        TableConfig {
            rows: rows_property,
            cols: columns_property,
            row_default_height: default_height_property,
            col_default_width: default_width_property,
            force_default_row_height: force_default_height_property,
            force_default_col_width: force_default_width_property,
        }
    }

    pub fn set_rows(&mut self, rows: usize) {
        self.rows.set(rows);
    }

    pub fn get_rows(&self) -> usize {
        self.rows.value
    }

    pub fn set_cols(&mut self, columns: usize) {
        self.cols.set(columns);
    }

    pub fn get_cols(&self) -> usize {
        self.cols.value
    }

    pub fn set_row_default_height(&mut self, default: usize) {
        self.row_default_height.set(default);
    }

    pub fn get_row_default_height(&self) -> usize {
        self.row_default_height.value
    }

    pub fn set_col_default_width(&mut self, default: usize) {
        self.col_default_width.set(default);
    }

    pub fn get_col_default_width(&self) -> usize {
        self.col_default_width.value
    }

    pub fn set_force_default_row_height(&mut self, force: bool) {
        self.force_default_row_height.set(force);
    }

    pub fn get_force_default_row_height(&self) -> bool {
        self.force_default_row_height.value
    }

    pub fn set_force_default_col_width(&mut self, force: bool) {
        self.force_default_col_width.set(force);
    }

    pub fn get_force_default_col_width(&self) -> bool {
        self.force_default_col_width.value
    }

    pub fn clean_up_properties(&self, scheduler: &mut SchedulerFrontend) {
        clean_up_property(scheduler, &self.rows.name);
        clean_up_property(scheduler, &self.cols.name);
        clean_up_property(scheduler, &self.force_default_row_height.name);
        clean_up_property(scheduler, &self.force_default_col_width.name);
    }
}


/// Composite object containing size related properties. If you want to bind a callback to height
/// or width, access them directly first.
#[derive(PartialEq, Clone, Debug)]
pub struct StateSize {
    pub width: EzProperty<usize>,
    pub height: EzProperty<usize>,
}
impl StateSize {

    pub fn new(width: usize, height: usize, name: String, scheduler: &mut SchedulerFrontend) -> Self {

        let width_property = scheduler.new_usize_property(
            format!("{}/width", name).as_str(), width);
        let height_property = scheduler.new_usize_property(
            format!("{}/height", name).as_str(), height);
        StateSize {
            width: width_property,
            height: height_property,
        }
    }

    pub fn set_width(&mut self, width: usize) { self.width.set(width); }

    pub fn get_width(&self) -> usize { self.width.value }

    pub fn set_height(&mut self, height: usize) { self.height.set(height); }

    pub fn get_height(&self) -> usize { self.height.value }

    pub fn clean_up_properties(&self, scheduler: &mut SchedulerFrontend) {
        clean_up_property(scheduler, &self.width.name);
        clean_up_property(scheduler, &self.height.name);
    }
}


/// Object containing infinite size properties. These are used for scrolling, indicating that
/// content can theoretically be infinite size on that axis.
#[derive(PartialEq, Clone, Debug, Default)]
pub struct InfiniteSize {
    pub width: bool,
    pub height: bool,
}
impl InfiniteSize {

    pub fn set_width(&mut self, force: bool) {
        self.width = force;
    }

    pub fn get_width(&self) -> bool { self.width }

    pub fn set_height(&mut self, force: bool) {
        self.height = force;
    }

    pub fn get_height(&self) -> bool { self.height }

}


/// Composite object containing both an X and a Y coordinate. If you want to set a callback for
/// position access the 'x' or 'y' property first.
#[derive(PartialEq, Clone, Debug)]
pub struct StateCoordinates {
    pub x: EzProperty<usize>,
    pub y: EzProperty<usize>,
}
impl StateCoordinates {
    pub fn new(x: usize, y: usize, name: String, scheduler: &mut SchedulerFrontend) -> Self {

        let x_property =
            scheduler.new_usize_property(format!("{}/x", name).as_str(), x);
        let y_property =
            scheduler.new_usize_property(format!("{}/y", name).as_str(), y);
        StateCoordinates{
            x: x_property,
            y: y_property
        }
    }

    pub fn set_x(&mut self, x: usize) { self.x.set(x); }

    pub fn get_x(&self) -> usize {
        self.x.value
    }

    pub fn set_y(&mut self, y: usize) {
        self.y.set(y);
    }

    pub fn get_y(&self) -> usize {
        self.y.value
    }

    pub fn clean_up_properties(&self, scheduler: &mut SchedulerFrontend) {
        clean_up_property(scheduler, &self.x.name);
        clean_up_property(scheduler, &self.y.name);
    }
}


/// Composite object containing both width and height autoscaling. If you want to set a callback for
/// either, access the 'width' or 'height' property first.
#[derive(PartialEq, Clone, Debug)]
pub struct AutoScale {
    pub auto_scale_width: EzProperty<bool>,
    pub auto_scale_height: EzProperty<bool>,
}
impl AutoScale {

    pub fn new(width: bool, height: bool, name: String, scheduler: &mut SchedulerFrontend) -> Self {
        let width_property =
            scheduler.new_bool_property(format!("{}/auto_scale_width", name).as_str(),
                                        width);
        let height_property =
            scheduler.new_bool_property(format!("{}/auto_scale_height", name).as_str(),
                                        height);
        AutoScale{ auto_scale_width: width_property, auto_scale_height: height_property}
    }

    pub fn set_auto_scale_width(&mut self, width: bool) {
        self.auto_scale_width.set(width);
    }

    pub fn get_auto_scale_width(&self) -> bool {
        self.auto_scale_width.value
    }

    pub fn set_auto_scale_height(&mut self, height: bool) {
        self.auto_scale_height.set(height);
    }

    pub fn get_auto_scale_height(&self) -> bool {
        self.auto_scale_height.value
    }

    pub fn clean_up_properties(&self, scheduler: &mut SchedulerFrontend) {
        clean_up_property(scheduler, &self.auto_scale_width.name);
        clean_up_property(scheduler, &self.auto_scale_height.name);
    }
}


/// Composite object containing both x and y size hints. If you want to set a callback for
/// either, access the 'x' or 'y' property first.
#[derive(PartialEq, Clone, Debug)]
pub struct SizeHint {
    pub size_hint_x: EzProperty<Option<f64>>,
    pub size_hint_y: EzProperty<Option<f64>>,
}
impl SizeHint {

    pub fn new(x: Option<f64>, y: Option<f64>, name: String, scheduler: &mut SchedulerFrontend)
        -> Self {
        let x_property =
            scheduler.new_size_hint_property(format!("{}/size_hint_x", name).as_str(),
                                        x);
        let y_property =
            scheduler.new_size_hint_property(format!("{}/size_hint_y", name).as_str(),
                                        y);
        SizeHint{ size_hint_x: x_property, size_hint_y: y_property}
    }

    pub fn set_size_hint_x(&mut self, x: Option<f64>) {
        self.size_hint_x.set(x);
    }

    pub fn get_size_hint_x(&self) -> Option<f64> {
        self.size_hint_x.value
    }

    pub fn set_size_hint_y(&mut self, y: Option<f64>) {
        self.size_hint_y.set(y);
    }

    pub fn get_size_hint_y(&self) -> Option<f64> {
        self.size_hint_y.value
    }

    pub fn clean_up_properties(&self, scheduler: &mut SchedulerFrontend) {
        clean_up_property(scheduler, &self.size_hint_x.name);
        clean_up_property(scheduler, &self.size_hint_y.name);
    }
}



/// Composite object containing both x and y pos hints. If you want to set a callback for
/// either, access the 'x' or 'y' property first.
#[derive(PartialEq, Clone, Debug)]
pub struct PosHint {
    pub pos_hint_x: EzProperty<HorizontalPosHint>,
    pub pos_hint_y: EzProperty<VerticalPosHint>,
}
impl PosHint {

    pub fn new(x: HorizontalPosHint, y: VerticalPosHint,
               name: String, scheduler: &mut SchedulerFrontend) -> Self {
        let x_property =
            scheduler.new_horizontal_pos_hint_property(
                format!("{}/pos_hint_x", name).as_str(),x);
        let y_property =
            scheduler.new_vertical_pos_hint_property(
                format!("{}/pos_hint_y", name).as_str(),y);
        PosHint{ pos_hint_x: x_property, pos_hint_y: y_property}
    }

    pub fn set_pos_hint_x(&mut self, x: HorizontalPosHint) {
        self.pos_hint_x.set(x);
    }

    pub fn get_pos_hint_x(&self) -> HorizontalPosHint {
        self.pos_hint_x.value
    }

    pub fn set_pos_hint_y(&mut self, y: VerticalPosHint) {
        self.pos_hint_y.set(y);
    }

    pub fn get_pos_hint_y(&self) -> VerticalPosHint {
        self.pos_hint_y.value
    }

    pub fn clean_up_properties(&self, scheduler: &mut SchedulerFrontend) {
        clean_up_property(scheduler, &self.pos_hint_x.name);
        clean_up_property(scheduler, &self.pos_hint_y.name);
    }
}


/// Convenience wrapper around a callback configuration. Here is an example of how to use this
/// object; we will set an on_press callback:
/// ```
/// use ez_term::*;
/// let (root_widget, mut state_tree, mut scheduler) = load_ui();
///
/// let my_callback = move |context: Context| {
///
///     true
/// };
/// let new_callback_config = CallbackConfig::from_on_press(Box::new(my_callback));
/// scheduler.update_callback_config("my_button", new_callback_config);
/// ```
/// For more information on each callback, see the docs on the properties of this struct.
#[derive(Default)]
pub struct CallbackConfig {

    /// This callback is activated when a widget is selected. A selection can occur when the user uses
    /// the keyboard up/down buttons (and the widget has a selection_order) or when the widget is
    /// hovered. Selectable widgets are: buttons, checkboxes, dropdowns, radio buttons and sliders.
    /// Text inputs are selectable by keyboard, but not by mouse hovering; instead they have to be
    /// clicked to be selected. The second argument in a on_select callback is an Option<Coordinates>.
    /// Is a widget was selected by keyboard, this argument will be None. If it was selected by mouse,
    /// it will contains coordinates.To set this callback with a closure:
    /// ```
    /// use ez_term::*;
    /// let (root_widget, mut state_tree, mut scheduler) = load_ui();
    ///
    /// let my_callback = move |context: Context, mouse_pos: Option<Coordinates>| {
    ///
    ///     true
    /// };
    /// let new_callback_config = CallbackConfig::from_on_select(Box::new(my_callback));
    /// scheduler.update_callback_config("my_button", new_callback_config);
    /// ```
    /// To set this callback with a function:
    /// ```
    /// use ez_term::*;
    /// let (root_widget, mut state_tree, mut scheduler) = load_ui();
    ///
    /// fn my_callback(context: Context, mouse_pos: Option<Coordinates>) -> bool {
    ///
    ///     true
    /// };
    /// let new_callback_config = CallbackConfig::from_on_select(Box::new(my_callback));
    /// scheduler.update_callback_config("my_button", new_callback_config);
    /// ```
    pub on_select: Option<OptionalMouseCallbackFunction> ,

    /// This callback is activated when a widget is deselected. A deselection occurs when the mouse
    /// cursor leaves the selection widget, or when the user uses the keyboard up/down buttons to move
    /// on from the selected widget. To set this callback with a closure:
    /// ```
    /// use ez_term::*;
    /// let (root_widget, mut state_tree, mut scheduler) = load_ui();
    ///
    /// let my_callback = move |context: Context| {
    ///
    ///     true
    /// };
    /// let new_callback_config = CallbackConfig::from_on_deselect(Box::new(my_callback));
    /// scheduler.update_callback_config("my_button", new_callback_config);
    /// ```
    /// To set this callback with a function:
    /// ```
    /// use ez_term::*;
    /// let (root_widget, mut state_tree, mut scheduler) = load_ui();
    ///
    /// fn my_callback(context: Context) -> bool {
    ///
    ///     true
    /// };
    /// let new_callback_config = CallbackConfig::from_on_deselect(Box::new(my_callback));
    /// scheduler.update_callback_config("my_button", new_callback_config);
    /// ```
    pub on_deselect: Option<GenericFunction>,

    /// This callback is activated when a widget is either clicked by the left mouse button, or
    /// keyboard entered when it is selected. In other words, it is a composite callback containing both
    /// on_keyboard_enter and on_left_mouse_click. This can be useful for example with buttons, where
    /// you want something done regardless of whether the user used his mouse or keyboard to press the
    /// button. To set this callback with a closure:
    /// ```
    /// use ez_term::*;
    /// let (root_widget, mut state_tree, mut scheduler) = load_ui();
    ///
    /// let my_callback = move |context: Context| {
    ///
    ///     true
    /// };
    /// let new_callback_config = CallbackConfig::from_on_press(Box::new(my_callback));
    /// scheduler.update_callback_config("my_button", new_callback_config);
    /// ```
    /// To set this callback with a function:
    /// ```
    /// use ez_term::*;
    /// let (root_widget, mut state_tree, mut scheduler) = load_ui();
    ///
    /// fn my_callback(context: Context) -> bool {
    ///
    ///     true
    /// };
    /// let new_callback_config = CallbackConfig::from_on_press(Box::new(my_callback));
    /// scheduler.update_callback_config("my_button", new_callback_config);
    /// ```
    pub on_press: Option<GenericFunction>,

    /// This callback is activated when a widget is selected and the 'enter' key is pressed on the
    /// keyboard.
    /// To set this callback with a closure:
    /// ```
    /// use ez_term::*;
    /// let (root_widget, mut state_tree, mut scheduler) = load_ui();
    ///
    /// let my_callback = move |context: Context| {
    ///
    ///     true
    /// };
    /// let new_callback_config = CallbackConfig::from_on_press(Box::new(my_callback));
    /// scheduler.update_callback_config("my_button", new_callback_config);
    /// ```
    /// To set this callback with a function:
    /// ```
    /// use ez_term::*;
    /// let (root_widget, mut state_tree, mut scheduler) = load_ui();
    ///
    /// fn my_callback(context: Context) -> bool {
    ///
    ///     true
    /// };
    /// let new_callback_config = CallbackConfig::from_on_press(Box::new(my_callback));
    /// scheduler.update_callback_config("my_button", new_callback_config);
    /// ```
    pub on_keyboard_enter: Option<GenericFunction>,

    /// This callback is activated when a widget is clicked by the left mouse button. Keep in mind that
    /// when a widget is clicked, any layouts underneath it are also clicked. The root layout is the
    /// first to receive the mouse click event, followed by sub layouts, and finally the widget. If any
    /// layout has a callback that returns true, the event is consumed and does not reach further
    /// layouts or widgets. To set this callback with a closure:
    /// ```
    /// use ez_term::*;
    /// let (root_widget, mut state_tree, mut scheduler) = load_ui();
    ///
    /// let my_callback = move |context: Context, mouse_pos: Coordinates| {
    ///
    ///     true
    /// };
    /// let new_callback_config = CallbackConfig::from_on_left_mouse_click(Box::new(my_callback));
    /// scheduler.update_callback_config("my_label", new_callback_config);
    /// ```
    /// To set this callback with a function:
    /// ```
    /// use ez_term::*;
    /// let (root_widget, mut state_tree, mut scheduler) = load_ui();
    ///
    /// fn my_callback(context: Context, mouse_pos: Coordinates) -> bool {
    ///
    ///     true
    /// };
    /// let new_callback_config = CallbackConfig::from_on_left_mouse_click(Box::new(my_callback));
    /// scheduler.update_callback_config("my_label", new_callback_config);
    /// ```
    pub on_left_mouse_click: Option<MouseCallbackFunction>,

    /// This callback is activated when a widget is clicked by the right mouse button. Keep in mind that
    /// when a widget is clicked, any layouts underneath it are also clicked. The root layout is the
    /// first to receive the mouse click event, followed by sub layouts, and finally the widget. If any
    /// layout has a callback that returns true, the event is consumed and does not reach further
    /// layouts or widgets. To set this callback with a closure:
    /// ```
    /// use ez_term::*;
    /// let (root_widget, mut state_tree, mut scheduler) = load_ui();
    ///
    /// let my_callback = move |context: Context, mouse_pos: Coordinates| {
    ///
    ///     true
    /// };
    /// let new_callback_config = CallbackConfig::from_on_right_mouse_click(Box::new(my_callback));
    /// scheduler.update_callback_config("my_button", new_callback_config);
    /// ```
    /// To set this callback with a function:
    /// ```
    /// use ez_term::*;
    /// let (root_widget, mut state_tree, mut scheduler) = load_ui();
    ///
    /// fn my_callback(context: Context, mouse_pos: Coordinates) -> bool {
    ///
    ///     true
    /// };
    /// let new_callback_config = CallbackConfig::from_on_right_mouse_click(Box::new(my_callback));
    /// scheduler.update_callback_config("my_button", new_callback_config);
    /// ```
    pub on_right_mouse_click: Option<MouseCallbackFunction>,

    /// This callback is activated when a widget is hovered by the mouse. Keep in mind that
    /// when a widget is hovered, any layouts underneath it are also hovered. The root layout is the
    /// first to receive the hover event, followed by sub layouts, and finally the widget. If any
    /// layout has a callback that returns true, the event is consumed and does not reach further
    /// layouts or widgets. To set this callback with a closure:
    /// ```
    /// use ez_term::*;
    /// let (root_widget, mut state_tree, mut scheduler) = load_ui();
    ///
    /// let my_callback = move |context: Context, mouse_pos: Coordinates| {
    ///
    ///     true
    /// };
    /// let new_callback_config = CallbackConfig::from_on_hover(Box::new(my_callback));
    /// scheduler.update_callback_config("my_label", new_callback_config);
    /// ```
    /// To set this callback with a function:
    /// ```
    /// use ez_term::*;
    /// let (root_widget, mut state_tree, mut scheduler) = load_ui();
    ///
    /// fn my_callback(context: Context, mouse_pos: Coordinates) -> bool {
    ///
    ///     true
    /// };
    /// let new_callback_config = CallbackConfig::from_on_hover(Box::new(my_callback));
    /// scheduler.update_callback_config("my_label", new_callback_config);
    /// ```
    pub on_hover: Option<MouseCallbackFunction>,

    /// This callback is activated when a widget is left mouse clicked and the click is not released.
    /// As long as the click is not released, the widget will receive a new event every time the mouse
    /// cursor changes position, as long as the mouse cursor stays on that widget. The callback receives
    /// two extra arguments: one is the previous drag position, and one is the current drag position.
    /// The previous drag position argument is an Option<Coordinates>; on the very first drag event,
    /// the previous drag position will be None. This is how you know the drag is new. Subsequently,
    /// the previous drag position will contain Coordinates. Because you have both the current and the
    /// previous position, you know which direction the drag is going.
    /// To set this callback with a closure:
    /// ```
    /// use ez_term::*;
    /// let (root_widget, mut state_tree, mut scheduler) = load_ui();
    ///
    /// let my_callback = move |context: Context, previous_mouse_pos: Option<Coordinates>,
    ///                         mouse_pos: Coordinates| {
    ///
    ///     true
    /// };
    /// let new_callback_config = CallbackConfig::from_on_drag(Box::new(my_callback));
    /// scheduler.update_callback_config("my_label", new_callback_config);
    /// ```
    /// To set this callback with a function:
    /// ```
    /// use ez_term::*;
    /// let (root_widget, mut state_tree, mut scheduler) = load_ui();
    ///
    /// fn my_callback(context: Context, previous_mouse_pos: Option<Coordinates>,
    ///                mouse_pos: Coordinates) -> bool {
    ///
    ///     true
    /// };
    /// let new_callback_config = CallbackConfig::from_on_drag(Box::new(my_callback));
    /// scheduler.update_callback_config("my_label", new_callback_config);
    /// ```
    pub on_drag: Option<MouseDragCallbackFunction>,

    /// This callback is activated when a widget is scrolled up by the mouse. Keep in mind that
    /// when a widget is scrolled, any layouts underneath it are also scrolled. The root layout is the
    /// first to receive the scroll event, followed by sub layouts, and finally the widget. If any
    /// layout has a callback that returns true, the event is consumed and does not reach further
    /// layouts or widgets. To set this callback with a closure:
    /// ```
    /// use ez_term::*;
    /// let (root_widget, mut state_tree, mut scheduler) = load_ui();
    ///
    /// let my_callback = move |context: EzContex| {
    ///
    ///     true
    /// };
    /// let new_callback_config = CallbackConfig::from_on_scroll_up(Box::new(my_callback));
    /// scheduler.update_callback_config("my_label", new_callback_config);
    /// ```
    /// To set this callback with a function:
    /// ```
    /// use ez_term::*;
    /// let (root_widget, mut state_tree, mut scheduler) = load_ui();
    ///
    /// fn my_callback(context: Context) -> bool {
    ///
    ///     true
    /// };
    /// let new_callback_config = CallbackConfig::from_on_scroll_up(Box::new(my_callback));
    /// scheduler.update_callback_config("my_label", new_callback_config);
    /// ```
    pub on_scroll_up: Option<GenericFunction>,

    /// This callback is activated when a widget is scrolled down by the mouse. Keep in mind that
    /// when a widget is scrolled, any layouts underneath it are also scrolled. The root layout is the
    /// first to receive the scroll event, followed by sub layouts, and finally the widget. If any
    /// layout has a callback that returns true, the event is consumed and does not reach further
    /// layouts or widgets. To set this callback with a closure:
    /// ```
    /// use ez_term::*;
    /// let (root_widget, mut state_tree, mut scheduler) = load_ui();
    ///
    /// let my_callback = move |context: EzContex| {
    ///
    ///     true
    /// };
    /// let new_callback_config = CallbackConfig::from_on_scroll_down(Box::new(my_callback));
    /// scheduler.update_callback_config("my_label", new_callback_config);
    /// ```
    /// To set this callback with a function:
    /// ```
    /// use ez_term::*;
    /// let (root_widget, mut state_tree, mut scheduler) = load_ui();
    ///
    /// fn my_callback(context: Context) -> bool {
    ///
    ///     true
    /// };
    /// let new_callback_config = CallbackConfig::from_on_scroll_down(Box::new(my_callback));
    /// scheduler.update_callback_config("my_label", new_callback_config);
    /// ```
    pub on_scroll_down: Option<GenericFunction>,

    /// This callback is activated when the value of a widget has changed. Only widgets with values
    /// support this, which are: checkbox, dropdown, radio button, text input and slider. The only
    /// special case is the radio button; when a radio button is activated, all other radio buttons in
    /// that group are deactivated (because they're mutually exclusive). For radio buttons,
    /// on_value_change is only called when a button becomes *active*.
    /// To set this callback with a closure:
    /// ```
    /// use ez_term::*;
    /// let (root_widget, mut state_tree, mut scheduler) = load_ui();
    ///
    /// let my_callback = move |context: Context| {
    ///
    ///     true
    /// };
    /// let new_callback_config = CallbackConfig::from_on_value_change(Box::new(my_callback));
    /// scheduler.update_callback_config("my_checkbox", new_callback_config);
    /// ```
    /// To set this callback with a function:
    /// ```
    /// use ez_term::*;
    /// let (root_widget, mut state_tree, mut scheduler) = load_ui();
    ///
    /// fn my_callback(context: Context) -> bool {
    ///
    ///     true
    /// };
    /// let new_callback_config = CallbackConfig::from_on_value_change(Box::new(my_callback));
    /// scheduler.update_callback_config("my_checkbox", new_callback_config);
    /// ```
    pub on_value_change: Option<GenericFunction>,

    /// Hash containing keyboard key and modifiers as key, and a callback as a value. As an
    /// end-user, use CallbackConfig.bind_key, or scheduler.bind_global_key.
    pub keymap: KeyMap,

    /// A list of callbacks to call when a property changes. It's recommended to call the 'bind'
    /// method on a property directly rather than using this directly.
    /// ```
    /// let state = state_tree.get_by_id_mut("my_widget").as_generic();
    /// state.size.height.bind(callback_func, scheduler);
    /// ```
    pub property_callbacks: Vec<GenericFunction>,
}
impl CallbackConfig {

    /// Create a [CallbackConfig] from a keybinding.
    /// the callback function signature should be: (Context, KeyCode)
    /// See [Context] for more information on the context. The KeyCode is the key that was pressed.
    pub fn bind_key(&mut self, key: KeyCode, modifiers: Option<Vec<KeyModifiers>>, func: KeyboardCallbackFunction) {
        self.keymap.bind_key(key, modifiers, func);
    }

    /// Create a [CallbackConfig] from an on_select callback.
    /// the callback function signature should be: (Context, Option`<Coordinates`>)
    /// See [Context] for more information on the context. The optional coordinates are the mouse
    /// position; if it is none, the selection was not made by mouse.
    pub fn from_on_select(func: OptionalMouseCallbackFunction) -> Self {
        let mut obj = CallbackConfig::default();
        obj.on_select = Some(func);
        obj
    }

    /// Create a [CallbackConfig] from an on_deselect callback.
    /// the callback function signature should be: (Context)
    /// See [Context] for more information on the context.
    pub fn from_on_deselect(func: GenericFunction) -> Self {
        let mut obj = CallbackConfig::default();
        obj.on_deselect = Some(func);
        obj
    }

    /// Create a [CallbackConfig] from an on_press callback.
    /// the callback function signature should be: (Context)
    /// See [Context] for more information on the context.
    pub fn from_on_press(func: GenericFunction) -> Self {
        let mut obj = CallbackConfig::default();
        obj.on_press = Some(func);
        obj
    }

    /// Create a [CallbackConfig] from an on_keyboard_enter callback.
    /// the callback function signature should be: (Context)
    /// See [Context] for more information on the context.
    pub fn from_on_keyboard_enter(func: GenericFunction) -> Self {
        let mut obj = CallbackConfig::default();
        obj.on_keyboard_enter = Some(func);
        obj
    }

    /// Create a [CallbackConfig] from an on_left_mouse_click callback.
    /// the callback function signature should be: (Context, Coordinates)
    /// See [Context] for more information on the context. The coordinates are the position of
    /// the mouse click click relative to the widget that was clicked.
    pub fn from_on_left_mouse_click(func: MouseCallbackFunction) -> Self {
        let mut obj = CallbackConfig::default();
        obj.on_left_mouse_click = Some(func);
        obj
    }

    /// Create a [CallbackConfig] from an on_right_mouse_click callback.
    /// the callback function signature should be: (Context, Coordinates)
    /// See [Context] for more information on the context. The coordinates are the position of
    /// the mouse click click relative to the widget that was clicked.
    pub fn from_on_right_mouse_click(func: MouseCallbackFunction) -> Self {
        let mut obj = CallbackConfig::default();
        obj.on_right_mouse_click = Some(func);
        obj
    }

    /// Create a [CallbackConfig] from an on_scroll_up callback.
    /// the callback function signature should be: (Context)
    /// See [Context] for more information on the context.
    pub fn from_on_scroll_up(func: GenericFunction) -> Self {
        let mut obj = CallbackConfig::default();
        obj.on_scroll_up = Some(func);
        obj
    }

    /// Create a [CallbackConfig] from an on_scroll_down callback.
    /// the callback function signature should be: (Context)
    /// See [Context] for more information on the context.
    pub fn from_on_scroll_down(func: GenericFunction) -> Self {
        let mut obj = CallbackConfig::default();
        obj.on_scroll_down = Some(func);
        obj
    }

    /// Create a [CallbackConfig] from an on_value_change callback.
    /// the callback function signature should be: (Context)
    /// See [Context] for more information on the context.
    pub fn from_on_value_change(func: GenericFunction) -> Self {
        let mut obj = CallbackConfig::default();
        obj.on_value_change = Some(func);
        obj
    }

    /// Create a [CallbackConfig] from an on_hover callback.
    /// the callback function signature should be: (Context, Coordinates)
    /// See [Context] for more information on the context. The coordinates are the position of
    /// the mouse click click relative to the widget that was clicked.
    pub fn from_on_hover(func: MouseCallbackFunction) -> Self {
        let mut obj = CallbackConfig::default();
        obj.on_hover = Some(func);
        obj
    }

    /// Create a [CallbackConfig] from an on_drag callback.
    /// the callback function signature should be: (Context, Option`<Coordinates`>, Coordinates)
    /// See [Context] for more information on the context. The optional coordinates are the
    /// previous drag position; if they are None, the drag just started. The second pair of
    /// coordinates are the current drag position. This allows your func to determine which way the
    /// drag is heading.
    pub fn from_on_drag(func: MouseDragCallbackFunction) -> Self {
        let mut obj = CallbackConfig::default();
        obj.on_drag = Some(func);
        obj
    }

    /// Create a new CallbackConfig based on an existing [KeyMap]. Allows you to fully set a KeyMap
    /// and then derive a CallbackConfig from it.
    pub fn from_keymap(keymap: KeyMap) -> Self {
        let mut obj = CallbackConfig::default();
        obj.keymap = keymap;
        obj
    }

    /// Update this CallbackConfig using another CallbackConfig. Any fields that are not None on
    /// the other object will be copied.
    pub fn update_from(&mut self, other: CallbackConfig)  {
        if let None = other.on_value_change {}
            else { self.on_value_change = other.on_value_change};
        if let None = other.on_deselect {}
            else { self.on_deselect = other.on_deselect};
        if let None = other.on_select {}
            else { self.on_select = other.on_select};
        if let None = other.on_press {}
            else { self.on_press = other.on_press};
        if let None = other.on_left_mouse_click {}
            else { self.on_left_mouse_click = other.on_left_mouse_click};
        if let None = other.on_right_mouse_click {}
            else { self.on_right_mouse_click = other.on_right_mouse_click};
        if let None = other.on_scroll_up {}
            else { self.on_scroll_up = other.on_scroll_up};
        if let None = other.on_scroll_down {}
            else { self.on_scroll_down = other.on_scroll_down};
        if let None = other.on_keyboard_enter {}
            else { self.on_keyboard_enter = other.on_keyboard_enter};
        if let None = other.on_hover {}
            else { self.on_hover = other.on_hover};
        if let None = other.on_drag {}
            else { self.on_drag = other.on_drag};
        self.keymap.keymap.extend(other.keymap.keymap);
    }

}


/// Keymap for binding keys to callbacks. As an end-user, use CallbackConfig.bind_key or
/// scheduler.bind_global_key
#[derive(Default)]
pub struct KeyMap {
    pub keymap: HashMap<(KeyCode, KeyModifiers), KeyboardCallbackFunction>,
}
impl KeyMap {

    pub fn new() -> Self {
        KeyMap { keymap: HashMap::new() }
    }


    pub fn bind_key(&mut self, key: KeyCode, modifiers: Option<Vec<KeyModifiers>>, func: KeyboardCallbackFunction) {
        let modifiers = create_keymap_modifiers(modifiers);
        self.keymap.insert((key, modifiers), func);
    }

    /// Wrap method that makes keycodes uppercase insensitive
    pub fn contains(&self, key: KeyCode, modifiers: KeyModifiers) -> bool {

        if let KeyCode::Char(i) = key {
            if i.is_alphabetic() {
                let other = if i.is_uppercase() {
                    KeyCode::Char(i.to_lowercase().into_iter().next().unwrap())
                } else {
                    KeyCode::Char(i.to_uppercase().into_iter().next().unwrap())
                };
                let mut contains = self.keymap.contains_key(&(key, modifiers));
                if !contains {
                    contains = self.keymap.contains_key(&(other, modifiers));
                }
                return contains
            }
        }
        self.keymap.contains_key(&(key, modifiers))
    }

     pub fn get(&self, key: KeyCode, modifiers: KeyModifiers) -> Option<&KeyboardCallbackFunction> {

         if let KeyCode::Char(i) = key {
             if i.is_alphabetic() {
                 let other = if i.is_uppercase() {
                     KeyCode::Char(i.to_lowercase().into_iter().next().unwrap())
                 } else {
                     KeyCode::Char(i.to_uppercase().into_iter().next().unwrap())
                 };
                 let mut contains = self.keymap.get(&(key, modifiers));
                 if contains.is_none() {
                     contains = self.keymap.get(&(other, modifiers));
                 }
                 return contains
             }
         }
         self.keymap.get(&(key, modifiers))
     }

    pub fn get_mut(&mut self, key: KeyCode, modifiers: KeyModifiers) -> Option<&mut KeyboardCallbackFunction> {

        if let KeyCode::Char(i) = key {
            if i.is_alphabetic() {
                let other = if i.is_uppercase() {
                    KeyCode::Char(i.to_lowercase().into_iter().next().unwrap())
                } else {
                    KeyCode::Char(i.to_uppercase().into_iter().next().unwrap())
                };
                if self.keymap.contains_key(&(key, modifiers)) {
                    return self.keymap.get_mut(&(key, modifiers))
                } else {
                    return self.keymap.get_mut(&(other, modifiers))
                }
            }
        }
        self.keymap.get_mut(&(key, modifiers))
    }
}

pub fn create_keymap_modifiers(modifiers: Option<Vec<KeyModifiers>>) -> KeyModifiers {
    if let Some(i) = modifiers {
        let mut obj = KeyModifiers::NONE;
        if i.contains(&KeyModifiers::CONTROL) {
            obj.set(KeyModifiers::CONTROL, true);
            obj.set(KeyModifiers::NONE, false);
        };
        if i.contains(&KeyModifiers::ALT) {
            obj.set(KeyModifiers::ALT, true);
            obj.set(KeyModifiers::NONE, false);
        };
        if i.contains(&KeyModifiers::SHIFT) {
            obj.set(KeyModifiers::SHIFT, true);
            obj.set(KeyModifiers::NONE, false);
        };
        obj
    } else {
        KeyModifiers::NONE
    }
}


/// Composite object containing all properties related to scrolling. As an end-user, you should
/// only set enable_x, enable_y, view_start_x and/or view_start_y. The other properties are set
/// automatically when constructing the layout.
#[derive(PartialEq, Clone, Debug)]
pub struct ScrollingConfig {

    /// Bool representing whether the x axis should be able to scroll
    pub scroll_x: EzProperty<bool>,

    /// Bool representing whether the y axis should be able to scroll
    pub scroll_y: EzProperty<bool>,

    /// Start of the view on the x axis, content is shown from here until view_start_x + width
    pub view_start_x: EzProperty<f64>,

    /// Start of the view on the y axis, content is shown from here until view_start_y + height
    pub view_start_y: EzProperty<f64>,

    /// Bool representing whether the owning object is actually scrolling, as it is possible for
    /// scrolling to be enabled but not active (i.e. content already fits within object)
    is_scrolling_x: bool,

    /// Bool representing whether the owning object is actually scrolling, as it is possible for
    /// scrolling to be enabled but not active (i.e. content already fits within object)
    is_scrolling_y: bool,

    /// Original height of the content being scrolled
    original_height: usize,

    /// Original width of the content being scrolled
    original_width: usize,
}
impl ScrollingConfig {

    pub fn new(scroll_x: bool, scroll_y: bool, view_start_x: f64, view_start_y: f64,
               name: String, scheduler: &mut SchedulerFrontend) -> Self {

        let x_property =
            scheduler.new_bool_property(format!("{}/scroll_x", name).as_str(),
                                        scroll_x);
        let y_property =
            scheduler.new_bool_property(format!("{}/scroll_y", name).as_str(),
                                        scroll_y);
        let view_start_x_property =
            scheduler.new_f64_property(format!("{}/view_start_x", name).as_str(),
                                        view_start_x);
        let view_start_y_property =
            scheduler.new_f64_property(format!("{}/view_start_y", name).as_str(),
                                        view_start_y);
        ScrollingConfig {
            scroll_x: x_property,
            scroll_y: y_property,
            view_start_x: view_start_x_property,
            view_start_y: view_start_y_property,
            is_scrolling_x: false,
            is_scrolling_y: false,
            original_height: 0,
            original_width: 0
        }
    }

    pub fn set_scroll_x(&mut self, x: bool) {
        self.scroll_x.set(x);
    }

    pub fn get_scroll_x(&self) -> bool {
        self.scroll_x.value
    }

    pub fn set_scroll_y(&mut self, y: bool) {
        self.scroll_y.set(y);
    }

    pub fn get_scroll_y(&self) -> bool {
        self.scroll_y.value
    }

    pub fn set_view_start_x(&mut self, view_start: f64) {
        self.view_start_x.set(view_start);
    }

    pub fn get_view_start_x(&self) -> f64 {
        self.view_start_x.value
    }

    pub fn set_view_start_y(&mut self, view_start: f64) {
        self.view_start_y.set(view_start);
    }

    pub fn get_view_start_y(&self) -> f64 {
        self.view_start_y.value
    }

    pub fn set_original_height(&mut self, height: usize) {
        self.original_height = height;
    }

    pub fn get_original_height(&self) -> usize {
        self.original_height
    }

    pub fn set_original_width(&mut self, width: usize) {
        self.original_width = width;
    }

    pub fn get_original_width(&self) -> usize {
        self.original_width
    }

    pub fn set_is_scrolling_x(&mut self, x: bool) {
        self.is_scrolling_x = x;
    }

    pub fn get_is_scrolling_x(&self) -> bool {
        self.is_scrolling_x
    }

    pub fn set_is_scrolling_y(&mut self, y: bool) {
        self.is_scrolling_y = y;
    }

    pub fn get_is_scrolling_y(&self) -> bool {
        self.is_scrolling_y
    }

    pub fn get_max_view_start_x(&self, effective_widget_width: usize) -> usize {
        self.get_original_width() - effective_widget_width
    }

    pub fn get_absolute_view_start_x(&self, effective_widget_width: usize) -> usize {
        (self.get_max_view_start_x(effective_widget_width) as f64 *
            self.get_view_start_x()).round() as usize
    }

    pub fn get_max_view_start_y(&self, effective_widget_height: usize) -> usize {
        self.get_original_height() - effective_widget_height
    }

    pub fn get_absolute_view_start_y(&self, effective_widget_height: usize) -> usize {
        (self.get_max_view_start_y(effective_widget_height) as f64 *
            self.get_view_start_y()).round() as usize
    }

    pub fn clean_up_properties(&self, scheduler: &mut SchedulerFrontend) {
        clean_up_property(scheduler, &self.scroll_x.name);
        clean_up_property(scheduler, &self.scroll_y.name);
    }
}


/// Composite object containing properties related to layout/widget borders. If you want to bind a
/// callback to one of the properties, access them directly first.
#[derive(PartialEq, Clone, Debug)]
pub struct BorderConfig {

    /// Bool representing whether an object should have a border
    pub border: EzProperty<bool>,

    /// The [Pixel.symbol] to use for the horizontal border if [border] is true
    pub horizontal_symbol: EzProperty<String>,
    
    /// The [Pixel.symbol] to use for the vertical border if [border] is true
    pub vertical_symbol: EzProperty<String>,
    
    /// The [Pixel.symbol] to use for the top left border if [border] is true
    pub top_left_symbol: EzProperty<String>,
    
    /// The [Pixel.symbol] to use for the top right border if [border] is true
    pub top_right_symbol: EzProperty<String>,
    
    /// The [Pixel.symbol] to use for the bottom left border if [border] is true
    pub bottom_left_symbol: EzProperty<String>,
    
    /// The [Pixel.symbol] to use for the bottom right border if [border] is true
    pub bottom_right_symbol: EzProperty<String>,
}

impl BorderConfig {

    pub fn new(enable: bool, name: String, scheduler: &mut SchedulerFrontend) -> Self {

        let enabled_property =
            scheduler.new_bool_property(format!("{}/border", name).as_str(),
                                        enable);
        let horizontal_symbol =
            scheduler.new_string_property(format!("{}/horizontal_symbol", name).as_str(),
                                          "─".to_string());
        let vertical_symbol =
            scheduler.new_string_property(format!("{}/vertical_symbol", name).as_str(),
                                          "│".to_string());
        let top_left_symbol =
            scheduler.new_string_property(format!("{}/top_left_symbol", name).as_str(),
                                          "┌".to_string());
        let top_right_symbol =
            scheduler.new_string_property(format!("{}/top_right_symbol", name).as_str(),
                                          "┐".to_string());
        let bottom_left_symbol =
            scheduler.new_string_property(format!("{}/bottom_left_symbol", name).as_str(),
                                          "└".to_string());
        let bottom_right_symbol =
            scheduler.new_string_property(format!("{}/bottom_right_symbol", name).as_str(),
                                          "┘".to_string());


       BorderConfig {
           border: enabled_property,
           horizontal_symbol,
           vertical_symbol,
           top_left_symbol,
           top_right_symbol,
           bottom_left_symbol,
           bottom_right_symbol,
       }
    }

    pub fn set_border(&mut self, enabled: bool) {
        self.border.set(enabled);
    }

    pub fn get_border(&self) -> bool {
        self.border.value
    }

    pub fn set_horizontal_symbol(&mut self, symbol: &str) {
        self.horizontal_symbol.set(symbol.to_string());
    }

    pub fn get_horizontal_symbol(&self) -> String {
        self.horizontal_symbol.value.clone()
    }

    pub fn set_vertical_symbol(&mut self, symbol: &str) {
        self.vertical_symbol.set(symbol.to_string());
    }

    pub fn get_vertical_symbol(&self) -> String {
        self.vertical_symbol.value.clone()
    }

    pub fn set_top_left_symbol(&mut self, symbol: &str) {
        self.top_left_symbol.set(symbol.to_string());
    }

    pub fn get_top_left_symbol(&self) -> String {
        self.top_left_symbol.value.clone()
    }

    pub fn set_top_right_symbol(&mut self, symbol: &str) {
        self.top_right_symbol.set(symbol.to_string());
    }

    pub fn get_top_right_symbol(&self) -> String {
        self.top_right_symbol.value.clone()
    }

    pub fn set_bottom_left_symbol(&mut self, symbol: &str) {
        self.bottom_left_symbol.set(symbol.to_string());
    }

    pub fn get_bottom_left_symbol(&self) -> String {
        self.bottom_left_symbol.value.clone()
    }

    pub fn set_bottom_right_symbol(&mut self, symbol: &str) {
        self.bottom_right_symbol.set(symbol.to_string());
    }

    pub fn get_bottom_right_symbol(&self) -> String {
        self.bottom_right_symbol.value.clone()
    }

    pub fn clean_up_properties(&self, scheduler: &mut SchedulerFrontend) {
        clean_up_property(scheduler, &self.border.name);
        clean_up_property(scheduler, &self.horizontal_symbol.name);
        clean_up_property(scheduler, &self.vertical_symbol.name);
        clean_up_property(scheduler, &self.top_left_symbol.name);
        clean_up_property(scheduler, &self.top_right_symbol.name);
        clean_up_property(scheduler, &self.bottom_left_symbol.name);
        clean_up_property(scheduler, &self.bottom_right_symbol.name);
    }
}


/// Composite object containing properties related to the colors of a widget or layout. If you want
/// to bind a callback to one of the properties, access it directly first.
#[derive(PartialEq, Clone, Debug)]
pub struct ColorConfig {

    /// The [Pixel.foreground_color] to use for this widgets' content
    pub fg_color: EzProperty<Color>,

    /// The [Pixel.background_color] to use for this widgets' content
    pub bg_color: EzProperty<Color>,

    /// The [Pixel.foreground_color] to use for this widgets' content when selected
    pub selection_fg_color: EzProperty<Color>,

    /// The [Pixel.background_color] to use for this widgets' content when selected
    pub selection_bg_color: EzProperty<Color>,

    /// The [Pixel.foreground_color] to use for this widgets' content is disabled
    pub disabled_fg_color: EzProperty<Color>,

    /// The [Pixel.background_color] to use for this widgets' content is disabled
    pub disabled_bg_color: EzProperty<Color>,

    /// The [Pixel.foreground_color] to use for this widgets' content is active
    pub active_fg_color: EzProperty<Color>,

    /// The [Pixel.background_color] to use for this widgets' content is active
    pub active_bg_color: EzProperty<Color>,

    /// The [Pixel.foreground_color] to use for this widgets' content when flashed
    pub flash_fg_color: EzProperty<Color>,

    /// The [Pixel.background_color] to use for this widgets' content when flashed
    pub flash_bg_color: EzProperty<Color>,

    /// The [Pixel.foreground_color] to use for tab headers
    pub tab_fg_color: EzProperty<Color>,

    /// The [Pixel.background_color] to use for tab headers
    pub tab_bg_color: EzProperty<Color>,

    /// The [Pixel.foreground_color] to use for tab header border pixels
    pub tab_border_fg_color: EzProperty<Color>,

    /// The [Pixel.background_color] to use for tab header border pixels
    pub tab_border_bg_color: EzProperty<Color>,

    /// The [Pixel.foreground_color] to use for filler pixels if [fill] is true
    pub filler_fg_color: EzProperty<Color>,

    /// The [Pixel.background_color] to use for filler pixels if [fill] is true
    pub filler_bg_color: EzProperty<Color>,

    /// The [Pixel.foreground_color] to use for border pixels
    pub border_fg_color: EzProperty<Color>,

    /// The [Pixel.background_color] to use for border pixels
    pub border_bg_color: EzProperty<Color>,

    /// The [Pixel.background_color] to use for this widgets' content when a position has been
    /// highlighted by the blinking cursor
    pub cursor_color: EzProperty<Color>,
}
impl ColorConfig {
    pub fn new(name: String, scheduler: &mut SchedulerFrontend) -> Self {

        let foreground = scheduler.new_color_property(
            format!("{}/fg_color", name).as_str(), Color::White);
        let background = scheduler.new_color_property(
            format!("{}/bg_color", name).as_str(), Color::Black);

        let selection_foreground = scheduler.new_color_property(
            format!("{}/selection_fg_color", name).as_str(), Color::Yellow);
        let selection_background = scheduler.new_color_property(
            format!("{}/selection_bg_color", name).as_str(), Color::Blue);

        let disabled_foreground = scheduler.new_color_property(
            format!("{}/disabled_fg_color", name).as_str(), Color::White);
        let disabled_background = scheduler.new_color_property(
            format!("{}/disabled_bg_color", name).as_str(), Color::Black);

        let active_foreground = scheduler.new_color_property(
            format!("{}/active_fg_color", name).as_str(), Color::White);
        let active_background = scheduler.new_color_property(
            format!("{}/active_bg_color", name).as_str(), Color::Black);

        let flash_foreground = scheduler.new_color_property(
            format!("{}/flash_fg_color", name).as_str(), Color::White);
        let flash_background = scheduler.new_color_property(
            format!("{}/flash_bg_color", name).as_str(), Color::White);

        let filler_foreground = scheduler.new_color_property(
            format!("{}/filler_fg_color", name).as_str(), Color::White);
        let filler_background = scheduler.new_color_property(
            format!("{}/filler_bg_color", name).as_str(), Color::Black);

        let tab_foreground = scheduler.new_color_property(
            format!("{}/tab_fg_color", name).as_str(), Color::White);
        let tab_background = scheduler.new_color_property(
            format!("{}/tab_bg_color", name).as_str(), Color::Black);

        let tab_border_foreground = scheduler.new_color_property(
            format!("{}/tab_border_fg_color", name).as_str(), Color::White);
        let tab_border_background = scheduler.new_color_property(
            format!("{}/tab_border_bg_color", name).as_str(), Color::Black);

        let border_foreground = scheduler.new_color_property(
            format!("{}/border_fg_color", name).as_str(), Color::White);
        let border_background = scheduler.new_color_property(
            format!("{}/border_bg_color", name).as_str(), Color::Black);

        let cursor = scheduler.new_color_property(
            format!("{}/cursor_color", name).as_str(), Color::DarkYellow);

        ColorConfig {
            fg_color: foreground,
            bg_color: background,
            selection_fg_color: selection_foreground,
            selection_bg_color: selection_background,
            disabled_fg_color: disabled_foreground,
            disabled_bg_color: disabled_background,
            active_fg_color: active_foreground,
            active_bg_color: active_background,
            flash_fg_color: flash_foreground,
            flash_bg_color: flash_background,
            tab_fg_color: tab_foreground,
            tab_bg_color: tab_background,
            tab_border_fg_color: tab_border_foreground,
            tab_border_bg_color: tab_border_background,
            filler_fg_color: filler_foreground,
            filler_bg_color: filler_background,
            border_fg_color: border_foreground,
            border_bg_color: border_background,
            cursor_color: cursor,
        }
    }

    pub fn set_fg_color(&mut self, color: Color) {
        self.fg_color.set(color);
    }

    pub fn get_fg_color(&self) -> Color {
        self.fg_color.value
    }

    pub fn set_bg_color(&mut self, color: Color) {
        self.bg_color.set(color);
    }

    pub fn get_bg_color(&self) -> Color {
        self.bg_color.value
    }

    pub fn set_selection_fg_color(&mut self, color: Color) {
        self.selection_fg_color.set(color);
    }

    pub fn get_selection_fg_color(&self) -> Color {
        self.selection_fg_color.value
    }

    pub fn set_selection_bg_color(&mut self, color: Color) {
        self.selection_bg_color.set(color);
    }

    pub fn get_selection_bg_color(&self) -> Color {
        self.selection_bg_color.value
    }

    pub fn set_disabled_fg_color(&mut self, color: Color) {
        self.disabled_fg_color.set(color);
    }

    pub fn get_disabled_fg_color(&self) -> Color {
        self.disabled_fg_color.value
    }

    pub fn set_disabled_bg_color(&mut self, color: Color) {
        self.disabled_bg_color.set(color);
    }

    pub fn get_disabled_bg_color(&self) -> Color {
        self.disabled_bg_color.value
    }

    pub fn set_active_fg_color(&mut self, color: Color) {
        self.active_fg_color.set(color);
    }

    pub fn get_active_fg_color(&self) -> Color {
        self.active_fg_color.value
    }

    pub fn set_active_bg_color(&mut self, color: Color) {
        self.active_bg_color.set(color);
    }

    pub fn get_active_bg_color(&self) -> Color {
        self.active_bg_color.value
    }

    pub fn set_flash_fg_color(&mut self, color: Color) {
        self.flash_fg_color.set(color);
    }

    pub fn get_flash_fg_color(&self) -> Color {
        self.flash_fg_color.value
    }

    pub fn set_flash_bg_color(&mut self, color: Color) {
        self.flash_bg_color.set(color);
    }

    pub fn get_flash_bg_color(&self) -> Color {
        self.flash_bg_color.value
    }

    pub fn set_tab_fg_color(&mut self, color: Color) {
        self.tab_fg_color.set(color);
    }

    pub fn get_tab_fg_color(&self) -> Color {
        self.tab_fg_color.value
    }

    pub fn set_tab_bg_color(&mut self, color: Color) {
        self.tab_bg_color.set(color);
    }

    pub fn get_tab_bg_color(&self) -> Color {
        self.tab_bg_color.value
    }

    pub fn set_tab_border_fg_color(&mut self, color: Color) {
        self.tab_border_fg_color.set(color);
    }

    pub fn get_tab_border_fg_color(&self) -> Color {
        self.tab_border_fg_color.value
    }

    pub fn set_tab_border_bg_color(&mut self, color: Color) {
        self.tab_border_bg_color.set(color);
    }

    pub fn get_tab_border_bg_color(&self) -> Color {
        self.tab_border_bg_color.value
    }

    pub fn set_filler_fg_color(&mut self, color: Color) {
        self.filler_fg_color.set(color);
    }

    pub fn get_filler_fg_color(&self) -> Color {
        self.filler_fg_color.value
    }

    pub fn set_filler_bg_color(&mut self, color: Color) {
        self.filler_bg_color.set(color);
    }

    pub fn get_filler_bg_color(&self) -> Color {
        self.filler_bg_color.value
    }

    pub fn set_border_fg_color(&mut self, color: Color) {
        self.border_fg_color.set(color);
    }

    pub fn get_border_fg_color(&self) -> Color {
        self.border_fg_color.value
    }

    pub fn set_border_bg_color(&mut self, color: Color) {
        self.border_bg_color.set(color);
    }

    pub fn get_border_bg_color(&self) -> Color {
        self.border_bg_color.value
    }

    pub fn get_cursor_color(&self) -> Color {
        self.cursor_color.value
    }

    pub fn set_cursor_color(&mut self, color: Color) {
        self.cursor_color.set(color);
    }

    pub fn clean_up_properties(&self, scheduler: &mut SchedulerFrontend) {
        clean_up_property(scheduler, &self.fg_color.name);
        clean_up_property(scheduler, &self.bg_color.name);
        clean_up_property(scheduler, &self.selection_fg_color.name);
        clean_up_property(scheduler, &self.selection_bg_color.name);
        clean_up_property(scheduler, &self.disabled_fg_color.name);
        clean_up_property(scheduler, &self.disabled_bg_color.name);
        clean_up_property(scheduler, &self.active_fg_color.name);
        clean_up_property(scheduler, &self.active_bg_color.name);
        clean_up_property(scheduler, &self.flash_fg_color.name);
        clean_up_property(scheduler, &self.flash_bg_color.name);
        clean_up_property(scheduler, &self.tab_fg_color.name);
        clean_up_property(scheduler, &self.tab_bg_color.name);
        clean_up_property(scheduler, &self.filler_fg_color.name);
        clean_up_property(scheduler, &self.filler_bg_color.name);
        clean_up_property(scheduler, &self.border_fg_color.name);
        clean_up_property(scheduler, &self.border_bg_color.name);
        clean_up_property(scheduler, &self.cursor_color.name);
    }
}


/// Composite object containing properties related to padding. If you want to set a callback to a
/// property, access is directly first.
#[derive(PartialEq, Clone, Debug)]
pub struct Padding {
    pub padding_top: EzProperty<usize>,
    pub padding_bottom: EzProperty<usize>,
    pub padding_left: EzProperty<usize>,
    pub padding_right: EzProperty<usize>,
}
impl Padding {
    pub fn new(top: usize, bottom: usize, left: usize, right: usize, name: String,
               scheduler: &mut SchedulerFrontend) -> Padding{


        let top_property = scheduler.new_usize_property(
            format!("{}/padding_top", name).as_str(), top);
        let bottom_property = scheduler.new_usize_property(
            format!("{}/padding_bottom", name).as_str(), bottom);
        let left_property = scheduler.new_usize_property(
            format!("{}/padding_left", name).as_str(), left);
        let right_property = scheduler.new_usize_property(
            format!("{}/padding_right", name).as_str(), right);
        Padding {
            padding_top: top_property,
            padding_bottom: bottom_property,
            padding_left: left_property,
            padding_right: right_property,
        }
    }

    pub fn set_padding_top(&mut self, padding: usize) {
        self.padding_top.set(padding);
    }

    pub fn get_padding_top(&self) -> usize {
        self.padding_top.value
    }

    pub fn set_padding_bottom(&mut self, padding: usize) {
        self.padding_bottom.set(padding);
    }

    pub fn get_padding_bottom(&self) -> usize {
        self.padding_bottom.value
    }

    pub fn set_padding_left(&mut self, padding: usize) {
        self.padding_left.set(padding);
    }

    pub fn get_padding_left(&self) -> usize {
        self.padding_left.value
    }

    pub fn set_padding_right(&mut self, padding: usize) {
        self.padding_right.set(padding);
    }

    pub fn get_padding_right(&self) -> usize {
        self.padding_right.value
    }

    pub fn clean_up_properties(&self, scheduler: &mut SchedulerFrontend) {
        clean_up_property(scheduler, &self.padding_top.name);
        clean_up_property(scheduler, &self.padding_bottom.name);
        clean_up_property(scheduler, &self.padding_left.name);
        clean_up_property(scheduler, &self.padding_right.name);
    }
}
