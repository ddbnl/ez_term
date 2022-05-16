//! A widget that displays text non-interactively.
use std::io::{Error, ErrorKind};
use crossterm::style::{Color};
use crate::common::{Coordinates, PixelMap};
use crate::widgets::widget::{EzWidget, Pixel, EzObject};
use crate::widgets::widget_state::{WidgetState, RedrawWidgetState};
use crate::ez_parser::{load_color_parameter, load_bool_parameter};

pub struct Label {

    /// ID of the widget, used to construct [path]
    pub id: String,

    /// Full path to this widget, e.g. "/root_layout/layout_2/THIS_ID"
    pub path: String,

    /// Horizontal position of this widget relative to its' parent [Layout]
    pub x: usize,

    /// Vertical position of this widget relative to its' parent [Layout]
    pub y: usize,

    /// Width of this widget
    pub width: usize,

    /// Absolute position of this layout on screen. Automatically propagated, do not set manually
    pub absolute_position: Coordinates,

    /// Bool representing whether this widget should have a surrounding border
    pub border: bool,

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

    /// Runtime state of this widget, see [LabelState] and [WidgetState]
    pub state: LabelState,
}

impl Default for Label {
    fn default() -> Self {
        Label {
            id: "".to_string(),
            path: String::new(),
            x: 0,
            y: 0,
            width: 0,
            absolute_position: (0, 0),
            border: false,
            border_horizontal_symbol: "━".to_string(),
            border_vertical_symbol: "│".to_string(),
            border_top_left_symbol: "┌".to_string(),
            border_top_right_symbol: "┐".to_string(),
            border_bottom_left_symbol: "└".to_string(),
            border_bottom_right_symbol: "┘".to_string(),
            border_foreground_color: Color::White,
            border_background_color: Color::Black,
            content_background_color: Color::Black,
            content_foreground_color: Color::White,
            state: LabelState {text: String::new(), force_redraw: false},
        }
    }
}


/// [WidgetState] implementation.
#[derive(Clone)]
pub struct LabelState {

    /// Text currently being displayed by the label
    pub text: String,

    /// If true this forces a global screen redraw on the next frame. Screen redraws are diffed
    /// so this can be called when needed without degrading performance. If only screen positions
    /// that fall within this widget must be redrawn, call [EzObject.redraw] instead.
    pub force_redraw: bool,
}
impl RedrawWidgetState for LabelState {
    fn set_force_redraw(&mut self, redraw: bool) { self.force_redraw = redraw }
    fn get_force_redraw(&self) -> bool { self.force_redraw }
}

impl EzObject for Label {

    fn load_ez_parameter(&mut self, parameter_name: String, mut parameter_value: String)
                         -> Result<(), Error> {
        match parameter_name.as_str() {
            "x" => self.x = parameter_value.trim().parse().unwrap(),
            "y" => self.y = parameter_value.trim().parse().unwrap(),
            "width" => self.width = parameter_value.trim().parse().unwrap(),
            "contentForegroundColor" =>
                self.content_foreground_color = load_color_parameter(parameter_value).unwrap(),
            "contentBackgroundColor" =>
                self.content_background_color = load_color_parameter(parameter_value).unwrap(),
            "text" => {
                if parameter_value.starts_with(' ') {
                    parameter_value = parameter_value.strip_prefix(' ').unwrap().to_string();
                }
                self.state.text = parameter_value
            },
            "border" => self.set_border(load_bool_parameter(parameter_value.trim())?),
            "borderHorizontalSymbol" => self.border_horizontal_symbol =
                parameter_value.trim().to_string(),
            "borderVerticalSymbol" => self.border_vertical_symbol =
                parameter_value.trim().to_string(),
            "borderTopRightSymbol" => self.border_top_right_symbol =
                parameter_value.trim().to_string(),
            "borderTopLeftSymbol" => self.border_top_left_symbol =
                parameter_value.trim().to_string(),
            "borderBottomLeftSymbol" => self.border_bottom_left_symbol =
                parameter_value.trim().to_string(),
            "borderBottomRightSymbol" => self.border_bottom_right_symbol =
                parameter_value.trim().to_string(),
            "borderForegroundColor" =>
                self.border_foreground_color = load_color_parameter(parameter_value).unwrap(),
            "borderBackgroundColor" =>
                self.border_background_color = load_color_parameter(parameter_value).unwrap(),
            _ => return Err(Error::new(ErrorKind::InvalidData,
                                       format!("Invalid parameter name for text box {}",
                                               parameter_name)))
        }
        Ok(())
    }

