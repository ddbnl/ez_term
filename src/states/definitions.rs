use crossterm::event::KeyCode;
use crossterm::style::Color;
use crate::common;
use crate::property::property::EzProperty;
use crate::scheduler::scheduler::Scheduler;
use crate::scheduler::scheduler_funcs::clean_up_property;


/// Used with Box mode [Layout], determines whether widgets are placed below or above each other.
#[derive(Clone, PartialEq, Debug)]
pub enum LayoutOrientation {
    Horizontal,
    Vertical
}


/// Different modes determining how widgets are placed in a [Layout].
#[derive(Clone, PartialEq, Debug)]
pub enum LayoutMode {

    /// # Box mode:
    /// Widgets are placed next to each other or under one another depending on orientation.
    /// In horizontal orientation widgets always use the full height of the layout, and in
    /// vertical position they always use the full with.
    Box,

    /// Float mode:
    /// Widgets are placed according to [PositionHint] or in their hardcoded XY positions.
    Float,

    /// # Screen mode:
    /// This layout can only contain other layouts. Only the root widget may be a Screen layout.
    /// Only the contents of the active screen will be shown. Active screen is controlled through
    /// [LayoutState.active_screen].
    Screen,

    ///# Tabbed mode:
    /// This layout can only contain other layouts and presents those as tabs. A tab header will
    /// automatically be added for each child Layout, so the user can switch between tabs. The tab
    /// header will display the [id] of the child Layout.
    Tabbed

    // todo table
    // todo stack
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
            format!("{}/width", name), width);
        let height_property = scheduler.new_usize_property(
            format!("{}/height", name), height);
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
            scheduler.new_usize_property(format!("{}/x", name), x);
        let y_property =
            scheduler.new_usize_property(format!("{}/y", name), y);
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
            scheduler.new_bool_property(format!("{}/autoscale_width", name),
                                        width);
        let height_property =
            scheduler.new_bool_property(format!("{}/autoscale_height", name),
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
            scheduler.new_size_hint_property(format!("{}/size_hint_width", name),
                                        x);
        let y_property =
            scheduler.new_size_hint_property(format!("{}/size_hint_height", name),
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
                format!("{}/pos_hint_x", name),x);
        let y_property =
            scheduler.new_vertical_pos_hint_property(
                format!("{}/pos_hint_y", name),y);
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
    pub on_select: Option<common::definitions::OptionalMouseCallbackFunction> ,

    /// Function to call when an object is deselected.
    pub on_deselect: Option<common::definitions::GenericEzFunction>,

    /// Function to call when an object is keyboard entered or left clicked,
    pub on_press: Option<common::definitions::GenericEzFunction>,

    /// Function to call when this widget is right clicked
    pub on_keyboard_enter: Option<common::definitions::GenericEzFunction>,

    /// Function to call when this widget is right clicked
    pub on_left_mouse_click: Option<common::definitions::MouseCallbackFunction>,

    /// Function to call when this widget is right clicked
    pub on_right_mouse_click: Option<common::definitions::MouseCallbackFunction>,

    /// Function to call when this widget is scrolled up
    pub on_scroll_up: Option<common::definitions::GenericEzFunction>,

    /// Function to call when this widget is scrolled down
    pub on_scroll_down: Option<common::definitions::GenericEzFunction>,

    /// Function to call when the value of an object changes
    pub on_value_change: Option<common::definitions::GenericEzFunction>,

    /// A Key to callback function lookup used to store keybinds for this widget. See
    /// [KeyboardCallbackFunction] type for callback function signature.
    pub keymap: common::definitions::KeyMap,
}
impl CallbackConfig {

    pub fn bind_key(&mut self, key: KeyCode, func: common::definitions::KeyboardCallbackFunction) {
        self.keymap.insert(key, func);
    }

    pub fn from_on_select(func: common::definitions::OptionalMouseCallbackFunction) -> Self {
        let mut obj = CallbackConfig::default();
        obj.on_select = Some(func);
        obj
    }

    pub fn from_on_deselect(func: common::definitions::GenericEzFunction) -> Self {
        let mut obj = CallbackConfig::default();
        obj.on_deselect = Some(func);
        obj
    }

    pub fn from_on_press(func: common::definitions::GenericEzFunction) -> Self {
        let mut obj = CallbackConfig::default();
        obj.on_press = Some(func);
        obj
    }

    pub fn from_on_keyboard_enter(func: common::definitions::GenericEzFunction) -> Self {
        let mut obj = CallbackConfig::default();
        obj.on_keyboard_enter = Some(func);
        obj
    }

