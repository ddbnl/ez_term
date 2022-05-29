//! # Canvas Widget
//! Module defining a canvas widget, which does not generate any content but should be 'painted'
//! manually by the user using the 'set_content' method.
use std::fs::File;
use std::io::prelude::*;
use crate::widgets::widget::{EzWidget, EzObject, Pixel};
use crate::states::canvas_state::CanvasState;
use crate::states::state::{self, GenericState};
use crate::common;
use std::io::{Error, ErrorKind};
use unicode_segmentation::UnicodeSegmentation;
use crate::ez_parser;

#[derive(Clone)]
pub struct CanvasWidget {
    /// ID of the widget, used to construct [path]
    pub id: String,

    /// Full path to this widget, e.g. "/root_layout/layout_2/THIS_ID"
    pub path: String,

    /// Optional file path to retrieve contents from
    pub from_file: Option<String>,

    /// Grid of pixels that will be written to screen for this widget
    pub contents: common::PixelMap,

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
            "id" => self.set_id(parameter_value.trim().to_string()),
            "x" => self.state.set_x(parameter_value.trim().parse().unwrap()),
            "y" => self.state.set_y(parameter_value.trim().parse().unwrap()),
            "pos" => self.state.set_position(
                ez_parser::load_pos_parameter(parameter_value.trim()).unwrap()),
            "size_hint" => self.state.set_size_hint(
                ez_parser::load_full_size_hint_parameter(parameter_value.trim()).unwrap()),
            "size_hint_x" => self.state.set_size_hint_x(
                ez_parser::load_size_hint_parameter(parameter_value.trim()).unwrap()),
            "size_hint_y" => self.state.set_size_hint_y(
                ez_parser::load_size_hint_parameter(parameter_value.trim()).unwrap()),
            "pos_hint" => self.state.set_pos_hint(
                ez_parser::load_full_pos_hint_parameter(parameter_value.trim()).unwrap()),
            "pos_hint_x" => self.state.set_pos_hint_x(
                ez_parser::load_pos_hint_x_parameter(parameter_value.trim()).unwrap()),
            "pos_hint_y" => self.state.set_pos_hint_y(
                ez_parser::load_pos_hint_y_parameter(parameter_value.trim()).unwrap()),
            "size" => self.state.set_size(
                ez_parser::load_size_parameter(parameter_value.trim()).unwrap()),
            "width" => self.state.set_width(parameter_value.trim().parse().unwrap()),
            "height" => self.state.set_height(parameter_value.trim().parse().unwrap()),
            "auto_scale" => self.state.set_auto_scale(ez_parser::load_full_auto_scale_parameter(
                parameter_value.trim())?),
            "auto_scale_width" =>
                self.state.set_auto_scale_width(ez_parser::load_bool_parameter(parameter_value.trim())?),
            "auto_scale_height" =>
                self.state.set_auto_scale_height(ez_parser::load_bool_parameter(parameter_value.trim())?),
            "padding" => self.state.set_padding(ez_parser::load_full_padding_parameter(
                parameter_value.trim())?),
            "padding_x" => self.state.set_padding(ez_parser::load_padding_x_parameter(
                parameter_value.trim())?),
            "padding_y" => self.state.set_padding(ez_parser::load_padding_y_parameter(
                parameter_value.trim())?),
            "padding_top" => self.state.set_padding_top(parameter_value.trim().parse().unwrap()),
            "padding_bottom" => self.state.set_padding_bottom(parameter_value.trim().parse().unwrap()),
            "padding_left" => self.state.set_padding_left(parameter_value.trim().parse().unwrap()),
            "padding_right" => self.state.set_padding_right(parameter_value.trim().parse().unwrap()),
            "halign" =>
                self.state.halign =  ez_parser::load_halign_parameter(parameter_value.trim()).unwrap(),
            "valign" =>
                self.state.valign =  ez_parser::load_valign_parameter(parameter_value.trim()).unwrap(),
            "border" => self.state.set_border(ez_parser::load_bool_parameter(parameter_value.trim())?),
            "border_horizontal_symbol" => self.state.border_config.horizontal_symbol =
                parameter_value.trim().to_string(),
            "border_vertical_symbol" => self.state.border_config.vertical_symbol =
                parameter_value.trim().to_string(),
            "border_top_right_symbol" => self.state.border_config.top_right_symbol =
                parameter_value.trim().to_string(),
            "border_top_left_symbol" => self.state.border_config.top_left_symbol =
                parameter_value.trim().to_string(),
            "border_bottom_left_symbol" => self.state.border_config.bottom_left_symbol =
                parameter_value.trim().to_string(),
            "border_bottom_right_symbol" => self.state.border_config.bottom_right_symbol =
                parameter_value.trim().to_string(),
            "border_fg_color" =>
                self.state.border_config.fg_color = ez_parser::load_color_parameter(parameter_value).unwrap(),
            "border_bg_color" =>
                self.state.border_config.bg_color = ez_parser::load_color_parameter(parameter_value).unwrap(),
            "fg_color" =>
                self.state.colors.foreground = ez_parser::load_color_parameter(parameter_value).unwrap(),
            "bg_color" =>
                self.state.colors.background = ez_parser::load_color_parameter(parameter_value).unwrap(),
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

