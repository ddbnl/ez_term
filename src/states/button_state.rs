use crossterm::style::{Color};
use crate::common::{Coordinates};
use crate::states::state::{GenericState, SelectableState, HorizontalAlignment, VerticalAlignment,
                           HorizontalPositionHint, VerticalPositionHint};


/// [State] implementation for [Button].
#[derive(Clone)]
pub struct ButtonState {

    /// Text currently being displayed by the label
    pub text: String,

    /// Bool representing whether this widget is currently selected.
    pub selected: bool,

    /// Bool representing whether this widget is currently displaying it's flash color.
    pub flashing: bool,

    /// Horizontal position of this widget relative to its' parent [Layout]
    pub x: usize,

    /// Vertical position of this widget relative to its' parent [Layout]
    pub y: usize,

    /// Absolute position of this layout on screen. Automatically propagated, do not set manually
    pub absolute_position: Coordinates,

    /// Width of this widget
    pub size_hint_x: Option<f64>,

    /// Width of this widget
    pub size_hint_y: Option<f64>,

    /// Pos hint for x position of this widget
    pub pos_hint_x: Option<(HorizontalPositionHint, f64)>,

    /// Pos hint for y position of this widget
    pub pos_hint_y: Option<(VerticalPositionHint, f64)>,

    /// Width of this widget
    pub width: usize,

    /// Width of this widget
    pub height: usize,

    /// Automatically adjust width of widget to content
    pub auto_scale_width: bool,

    /// Automatically adjust width of widget to content
    pub auto_scale_height: bool,

    /// Horizontal alignment of this widget
    pub halign: HorizontalAlignment,

    /// Vertical alignment of this widget
    pub valign: VerticalAlignment,

    /// The [Pixel.symbol] to use for the horizontal border if [border] is true
    pub border_horizontal_symbol: String,

    /// The [Pixel.symbol] to use for the vertical border if [border] is true
    pub border_vertical_symbol: String,

    /// The [Pixel.symbol] to use for the top left border if [border] is true
    pub border_top_left_symbol: String,

    /// The [Pixel.symbol] to use for the top left border if [border] is true
    pub border_top_right_symbol: String,

    /// The [Pixel.symbol] to use for the bottom left border if [border] is true
    pub border_bottom_left_symbol: String,

    /// The [Pixel.symbol] to use for the bottom right border if [border] is true
    pub border_bottom_right_symbol: String,

    /// The[Pixel.foreground_color]  to use for the border if [border] is true
    pub border_foreground_color: Color,

    /// The [Pixel.background_color] to use for the border if [border] is true
    pub border_background_color: Color,

    /// The [Pixel.foreground_color] to use for this widgets' content
    pub content_foreground_color: Color,

    /// The [Pixel.background_color] to use for this widgets' content
    pub content_background_color: Color,

    /// The [Pixel.foreground_color] to use for this widgets' content when selected
    pub selection_foreground_color: Color,

    /// The [Pixel.background_color] to use for this widgets' content when selected
    pub selection_background_color: Color,

    /// The [Pixel.foreground_color] to use for this widgets' content when flashed
    pub flash_foreground_color: Color,

    /// The [Pixel.background_color] to use for this widgets' content when flashed
    pub flash_background_color: Color,

    /// Bool representing if state has changed. Triggers widget redraw.
    pub changed: bool,

    /// If true this forces a global screen redraw on the next frame. Screen redraws are diffed
    /// so this can be called when needed without degrading performance. If only screen positions
    /// that fall within this widget must be redrawn, call [EzObject.redraw] instead.
    pub force_redraw: bool,
}
impl Default for ButtonState {
    fn default() -> Self {

       ButtonState {
           x: 0,
           y: 0,
           absolute_position: (0, 0),
           size_hint_x: Some(1.0),
           size_hint_y: Some(1.0),
           pos_hint_x: None,
           pos_hint_y: None,
           auto_scale_width: false,
           auto_scale_height: false,
           width: 0,
           height: 0,
           halign: HorizontalAlignment::Left,
           valign: VerticalAlignment::Top,
           text: String::new(),
           selected: false,
           flashing: false,
           border_horizontal_symbol: "━".to_string(),
           border_vertical_symbol: "│".to_string(),
           border_top_left_symbol: "┌".to_string(),
           border_top_right_symbol: "┐".to_string(),
           border_bottom_left_symbol: "└".to_string(),
           border_bottom_right_symbol: "┘".to_string(),
           border_foreground_color: Color::White,
           border_background_color: Color::Black,
           content_foreground_color: Color::White,
           content_background_color: Color::Black,
           selection_foreground_color: Color::Yellow,
           selection_background_color: Color::Blue,
           flash_foreground_color: Color::Yellow,
           flash_background_color: Color::White,
           changed: false,
           force_redraw: false,
       }
    }
}
impl GenericState for ButtonState {