    pub fn from_on_left_mouse_click(func: common::definitions::MouseCallbackFunction) -> Self {
        let mut obj = CallbackConfig::default();
        obj.on_left_mouse_click = Some(func);
        obj
    }

    pub fn from_on_right_mouse_click(func: common::definitions::MouseCallbackFunction) -> Self {
        let mut obj = CallbackConfig::default();
        obj.on_right_mouse_click = Some(func);
        obj
    }

    pub fn from_on_scroll_up(func: common::definitions::GenericEzFunction) -> Self {
        let mut obj = CallbackConfig::default();
        obj.on_scroll_up = Some(func);
        obj
    }

    pub fn from_on_scroll_down(func: common::definitions::GenericEzFunction) -> Self {
        let mut obj = CallbackConfig::default();
        obj.on_scroll_down = Some(func);
        obj
    }

    pub fn from_on_value_change(func: common::definitions::GenericEzFunction) -> Self {
        let mut obj = CallbackConfig::default();
        obj.on_value_change = Some(func);
        obj
    }

    pub fn from_keymap(keymap: common::definitions::KeyMap) -> Self {
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
        self.keymap.extend(other.keymap);
    }

}


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
            scheduler.new_bool_property(format!("{}/scrolling_enable_x", name),
                                        enable_x);
        let y_property =
            scheduler.new_bool_property(format!("{}/scrolling_enable_y", name),
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
            scheduler.new_bool_property(format!("{}/border_enabled", name),
                                        enable);
        let horizontal_symbol =
            scheduler.new_string_property(format!("{}/border_horizontal", name),
                                          "━".to_string());
        let vertical_symbol =
            scheduler.new_string_property(format!("{}/border_vertical", name),
                                          "│".to_string());
        let top_left_symbol =
            scheduler.new_string_property(format!("{}/border_top_left", name),
                                          "┍".to_string());
        let top_right_symbol =
            scheduler.new_string_property(format!("{}/border_top_right", name),
                                          "┑".to_string());
        let bottom_left_symbol =
            scheduler.new_string_property(format!("{}/border_bottom_left", name),
                                          "┕".to_string());
        let bottom_right_symbol =
            scheduler.new_string_property(format!("{}/border_bottom_right", name),
                                          "┙".to_string());
        let fg_color =
            scheduler.new_color_property(format!("{}/border_fg_color", name),
                                          Color::White);
        let bg_color =
            scheduler.new_color_property(format!("{}/border_bg_color", name),
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
            format!("{}/color_fg", name), Color::White);
        let background = scheduler.new_color_property(
            format!("{}/color_bg", name), Color::Black);

        let selection_foreground = scheduler.new_color_property(
            format!("{}/color_selection_fg", name), Color::Yellow);
        let selection_background = scheduler.new_color_property(
            format!("{}/color_selection_bg", name), Color::Blue);

        let disabled_foreground = scheduler.new_color_property(
            format!("{}/color_disabled_fg", name), Color::White);
        let disabled_background = scheduler.new_color_property(
            format!("{}/color_disabled_bg", name), Color::Black);

        let active_foreground = scheduler.new_color_property(
            format!("{}/color_active_fg", name), Color::Red);
        let active_background = scheduler.new_color_property(
            format!("{}/color_active_bg", name), Color::Black);

        let flash_foreground = scheduler.new_color_property(
            format!("{}/color_flash_fg", name), Color::Yellow);
        let flash_background = scheduler.new_color_property(
            format!("{}/color_flash_bg", name), Color::White);

        let filler_foreground = scheduler.new_color_property(
            format!("{}/color_filler_fg", name), Color::White);
        let filler_background = scheduler.new_color_property(
            format!("{}/color_filler_bg", name), Color::Black);

        let tab_foreground = scheduler.new_color_property(
            format!("{}/color_tab_fg", name), Color::White);
        let tab_background = scheduler.new_color_property(
            format!("{}/color_tab_bg", name), Color::Black);

        let cursor = scheduler.new_color_property(
            format!("{}/color_cursor", name), Color::DarkYellow);

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
            format!("{}/padding_top", name), top);
        let bottom_property = scheduler.new_usize_property(
            format!("{}/padding_bottom", name), bottom);
        let left_property = scheduler.new_usize_property(
            format!("{}/padding_left", name), left);
        let right_property = scheduler.new_usize_property(
            format!("{}/padding_right", name), right);
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
