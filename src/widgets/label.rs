//! A widget that displays text non-interactively.
use std::fs::File;
use std::io::Error;
use std::io::prelude::*;
use std::collections::HashMap;  // For ez_file_gen.rs

use crate::parser::load_base_properties;
use crate::parser::load_common_properties::load_common_property;
use crate::run::definitions::{Pixel, PixelMap, StateTree};
use crate::scheduler::scheduler::SchedulerFrontend;
use crate::states::ez_state::{EzState, GenericState};
use crate::states::label_state::LabelState;
use crate::widgets::ez_object::{EzObject, EzObjects};
use crate::widgets::helper_functions::{add_border, add_padding, wrap_text};
include!(concat!(env!("OUT_DIR"), "/ez_file_gen.rs"));


#[derive(Clone, Debug)]
pub struct Label {

    /// ID of the widget, used to construct [path]
    pub id: String,

    /// Full path to this widget, e.g. "/root_layout/layout_2/THIS_ID"
    pub path: String,

    /// Runtime state of this widget, see [LabelState] and [State]
    pub state: LabelState,
}

impl Label {
    pub fn new(id: String, path: String, scheduler: &mut SchedulerFrontend) -> Self {
        Label {
            id,
            path: path.clone(),
            state: LabelState::new(path, scheduler),
        }
    }

    pub fn from_state(id: String, path: String, _scheduler: &mut SchedulerFrontend, state: EzState) -> Self {
        Label {
            id,
            path: path.clone(),
            state: state.as_label().to_owned(),
        }
    }
}


impl EzObject for Label {

    fn load_ez_parameter(&mut self, parameter_name: String, parameter_value: String,
                         scheduler: &mut SchedulerFrontend) -> Result<(), Error> {

        let consumed = load_common_property(
            &parameter_name, parameter_value.clone(),self, scheduler)?;
        if consumed { return Ok(()) }
        match parameter_name.as_str() {
            "from_file" => load_base_properties::load_string_property(
                parameter_value.trim(), scheduler, self.path.clone(),
                &parameter_name, self.get_state_mut())?,
            "text" => load_base_properties::load_string_property(
                parameter_value.strip_prefix(' ').unwrap_or(
                    parameter_value.as_str()), scheduler, self.path.clone(),
                &parameter_name, self.get_state_mut())?,
            _ => panic!("Invalid parameter name for label: {}", parameter_name)
        }
        Ok(())
    }

    fn set_id(&mut self, id: &str) { self.id = id.to_string() }

    fn get_id(&self) -> String { self.id.clone() }

    fn set_path(&mut self, id: &str) { self.id = id.to_string() }

    fn get_path(&self) -> String { self.path.clone() }

    fn get_state(&self) -> EzState { EzState::Label(self.state.clone()) }

    fn get_state_mut(&mut self) -> &mut dyn GenericState { &mut self.state }

    fn get_contents(&self, state_tree: &mut StateTree) -> PixelMap {

        let state = state_tree
            .get_mut(&self.get_path()).as_label_mut();
        let mut text;
        // Load text from file
        if !state.get_from_file().is_empty() {
            let includes = ez_includes();
            let key = &state.get_from_file().replace("\\", "\\\\");
            text = includes.get(key).unwrap_or_else(
                || panic!("Unable to open file for label: {}", state.get_from_file())).clone();
        // or take text from widget state
        } else {
            text = state.get_text();
        }
        
        let chunk_size =
            if state.get_infinite_size().width ||
                state.get_auto_scale().get_auto_scale_width() {text.len() + 1}
            else {state.get_effective_size().width};
        let content_lines = wrap_text(text, chunk_size);
        // If content is scrolled simply scale to length of content on that axis
        if state.get_infinite_size().width {
            let longest_line = content_lines.iter().map(|x| x.len()).max();
            let width = if let Some(i) = longest_line { i } else { 0 };
            state.set_effective_width(width);
        }
        if state.get_infinite_size().height {
            let height = content_lines.len();
            state.set_effective_height(height);
        }
        // If user wants to autoscale we set size to size of content or if that does not it to
        // size of the widget
        if state.get_auto_scale().get_auto_scale_width() {
            let longest_line = content_lines.iter().map(|x| x.len()).max();
            let auto_scale_width = if let Some(i) = longest_line { i } else { 0 };
            if auto_scale_width < state.get_effective_size().width {
                state.set_effective_width(auto_scale_width);
            }
        }
        if state.get_auto_scale().get_auto_scale_height() {
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
                        foreground_color: state.get_color_config().get_fg_color(),
                        background_color: state.get_color_config().get_bg_color(),
                        underline: false
                    })
                } else {
                    new_y.push(Pixel {
                        symbol: " ".to_string(),
                        foreground_color: state.get_color_config().get_fg_color(),
                        background_color: state.get_color_config().get_bg_color(),
                        underline: false
                    })
                }
            }
            contents.push(new_y);
        }
        if state.get_border_config().get_border() {
            contents = add_border(contents, state.get_border_config(),
                                 state.get_color_config());
        }
        let state = state_tree.get(&self.get_path()).as_label();
        let parent_colors = state_tree.get(self.get_path()
            .rsplit_once('/').unwrap().0).as_generic().get_color_config();
        contents = add_padding(
            contents, state.get_padding(), parent_colors.get_bg_color(),
            parent_colors.get_fg_color());
        contents
    }

    fn get_clone(&self, scheduler: &mut SchedulerFrontend) -> EzObjects {

        let mut clone = self.clone();
        let mut new_state = LabelState::new(self.path.clone(), scheduler);
        new_state.copy_state_values(self.get_state());
        clone.state = new_state;
        EzObjects::Label(clone)
    }
}
impl Label {

    /// Initialize an instance of this object using the passed config coming from [ez_parser]
    pub fn from_config(config: Vec<String>, id: String, path: String, scheduler: &mut SchedulerFrontend,
                       file: String, line: usize) -> Self {

        let mut obj = Label::new(id, path, scheduler);
        obj.load_ez_config(config, scheduler, file, line).unwrap();
        obj
    }
}
