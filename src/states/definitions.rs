use std::collections::HashMap;
use crossterm::event::KeyCode;
use crossterm::style::Color;
use crate::property::ez_property::EzProperty;
use crate::scheduler::definitions::{GenericEzFunction, KeyboardCallbackFunction, MouseCallbackFunction, MouseDragCallbackFunction, OptionalMouseCallbackFunction};
use crate::scheduler::scheduler::{SchedulerFrontend};
use crate::scheduler::scheduler_funcs::clean_up_property;


/// Different modes determining how widgets are placed in a [layout].
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


/// Used with Box mode [layout], determines whether widgets are placed below or above each other.
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


#[derive(PartialEq, Clone, Copy, Debug)]
pub enum HorizontalAlignment {
    Left,
    Right,
    Center
}


#[derive(PartialEq, Clone, Copy, Debug)]
pub enum VerticalAlignment {
    Top,
    Bottom,
    Middle
}

/// Convenience wrapper around settings for Layout Table mode.
#[derive(PartialEq, Clone, Debug)]
pub struct TableConfig {

    /// Maximum amount of rows. Usually you want to set either the maximum amount of rows or the
    /// maximum amount of columns, and let the other one grow with the amount of content
    rows: EzProperty<usize>,

    /// Maximum amount of columns. Usually you want to set either the maximum amount of rows or the
    /// maximum amount of columns, and let the other one grow with the amount of content
    columns: EzProperty<usize>,

    /// Default height of rows. If kept at 0, it will be set to the height of the parent divided by
    /// the amount of rows. If force_default_height is false, widgets are allowed to be larger
    /// than default_height, in which case each row will grow to its' largest widget.
    default_height: EzProperty<usize>,

    /// Default width of columns. If kept at 0, it will be set to the width of the parent divided by
    /// the amount of columns. If force_default_width is false, widgets are allowed to be larger
    /// than default_width, in which case each column will grow to its' largest widget.
    default_width: EzProperty<usize>,

    /// Each row will be exactly default_height. If default_height is 0, it will be set to the
    /// height of the parent divided by the amount of rows.
    force_default_height: EzProperty<bool>,

    /// Each column will be exactly default_width. If default_width is 0, it will be set to the
    /// width of the parent divided by the amount of columns.
    force_default_width: EzProperty<bool>,
}
impl TableConfig {

    pub fn new(name: String, scheduler: &mut SchedulerFrontend) -> Self {

        let rows_property = scheduler.new_usize_property(
            format!("{}/table_rows", name).as_str(), 0);
        let columns_property = scheduler.new_usize_property(
            format!("{}/table_columns", name).as_str(), 4);

        let default_height_property = scheduler.new_usize_property(
            format!("{}/table_default_height", name).as_str(), 0);
        let default_width_property = scheduler.new_usize_property(
            format!("{}/table_default_width", name).as_str(), 0);

        let force_default_height_property = scheduler.new_bool_property(
            format!("{}/table_default_row_height", name).as_str(),
            false);
        let force_default_width_property = scheduler.new_bool_property(
            format!("{}/table_default_column_width", name).as_str(),
            false);

        TableConfig {
            rows: rows_property,
            columns: columns_property,
            default_height: default_height_property,
            default_width: default_width_property,
            force_default_height: force_default_height_property,
            force_default_width: force_default_width_property,
        }
    }

    pub fn set_rows(&mut self, rows: usize) {
        self.rows.set(rows);
    }

    pub fn get_rows(&self) -> usize {
        self.rows.value
    }

    pub fn set_columns(&mut self, columns: usize) {
        self.columns.set(columns);
    }

    pub fn get_columns(&self) -> usize {
        self.columns.value
    }

    pub fn set_default_height(&mut self, default: usize) {
        self.default_height.set(default);
    }

    pub fn get_default_height(&self) -> usize {
        self.default_height.value
    }

    pub fn set_default_width(&mut self, default: usize) {
        self.default_width.set(default);
    }

    pub fn get_default_width(&self) -> usize {
        self.default_width.value
    }

    pub fn set_force_default_height(&mut self, force: bool) {
        self.force_default_height.set(force);
    }