    fn set_changed(&mut self, changed: bool) { self.changed = changed }

    fn get_changed(&self) -> bool { self.changed }

    fn set_size_hint_x(&mut self, size_hint: Option<f64>) {
        self.size_hint_x = size_hint;
        self.changed = true; 
    }

    fn get_size_hint_x(&self) -> Option<f64> { self.size_hint_x }

    fn set_size_hint_y(&mut self, size_hint: Option<f64>) {
        self.size_hint_y = size_hint;
        self.changed = true;
    }

    fn get_size_hint_y(&self) -> Option<f64> { self.size_hint_y }

    fn set_pos_hint_x(&mut self, pos_hint: Option<(HorizontalPositionHint, f64)>) {
        self.pos_hint_x = pos_hint;
        self.changed = true;
    }

    fn get_pos_hint_x(&self) -> &Option<(HorizontalPositionHint, f64)> { &self.pos_hint_x }

    fn set_pos_hint_y(&mut self, pos_hint: Option<(VerticalPositionHint, f64)>) {
        self.pos_hint_y = pos_hint;
        self.changed = true;
    }

    fn get_pos_hint_y(&self) -> &Option<(VerticalPositionHint, f64)> { &self.pos_hint_y }

    fn set_auto_scale_width(&mut self, auto_scale: bool) {
        self.auto_scale_width = auto_scale;
        self.changed = true;
    }

    fn get_auto_scale_width(&self) -> bool { self.auto_scale_width }

    fn set_auto_scale_height(&mut self, auto_scale: bool) {
        self.auto_scale_height = auto_scale;
        self.changed = true;
    }

    fn get_auto_scale_height(&self) -> bool { self.auto_scale_height }

    fn set_width(&mut self, width: usize) { self.width = width; self.changed = true; }

    fn get_width(&self) -> usize { self.width }

    fn set_effective_width(&mut self, width: usize) { self.set_width(width + 2) }

    fn get_effective_width(&self) -> usize {
        if self.get_width() < 2 {0} else {self.get_width() - 2 }}

    fn set_height(&mut self, height: usize) { self.height = height }

    /// Button returns always at least 3 height, as it needs 1 height for text and 2 for borders.
    fn get_height(&self) -> usize { if self.height >= 3 {self.height} else {3}}

    fn set_effective_height(&mut self, height: usize) { self.set_height(height + 2) }

    fn get_effective_height(&self) -> usize {
        if self.get_height() < 2 {0} else {self.get_height() - 2 }
    }

    fn set_position(&mut self, position: Coordinates) {
        self.x = position.0;
        self.y = position.1;
        self.changed = true;
    }

    fn get_position(&self) -> Coordinates { (self.x, self.y) }

    fn get_effective_position(&self) -> Coordinates {
        (self.x +if self.has_border() {1} else {0},
         self.y +if self.has_border() {1} else {0})
    }

    fn set_absolute_position(&mut self, pos: Coordinates) { self.absolute_position = pos }

    fn get_absolute_position(&self) -> Coordinates { self.absolute_position }

    fn get_effective_absolute_position(&self) -> Coordinates {
        let (x, y) = self.get_absolute_position();
        (x +if self.has_border() {1} else {0}, y +if self.has_border() {1} else {0})
    }

    fn set_horizontal_alignment(&mut self, alignment: HorizontalAlignment) {
        self.halign = alignment;
        self.changed = true;
    }

    fn get_horizontal_alignment(&self) -> HorizontalAlignment { self.halign }

