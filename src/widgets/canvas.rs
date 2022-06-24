//! # Canvas Widget
//! Module defining a canvas widget, which does not generate any content but should be 'painted'
//! manually by the user using the 'set_content' method.
use std::fs::File;
use std::io::prelude::*;
use crate::widgets::widget::{EzObject, Pixel};
use crate::states::canvas_state::CanvasState;
use crate::states::state::{EzState, GenericState};
use crate::common;
use unicode_segmentation::UnicodeSegmentation;
use crate::parser;
use crate::scheduler::Scheduler;

#[derive(Clone, Debug)]
pub struct Canvas {
    /// ID of the widget, used to construct [path]
    pub id: String,

    /// Full path to this widget, e.g. "/root_layout/layout_2/THIS_ID"
    pub path: String,

    /// Optional file path to retrieve contents from
    pub from_file: Option<String>,

    /// Grid of pixels that will be written to screen for this widget
    pub contents: common::definitions::PixelMap,

    /// Runtime state of this widget, see [CanvasState] and [State]
    pub state: CanvasState,
}


impl Canvas {

    pub fn new(id: String, path: String, scheduler: &mut Scheduler) -> Self {

        Canvas {
            id,
            path: path.clone(),
            from_file: None,
            contents: Vec::new(),
            state: CanvasState::new(path, scheduler),
        }
    }
}


impl EzObject for Canvas {
    fn load_ez_parameter(&mut self, parameter_name: String, parameter_value: String,
                         scheduler: &mut Scheduler) {

        let consumed = parser::load_common_property(
            &parameter_name, parameter_value.clone(),self, scheduler);
        if consumed { return }
        match parameter_name.as_str() {
            "from_file" => self.from_file = Some(parameter_value.trim().to_string()),
            _ => panic!("Invalid parameter name for canvas widget {}", parameter_name)
        }
    }

    fn set_id(&mut self, id: String) { self.id = id }

    fn get_id(&self) -> String { self.id.clone() }

    fn set_full_path(&mut self, path: String) { self.path = path }

    fn get_full_path(&self) -> String { self.path.clone() }

    fn get_state(&self) -> EzState { EzState::Canvas(self.state.clone()) }

    fn get_state_mut(&mut self) -> &mut dyn GenericState { &mut self.state }

    /// Set the content of this Widget. You must manually fill a [PixelMap] of the same
    /// [height] and [width] as this widget and pass it here.
    fn set_contents(&mut self, contents: common::definitions::PixelMap) {
       let mut valid_contents = Vec::new();
       for x in 0..self.state.get_size().width.value as usize {
           valid_contents.push(Vec::new());
           for y in 0..self.state.get_size().height.value as usize {
               valid_contents[x].push(contents[x][y].clone())
           }
       }
       self.contents = valid_contents
    }

    fn get_contents(&self, state_tree: &mut common::definitions::StateTree) -> common::definitions::PixelMap {

        let state = state_tree
            .get_by_path_mut(&self.get_full_path()).as_canvas_mut();
        let mut contents;
        if let Some(path) = self.from_file.clone() {
            let mut file = File::open(path).expect("Unable to open file");
            let mut file_content = String::new();
            file.read_to_string(&mut file_content).expect("Unable to read file");
            let mut lines: Vec<String> = file_content.lines()
                .map(|x| x.graphemes(true).rev().collect())
                .collect();

            if state.get_auto_scale().width.value {
                let longest_line = lines.iter().map(|x| x.chars().count()).max();
                let auto_scale_width =
                    if let Some(i) = longest_line { i } else { 0 };
                if auto_scale_width < state.get_effective_size().width || state.get_size().infinite_width {
                    state.set_effective_width(auto_scale_width);
                }
            }
            if state.get_auto_scale().height.value {
                let auto_scale_height = lines.len();
                if auto_scale_height < state.get_effective_size().height || state.get_size().infinite_height{
                    state.set_effective_height(auto_scale_height);
                }
            }

            let mut widget_content = common::definitions::PixelMap::new();
            for x in 0..state.get_effective_size().width {
                widget_content.push(Vec::new());
                for y in 0..state.get_effective_size().height {
                    if y < lines.len() && !lines[y].is_empty() {
                        widget_content[x].push(Pixel {
                            symbol: lines[y].pop().unwrap().to_string(),
                            foreground_color: state.get_color_config().foreground.value,
                            background_color: state.get_color_config().background.value,
                            underline: false})
                    } else {
                        widget_content[x].push(Pixel {
                            symbol: " ".to_string(),
                            foreground_color: state.get_color_config().foreground.value,
                            background_color: state.get_color_config().background.value,
                            underline: false})
                    }
                }
            }
            contents = widget_content;
        } else {
            contents = self.contents.clone();
        }
        if state.get_border_config().enabled.value {
            contents = common::widget_functions::add_border(
                contents, state.get_border_config());
        }
        let state = state_tree.get_by_path(&self.get_full_path()).as_canvas();
        let parent_colors = state_tree.get_by_path(self.get_full_path()
            .rsplit_once('/').unwrap().0).as_generic().get_color_config();
        contents = common::widget_functions::add_padding(
            contents, state.get_padding(),parent_colors.background.value,
            parent_colors.foreground.value);
        contents
    }
}
impl Canvas {

    /// Load this widget from a config vector coming from [ez_parser]
    pub fn from_config(config: Vec<String>, id: String, path: String, scheduler: &mut Scheduler)
                       -> Self {

        let mut obj = Canvas::new(id, path, scheduler);
        obj.load_ez_config(config, scheduler).unwrap();
        obj
    }
}