    pub fn get_force_default_height(&self) -> bool {
        self.force_default_height.value
    }

    pub fn set_force_default_width(&mut self, force: bool) {
        self.force_default_width.set(force);
    }

    pub fn get_force_default_width(&self) -> bool {
        self.force_default_width.value
    }

    pub fn clean_up_properties(&self, scheduler: &mut SchedulerFrontend) {
        clean_up_property(scheduler, &self.rows.name);
        clean_up_property(scheduler, &self.columns.name);
        clean_up_property(scheduler, &self.force_default_height.name);
        clean_up_property(scheduler, &self.force_default_width.name);
    }
}


/// Convenience wrapper around a size tuple.
#[derive(PartialEq, Clone, Debug)]
pub struct StateSize {
    width: EzProperty<usize>,
    height: EzProperty<usize>,
    infinite_width: bool,
    infinite_height: bool,
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
            infinite_width: false,
            infinite_height: false
        }
    }

    pub fn set_width(&mut self, width: usize) { self.width.set(width); }

    pub fn get_width(&self) -> usize { self.width.value }

    pub fn set_height(&mut self, height: usize) { self.height.set(height); }

    pub fn get_height(&self) -> usize { self.height.value }

    pub fn set_infinite_width(&mut self, force: bool) {
        self.infinite_width = force;
    }

    pub fn get_infinite_width(&self) -> bool { self.infinite_width }

    pub fn set_infinite_height(&mut self, force: bool) {
        self.infinite_height = force;
    }

    pub fn get_infinite_height(&self) -> bool { self.infinite_height }

    pub fn clean_up_properties(&self, scheduler: &mut SchedulerFrontend) {
        clean_up_property(scheduler, &self.width.name);
        clean_up_property(scheduler, &self.height.name);
    }
}


/// Convenience wrapper around an XY tuple.
#[derive(PartialEq, Clone, Debug)]
pub struct StateCoordinates {
    x: EzProperty<usize>,
    y: EzProperty<usize>,
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


/// Convenience wrapper around an size_hint tuple.
#[derive(PartialEq, Clone, Debug)]
pub struct AutoScale {
    width: EzProperty<bool>,
    height: EzProperty<bool>,
}
impl AutoScale {

    pub fn new(width: bool, height: bool, name: String, scheduler: &mut SchedulerFrontend) -> Self {
        let width_property =
            scheduler.new_bool_property(format!("{}/autoscale_width", name).as_str(),
                                        width);
        let height_property =
            scheduler.new_bool_property(format!("{}/autoscale_height", name).as_str(),
                                        height);
        AutoScale{width: width_property, height: height_property}
    }

    pub fn set_width(&mut self, width: bool) {
        self.width.set(width);
    }

    pub fn get_width(&self) -> bool {
        self.width.value
    }

    pub fn set_height(&mut self, height: bool) {
        self.height.set(height);
    }

    pub fn get_height(&self) -> bool {
        self.height.value
    }
    pub fn clean_up_properties(&self, scheduler: &mut SchedulerFrontend) {
        clean_up_property(scheduler, &self.width.name);
        clean_up_property(scheduler, &self.height.name);
    }
}


/// Convenience wrapper around an size_hint tuple.
#[derive(PartialEq, Clone, Debug)]
pub struct SizeHint {
    x: EzProperty<Option<f64>>,
    y: EzProperty<Option<f64>>,
}
impl SizeHint {

    pub fn new(x: Option<f64>, y: Option<f64>, name: String, scheduler: &mut SchedulerFrontend)
        -> Self {
        let x_property =
            scheduler.new_size_hint_property(format!("{}/size_hint_width", name).as_str(),
                                        x);
        let y_property =
            scheduler.new_size_hint_property(format!("{}/size_hint_height", name).as_str(),
                                        y);
        SizeHint{x: x_property, y: y_property}
    }

    pub fn set_x(&mut self, x: Option<f64>) {
        self.x.set(x);
    }

    pub fn get_x(&self) -> Option<f64> {
        self.x.value
    }

    pub fn set_y(&mut self, y: Option<f64>) {
        self.y.set(y);
    }

