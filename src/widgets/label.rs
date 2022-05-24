//! A widget that displays text non-interactively.
use std::fs::File;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use crate::common;
use crate::common::{PixelMap, StateTree};
use crate::widgets::widget::{Pixel, EzObject, EzWidget};
use crate::states::label_state::LabelState;
use crate::states::state::{EzState, GenericState};
use crate::ez_parser::{load_color_parameter, load_bool_parameter, load_size_hint_parameter,
                       load_halign_parameter, load_valign_parameter, load_pos_hint_x_parameter,
                       load_pos_hint_y_parameter};

pub struct Label {

    /// ID of the widget, used to construct [path]
    pub id: String,

    /// Full path to this widget, e.g. "/root_layout/layout_2/THIS_ID"
    pub path: String,

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
            from_file: None,
            state: LabelState::default(),
        }
    }
}


impl EzObject for Label {

    fn load_ez_parameter(&mut self, parameter_name: String, mut parameter_value: String)
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

    fn update_state(&mut self, new_state: &EzState) {
        let state = new_state.as_label();
        self.state = state.clone();
        self.state.changed = false;
        self.state.force_redraw = false;
    }

    fn get_state(&self) -> EzState { EzState::Label(self.state.clone()) }

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
