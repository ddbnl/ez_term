use std::collections::HashMap;
use crossterm::event::KeyCode;
use crossterm::style::Color;
use crate::property::ez_property::EzProperty;
use crate::scheduler::definitions::{GenericEzFunction, KeyboardCallbackFunction, MouseCallbackFunction, MouseDragCallbackFunction, OptionalMouseCallbackFunction};
use crate::scheduler::scheduler::Scheduler;
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
    Tabbed,
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
    pub rows: EzProperty<usize>,

    /// Maximum amount of columns. Usually you want to set either the maximum amount of rows or the
    /// maximum amount of columns, and let the other one grow with the amount of content
    pub columns: EzProperty<usize>,

    /// Default height of rows. If kept at 0, it will be set to the height of the parent divided by
    /// the amount of rows. If force_default_height is false, widgets are allowed to be larger
    /// than default_height, in which case each row will grow to its' largest widget.
    pub default_height: EzProperty<usize>,

    /// Default width of columns. If kept at 0, it will be set to the width of the parent divided by
    /// the amount of columns. If force_default_width is false, widgets are allowed to be larger
    /// than default_width, in which case each column will grow to its' largest widget.
    pub default_width: EzProperty<usize>,

    /// Each row will be exactly default_height. If default_height is 0, it will be set to the
    /// height of the parent divided by the amount of rows.
    pub force_default_height: EzProperty<bool>,

    /// Each column will be exactly default_width. If default_width is 0, it will be set to the
    /// width of the parent divided by the amount of columns.
    pub force_default_width: EzProperty<bool>,
}
impl TableConfig {

    pub fn new(name: String, scheduler: &mut Scheduler) -> Self {

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

    pub fn clean_up_properties(&self, scheduler: &mut Scheduler) {
        clean_up_property(scheduler, &self.rows.name);
        clean_up_property(scheduler, &self.columns.name);
        clean_up_property(scheduler, &self.force_default_height.name);
        clean_up_property(scheduler, &self.force_default_width.name);
    }
}


/// Convenience wrapper around a size tuple.
#[derive(PartialEq, Clone, Debug)]
pub struct StateSize {
    pub width: EzProperty<usize>,
    pub height: EzProperty<usize>,
    pub infinite_width: bool,
    pub infinite_height: bool,
}
impl StateSize {

    pub fn new(width: usize, height: usize, name: String, scheduler: &mut Scheduler) -> Self {

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

    pub fn clean_up_properties(&self, scheduler: &mut Scheduler) {
        clean_up_property(scheduler, &self.width.name);
        clean_up_property(scheduler, &self.height.name);
    }
}


/// Convenience wrapper around an XY tuple.
#[derive(PartialEq, Clone, Debug)]
pub struct StateCoordinates {
    pub x: EzProperty<usize>,
    pub y: EzProperty<usize>,
}
impl StateCoordinates {
    pub fn new(x: usize, y: usize, name: String, scheduler: &mut Scheduler) -> Self {

        let x_property =
            scheduler.new_usize_property(format!("{}/x", name).as_str(), x);
        let y_property =
            scheduler.new_usize_property(format!("{}/y", name).as_str(), y);
        StateCoordinates{
            x: x_property,
            y: y_property
        }
    }

    pub fn clean_up_properties(&self, scheduler: &mut Scheduler) {
        clean_up_property(scheduler, &self.x.name);
        clean_up_property(scheduler, &self.y.name);
    }
}


/// Convenience wrapper around an size_hint tuple.
#[derive(PartialEq, Clone, Debug)]
pub struct AutoScale {
    pub width: EzProperty<bool>,
    pub height: EzProperty<bool>,
}
impl AutoScale {

    pub fn new(width: bool, height: bool, name: String, scheduler: &mut Scheduler) -> Self {
        let width_property =
            scheduler.new_bool_property(format!("{}/autoscale_width", name).as_str(),
                                        width);
        let height_property =
            scheduler.new_bool_property(format!("{}/autoscale_height", name).as_str(),
                                        height);
        AutoScale{width: width_property, height: height_property}
    }

    pub fn clean_up_properties(&self, scheduler: &mut Scheduler) {
        clean_up_property(scheduler, &self.width.name);
        clean_up_property(scheduler, &self.height.name);
    }
}


/// Convenience wrapper around an size_hint tuple.
#[derive(PartialEq, Clone, Debug)]
pub struct SizeHint {
    pub x: EzProperty<Option<f64>>,
    pub y: EzProperty<Option<f64>>,
}
impl SizeHint {
    pub fn new(x: Option<f64>, y: Option<f64>, name: String, scheduler: &mut Scheduler) -> Self {
        let x_property =
            scheduler.new_size_hint_property(format!("{}/size_hint_width", name).as_str(),
                                        x);
        let y_property =
            scheduler.new_size_hint_property(format!("{}/size_hint_height", name).as_str(),
                                        y);
        SizeHint{x: x_property, y: y_property}
    }