    pub fn get_y(&self) -> Option<f64> {
        self.y.value
    }

    pub fn clean_up_properties(&self, scheduler: &mut SchedulerFrontend) {
        clean_up_property(scheduler, &self.x.name);
        clean_up_property(scheduler, &self.y.name);
    }
}


/// Convenience wrapper around an pos_hint tuple.
#[derive(PartialEq, Clone, Debug)]
pub struct PosHint {
    x: EzProperty<Option<(HorizontalAlignment, f64)>>,
    y: EzProperty<Option<(VerticalAlignment, f64)>>,
}
impl PosHint {

    pub fn new(x: Option<(HorizontalAlignment, f64)>, y: Option<(VerticalAlignment, f64)>,
               name: String, scheduler: &mut SchedulerFrontend) -> Self {
        let x_property =
            scheduler.new_horizontal_pos_hint_property(
                format!("{}/pos_hint_x", name).as_str(),x);
        let y_property =
            scheduler.new_vertical_pos_hint_property(
                format!("{}/pos_hint_y", name).as_str(),y);
        PosHint{x: x_property, y: y_property}
    }

    pub fn set_x(&mut self, x: Option<(HorizontalAlignment, f64)>) {
        self.x.set(x);
    }

    pub fn get_x(&self) -> Option<(HorizontalAlignment, f64)> {
        self.x.value
    }

    pub fn set_y(&mut self, y: Option<(VerticalAlignment, f64)>) {
        self.y.set(y);
    }

    pub fn get_y(&self) -> Option<(VerticalAlignment, f64)> {
        self.y.value
    }

    pub fn clean_up_properties(&self, scheduler: &mut SchedulerFrontend) {
        clean_up_property(scheduler, &self.x.name);
        clean_up_property(scheduler, &self.y.name);
    }
}


// Convenience wrapper around a callback configuration
#[derive(Default)]
pub struct CallbackConfig {

    /// Function to call when an object is selected.
    pub on_select: Option<OptionalMouseCallbackFunction> ,

    /// Function to call when an object is deselected.
    pub on_deselect: Option<GenericEzFunction>,

    /// Function to call when an object is keyboard entered or left clicked,
    pub on_press: Option<GenericEzFunction>,

    /// Function to call when this widget is right clicked
    pub on_keyboard_enter: Option<GenericEzFunction>,

    /// Function to call when this widget is right clicked
    pub on_left_mouse_click: Option<MouseCallbackFunction>,

    /// Function to call when this widget is right clicked
    pub on_right_mouse_click: Option<MouseCallbackFunction>,

    /// Function to call when this widget is mouse hovered
    pub on_hover: Option<MouseCallbackFunction>,

    /// Function to call when this widget is left mouse dragged
    pub on_drag: Option<MouseDragCallbackFunction>,

    /// Function to call when this widget is scrolled up
    pub on_scroll_up: Option<GenericEzFunction>,

    /// Function to call when this widget is scrolled down
    pub on_scroll_down: Option<GenericEzFunction>,

    /// Function to call when the value of an object changes
    pub on_value_change: Option<GenericEzFunction>,

    /// A Key to callback function lookup used to store keybinds for this widget. See
    /// [KeyboardCallbackFunction] type for callback function signature.
    pub keymap: KeyMap,
}
impl CallbackConfig {

    /// Create a [CallbackConfig] from a keybinding.
    /// the callback function signature should be: (EzContext, KeyCode)
    /// See [EzContext] for more information on the context. The KeyCode is the key that was pressed.
    pub fn bind_key(&mut self, key: KeyCode, func: KeyboardCallbackFunction) {
        self.keymap.insert(key, func);
    }

    /// Create a [CallbackConfig] from an on_select callback.
    /// the callback function signature should be: (EzContext, Option`<Coordinates`>)
    /// See [EzContext] for more information on the context. The optional coordinates are the mouse
    /// position; if it is none, the selection was not made by mouse.
    pub fn from_on_select(func: OptionalMouseCallbackFunction) -> Self {
        let mut obj = CallbackConfig::default();
        obj.on_select = Some(func);
        obj
    }

