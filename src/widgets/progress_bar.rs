//! A widget that displays text non-interactively.
use crate::states::ez_state::{EzState, GenericState};
use crate::widgets::ez_object::{EzObject};
use crate::parser::load_properties::load_common_property;
use crate::parser::load_base_properties::{load_ez_usize_property};
use crate::property::ez_values::EzValues;
use crate::run::definitions::{Pixel, PixelMap, StateTree};
use crate::scheduler::scheduler::Scheduler;
use crate::states::progress_bar_state::ProgressBarState;
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
    fn new(id: String, path: String, scheduler: &mut Scheduler) -> Self {
        ProgressBar {
            id,
            path: path.clone(),
            state: ProgressBarState::new(path, scheduler),
        }
    }
}


impl EzObject for ProgressBar {

    fn load_ez_parameter(&mut self, parameter_name: String, parameter_value: String,
                         scheduler: &mut Scheduler) {
        let consumed = load_common_property(
            &parameter_name, parameter_value.clone(), self, scheduler);
        if consumed { return }
        match parameter_name.as_str() {
            "value" => {
                let path = self.path.clone();
                self.state.set_value(
                    load_ez_usize_property(
                        parameter_value.trim(), scheduler,
                        self.path.clone(),
                        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
                            let state = state_tree.get_by_path_mut(&path)
                                .as_progress_bar_mut();
                            state.set_value(*val.as_usize());
                            path.clone()
                        })))
            },
            "max" => self.state.set_value(parameter_value.trim().parse().unwrap()),
            _ => panic!("Invalid parameter name for progress bar {}", parameter_name)
        }
    }

    fn set_id(&mut self, id: String) { self.id = id }

    fn get_id(&self) -> String { self.id.clone() }

    fn set_full_path(&mut self, path: String) { self.path = path }

    fn get_full_path(&self) -> String { self.path.clone() }

    fn get_state(&self) -> EzState { EzState::ProgressBar(self.state.clone()) }

    fn get_state_mut(&mut self) -> &mut dyn GenericState { &mut self.state }

    fn get_contents(&self, state_tree: &mut StateTree) -> PixelMap {

        let state = state_tree.get_by_path_mut(&self.get_full_path())
            .as_progress_bar_mut();
        state.size.height.set(1);

        let mut contents = PixelMap::new();

        let value_pos = ((state.get_effective_size().width - 1) as f64 *
            state.get_normalized_value()) as usize;

        for x in 0..state.get_effective_size().width {
            let symbol = if value_pos != 0 && x <= value_pos { "█" } else {"░"};
            contents.push(vec!(Pixel::new(symbol.to_string(),
                                          state.get_color_config().foreground.value,
                                     state.get_color_config().background.value)));
        }
        if state.get_border_config().enabled.value {
            contents = add_border(contents, state.get_border_config());
        }
        let state = state_tree
            .get_by_path(&self.get_full_path()).as_progress_bar();
        let parent_colors = state_tree.get_by_path(self.get_full_path()
            .rsplit_once('/').unwrap().0).as_generic().get_color_config();
        contents = add_padding(
            contents, state.get_padding(), parent_colors.background.value,
            parent_colors.foreground.value);
        contents
    }
}
impl ProgressBar {

    /// Initialize an instance of this object using the passed config coming from [ez_parser]
    pub fn from_config(config: Vec<String>, id: String, path: String, scheduler: &mut Scheduler)
                       -> Self {

        let mut obj = ProgressBar::new(id, path, scheduler);
        obj.load_ez_config(config, scheduler).unwrap();
        obj
    }
}