    fn set_vertical_alignment(&mut self, alignment: VerticalAlignment) {
        self.valign = alignment;
        self.changed = true;
    }

    fn get_vertical_alignment(&self) -> VerticalAlignment { self.valign }

    fn set_force_redraw(&mut self, redraw: bool) {
        self.force_redraw = redraw;
        self.changed = true;
    }

    fn get_force_redraw(&self) -> bool { self.force_redraw }
}
impl SelectableState for ButtonState {
    fn set_selected(&mut self, state: bool) {
        self.selected = state;
        self.changed = true;
    }
    fn get_selected(&self) -> bool { self.selected }
}
impl ButtonState {

    pub fn set_text(&mut self, text: String) {
        self.text = text;
        self.changed = true;
    }

    pub fn get_text(&self) -> String { self.text.clone() }

    pub fn set_flashing(&mut self, flashing: bool) {
        self.flashing = flashing;
        self.changed = true;
    }

    pub fn get_flashing(&self) -> bool { self.flashing }

    pub fn set_border_horizontal_symbol(&mut self, symbol: String) {
        self.border_horizontal_symbol = symbol
    }

    pub fn get_border_horizontal_symbol(&self) -> String { self.border_horizontal_symbol.clone() }

    pub fn set_border_vertical_symbol(&mut self, symbol: String) {
        self.border_vertical_symbol = symbol
    }

    pub fn get_border_vertical_symbol(&self) -> String { self.border_vertical_symbol.clone() }

    pub fn set_border_bottom_left_symbol(&mut self, symbol: String) {
        self.border_bottom_left_symbol = symbol
    }

    pub fn get_border_bottom_left_symbol(&self) -> String { self.border_bottom_left_symbol.clone() }

    pub fn set_border_bottom_right_symbol(&mut self, symbol: String) {
        self.border_bottom_right_symbol = symbol
    }

    pub fn get_border_bottom_right_symbol(&self) -> String { self.border_bottom_right_symbol.clone() }

    pub fn set_border_top_left_symbol(&mut self, symbol: String) {
        self.border_top_left_symbol = symbol
    }

    pub fn get_border_top_left_symbol(&self) -> String { self.border_top_left_symbol.clone() }

    pub fn set_border_top_right_symbol(&mut self, symbol: String) {
        self.border_top_right_symbol = symbol
    }

    pub fn get_border_top_right_symbol(&self) -> String { self.border_top_right_symbol.clone() }

    pub fn has_border(&self) -> bool { true }

    pub fn set_border_foreground_color(&mut self, color: Color) {
        self.border_foreground_color = color;
        self.changed = true;
    }

    pub fn get_border_foreground_color(&self) -> Color { self.border_foreground_color }

    pub fn set_border_background_color(&mut self, color: Color) {
        self.border_background_color = color;
        self.changed = true;
    }

    pub fn get_border_background_color(&self) -> Color { self.border_background_color }

    pub fn set_content_foreground_color(&mut self, color: Color) {
        self.content_foreground_color = color;
        self.changed = true;
    }

    pub fn get_content_foreground_color(&self) -> Color { self.content_foreground_color }

    pub fn set_content_background_color(&mut self, color: Color) {
        self.content_background_color = color;
        self.changed = true;
    }

    pub fn get_content_background_color(&self) -> Color { self.content_background_color }

    pub fn set_selection_foreground_color(&mut self, color: Color) {
        self.selection_foreground_color = color;
        self.changed = true;
    }

    pub fn get_selection_foreground_color(&self) -> Color { self.selection_foreground_color }

    pub fn set_selection_background_color(&mut self, color: Color) {
        self.selection_background_color = color;
        self.changed = true;
    }

    pub fn get_selection_background_color(&self) -> Color {
        self.selection_background_color
    }

    pub fn set_flash_foreground_color(&mut self, color: Color) {
        self.flash_foreground_color = color;
        self.changed = true;
    }

    pub fn get_flash_foreground_color(&self) -> Color { self.flash_foreground_color }

    pub fn set_flash_background_color(&mut self, color: Color) {
        self.flash_background_color = color;
        self.changed = true;
    }

    pub fn get_flash_background_color(&self) -> Color { self.flash_background_color }
}