    fn update_state(&mut self, new_state: &state::EzState) {

        let state = new_state.as_canvas();
        self.state = state.clone();
        self.state.changed = false;
        self.state.force_redraw = false;
    }

    fn get_state(&self) -> state::EzState { state::EzState::CanvasWidget(self.state.clone()) }

    /// Set the content of this Widget. You must manually fill a [PixelMap] of the same
    /// [height] and [width] as this widget and pass it here.
    fn set_contents(&mut self, contents: common::PixelMap) {
       let mut valid_contents = Vec::new();
       for x in 0..self.state.get_size().width as usize {
           valid_contents.push(Vec::new());
           for y in 0..self.state.get_size().height as usize {
               valid_contents[x].push(contents[x][y].clone())
           }
       }
       self.contents = valid_contents
    }

    fn get_contents(&self, state_tree: &mut common::StateTree) -> common::PixelMap {

        let state = state_tree
            .get_mut(&self.get_full_path()).unwrap().as_canvas_mut();
        let mut contents;
        if let Some(path) = self.from_file.clone() {
            let mut file = File::open(path).expect("Unable to open file");
            let mut file_content = String::new();
            file.read_to_string(&mut file_content).expect("Unable to read file");
            let mut lines: Vec<String> = file_content.lines()
                .map(|x| x.graphemes(true).rev().collect())
                .collect();

            if state.get_auto_scale().width {
                let longest_line = lines.iter().map(|x| x.chars().count()).max();
                let auto_scale_width =
                    if let Some(i) = longest_line { i } else { 0 };
                if auto_scale_width < state.get_effective_size().width {
                    state.set_effective_width(auto_scale_width);
                }
            }
            if state.get_auto_scale().height {
                let auto_scale_height = lines.len();
                if auto_scale_height < state.get_effective_size().height {
                    state.set_effective_height(auto_scale_height);
                }
            }

            let mut widget_content = common::PixelMap::new();
            for x in 0..state.get_effective_size().width {
                widget_content.push(Vec::new());
                for y in 0..state.get_effective_size().height {
                    if y < lines.len() && !lines[y].is_empty() {
                        widget_content[x].push(Pixel {
                            symbol: lines[y].pop().unwrap().to_string(),
                            foreground_color: state.get_colors().foreground,
                            background_color: state.get_colors().background,
                            underline: false})
                    } else {
                        widget_content[x].push(Pixel {
                            symbol: " ".to_string(),
                            foreground_color: state.get_colors().foreground,
                            background_color: state.get_colors().background,
                            underline: false})
                    }
                }
            }
            contents = widget_content;
        } else {
            contents = self.contents.clone();
        }
        if state.has_border() {
            contents = common::add_border(contents, state.get_border_config());
        }
        let state = state_tree.get(&self.get_full_path()).unwrap().as_canvas();
        let parent_colors = state_tree.get(self.get_full_path()
            .rsplit_once('/').unwrap().0).unwrap().as_generic().get_colors();
        contents = common::add_padding(
            contents, state.get_padding(),parent_colors.background,
            parent_colors.foreground);
        contents
    }
}

impl EzWidget for CanvasWidget {}

impl CanvasWidget {

    /// Load this widget from a config vector coming from [ez_parser]
    pub fn from_config(config: Vec<String>) -> Self {
        let mut obj = CanvasWidget::default();
        obj.load_ez_config(config).unwrap();
        obj
    }
}
