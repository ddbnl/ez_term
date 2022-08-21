//! A widget that displays text non-interactively.
use std::io::{Error, ErrorKind};

use crate::parser::load_base_properties;
use crate::parser::load_common_properties::load_common_property;
use crate::run::definitions::{Pixel, PixelMap, StateTree};
use crate::scheduler::scheduler::SchedulerFrontend;
use crate::states::ez_state::{EzState, GenericState};
use crate::states::progress_bar_state::ProgressBarState;
use crate::widgets::ez_object::{EzObject, EzObjects};
use crate::widgets::helper_functions::{add_border, add_padding};


#[derive(Clone, Debug)]
pub struct ProgressBar {

    /// ID of the widget, used to construct [path]
    pub id: String,

    /// Full path to this widget, e.g. "/root_layout/layout_2/THIS_ID"
    pub path: String,

    /// Runtime state of this widget, see [LabelState] and [State]
    pub state: ProgressBarState,
}

impl ProgressBar {
    pub fn new(id: String, path: String, scheduler: &mut SchedulerFrontend) -> Self {
        ProgressBar {
            id,
            path: path.clone(),
            state: ProgressBarState::new(path, scheduler),
        }
    }

    pub fn from_state(id: String, path: String, _scheduler: &mut SchedulerFrontend, state: EzState) -> Self {
        ProgressBar {
            id,
            path: path.clone(),
            state: state.as_progress_bar().to_owned(),
        }
    }
}


impl EzObject for ProgressBar {

    fn load_ez_parameter(&mut self, parameter_name: String, parameter_value: String,
                         scheduler: &mut SchedulerFrontend) -> Result<(), Error> {
        let consumed = load_common_property(
            &parameter_name, parameter_value.clone(), self, scheduler)?;
        if consumed { return Ok(())}
        match parameter_name.as_str() {
            "value" => load_base_properties::load_usize_property(
                parameter_value.trim(), scheduler, self.path.clone(),
                &parameter_name, self.get_state_mut())?,
            "max" => load_base_properties::load_usize_property(
                parameter_value.trim(), scheduler, self.path.clone(),
                &parameter_name, self.get_state_mut())?,
            _ => return Err(
                Error::new(ErrorKind::InvalidData,
                           format!("Invalid parameter name for progress bar: {}",
                                   parameter_name)))
        }
        Ok(())
    }

    fn set_id(&mut self, id: &str) { self.id = id.to_string() }

    fn get_id(&self) -> String { self.id.clone() }

    fn set_path(&mut self, id: &str) { self.id = id.to_string() }

    fn get_path(&self) -> String { self.path.clone() }

    fn get_state(&self) -> EzState { EzState::ProgressBar(self.state.clone()) }

    fn get_state_mut(&mut self) -> &mut dyn GenericState { &mut self.state }

    fn get_contents(&self, state_tree: &mut StateTree) -> PixelMap {

        let state = state_tree.get_mut(&self.get_path())
            .as_progress_bar_mut();
        state.set_height(1);

        let mut contents = PixelMap::new();

        let value_pos = ((state.get_effective_size().width - 1) as f64 *
            state.get_normalized_value()) as usize;

        for x in 0..state.get_effective_size().width {
            let symbol = if value_pos != 0 && x <= value_pos { "█" } else {"░"};
            contents.push(vec!(Pixel::new(symbol.to_string(),
                                          state.get_color_config().get_fg_color(),
                                          state.get_color_config().get_bg_color())));
        }
        if state.get_border_config().get_border() {
            contents = add_border(contents, state.get_border_config(),
                                state.get_color_config());
        }
        let state = state_tree
            .get(&self.get_path()).as_progress_bar();
        let parent_colors = state_tree.get(self.get_path()
            .rsplit_once('/').unwrap().0).as_generic().get_color_config();
        contents = add_padding(
            contents, state.get_padding(), parent_colors.get_bg_color(),
            parent_colors.get_fg_color());
        contents
    }

    fn get_clone(&self, scheduler: &mut SchedulerFrontend) -> EzObjects {

        let mut clone = self.clone();
        let mut new_state = ProgressBarState::new(self.path.clone(), scheduler);
        new_state.copy_state_values(self.get_state());
        clone.state = new_state;
        EzObjects::ProgressBar(clone)
    }
}
impl ProgressBar {

    /// Initialize an instance of this object using the passed config coming from [ez_parser]
    pub fn from_config(config: Vec<String>, id: String, path: String, scheduler: &mut SchedulerFrontend,
                       file: String, line: usize) -> Self {

        let mut obj = ProgressBar::new(id, path, scheduler);
        obj.load_ez_config(config, scheduler, file, line).unwrap();
        obj
    }
}