    /// Create a [CallbackConfig] from an on_deselect callback.
    /// the callback function signature should be: (EzContext)
    /// See [EzContext] for more information on the context.
    pub fn from_on_deselect(func: GenericEzFunction) -> Self {
        let mut obj = CallbackConfig::default();
        obj.on_deselect = Some(func);
        obj
    }

    /// Create a [CallbackConfig] from an on_press callback.
    /// the callback function signature should be: (EzContext)
    /// See [EzContext] for more information on the context.
    pub fn from_on_press(func: GenericEzFunction) -> Self {
        let mut obj = CallbackConfig::default();
        obj.on_press = Some(func);
        obj
    }

    /// Create a [CallbackConfig] from an on_keyboard_enter callback.
    /// the callback function signature should be: (EzContext)
    /// See [EzContext] for more information on the context.
    pub fn from_on_keyboard_enter(func: GenericEzFunction) -> Self {
        let mut obj = CallbackConfig::default();
        obj.on_keyboard_enter = Some(func);
        obj
    }

    /// Create a [CallbackConfig] from an on_left_mouse_click callback.
    /// the callback function signature should be: (EzContext, Coordinates)
    /// See [EzContext] for more information on the context. The coordinates are the position of
    /// the mouse click click relative to the widget that was clicked.
    pub fn from_on_left_mouse_click(func: MouseCallbackFunction) -> Self {
        let mut obj = CallbackConfig::default();
        obj.on_left_mouse_click = Some(func);
        obj
    }

    /// Create a [CallbackConfig] from an on_right_mouse_click callback.
    /// the callback function signature should be: (EzContext, Coordinates)
    /// See [EzContext] for more information on the context. The coordinates are the position of
    /// the mouse click click relative to the widget that was clicked.
    pub fn from_on_right_mouse_click(func: MouseCallbackFunction) -> Self {
        let mut obj = CallbackConfig::default();
        obj.on_right_mouse_click = Some(func);
        obj
    }

    /// Create a [CallbackConfig] from an on_scroll_up callback.
    /// the callback function signature should be: (EzContext)
    /// See [EzContext] for more information on the context.
    pub fn from_on_scroll_up(func: GenericEzFunction) -> Self {
        let mut obj = CallbackConfig::default();
        obj.on_scroll_up = Some(func);
        obj
    }

    /// Create a [CallbackConfig] from an on_scroll_down callback.
    /// the callback function signature should be: (EzContext)
    /// See [EzContext] for more information on the context.
    pub fn from_on_scroll_down(func: GenericEzFunction) -> Self {
        let mut obj = CallbackConfig::default();
        obj.on_scroll_down = Some(func);
        obj
    }

    /// Create a [CallbackConfig] from an on_value_change callback.
    /// the callback function signature should be: (EzContext)
    /// See [EzContext] for more information on the context.
    pub fn from_on_value_change(func: GenericEzFunction) -> Self {
        let mut obj = CallbackConfig::default();
        obj.on_value_change = Some(func);
        obj
    }

    /// Create a [CallbackConfig] from an on_hover callback.
    /// the callback function signature should be: (EzContext, Coordinates)
    /// See [EzContext] for more information on the context. The coordinates are the position of
    /// the mouse click click relative to the widget that was clicked.
    pub fn from_on_hover(func: MouseCallbackFunction) -> Self {
        let mut obj = CallbackConfig::default();
        obj.on_hover = Some(func);
        obj
    }

    /// Create a [CallbackConfig] from an on_drag callback.
    /// the callback function signature should be: (EzContext, Option`<Coordinates`>, Coordinates)
    /// See [EzContext] for more information on the context. The optional coordinates are the
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
        self.keymap.extend(other.keymap);
    }

}


/// ## Key map
/// A crossterm KeyCode > Callback function lookup. Used for custom user keybinds
pub type KeyMap = HashMap<KeyCode, KeyboardCallbackFunction>;


/// Convenience wrapper around a [LayoutState] scrolling configuration
#[derive(PartialEq, Clone, Debug)]
pub struct ScrollingConfig {

    /// Bool representing whether the x axis should be able to scroll
    enable_x: EzProperty<bool>,

    /// Bool representing whether the y axis should be able to scroll
    enable_y: EzProperty<bool>,

