//! A widget that displays text non-interactively.
use std::fs::File;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use crossterm::style::{Color};
use crate::common;
use crate::common::{Coordinates, PixelMap, StateTree};
use crate::widgets::widget::{Pixel, EzObject, EzWidget};
use crate::widgets::state::{State, GenericState, HorizontalAlignment, VerticalAlignment};
use crate::ez_parser::{load_color_parameter, load_bool_parameter, load_size_hint,
                       load_halign_parameter, load_valign_parameter};

pub struct Label {

    /// ID of the widget, used to construct [path]
    pub id: String,

    /// Full path to this widget, e.g. "/root_layout/layout_2/THIS_ID"
    pub path: String,

    /// Absolute position of this layout on screen. Automatically propagated, do not set manually
    pub absolute_position: Coordinates,

    /// Optional file path to retrieve text from
    pub from_file: Option<String>,

    /// Runtime state of this widget, see [LabelState] and [State]
    pub state: LabelState,
}

impl Default for Label {
    fn default() -> Self {
        Label {
            id: "".to_string(),
            path: String::new(),
            absolute_position: (0, 0),
            from_file: None,
            state: LabelState::default(),
        }
    }
}


/// [State] implementation.
#[derive(Clone)]
pub struct LabelState {

    /// Text currently being displayed by the label
    pub text: String,

    /// Horizontal position of this widget relative to its' parent [Layout]
    pub x: usize,

    /// Vertical position of this widget relative to its' parent [Layout]
    pub y: usize,

    /// Width of this widget
    pub size_hint_x: Option<f64>,

    /// Width of this widget
    pub size_hint_y: Option<f64>,

    /// Width of this widget
    pub width: usize,

    /// Height of this widget
    pub height: usize,

    /// Automatically adjust width of widget to content
    pub auto_scale_width: bool,

    /// Automatically adjust width of widget to content
    pub auto_scale_height: bool,

    /// Horizontal alignment of this widget
    pub halign: HorizontalAlignment,

    /// Vertical alignment of this widget
    pub valign: VerticalAlignment,

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

    /// Bool representing if state has changed. Triggers widget redraw.
    pub changed: bool,

    /// If true this forces a global screen redraw on the next frame. Screen redraws are diffed
    /// so this can be called when needed without degrading performance. If only screen positions
    /// that fall within this widget must be redrawn, call [EzObject.redraw] instead.
    pub force_redraw: bool,
}
impl Default for LabelState {
    fn default() -> Self {
       LabelState {
           x: 0,
           y: 0,
           size_hint_x: Some(1.0),
           size_hint_y: Some(1.0),
           auto_scale_width: false,
           auto_scale_height: false,
           width: 0,
           height: 0,
           halign: HorizontalAlignment::Left,
           valign: VerticalAlignment::Top,
           text: String::new(),
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
           changed: false,
           force_redraw: false
       }
    }
}
impl GenericState for LabelState {

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

    fn set_effective_width(&mut self, width: usize) {
       self.set_width(width +if self.has_border() {2} else {0})
    }

    fn get_effective_width(&self) -> usize {self.get_width() -if self.has_border() {2} else {0} }

    fn set_height(&mut self, height: usize) { self.height = height; self.changed = true; }

    fn get_height(&self) -> usize { self.height }

    fn set_effective_height(&mut self, height: usize) {
        self.set_height(height +if self.has_border() {2} else {0})
    }

    fn get_effective_height(&self) -> usize {self.get_height() -if self.has_border() {2} else {0} }

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
impl LabelState {

    pub fn set_text(&mut self, text: String) {
        self.text = text;
        self.changed = true;
    }

    pub fn get_text(&self) -> String { self.text.clone() }


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

    pub fn set_border(&mut self, enabled: bool) { self.border = enabled }

    pub fn has_border(&self) -> bool { self.border }

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

}

impl EzObject for Label {

