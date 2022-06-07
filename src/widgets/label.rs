//! A widget that displays text non-interactively.
use std::fs::File;
use std::io::prelude::*;
use crate::common;
use crate::widgets::widget::{Pixel, EzObject};
use crate::states::label_state::LabelState;
use crate::states::state::{EzState, GenericState};
use crate::ez_parser;

#[derive(Clone)]
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

    fn load_ez_parameter(&mut self, parameter_name: String, mut parameter_value: String) {
        
        match parameter_name.as_str() {
            "id" => self.set_id(parameter_value.trim().to_string()),
            "x" => self.state.set_x(parameter_value.trim().parse().unwrap()),
            "y" => self.state.set_y(parameter_value.trim().parse().unwrap()),
            "pos" => self.state.set_position(
                ez_parser::load_pos_parameter(parameter_value.trim())),
            "size_hint" => self.state.set_size_hint(
                ez_parser::load_full_size_hint_parameter(parameter_value.trim())),
            "size_hint_x" => self.state.set_size_hint_x(
                ez_parser::load_size_hint_parameter(parameter_value.trim())),
            "size_hint_y" => self.state.set_size_hint_y(
                ez_parser::load_size_hint_parameter(parameter_value.trim())),
            "pos_hint" => self.state.set_pos_hint(
                ez_parser::load_full_pos_hint_parameter(parameter_value.trim())),
            "pos_hint_x" => self.state.set_pos_hint_x(
                ez_parser::load_pos_hint_x_parameter(parameter_value.trim())),
            "pos_hint_y" => self.state.set_pos_hint_y(
                ez_parser::load_pos_hint_y_parameter(parameter_value.trim())),
            "size" => self.state.set_size(
                ez_parser::load_size_parameter(parameter_value.trim())),
            "width" => self.state.get_size_mut().width = parameter_value.trim().parse().unwrap(),
            "height" => self.state.get_size_mut().height = parameter_value.trim().parse().unwrap(),
            "auto_scale" => self.state.set_auto_scale(ez_parser::load_full_auto_scale_parameter(
                parameter_value.trim())),
            "auto_scale_width" =>
                self.state.set_auto_scale_width(
                    ez_parser::load_bool_parameter(parameter_value.trim())),
            "auto_scale_height" =>
                self.state.set_auto_scale_height(
                    ez_parser::load_bool_parameter(parameter_value.trim())),
            "padding" => self.state.set_padding(
                ez_parser::load_full_padding_parameter(
                parameter_value.trim())),
            "padding_x" => self.state.set_padding(ez_parser::load_padding_x_parameter(
                parameter_value.trim())),
            "padding_y" => self.state.set_padding(ez_parser::load_padding_y_parameter(
                parameter_value.trim())),
            "padding_top" =>
                self.state.set_padding_top(parameter_value.trim().parse().unwrap()),
            "padding_bottom" =>
                self.state.set_padding_bottom(parameter_value.trim().parse().unwrap()),
            "padding_left" =>
                self.state.set_padding_left(parameter_value.trim().parse().unwrap()),
            "padding_right" =>
                self.state.set_padding_right(parameter_value.trim().parse().unwrap()),
            "halign" =>
                self.state.halign =  ez_parser::load_halign_parameter(parameter_value.trim()),
            "valign" =>
                self.state.valign =  ez_parser::load_valign_parameter(parameter_value.trim()),
            "border" => self.state.border_config.enabled =
                ez_parser::load_bool_parameter(parameter_value.trim()),
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
                self.state.border_config.fg_color =
                    ez_parser::load_color_parameter(parameter_value),
            "border_bg_color" =>
                self.state.border_config.bg_color =
                    ez_parser::load_color_parameter(parameter_value),
            "fg_color" =>
                self.state.colors.foreground = ez_parser::load_color_parameter(parameter_value),
            "bg_color" =>
                self.state.colors.background = ez_parser::load_color_parameter(parameter_value),
            "text" => {
                if parameter_value.starts_with(' ') {
                    parameter_value = parameter_value.strip_prefix(' ').unwrap().to_string();
                }
                self.state.text = parameter_value
            },
            "from_file" => self.from_file = Some(parameter_value.trim().to_string()),
            _ => panic!("Invalid parameter name for label {}", parameter_name)
        }
    }

    fn set_id(&mut self, id: String) { self.id = id }

    fn get_id(&self) -> String { self.id.clone() }

    fn set_full_path(&mut self, path: String) { self.path = path }

    fn get_full_path(&self) -> String { self.path.clone() }

    fn get_state(&self) -> EzState { EzState::Label(self.state.clone()) }

    fn get_contents(&self, state_tree: &mut common::definitions::StateTree)
        -> common::definitions::PixelMap {

        let state = state_tree
            .get_mut(&self.get_full_path()).unwrap().as_label_mut();
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


        let chunk_size = if state.get_size().infinite_width {text.len()}
                               else {state.get_effective_size().width};
        let content_lines = common::widget_functions::wrap_text(text, chunk_size);
        // If content is scrolled simply scale to length of content on that axis
        if state.get_size().infinite_width {
            let longest_line = content_lines.iter().map(|x| x.len()).max();
            let width = if let Some(i) = longest_line { i } else { 0 };
            state.set_effective_width(width);
        }
        if state.get_size().infinite_height {
            let height = content_lines.len();
            state.set_effective_height(height);
        }
        // If user wants to autoscale we set size to size of content or if that does not it to
        // size of the widget
        if state.get_auto_scale().width {
            let longest_line = content_lines.iter().map(|x| x.len()).max();
            let auto_scale_width = if let Some(i) = longest_line { i } else { 0 };
            if auto_scale_width < state.get_effective_size().width {
                state.set_effective_width(auto_scale_width);
            }
        }
        if state.get_auto_scale().height {
            let auto_scale_height = content_lines.len();
            if auto_scale_height < state.get_effective_size().height {
                state.set_effective_height(auto_scale_height);
            }
        }

        // Now we'll create the actual PixelMap using the lines we've created by wrapping the text
        let mut contents = Vec::new();
        for x in 0..state.get_effective_size().width {
            let mut new_y = Vec::new();
            for y in 0..state.get_effective_size().height {
                if y < content_lines.len() && x < content_lines[y].len() {
                    new_y.push(Pixel {
                        symbol: content_lines[y][x..x+1].to_string(),
                        foreground_color: state.get_color_config().foreground,
                        background_color: state.get_color_config().background,
                        underline: false
                    })
                } else {
                    new_y.push(Pixel {
                        symbol: " ".to_string(),
                        foreground_color: state.get_color_config().foreground,
                        background_color: state.get_color_config().background,
                        underline: false
                    })
                }
            }
            contents.push(new_y);
        }
        if state.get_border_config().enabled {
            contents = common::widget_functions::add_border(
                contents, state.get_border_config());
        }
        let state = state_tree.get(&self.get_full_path()).unwrap().as_label();
        let parent_colors = state_tree.get(self.get_full_path()
            .rsplit_once('/').unwrap().0).unwrap().as_generic().get_color_config();
        contents = common::widget_functions::add_padding(
            contents, state.get_padding(), parent_colors.background,
            parent_colors.foreground);
        contents
    }
}
impl Label {

    /// Initialize an instance of this object using the passed config coming from [ez_parser]
    pub fn from_config(config: Vec<String>) -> Self {
        let mut obj = Label::default();
        obj.load_ez_config(config).unwrap();
        obj
    }
}