    /// Start of the view on the x axis, content is shown from here until view_start_x + width
    view_start_x: usize,

    /// Start of the view on the y axis, content is shown from here until view_start_y + height
    view_start_y: usize,

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

    pub fn new(enable_x: bool, enable_y: bool, name: String, scheduler: &mut SchedulerFrontend) -> Self {

        let x_property =
            scheduler.new_bool_property(format!("{}/scrolling_enable_x", name).as_str(),
                                        enable_x);
        let y_property =
            scheduler.new_bool_property(format!("{}/scrolling_enable_y", name).as_str(),
                                        enable_y);
        ScrollingConfig {
            enable_x: x_property,
            enable_y: y_property,
            view_start_x: 0,
            view_start_y: 0,
            is_scrolling_x: false,
            is_scrolling_y: false,
            original_height: 0,
            original_width: 0
        }
    }

    pub fn set_enable_x(&mut self, x: bool) {
        self.enable_x.set(x);
    }

    pub fn get_enable_x(&self) -> bool {
        self.enable_x.value
    }

    pub fn set_enable_y(&mut self, y: bool) {
        self.enable_y.set(y);
    }

    pub fn get_enable_y(&self) -> bool {
        self.enable_y.value
    }

    pub fn set_view_start_x(&mut self, view_start: usize) {
        self.view_start_x = view_start;
    }

    pub fn get_view_start_x(&self) -> usize {
        self.view_start_x
    }

    pub fn set_view_start_y(&mut self, view_start: usize) {
        self.view_start_y = view_start;
    }

    pub fn get_view_start_y(&self) -> usize {
        self.view_start_y
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

    pub fn clean_up_properties(&self, scheduler: &mut SchedulerFrontend) {
        clean_up_property(scheduler, &self.enable_x.name);
        clean_up_property(scheduler, &self.enable_y.name);
    }
}


/// Convenience wrapper around a border configuration
#[derive(PartialEq, Clone, Debug)]
pub struct BorderConfig {

    /// Bool representing whether an object should have a border
    enabled: EzProperty<bool>,

    /// The [Pixel.symbol] to use for the horizontal border if [border] is true
    horizontal_symbol: EzProperty<String>,
    
    /// The [Pixel.symbol] to use for the vertical border if [border] is true
    vertical_symbol: EzProperty<String>,
    
    /// The [Pixel.symbol] to use for the top left border if [border] is true
    top_left_symbol: EzProperty<String>,
    
    /// The [Pixel.symbol] to use for the top right border if [border] is true
    top_right_symbol: EzProperty<String>,
    
    /// The [Pixel.symbol] to use for the bottom left border if [border] is true
    bottom_left_symbol: EzProperty<String>,
    
    /// The [Pixel.symbol] to use for the bottom right border if [border] is true
    bottom_right_symbol: EzProperty<String>,
}

impl BorderConfig {

    pub fn new(enable: bool, name: String, scheduler: &mut SchedulerFrontend) -> Self {

        let enabled_property =
            scheduler.new_bool_property(format!("{}/border_enabled", name).as_str(),
                                        enable);
        let horizontal_symbol =
            scheduler.new_string_property(format!("{}/border_horizontal", name).as_str(),
                                          "─".to_string());
        let vertical_symbol =
            scheduler.new_string_property(format!("{}/border_vertical", name).as_str(),
                                          "│".to_string());
        let top_left_symbol =
            scheduler.new_string_property(format!("{}/border_top_left", name).as_str(),
                                          "┌".to_string());
        let top_right_symbol =
            scheduler.new_string_property(format!("{}/border_top_right", name).as_str(),
                                          "┐".to_string());
        let bottom_left_symbol =
            scheduler.new_string_property(format!("{}/border_bottom_left", name).as_str(),
                                          "└".to_string());
        let bottom_right_symbol =
            scheduler.new_string_property(format!("{}/border_bottom_right", name).as_str(),
                                          "┘".to_string());


       BorderConfig {
           enabled: enabled_property,
           horizontal_symbol,
           vertical_symbol,
           top_left_symbol,
           top_right_symbol,
           bottom_left_symbol,
           bottom_right_symbol,
       }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled.set(enabled);
    }

