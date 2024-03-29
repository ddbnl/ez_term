//! # Canvas Widget
//! Module defining a canvas widget, which does not generate any content but should be 'painted'
//! manually by the user using the 'set_content' method.
use std::collections::HashMap;
use std::io::{Error, ErrorKind}; // For ez_file_gen.rs

use crate::parser::load_base_properties;
use unicode_segmentation::UnicodeSegmentation;

use crate::parser::load_common_properties::load_common_property;
use crate::run::definitions::{Pixel, PixelMap, StateTree};
use crate::scheduler::scheduler::SchedulerFrontend;
use crate::states::canvas_state::CanvasState;
use crate::states::ez_state::{EzState, GenericState};
use crate::widgets::ez_object::{EzObject, EzObjects};
use crate::widgets::helper_functions::{add_border, add_padding};
include!(concat!(env!("OUT_DIR"), "/ez_file_gen.rs"));

#[derive(Clone, Debug)]
pub struct Canvas {
    /// ID of the widget, used to construct [path]
    pub id: String,

    /// Full path to this widget, e.g. "/root_layout/layout_2/THIS_ID"
    pub path: String,

    /// Runtime state of this widget, see [CanvasState] and [State]
    pub state: CanvasState,
}

impl Canvas {
    pub fn new(id: String, path: String, scheduler: &mut SchedulerFrontend) -> Self {
        Canvas {
            id,
            path: path.clone(),
            state: CanvasState::new(path, scheduler),
        }
    }

    pub fn from_state(
        id: String,
        path: String,
        _scheduler: &mut SchedulerFrontend,
        state: EzState,
    ) -> Self {
        Canvas {
            id,
            path: path.clone(),
            state: state.as_canvas().to_owned(),
        }
    }
}

impl EzObject for Canvas {
    fn load_ez_parameter(
        &mut self,
        parameter_name: String,
        parameter_value: String,
        scheduler: &mut SchedulerFrontend,
    ) -> Result<(), Error> {
        let consumed =
            load_common_property(&parameter_name, parameter_value.clone(), self, scheduler)?;
        if consumed {
            return Ok(());
        }
        match parameter_name.as_str() {
            "from_file" => load_base_properties::load_string_property(
                parameter_value.trim(),
                scheduler,
                self.path.clone(),
                &parameter_name,
                self.get_state_mut(),
            )?,
            _ => {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("Invalid parameter name for canvas: {}", parameter_name),
                ))
            }
        }
        Ok(())
    }

    fn set_id(&mut self, id: &str) {
        self.id = id.to_string()
    }

    fn get_id(&self) -> String {
        self.id.clone()
    }

    fn set_path(&mut self, id: &str) {
        self.id = id.to_string()
    }

    fn get_path(&self) -> String {
        self.path.clone()
    }

    fn get_state(&self) -> EzState {
        EzState::Canvas(self.state.clone())
    }

    fn get_state_mut(&mut self) -> &mut dyn GenericState {
        &mut self.state
    }

    fn get_contents(&self, state_tree: &mut StateTree) -> PixelMap {
        let state = state_tree.get_mut(&self.get_path()).as_canvas_mut();
        let mut contents;
        if !state.get_from_file().is_empty() {
            let includes = ez_includes();
            let key = &state.get_from_file().replace("\\", "\\\\");
            let file_content = includes
                .get(key)
                .unwrap_or_else(|| {
                    panic!(
                        "Unable to open file for canvas widget: {}",
                        state.get_from_file()
                    )
                })
                .clone();
            let mut lines: Vec<String> = file_content
                .lines()
                .map(|x| x.graphemes(true).rev().collect())
                .collect();

            if state.get_auto_scale().get_auto_scale_width() {
                let longest_line = lines.iter().map(|x| x.chars().count()).max();
                let auto_scale_width = if let Some(i) = longest_line { i } else { 0 };
                if auto_scale_width < state.get_effective_size().width
                    || state.get_infinite_size().width
                {
                    state.set_effective_width(auto_scale_width);
                }
            }
            if state.get_auto_scale().get_auto_scale_height() {
                let auto_scale_height = lines.len();
                if auto_scale_height < state.get_effective_size().height
                    || state.get_infinite_size().height
                {
                    state.set_effective_height(auto_scale_height);
                }
            }

            let mut widget_content = PixelMap::new();
            for x in 0..state.get_effective_size().width {
                widget_content.push(Vec::new());
                for y in 0..state.get_effective_size().height {
                    if y < lines.len() && !lines[y].is_empty() {
                        widget_content[x].push(Pixel::new(
                            lines[y].pop().unwrap().to_string(),
                            state.get_color_config().get_fg_color(),
                            state.get_color_config().get_bg_color()));
                    } else {
                        widget_content[x].push(Pixel::new(
                            " ".to_string(),
                            state.get_color_config().get_fg_color(),
                            state.get_color_config().get_bg_color()));
                    }
                }
            }
            contents = widget_content;
        } else {
            contents = state.get_contents().clone();
        }
        if state.get_border_config().get_border() {
            contents = add_border(
                contents,
                state.get_border_config(),
                state.get_color_config(),
            );
        }
        let state = state_tree.get(&self.get_path()).as_canvas();
        let parent_colors = state_tree
            .get(self.get_path().rsplit_once('/').unwrap().0)
            .as_generic()
            .get_color_config();
        contents = add_padding(
            contents,
            state.get_padding(),
            parent_colors.get_bg_color(),
            parent_colors.get_fg_color(),
        );
        contents
    }

    fn get_clone(&self, scheduler: &mut SchedulerFrontend) -> EzObjects {
        let mut clone = self.clone();
        let mut new_state = CanvasState::new(self.path.clone(), scheduler);
        new_state.copy_state_values(self.get_state());
        clone.state = new_state;
        EzObjects::Canvas(clone)
    }
}
impl Canvas {
    /// Initialize an instance of this object using the passed config coming from [ez_parser]
    pub fn from_config(
        config: Vec<String>,
        id: String,
        path: String,
        scheduler: &mut SchedulerFrontend,
        file: String,
        line: usize,
    ) -> Self {
        let mut obj = Canvas::new(id, path, scheduler);
        obj.load_ez_config(config, scheduler, file, line).unwrap();
        obj
    }
}