    pub fn clean_up_properties(&self, scheduler: &mut Scheduler) {
        clean_up_property(scheduler, &self.x.name);
        clean_up_property(scheduler, &self.y.name);
    }
}


/// Convenience wrapper around an pos_hint tuple.
#[derive(PartialEq, Clone, Debug)]
pub struct PosHint {
    pub x: EzProperty<Option<(HorizontalAlignment, f64)>>,
    pub y: EzProperty<Option<(VerticalAlignment, f64)>>,
}
impl PosHint {

    pub fn new(x: Option<(HorizontalAlignment, f64)>, y: Option<(VerticalAlignment, f64)>,
               name: String, scheduler: &mut Scheduler) -> Self {
        let x_property =
            scheduler.new_horizontal_pos_hint_property(
                format!("{}/pos_hint_x", name).as_str(),x);
        let y_property =
            scheduler.new_vertical_pos_hint_property(
                format!("{}/pos_hint_y", name).as_str(),y);
        PosHint{x: x_property, y: y_property}
    }

    pub fn clean_up_properties(&self, scheduler: &mut Scheduler) {
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

    pub fn bind_key(&mut self, key: KeyCode, func: KeyboardCallbackFunction) {
        self.keymap.insert(key, func);
    }

    pub fn from_on_select(func: OptionalMouseCallbackFunction) -> Self {
        let mut obj = CallbackConfig::default();
        obj.on_select = Some(func);
        obj
    }

    pub fn from_on_deselect(func: GenericEzFunction) -> Self {
        let mut obj = CallbackConfig::default();
        obj.on_deselect = Some(func);
        obj
    }

    pub fn from_on_press(func: GenericEzFunction) -> Self {
        let mut obj = CallbackConfig::default();
        obj.on_press = Some(func);
        obj
    }

    pub fn from_on_keyboard_enter(func: GenericEzFunction) -> Self {
        let mut obj = CallbackConfig::default();
        obj.on_keyboard_enter = Some(func);
        obj
    }

    pub fn from_on_left_mouse_click(func: MouseCallbackFunction) -> Self {
        let mut obj = CallbackConfig::default();
        obj.on_left_mouse_click = Some(func);
        obj
    }

    pub fn from_on_right_mouse_click(func: MouseCallbackFunction) -> Self {
        let mut obj = CallbackConfig::default();
        obj.on_right_mouse_click = Some(func);
        obj
    }

    pub fn from_on_scroll_up(func: GenericEzFunction) -> Self {
        let mut obj = CallbackConfig::default();
        obj.on_scroll_up = Some(func);
        obj
    }

    pub fn from_on_scroll_down(func: GenericEzFunction) -> Self {
        let mut obj = CallbackConfig::default();
        obj.on_scroll_down = Some(func);
        obj
    }

    pub fn from_on_value_change(func: GenericEzFunction) -> Self {
        let mut obj = CallbackConfig::default();
        obj.on_value_change = Some(func);
        obj
    }

    pub fn from_on_hover(func: MouseCallbackFunction) -> Self {
        let mut obj = CallbackConfig::default();
        obj.on_hover = Some(func);
        obj
    }

    pub fn from_on_drag(func: MouseDragCallbackFunction) -> Self {
        let mut obj = CallbackConfig::default();
        obj.on_drag = Some(func);
        obj
    }

    pub fn from_keymap(keymap: KeyMap) -> Self {
        let mut obj = CallbackConfig::default();
        obj.keymap = keymap;
        obj
    }

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
    pub enable_x: EzProperty<bool>,

    /// Bool representing whether the y axis should be able to scroll
    pub enable_y: EzProperty<bool>,

    /// Start of the view on the x axis, content is shown from here until view_start_x + width
    pub view_start_x: usize,

    /// Start of the view on the y axis, content is shown from here until view_start_y + height
    pub view_start_y: usize,

    /// Bool representing whether the owning object is actually scrolling, as it is possible for
    /// scrolling to be enabled but not active (i.e. content already fits within object)
    pub is_scrolling_x: bool,

    /// Bool representing whether the owning object is actually scrolling, as it is possible for
    /// scrolling to be enabled but not active (i.e. content already fits within object)
    pub is_scrolling_y: bool,

    /// Original height of the content being scrolled
    pub original_height: usize,

    /// Original width of the content being scrolled
    pub original_width: usize,
}
impl ScrollingConfig {

    pub fn new(enable_x: bool, enable_y: bool, name: String, scheduler: &mut Scheduler) -> Self {

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
    pub fn clean_up_properties(&self, scheduler: &mut Scheduler) {
        clean_up_property(scheduler, &self.enable_x.name);
        clean_up_property(scheduler, &self.enable_y.name);
    }
}


/// Convenience wrapper around a border configuration
#[derive(PartialEq, Clone, Debug)]
pub struct BorderConfig {

    /// Bool representing whether an object should have a border
    pub enabled: EzProperty<bool>,

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
    
    /// The [Pixel.foreground_color]  to use for the border if [border] is true
    pub fg_color: EzProperty<Color>,
    
    /// The [Pixel.background_color] to use for the border if [border] is true
    pub bg_color: EzProperty<Color>,
}

impl BorderConfig {

