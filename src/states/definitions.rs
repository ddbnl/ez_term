use crossterm::event::KeyCode;
use crossterm::style::Color;
use crate::common;
use crate::property::UsizeProperty;
use crate::scheduler::Scheduler;


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
    pub width: UsizeProperty,
    pub height: UsizeProperty,
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
}


/// Convenience wrapper around an XY tuple.
#[derive(PartialEq, Clone, Debug)]
pub struct StateCoordinates {
    pub x: UsizeProperty,
    pub y: UsizeProperty,
}
impl StateCoordinates {
    pub fn new(x: usize, y: usize, name: String, scheduler: &mut Scheduler) -> Self {

        let x_property =
            scheduler.new_usize_property(format!("{}/x", name.clone()), x);
        let y_property =
            scheduler.new_usize_property(format!("{}/y", name.clone()), y);
        StateCoordinates{
            x: x_property,
            y: y_property
        }
    }
}


/// Convenience wrapper around an size_hint tuple.
#[derive(PartialEq, Copy, Clone, Default, Debug)]
pub struct AutoScale {
    pub width: bool,
    pub height: bool,
}
impl AutoScale {
    pub fn new(width: bool, height: bool) -> Self { AutoScale{width, height} }
}


/// Convenience wrapper around an size_hint tuple.
#[derive(PartialEq, Copy, Clone, Debug)]
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
#[derive(PartialEq, Copy, Clone, Debug)]
pub struct PosHint {
    pub x: Option<(HorizontalAlignment, f64)>,
    pub y: Option<(VerticalAlignment, f64)>,
}
impl PosHint {
    pub fn new(x: Option<(HorizontalAlignment, f64)>,
               y: Option<(VerticalAlignment, f64)>) -> Self {
        PosHint{x, y}
    }
}
impl Default for PosHint {
    fn default() -> Self { PosHint{x: Some((HorizontalAlignment::Left, 1.0)),
                                    y: Some((VerticalAlignment::Top, 1.0)) }}
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
#[derive(PartialEq, Clone, Debug, Default)]
pub struct ScrollingConfig {

    /// Bool representing whether the x axis should be able to scroll
    pub enable_x: bool,

    /// Bool representing whether the y axis should be able to scroll
    pub enable_y: bool,

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

/// Convenience wrapper around a border configuration
#[derive(PartialEq, Clone, Debug)]
pub struct BorderConfig {

    /// Bool representing whether an object should have a border
    pub enabled: bool,

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
           enabled: false,
           horizontal_symbol: "━".to_string(),
           vertical_symbol: "│".to_string(),
           top_left_symbol: "┍".to_string(),
           top_right_symbol: "┑".to_string(),
           bottom_left_symbol: "┕".to_string(),
           bottom_right_symbol: "┙".to_string(),
           fg_color: Color::White,
           bg_color: Color::Black,
       } 
    }
}


#[derive(PartialEq, Clone, Debug)]
pub struct ColorConfig {

    /// The [Pixel.foreground_color] to use for this widgets' content
    pub foreground: Color,

    /// The [Pixel.background_color] to use for this widgets' content
    pub background: Color,

    /// The [Pixel.foreground_color] to use for this widgets' content when selected
    pub selection_foreground: Color,

    /// The [Pixel.background_color] to use for this widgets' content when selected
    pub selection_background: Color,

    /// The [Pixel.foreground_color] to use for this widgets' content is disabled
    pub disabled_foreground: Color,

    /// The [Pixel.background_color] to use for this widgets' content is disabled
    pub disabled_background: Color,

    /// The [Pixel.foreground_color] to use for this widgets' content is active
    pub active_foreground: Color,

    /// The [Pixel.background_color] to use for this widgets' content is active
    pub active_background: Color,

    /// The [Pixel.foreground_color] to use for this widgets' content when flashed
    pub flash_foreground: Color,

    /// The [Pixel.background_color] to use for this widgets' content when flashed
    pub flash_background: Color,

    /// The [Pixel.foreground_color] to use for tab headers
    pub tab_foreground: Color,

    /// The [Pixel.background_color] to use for tab headers
    pub tab_background: Color,

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
            disabled_foreground: Color::White,
            disabled_background: Color::Black,
            active_foreground: Color::Red,
            active_background: Color::Black,
            flash_foreground: Color::Yellow,
            flash_background: Color::White,
            tab_foreground: Color::White,
            tab_background: Color::Black,
            filler_foreground: Color::White,
            filler_background: Color::Black,
            cursor: Color::DarkYellow,
        }
    }
}


#[derive(PartialEq, Clone, Debug)]
pub struct Padding {
    pub top: UsizeProperty,
    pub bottom: UsizeProperty,
    pub left: UsizeProperty,
    pub right: UsizeProperty,
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
}
