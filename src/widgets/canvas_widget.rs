//! # Canvas Widget
//! Module defining a canvas widget, which does not generate any content but should be 'painted'
//! manually by the user using the 'set_content' method.
use std::fs::File;
use std::io::prelude::*;
use crate::widgets::widget::{EzWidget, EzObject, Pixel};
use crate::widgets::state::{State, GenericState};
use crate::common::{Coordinates, PixelMap, StateTree};
use std::io::{Error, ErrorKind};
use crossterm::style::{Color};
use unicode_segmentation::UnicodeSegmentation;
use crate::ez_parser::{load_color_parameter, load_size_hint};

pub struct CanvasWidget {
    /// ID of the widget, used to construct [path]
    pub id: String,

    /// Full path to this widget, e.g. "/root_layout/layout_2/THIS_ID"
    pub path: String,

    /// Optional file path to retrieve contents from
    pub from_file: Option<String>,

    /// Absolute position of this widget on screen. Automatically propagated, do not set manually
    pub absolute_position: Coordinates,

    /// Grid of pixels that will be written to screen for this widget
    pub contents: PixelMap,

    /// Runtime state of this widget, see [CanvasState] and [State]
    pub state: CanvasState,
}


impl Default for CanvasWidget {

    fn default() -> Self {

        CanvasWidget {
            id: "".to_string(),
            path: String::new(),
            from_file: None,
            absolute_position: (0, 0),
            contents: Vec::new(),
            state: CanvasState::default(),
        }
    }
}


/// [State] implementation.
#[derive(Clone)]
pub struct CanvasState {

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
impl Default for CanvasState {
    fn default() -> Self {
        CanvasState{
            x: 0,
            y: 0,
            size_hint_x: Some(1.0),
            size_hint_y: Some(1.0),
            width: 0,
            height: 0,
            content_foreground_color: Color::White,
            content_background_color: Color::Black,
            changed: false,
            force_redraw: false,
        }
    }
}
impl GenericState for CanvasState {

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

    fn set_width(&mut self, width: usize) { self.width = width; self.changed = true; }

    fn get_width(&self) -> usize { self.width }

    fn set_height(&mut self, height: usize) { self.height = height; self.changed = true; }

    fn get_height(&self) -> usize { self.height }

    fn set_position(&mut self, position: Coordinates) {
        self.x = position.0;
        self.y = position.1;
        self.changed = true;
    }

    fn get_position(&self) -> Coordinates { (self.x, self.y) }

    fn set_force_redraw(&mut self, redraw: bool) {
        self.force_redraw = redraw;
        self.changed = true;
    }

    fn get_force_redraw(&self) -> bool { self.force_redraw }
}
impl CanvasState {

    pub fn set_content_foreground_color(&mut self, color: Color) {
        self.content_foreground_color = color;
        self.changed = true;
    }

    pub fn get_content_foreground_color(&self) -> Color {
        self.content_foreground_color
    }

    pub fn set_content_background_color(&mut self, color: Color) {
        self.content_background_color = color;
        self.changed = true;
    }

    pub fn get_content_background_color(&self) -> Color {
        self.content_background_color
    }

}

impl EzObject for CanvasWidget {
    fn load_ez_parameter(&mut self, parameter_name: String, parameter_value: String)
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
            "fg_color" =>
                self.state.content_foreground_color = load_color_parameter(parameter_value).unwrap(),
            "bg_color" =>
                self.state.content_background_color = load_color_parameter(parameter_value).unwrap(),
            "from_file" => self.from_file = Some(parameter_value.trim().to_string()),
            _ => return Err(Error::new(ErrorKind::InvalidData,
                                format!("Invalid parameter name for canvas widget {}",
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

        let state = new_state.as_canvas();
        self.state = state.clone();
        self.state.changed = false;
        self.state.force_redraw = false;
    }

    fn get_state(&self) -> State {
        State::CanvasWidget(self.state.clone())
    }

    /// Set the content of this Widget. You must manually fill a [PixelMap] of the same
    /// [height] and [width] as this widget and pass it here.
    fn set_contents(&mut self, contents: PixelMap) {
       let mut valid_contents = Vec::new();
       for x in 0..self.state.width as usize {
           valid_contents.push(Vec::new());
           for y in 0..self.state.height as usize {
               valid_contents[x].push(contents[x][y].clone())
           }
       }
       self.contents = valid_contents
    }

    fn get_contents(&self, state_tree: &mut StateTree) -> PixelMap {

        let state = state_tree.get(&self.get_full_path()).unwrap().as_canvas();
        if let Some(path) = self.from_file.clone() {
            let mut file = File::open(path).expect("Unable to open file");
            let mut file_content = String::new();
            file.read_to_string(&mut file_content).expect("Unable to read file");
            let mut lines: Vec<String> = file_content.lines()
                .map(|x| x.graphemes(true).rev().collect())
                .collect();
            let mut widget_content = PixelMap::new();
            for x in 0..state.get_width() {
                widget_content.push(Vec::new());
                for y in 0..state.get_height() {
                    if y < lines.len() && !lines[y].is_empty() {
                        widget_content[x].push(Pixel { symbol: lines[y].pop().unwrap().to_string(),
                            foreground_color: self.state.content_foreground_color,
                            background_color: self.state.content_background_color, underline: false})
                    } else {
                        widget_content[x].push(Pixel { symbol: " ".to_string(),
                            foreground_color: self.state.content_foreground_color,
                            background_color: self.state.content_background_color, underline: false})
                    }
                }
            }
            widget_content
        } else {
            self.contents.clone()
        }
    }

    fn set_absolute_position(&mut self, pos: Coordinates) { self.absolute_position = pos }

    fn get_absolute_position(&self) -> Coordinates { self.absolute_position }

}

impl EzWidget for CanvasWidget {}

impl CanvasWidget {

    /// Load this widget from a config vector coming from [ez_parser]
    pub fn from_config(config: Vec<&str>, id: String) -> Self {
        let mut obj = CanvasWidget::default();
        obj.set_id(id);
        obj.load_ez_config(config).unwrap();
        obj
    }
}