    pub fn get_enabled(&self) -> bool {
        self.enabled.value
    }

    pub fn set_horizontal_symbol(&mut self, symbol: String) {
        self.horizontal_symbol.set(symbol);
    }

    pub fn get_horizontal_symbol(&self) -> String {
        self.horizontal_symbol.value.clone()
    }

    pub fn set_vertical_symbol(&mut self, symbol: String) {
        self.vertical_symbol.set(symbol);
    }

    pub fn get_vertical_symbol(&self) -> String {
        self.vertical_symbol.value.clone()
    }

    pub fn set_top_left_symbol(&mut self, symbol: String) {
        self.top_left_symbol.set(symbol);
    }

    pub fn get_top_left_symbol(&self) -> String {
        self.top_left_symbol.value.clone()
    }

    pub fn set_top_right_symbol(&mut self, symbol: String) {
        self.top_right_symbol.set(symbol);
    }

    pub fn get_top_right_symbol(&self) -> String {
        self.top_right_symbol.value.clone()
    }

    pub fn set_bottom_left_symbol(&mut self, symbol: String) {
        self.bottom_left_symbol.set(symbol);
    }

    pub fn get_bottom_left_symbol(&self) -> String {
        self.bottom_left_symbol.value.clone()
    }

    pub fn set_bottom_right_symbol(&mut self, symbol: String) {
        self.bottom_right_symbol.set(symbol);
    }

    pub fn get_bottom_right_symbol(&self) -> String {
        self.bottom_right_symbol.value.clone()
    }

    pub fn clean_up_properties(&self, scheduler: &mut SchedulerFrontend) {
        clean_up_property(scheduler, &self.enabled.name);
        clean_up_property(scheduler, &self.horizontal_symbol.name);
        clean_up_property(scheduler, &self.vertical_symbol.name);
        clean_up_property(scheduler, &self.top_left_symbol.name);
        clean_up_property(scheduler, &self.top_right_symbol.name);
        clean_up_property(scheduler, &self.bottom_left_symbol.name);
        clean_up_property(scheduler, &self.bottom_right_symbol.name);
    }
}


#[derive(PartialEq, Clone, Debug)]
pub struct ColorConfig {

    /// The [Pixel.foreground_color] to use for this widgets' content
    foreground: EzProperty<Color>,

    /// The [Pixel.background_color] to use for this widgets' content
    background: EzProperty<Color>,

    /// The [Pixel.foreground_color] to use for this widgets' content when selected
    selection_foreground: EzProperty<Color>,

    /// The [Pixel.background_color] to use for this widgets' content when selected
    selection_background: EzProperty<Color>,

    /// The [Pixel.foreground_color] to use for this widgets' content is disabled
    disabled_foreground: EzProperty<Color>,

    /// The [Pixel.background_color] to use for this widgets' content is disabled
    disabled_background: EzProperty<Color>,

    /// The [Pixel.foreground_color] to use for this widgets' content is active
    active_foreground: EzProperty<Color>,

    /// The [Pixel.background_color] to use for this widgets' content is active
    active_background: EzProperty<Color>,

    /// The [Pixel.foreground_color] to use for this widgets' content when flashed
    flash_foreground: EzProperty<Color>,

    /// The [Pixel.background_color] to use for this widgets' content when flashed
    flash_background: EzProperty<Color>,

    /// The [Pixel.foreground_color] to use for tab headers
    tab_foreground: EzProperty<Color>,

    /// The [Pixel.background_color] to use for tab headers
    tab_background: EzProperty<Color>,

    /// The [Pixel.foreground_color] to use for filler pixels if [fill] is true
    filler_foreground: EzProperty<Color>,

    /// The [Pixel.background_color] to use for filler pixels if [fill] is true
    filler_background: EzProperty<Color>,

    /// The [Pixel.foreground_color] to use for border pixels
    border_foreground: EzProperty<Color>,

    /// The [Pixel.background_color] to use for border pixels
    border_background: EzProperty<Color>,