    fn load_ez_parameter(&mut self, parameter_name: String, mut parameter_value: String)
                         -> Result<(), Error> {
        
        match parameter_name.as_str() {
            "x" => self.state.x = parameter_value.trim().parse().unwrap(),
            "y" => self.state.y = parameter_value.trim().parse().unwrap(),
            "size_hint_x" => self.state.size_hint_x =
                load_size_hint(parameter_value.trim()).unwrap(),
            "size_hint_y" => self.state.size_hint_y =
                load_size_hint(parameter_value.trim()).unwrap(),
            "width" => self.state.width = parameter_value.trim().parse().unwrap(),
            "height" => self.state.height = parameter_value.trim().parse().unwrap(),
            "auto_scale_width" =>
                self.state.set_auto_scale_width(load_bool_parameter(parameter_value.trim())?),
            "auto_scale_height" =>
                self.state.set_auto_scale_height(load_bool_parameter(parameter_value.trim())?),
            "halign" =>
                self.state.halign =  load_halign_parameter(parameter_value.trim()).unwrap(),
            "valign" =>
                self.state.valign =  load_valign_parameter(parameter_value.trim()).unwrap(),
            "fg_color" =>
                self.state.content_foreground_color = load_color_parameter(parameter_value).unwrap(),
            "bg_color" =>
                self.state.content_background_color = load_color_parameter(parameter_value).unwrap(),
            "text" => {
                if parameter_value.starts_with(' ') {
                    parameter_value = parameter_value.strip_prefix(' ').unwrap().to_string();
                }
                self.state.text = parameter_value
            },
            "from_file" => self.from_file = Some(parameter_value.trim().to_string()),
            "border" => self.state.set_border(load_bool_parameter(parameter_value.trim())?),
            "border_horizontal_symbol" => self.state.border_horizontal_symbol =
                parameter_value.trim().to_string(),
            "border_vertical_symbol" => self.state.border_vertical_symbol =
                parameter_value.trim().to_string(),
            "border_top_right_symbol" => self.state.border_top_right_symbol =
                parameter_value.trim().to_string(),
            "border_top_left_symbol" => self.state.border_top_left_symbol =
                parameter_value.trim().to_string(),
            "border_bottom_left_symbol" => self.state.border_bottom_left_symbol =
                parameter_value.trim().to_string(),
            "border_bottom_right_symbol" => self.state.border_bottom_right_symbol =
                parameter_value.trim().to_string(),
            "border_fg_color" =>
                self.state.border_foreground_color = load_color_parameter(parameter_value).unwrap(),
            "border_bg_color" =>
                self.state.border_background_color = load_color_parameter(parameter_value).unwrap(),
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

    fn update_state(&mut self, new_state: &State) {
        let state = new_state.as_label();
        self.state = state.clone();
        self.state.changed = false;
        self.state.force_redraw = false;
    }

    fn get_state(&self) -> State { State::Label(self.state.clone()) }

    fn get_contents(&self, state_tree: &mut StateTree) -> PixelMap {

        let state = state_tree.get_mut(&self.get_full_path()).unwrap().as_label_mut();
        let mut text;
        // Load text from file
        if let Some(path) = self.from_file.clone() {
            let mut file = File::open(path).expect("Unable to open file");
            text = String::new();
            file.read_to_string(&mut text).expect("Unable to read file");
        // or take text from widget state
        } else {
            text = state.text.clone();
        }


        let content_lines = common::wrap_text(text, state.get_effective_width());
        // If user wants to autoscale width we set width to the longest line
        if state.get_auto_scale_width() {
            let longest_line = content_lines.iter().map(|x| x.len()).max();
            let auto_scale_width =
                if let Some(i) = longest_line { i } else { 0 };
            if auto_scale_width < state.get_effective_width() {
                state.set_effective_width(auto_scale_width);
            }
        }
        // If user wants to autoscale height we set height to the amount of lines we generated
        if state.get_auto_scale_height() {
            let auto_scale_height = content_lines.len();
            if auto_scale_height < state.get_effective_height() {
                state.set_effective_height(auto_scale_height);
            }
        }

        // Now we'll create the actual PixelMap using the lines we've created by wrapping the text
        let mut contents = Vec::new();
        for x in 0..state.get_effective_width() {
            let mut new_y = Vec::new();
            for y in 0..state.get_effective_height() {
                if y < content_lines.len() && x < content_lines[y].len() {
                    new_y.push(Pixel {
                        symbol: content_lines[y][x..x+1].to_string(),
                        foreground_color: state.content_foreground_color,
                        background_color: state.content_background_color,
                        underline: false
                    })
                } else {
                    new_y.push(Pixel {
                        symbol: " ".to_string(),
                        foreground_color: state.content_foreground_color,
                        background_color: state.content_background_color,
                        underline: false
                    })
                }
            }
            contents.push(new_y);
        }
        if state.has_border() {
            contents = common::add_border(
                contents, state.border_horizontal_symbol.clone(),
                state.border_vertical_symbol.clone(),
                state.border_top_left_symbol.clone(),
                state.border_top_right_symbol.clone(),
                state.border_bottom_left_symbol.clone(),
                state.border_bottom_right_symbol.clone(),
                state.border_background_color,
                state.border_foreground_color)
        }
        contents
    }

    fn set_absolute_position(&mut self, pos: Coordinates) { self.absolute_position = pos }

    fn get_absolute_position(&self) -> Coordinates { self.absolute_position }

    fn get_effective_absolute_position(&self) -> Coordinates {
        let (x, y) = self.get_absolute_position();
        (x +if self.state.has_border() {1} else {0}, y +if self.state.has_border() {1} else {0})
    }
}

impl EzWidget for Label {}

impl Label {

    /// Initialize an instance of this object using the passed config coming from [ez_parser]
    pub fn from_config(config: Vec<&str>, id: String) -> Self {
        let mut obj = Label::default();
        obj.set_id(id);
        obj.load_ez_config(config).unwrap();
        obj
    }
}