    pub fn new(enable: bool, name: String, scheduler: &mut Scheduler) -> Self {

        let enabled_property =
            scheduler.new_bool_property(format!("{}/border_enabled", name).as_str(),
                                        enable);
        let horizontal_symbol =
            scheduler.new_string_property(format!("{}/border_horizontal", name).as_str(),
                                          "━".to_string());
        let vertical_symbol =
            scheduler.new_string_property(format!("{}/border_vertical", name).as_str(),
                                          "│".to_string());
        let top_left_symbol =
            scheduler.new_string_property(format!("{}/border_top_left", name).as_str(),
                                          "┍".to_string());
        let top_right_symbol =
            scheduler.new_string_property(format!("{}/border_top_right", name).as_str(),
                                          "┑".to_string());
        let bottom_left_symbol =
            scheduler.new_string_property(format!("{}/border_bottom_left", name).as_str(),
                                          "┕".to_string());
        let bottom_right_symbol =
            scheduler.new_string_property(format!("{}/border_bottom_right", name).as_str(),
                                          "┙".to_string());
        let fg_color =
            scheduler.new_color_property(format!("{}/border_fg_color", name).as_str(),
                                          Color::White);
        let bg_color =
            scheduler.new_color_property(format!("{}/border_bg_color", name).as_str(),
                                         Color::Black);

       BorderConfig {
           enabled: enabled_property,
           horizontal_symbol,
           vertical_symbol,
           top_left_symbol,
           top_right_symbol,
           bottom_left_symbol,
           bottom_right_symbol,
           fg_color,
           bg_color,
       } 
    }

    pub fn clean_up_properties(&self, scheduler: &mut Scheduler) {
        clean_up_property(scheduler, &self.enabled.name);
        clean_up_property(scheduler, &self.horizontal_symbol.name);
        clean_up_property(scheduler, &self.vertical_symbol.name);
        clean_up_property(scheduler, &self.top_left_symbol.name);
        clean_up_property(scheduler, &self.top_right_symbol.name);
        clean_up_property(scheduler, &self.bottom_left_symbol.name);
        clean_up_property(scheduler, &self.bottom_right_symbol.name);
        clean_up_property(scheduler, &self.fg_color.name);
        clean_up_property(scheduler, &self.bg_color.name);
    }
}


#[derive(PartialEq, Clone, Debug)]
pub struct ColorConfig {

    /// The [Pixel.foreground_color] to use for this widgets' content
    pub foreground: EzProperty<Color>,

    /// The [Pixel.background_color] to use for this widgets' content
    pub background: EzProperty<Color>,

    /// The [Pixel.foreground_color] to use for this widgets' content when selected
    pub selection_foreground: EzProperty<Color>,

    /// The [Pixel.background_color] to use for this widgets' content when selected
    pub selection_background: EzProperty<Color>,

    /// The [Pixel.foreground_color] to use for this widgets' content is disabled
    pub disabled_foreground: EzProperty<Color>,

    /// The [Pixel.background_color] to use for this widgets' content is disabled
    pub disabled_background: EzProperty<Color>,

    /// The [Pixel.foreground_color] to use for this widgets' content is active
    pub active_foreground: EzProperty<Color>,

    /// The [Pixel.background_color] to use for this widgets' content is active
    pub active_background: EzProperty<Color>,

    /// The [Pixel.foreground_color] to use for this widgets' content when flashed
    pub flash_foreground: EzProperty<Color>,

    /// The [Pixel.background_color] to use for this widgets' content when flashed
    pub flash_background: EzProperty<Color>,

    /// The [Pixel.foreground_color] to use for tab headers
    pub tab_foreground: EzProperty<Color>,

    /// The [Pixel.background_color] to use for tab headers
    pub tab_background: EzProperty<Color>,

    /// The [Pixel.foreground_color] to use for filler pixels if [fill] is true
    pub filler_foreground: EzProperty<Color>,

    /// The [Pixel.background_color] to use for filler pixels if [fill] is true
    pub filler_background: EzProperty<Color>,

    /// The [Pixel.background_color] to use for this widgets' content when a position has been
    /// highlighted by the blinking cursor
    pub cursor: EzProperty<Color>,
}
impl ColorConfig {
    pub fn new(name: String, scheduler: &mut Scheduler) -> Self {

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
            cursor,
        }
    }

    pub fn clean_up_properties(&self, scheduler: &mut Scheduler) {
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
        clean_up_property(scheduler, &self.cursor.name);
    }
}


#[derive(PartialEq, Clone, Debug)]
pub struct Padding {
    pub top: EzProperty<usize>,
    pub bottom: EzProperty<usize>,
    pub left: EzProperty<usize>,
    pub right: EzProperty<usize>,
}
impl Padding {
    pub fn new(top: usize, bottom: usize, left: usize, right: usize, name: String,
               scheduler: &mut Scheduler) -> Padding{


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

    pub fn clean_up_properties(&self, scheduler: &mut Scheduler) {
        clean_up_property(scheduler, &self.top.name);
        clean_up_property(scheduler, &self.bottom.name);
        clean_up_property(scheduler, &self.left.name);
        clean_up_property(scheduler, &self.right.name);
    }
}
