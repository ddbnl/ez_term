use crossterm::style::{Color, StyledContent, Stylize};
use crate::CallbackConfig;
use crate::run::tree::Tree;
use crate::states::ez_state::EzState;
use crate::widgets::ez_object::EzObjects;

/// Convenience wrapper around an XY tuple.
#[derive(PartialEq, Clone, Copy, Debug, Default)]
pub struct Coordinates {
    pub x: usize,
    pub y: usize,
}
impl Coordinates {
    pub fn new(x: usize, y: usize) -> Self { Coordinates{x, y}}
}



/// Convenience wrapper around a size tuple.
#[derive(PartialEq, Clone, Copy, Debug, Default)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}
impl Size {
    pub fn new(width: usize, height: usize) -> Self { Size{width, height}}
}


/// Struct representing a single X,Y position on the screen. It has a symbol, colors, and other
/// properties governing how the position will look on screen.
#[derive(Clone, Debug)]
pub struct Pixel {

    /// Symbol drawn on screen.
    pub symbol: String,

    /// Foreground color in crossterm::style::color
    pub foreground_color: Color,

    /// Background color in crossterm::style::color
    pub background_color: Color,

    /// Whether symbol should be underlined
    pub underline: bool
}
impl Pixel {
    /// Turn into a crossterm StyledContent which can be drawn on screen.
    pub fn get_pixel(&self) -> StyledContent<String> {

        let mut pixel = self.symbol.clone()
            .with(self.foreground_color)
            .on(self.background_color);
        if self.underline {
            pixel = pixel.underlined();
        }
        pixel
    }
}
impl Pixel {
    pub fn new(symbol: String, foreground_color: Color, background_color: Color) -> Self {
        Pixel { symbol, foreground_color, background_color, underline: false }
    }
}
impl Default for Pixel {
    fn default() -> Self {
        Pixel{
            symbol: " ".to_string(),
            foreground_color: Color::White,
            background_color: Color::Blue,
            underline: false
        }
    }
}

/// # Convenience types
/// ## Pixel maps:
/// Used to represent the visual content of widgets. Pixels are a wrapper around
/// Crossterm StyledContent, so PixelMaps are essentially a grid of StyledContent to display.
pub type PixelMap = Vec<Vec<Pixel>>;


/// ## State tree:
/// A <WidgetPath, State> HashMap. The State contains all run-time information for a
/// widget, such as the text of a label, or whether a checkbox is currently checked. Callbacks
/// receive a mutable reference to the widget state and can change what they need. Then after each
/// frame the updated StateTree is diffed with the old one, and only changed widgets are redrawn.
pub type StateTree = Tree<EzState>;


/// ## Widget tree:
/// A read-only list of all widgets, passed to callbacks. Can be used to access static information
/// of a widget that is not in its' State. Widgets are represented by the EzWidget enum, but
/// can be cast to the generic UxObject or IsWidget trait. If you are sure of the type of widget
/// you are dealing with it can also be cast to specific widget types.
pub type CallbackTree = Tree<CallbackConfig>;


/// ## Widget tree:
/// A read-only list of all widgets, passed to callbacks. Can be used to access static information
/// of a widget that is not in its' State. Widgets are represented by the EzWidget enum, but
/// can be cast to the generic UxObject or IsWidget trait. If you are sure of the type of widget
/// you are dealing with it can also be cast to specific widget types.
pub type WidgetTree<'a> = Tree<&'a EzObjects>;
