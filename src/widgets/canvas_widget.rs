//! # Canvas Widget
//! Module defining a canvas widget, which does not generate any content but should be 'painted'
//! manually by the user using the 'set_content' method.
use std::fs::File;
use std::io::prelude::*;
use crate::widgets::widget::{EzWidget, EzObject, Pixel};
use crate::widgets::widget_state::{WidgetState, RedrawWidgetState};
use crate::common::{Coordinates, PixelMap};
use std::io::{Error, ErrorKind};
use crossterm::style::{Color, Stylize};
use unicode_segmentation::UnicodeSegmentation;
use crate::ez_parser::load_color_parameter;

pub struct CanvasWidget {
    /// ID of the widget, used to construct [path]
    pub id: String,

    /// Full path to this widget, e.g. "/root_layout/layout_2/THIS_ID"
    pub path: String,

    /// Width of this widget
    pub width: usize,

    /// Height of this widget
    pub height: usize,

    /// Horizontal position of this widget relative to its' parent [Layout]
    pub x: usize,

    /// Vertical position of this widget relative to its' parent [Layout]
    pub y: usize,

    /// Optional file path to retrieve contents from
    pub from_file: Option<String>,

    /// Absolute position of this widget on screen. Automatically propagated, do not set manually
    pub absolute_position: Coordinates,

    /// Grid of pixels that will be written to screen for this widget
    pub contents: PixelMap,

    /// Runtime state of this widget, see [CanvasState] and [WidgetState]
    pub state: CanvasState,
}


impl Default for CanvasWidget {

    fn default() -> Self {

        CanvasWidget {
            id: "".to_string(),
            path: String::new(),
            width: 0,
            height: 0,
            x: 0,
            y: 0,
            from_file: None,
            absolute_position: (0, 0),
            contents: Vec::new(),
            state: CanvasState{force_redraw: false, content_foreground_color: Color::White,
            content_background_color: Color::Black },
        }
    }
}


/// [WidgetState] implementation.
#[derive(Clone)]
pub struct CanvasState {

    /// If true this forces a global screen redraw on the next frame. Screen redraws are diffed
    /// so this can be called when needed without degrading performance. If only screen positions
    /// that fall within this widget must be redrawn, call [EzObject.redraw] instead.
    pub force_redraw: bool,

    /// The [Pixel.foreground_color] to use for this widgets' content
    pub content_foreground_color: Color,

    /// The [Pixel.background_color] to use for this widgets' content
    pub content_background_color: Color,
}
impl RedrawWidgetState for CanvasState {
    fn set_force_redraw(&mut self, redraw: bool) { self.force_redraw = redraw }
    fn get_force_redraw(&self) -> bool { self.force_redraw }
}


impl EzObject for CanvasWidget {

    fn load_ez_parameter(&mut self, parameter_name: String, parameter_value: String)
                         -> Result<(), Error> {

        match parameter_name.as_str() {
            "x" => self.x = parameter_value.trim().parse().unwrap(),
            "y" => self.y = parameter_value.trim().parse().unwrap(),
            "width" => self.width = parameter_value.trim().parse().unwrap(),
            "height" => self.height = parameter_value.trim().parse().unwrap(),
            "contentForegroundColor" =>
                self.state.content_foreground_color = load_color_parameter(parameter_value).unwrap(),
            "contentBackgroundColor" =>
                self.state.content_background_color = load_color_parameter(parameter_value).unwrap(),
            "fromFile" => self.from_file = Some(parameter_value.trim().to_string()),
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

    /// Set the content of this Widget. You must manually fill a [PixelMap] of the same
    /// [height] and [width] as this widget and pass it here.
    fn set_contents(&mut self, contents: PixelMap) {
       let mut valid_contents = Vec::new();
       for x in 0..self.width as usize {
           valid_contents.push(Vec::new());
           for y in 0..self.height as usize {
               valid_contents[x].push(contents[x][y].clone())
           }
       }
       self.contents = valid_contents
    }

    fn get_contents(&mut self) -> PixelMap {
        if let Some(path) = self.from_file.clone() {
            let mut file = File::open(path).expect("Unable to open file");
            let mut file_content = String::new();
            file.read_to_string(&mut file_content).expect("Unable to read file");
            let mut lines: Vec<String> = file_content.lines()
                .map(|x| x.graphemes(true).rev().collect())
                .collect();
            let mut widget_content = PixelMap::new();
            for x in 0..self.get_width() {
                widget_content.push(Vec::new());
                for y in 0..self.get_height() {
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

    fn set_width(&mut self, width: usize) { self.width = width; }

    fn get_width(&self) -> usize { self.width }

    fn set_height(&mut self, height: usize) { self.height = height; }

    fn get_height(&self) -> usize { self.height }

    fn set_position(&mut self, position: Coordinates) {
        self.x = position.0;
        self.y = position.1;
    }

    fn get_position(&self) -> Coordinates { (self.x, self.y) }

    fn set_absolute_position(&mut self, pos: Coordinates) { self.absolute_position = pos }

    fn get_absolute_position(&self) -> Coordinates { self.absolute_position }
}

impl EzWidget for CanvasWidget {

    fn get_state(&self) -> WidgetState {
        WidgetState::CanvasWidget(self.state.clone())
    }

    fn set_content_foreground_color(&mut self, color: Color) { self.state.content_foreground_color = color }

    fn get_content_foreground_color(&self) -> Color { self.state.content_foreground_color }

    fn set_content_background_color(&mut self, color: Color) { self.state.content_background_color = color }

    fn get_content_background_color(&self) -> Color { self.state.content_background_color }
    fn state_changed(&self, other_state: &WidgetState) -> bool {
        let state = other_state.as_canvas();
        if state.content_foreground_color != self.state.content_foreground_color { return true }
        if state.content_background_color != self.state.content_background_color { return true }
        false
    }
    fn update_state(&mut self, new_state: &WidgetState) {

        let state = new_state.as_canvas();
        self.state = state.clone();
        self.state.force_redraw = false;
    }
}

impl CanvasWidget {

    /// Load this widget from a config vector coming from [ez_parser]
    pub fn from_config(config: Vec<&str>, id: String) -> Self {
        let mut obj = CanvasWidget::default();
        obj.set_id(id);
        obj.load_ez_config(config).unwrap();
        obj
    }
}
