//! # Canvas Widget
//! Module defining a canvas widget, which does not generate any content but should be 'painted'
//! manually by the user using the 'set_content' method.
use std::fs::File;
use std::io::prelude::*;
use crate::widgets::widget::{EzWidget, EzObject, Pixel};
use crate::states::canvas_state::CanvasState;
use crate::states::state::{EzState, GenericState};
use crate::common::{PixelMap, StateTree};
use std::io::{Error, ErrorKind};
use unicode_segmentation::UnicodeSegmentation;
use crate::ez_parser::{load_color_parameter, load_size_hint_parameter, load_halign_parameter,
                       load_valign_parameter, load_bool_parameter, load_pos_hint_x_parameter, load_pos_hint_y_parameter};

pub struct CanvasWidget {
    /// ID of the widget, used to construct [path]
    pub id: String,

    /// Full path to this widget, e.g. "/root_layout/layout_2/THIS_ID"
    pub path: String,

    /// Optional file path to retrieve contents from
    pub from_file: Option<String>,

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
            contents: Vec::new(),
            state: CanvasState::default(),
        }
    }
}


impl EzObject for CanvasWidget {
    fn load_ez_parameter(&mut self, parameter_name: String, parameter_value: String)
                         -> Result<(), Error> {

        match parameter_name.as_str() {
            "x" => self.state.x = parameter_value.trim().parse().unwrap(),
            "y" => self.state.y = parameter_value.trim().parse().unwrap(),
            "size_hint_x" => self.state.size_hint_x =
                load_size_hint_parameter(parameter_value.trim()).unwrap(),
            "size_hint_y" => self.state.size_hint_y =
                load_size_hint_parameter(parameter_value.trim()).unwrap(),
            "pos_hint_x" => self.state.set_pos_hint_x(
                load_pos_hint_x_parameter(parameter_value.trim()).unwrap()),
            "pos_hint_y" => self.state.set_pos_hint_y(
                load_pos_hint_y_parameter(parameter_value.trim()).unwrap()),
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
            "from_file" => self.from_file = Some(parameter_value.trim().to_string()),
            _ => return Err(Error::new(ErrorKind::InvalidData,
                                format!("Invalid parameter name for canvas widget {}",
                                        parameter_name)))
        }
        Ok(())
    }

    fn set_id(&mut self, id: String) { self.id = id }

    fn get_id(&self) -> String { self.id.clone() }

    fn set_full_path(&mut self, path: String) { self.path = path }

    fn get_full_path(&self) -> String { self.path.clone() }

    fn update_state(&mut self, new_state: &EzState) {

        let state = new_state.as_canvas();
        self.state = state.clone();
        self.state.changed = false;
        self.state.force_redraw = false;
    }

    fn get_state(&self) -> EzState { EzState::CanvasWidget(self.state.clone()) }

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

        let state = state_tree.get_mut(&self.get_full_path()).unwrap().as_canvas_mut();
        if let Some(path) = self.from_file.clone() {
            let mut file = File::open(path).expect("Unable to open file");
            let mut file_content = String::new();
            file.read_to_string(&mut file_content).expect("Unable to read file");
            let mut lines: Vec<String> = file_content.lines()
                .map(|x| x.graphemes(true).rev().collect())
                .collect();

            if state.get_auto_scale_width() {
                let longest_line = lines.iter().map(|x| x.chars().count()).max();
                let auto_scale_width =
                    if let Some(i) = longest_line { i } else { 0 };
                if auto_scale_width < state.get_effective_width() {
                    state.set_effective_width(auto_scale_width);
                }
            }
            if state.get_auto_scale_height() {
                let auto_scale_height = lines.len();
                if auto_scale_height < state.get_effective_height() {
                    state.set_effective_height(auto_scale_height);
                }
            }

            let mut widget_content = PixelMap::new();
            for x in 0..state.get_effective_width() {
                widget_content.push(Vec::new());
                for y in 0..state.get_effective_height() {
                    if y < lines.len() && !lines[y].is_empty() {
                        widget_content[x].push(Pixel { symbol: lines[y].pop().unwrap().to_string(),
                            foreground_color: state.content_foreground_color,
                            background_color: state.content_background_color, underline: false})
                    } else {
                        widget_content[x].push(Pixel { symbol: " ".to_string(),
                            foreground_color: state.content_foreground_color,
                            background_color: state.content_background_color, underline: false})
                    }
                }
            }
            widget_content
        } else {
            self.contents.clone()
        }
    }
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