    /// The [Pixel.background_color] to use for this widgets' content when a position has been
    /// highlighted by the blinking cursor
    cursor: EzProperty<Color>,
}
impl ColorConfig {
    pub fn new(name: String, scheduler: &mut SchedulerFrontend) -> Self {

        let foreground = scheduler.new_color_property(
            format!("{}/color_fg", name).as_str(), Color::White);
        let background = scheduler.new_color_property(
            format!("{}/color_bg", name).as_str(), Color::Black);

        let selection_foreground = scheduler.new_color_property(
            format!("{}/color_selection_fg", name).as_str(), Color::Yellow);
        let selection_background = scheduler.new_color_property(
            format!("{}/color_selection_bg", name).as_str(), Color::Blue);

        let disabled_foreground = scheduler.new_color_property(
            format!("{}/color_disabled_fg", name).as_str(), Color::White);
        let disabled_background = scheduler.new_color_property(
            format!("{}/color_disabled_bg", name).as_str(), Color::Black);

        let active_foreground = scheduler.new_color_property(
            format!("{}/color_active_fg", name).as_str(), Color::Red);
        let active_background = scheduler.new_color_property(
            format!("{}/color_active_bg", name).as_str(), Color::Black);

        let flash_foreground = scheduler.new_color_property(
            format!("{}/color_flash_fg", name).as_str(), Color::Yellow);
        let flash_background = scheduler.new_color_property(
            format!("{}/color_flash_bg", name).as_str(), Color::White);

        let filler_foreground = scheduler.new_color_property(
            format!("{}/color_filler_fg", name).as_str(), Color::White);
        let filler_background = scheduler.new_color_property(
            format!("{}/color_filler_bg", name).as_str(), Color::Black);

        let tab_foreground = scheduler.new_color_property(
            format!("{}/color_tab_fg", name).as_str(), Color::White);
        let tab_background = scheduler.new_color_property(
            format!("{}/color_tab_bg", name).as_str(), Color::Black);

        let border_foreground = scheduler.new_color_property(
            format!("{}/color_border_fg", name).as_str(), Color::White);
        let border_background = scheduler.new_color_property(
            format!("{}/color_border_bg", name).as_str(), Color::Black);

        let cursor = scheduler.new_color_property(
            format!("{}/color_cursor", name).as_str(), Color::DarkYellow);

        ColorConfig {
            foreground,
            background,
            selection_foreground,
            selection_background,
            disabled_foreground,
            disabled_background,
            active_foreground,
            active_background,
            flash_foreground,
            flash_background,
            tab_foreground,
            tab_background,
            filler_foreground,
            filler_background,
            border_foreground,
            border_background,
            cursor,
        }
    }

    pub fn set_foreground(&mut self, color: Color) {
        self.foreground.set(color);
    }

    pub fn get_foreground(&self) -> Color {
        self.foreground.value
    }

    pub fn set_background(&mut self, color: Color) {
        self.background.set(color);
    }

    pub fn get_background(&self) -> Color {
        self.background.value
    }

    pub fn set_selection_foreground(&mut self, color: Color) {
        self.selection_foreground.set(color);
    }

    pub fn get_selection_foreground(&self) -> Color {
        self.selection_foreground.value
    }

    pub fn set_selection_background(&mut self, color: Color) {
        self.selection_background.set(color);
    }

    pub fn get_selection_background(&self) -> Color {
        self.selection_background.value
    }

    pub fn set_disabled_foreground(&mut self, color: Color) {
        self.disabled_foreground.set(color);
    }

    pub fn get_disabled_foreground(&self) -> Color {
        self.disabled_foreground.value
    }

    pub fn set_disabled_background(&mut self, color: Color) {
        self.disabled_background.set(color);
    }

    pub fn get_disabled_background(&self) -> Color {
        self.disabled_background.value
    }

    pub fn set_active_foreground(&mut self, color: Color) {
        self.active_foreground.set(color);
    }

    pub fn get_active_foreground(&self) -> Color {
        self.active_foreground.value
    }

    pub fn set_active_background(&mut self, color: Color) {
        self.active_background.set(color);
    }

    pub fn get_active_background(&self) -> Color {
        self.active_background.value
    }