    fn set_id(&mut self, id: String) { self.id = id }

    fn get_id(&self) -> String { self.id.clone() }

    fn set_full_path(&mut self, path: String) {
        self.path = path
    }

    fn get_full_path(&self) -> String {
        self.path.clone()
    }

    fn get_contents(&mut self) -> PixelMap {
        let mut text = self.state.text.clone().chars().rev().collect::<String>();
        let mut contents = Vec::new();
        for _ in 0..self.get_width() {
            let mut new_y = Vec::new();
            for _ in 0..self.get_height() {
                if !text.is_empty() {
                    new_y.push(Pixel {
                        symbol: text.pop().unwrap().to_string(),
                        foreground_color: self.get_content_foreground_color(),
                        background_color: self.get_content_background_color(),
                        underline: false
                    })
                } else {
                    new_y.push(Pixel {
                        symbol: " ".to_string(),
                        foreground_color: self.get_content_foreground_color(),
                        background_color: self.get_content_background_color(),
                        underline: false
                    })
                }
            }
            contents.push(new_y);
        }
        contents
    }

    fn get_width(&self) -> usize { self.width }

    fn get_height(&self) -> usize { 1 }

    fn set_position(&mut self, position: Coordinates) {
        self.x = position.0;
        self.y = position.1;
    }

    fn get_position(&self) -> Coordinates { (self.x, self.y) }

    fn set_absolute_position(&mut self, pos: Coordinates) {
        self.absolute_position = pos
    }

    fn get_absolute_position(&self) -> Coordinates {
        self.absolute_position
    }

    fn set_border_horizontal_symbol(&mut self, symbol: String) {
        self.border_horizontal_symbol = symbol
    }

    fn get_border_horizontal_symbol(&self) -> String { self.border_horizontal_symbol.clone() }

    fn set_border_vertical_symbol(&mut self, symbol: String) {
        self.border_vertical_symbol = symbol
    }

    fn get_border_vertical_symbol(&self) -> String { self.border_vertical_symbol.clone() }

    fn set_border_bottom_left_symbol(&mut self, symbol: String) {
        self.border_bottom_left_symbol = symbol
    }

    fn get_border_bottom_left_symbol(&self) -> String { self.border_bottom_left_symbol.clone() }

    fn set_border_bottom_right_symbol(&mut self, symbol: String) {
        self.border_bottom_right_symbol = symbol
    }

    fn get_border_bottom_right_symbol(&self) -> String { self.border_bottom_right_symbol.clone() }

    fn set_border_top_left_symbol(&mut self, symbol: String) {
        self.border_top_left_symbol = symbol
    }

    fn get_border_top_left_symbol(&self) -> String { self.border_top_left_symbol.clone() }

    fn set_border_top_right_symbol(&mut self, symbol: String) {
        self.border_top_right_symbol = symbol
    }

    fn get_border_top_right_symbol(&self) -> String { self.border_top_right_symbol.clone() }

    fn set_border_foreground_color(&mut self, color: Color) { self.border_foreground_color = color }

    fn get_border_foreground_color(&self) -> Color { self.border_foreground_color }

    fn set_border_background_color(&mut self, color: Color) { self.border_background_color = color }

    fn get_border_background_color(&self) -> Color { self.border_background_color }

    fn set_border(&mut self, enabled: bool) { self.border = enabled }

    fn has_border(&self) -> bool { self.border }
}


impl EzWidget for Label {

    fn get_state(&self) -> WidgetState {
        WidgetState::Label(self.state.clone())
    }

    fn set_content_foreground_color(&mut self, color: Color) {
        self.content_foreground_color = color }

    fn get_content_foreground_color(&self) -> Color { self.content_foreground_color }

    fn set_content_background_color(&mut self, color: Color) {
        self.content_background_color = color }
    fn get_content_background_color(&self) -> Color { self.content_background_color }

    fn state_changed(&self, other_state: &WidgetState) -> bool {
        let state = other_state.as_label();
        if state.text != self.state.text { return true }
        false
    }

    fn update_state(&mut self, new_state: &WidgetState) {
        let state = new_state.as_label();
        self.state = state.clone();
        self.state.force_redraw = false;
    }
}

impl Label {

    /// Initialize an instance of this object using the passed config coming from [ez_parser]
    pub fn from_config(config: Vec<&str>, id: String) -> Self {
        let mut obj = Label::default();
        obj.set_id(id);
        obj.load_ez_config(config).unwrap();
        obj
    }
}
