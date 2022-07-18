//! # Run Definitions
//!
//! This module contains definitions common to run functions.
use crossterm::style::{Color, StyledContent, Stylize};

use crate::CallbackConfig;
use crate::run::tree::Tree;
use crate::states::definitions::StateSize;
use crate::states::ez_state::EzState;


/// Convenience wrapper around an XY tuple, represents coordinates. Makes reading code more clear
/// when explicitly dealing with 'x' and 'y'.
#[derive(PartialEq, Clone, Copy, Debug, Default)]
pub struct Coordinates {
    pub x: usize,
    pub y: usize,
}
impl Coordinates {
    pub fn new(x: usize, y: usize) -> Self { Coordinates{x, y}}
}



/// Convenience wrapper around a width/height tuple, represents size. Makes reading code more clear
/// when explicitly dealing with 'width' and 'height'.
#[derive(PartialEq, Clone, Copy, Debug, Default)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}
impl Size {

    pub fn new(width: usize, height: usize) -> Self { Size{width, height}}

    pub fn from_state_size(other: &StateSize) -> Self {
        Size {
            width: other.width.value,
            height: other.height.value,
        }
    }
}


/// Struct representing the graphic representation of a single X,Y position on the screen.
/// It has a symbol, colors, and other properties governing how the position will look on screen.
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

    /// Turn this pixel into a crossterm StyledContent which can be drawn on screen.
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

/// Used to represent the visual content of widgets. Pixels are a wrapper around
/// Crossterm StyledContent, so PixelMaps are essentially a grid of StyledContent to display.
pub type PixelMap = Vec<Vec<Pixel>>;


/// A wrapper around a <WidgetPath, State> HashMap. The State contains all run-time information for a
/// widget, such as the text of a label, or whether a checkbox is currently checked. Callbacks
/// receive a mutable reference to the widget state and can change what they need. If update is
/// called on the widget, the new state will be drawn on screen on the next frame.
pub type StateTree = Tree<EzState>;


/// A wrapper around a <WidgetPath, [CallbackConfig]> HashMap. Can be used to access callbacks
/// bound to a widget. A [CallbackConfig] for a wiget can be updated through the [Scheduler].
pub type CallbackTree = Tree<CallbackConfig>;