    pub fn set_flash_foreground(&mut self, color: Color) {
        self.flash_foreground.set(color);
    }

    pub fn get_flash_foreground(&self) -> Color {
        self.flash_foreground.value
    }

    pub fn set_flash_background(&mut self, color: Color) {
        self.flash_background.set(color);
    }

    pub fn get_flash_background(&self) -> Color {
        self.flash_background.value
    }

    pub fn set_tab_foreground(&mut self, color: Color) {
        self.tab_foreground.set(color);
    }

    pub fn get_tab_foreground(&self) -> Color {
        self.tab_foreground.value
    }

    pub fn set_tab_background(&mut self, color: Color) {
        self.tab_background.set(color);
    }

    pub fn get_tab_background(&self) -> Color {
        self.tab_background.value
    }

    pub fn set_filler_foreground(&mut self, color: Color) {
        self.filler_foreground.set(color);
    }

    pub fn get_filler_foreground(&self) -> Color {
        self.filler_foreground.value
    }

    pub fn set_filler_background(&mut self, color: Color) {
        self.filler_background.set(color);
    }

    pub fn get_filler_background(&self) -> Color {
        self.filler_background.value
    }

    pub fn set_border_foreground(&mut self, color: Color) {
        self.border_foreground.set(color);
    }

    pub fn get_border_foreground(&self) -> Color {
        self.border_foreground.value
    }

    pub fn set_border_background(&mut self, color: Color) {
        self.border_background.set(color);
    }

    pub fn get_border_background(&self) -> Color {
        self.border_background.value
    }

    pub fn get_cursor(&self) -> Color {
        self.cursor.value
    }

    pub fn set_cursor(&mut self, color: Color) {
        self.cursor.set(color);
    }

    pub fn clean_up_properties(&self, scheduler: &mut SchedulerFrontend) {
        clean_up_property(scheduler, &self.foreground.name);
        clean_up_property(scheduler, &self.background.name);
        clean_up_property(scheduler, &self.selection_foreground.name);
        clean_up_property(scheduler, &self.selection_background.name);
        clean_up_property(scheduler, &self.disabled_foreground.name);
        clean_up_property(scheduler, &self.disabled_background.name);
        clean_up_property(scheduler, &self.active_foreground.name);
        clean_up_property(scheduler, &self.active_background.name);
        clean_up_property(scheduler, &self.flash_foreground.name);
        clean_up_property(scheduler, &self.flash_background.name);
        clean_up_property(scheduler, &self.tab_foreground.name);
        clean_up_property(scheduler, &self.tab_background.name);
        clean_up_property(scheduler, &self.filler_foreground.name);
        clean_up_property(scheduler, &self.filler_background.name);
        clean_up_property(scheduler, &self.border_foreground.name);
        clean_up_property(scheduler, &self.border_background.name);
        clean_up_property(scheduler, &self.cursor.name);
    }
}


#[derive(PartialEq, Clone, Debug)]
pub struct Padding {
    top: EzProperty<usize>,
    bottom: EzProperty<usize>,
    left: EzProperty<usize>,
    right: EzProperty<usize>,
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
            top: top_property,
            bottom: bottom_property,
            left: left_property,
            right: right_property,
        }
    }

    pub fn set_top(&mut self, padding: usize) {
        self.top.set(padding);
    }

    pub fn get_top(&self) -> usize {
        self.top.value
    }

    pub fn set_bottom(&mut self, padding: usize) {
        self.bottom.set(padding);
    }

    pub fn get_bottom(&self) -> usize {
        self.bottom.value
    }

    pub fn set_left(&mut self, padding: usize) {
        self.left.set(padding);
    }

    pub fn get_left(&self) -> usize {
        self.left.value
    }

    pub fn set_right(&mut self, padding: usize) {
        self.right.set(padding);
    }

    pub fn get_right(&self) -> usize {
        self.right.value
    }

    pub fn clean_up_properties(&self, scheduler: &mut SchedulerFrontend) {
        clean_up_property(scheduler, &self.top.name);
        clean_up_property(scheduler, &self.bottom.name);
        clean_up_property(scheduler, &self.left.name);
        clean_up_property(scheduler, &self.right.name);
    }
}
