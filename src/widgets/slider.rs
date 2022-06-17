//! A widget that displays text non-interactively.
use std::cmp::min;
use std::io::Error;
use crossterm::event::{Event, KeyCode};
use crate::states::state::{EzState, GenericState};
use crate::common;
use crate::common::definitions::{CallbackTree, EzContext, PixelMap, StateTree, ViewTree, WidgetTree};
use crate::widgets::widget::{Pixel, EzObject};
use crate::ez_parser;
use crate::scheduler::Scheduler;
use crate::states::definitions::Coordinates;
use crate::states::slider_state::SliderState;

#[derive(Clone)]
pub struct Slider {

    /// ID of the widget, used to construct [path]
    pub id: String,

    /// Full path to this widget, e.g. "/root_layout/layout_2/THIS_ID"
    pub path: String,

    /// Runtime state of this widget, see [LabelState] and [State]
    pub state: SliderState,
}

impl Default for Slider {
    fn default() -> Self {
        Slider {
            id: "".to_string(),
            path: String::new(),
            state: SliderState::default(),
        }
    }
}


impl EzObject for Slider {

    fn load_ez_parameter(&mut self, parameter_name: String, parameter_value: String) {
        let consumed = ez_parser::load_common_parameters(
            &parameter_name, parameter_value.clone(), Box::new(self));
        if consumed { return }
        match parameter_name.as_str() {
            "value" => self.state.set_value(parameter_value.trim().parse().unwrap()),
            "minimum" => self.state.set_minimum(parameter_value.trim().parse().unwrap()),
            "maximum" => self.state.set_maximum(parameter_value.trim().parse().unwrap()),
            "step" => self.state.set_step(parameter_value.trim().parse().unwrap()),
            _ => panic!("Invalid parameter name for slider {}", parameter_name)
        }
    }

    fn set_id(&mut self, id: String) { self.id = id }

    fn get_id(&self) -> String { self.id.clone() }

    fn set_full_path(&mut self, path: String) { self.path = path }

    fn get_full_path(&self) -> String { self.path.clone() }

    fn get_state(&self) -> EzState { EzState::Slider(self.state.clone()) }

    fn get_state_mut(&mut self) -> Box<&mut dyn GenericState>{ Box::new(&mut self.state) }

    fn get_contents(&self, state_tree: &mut StateTree) -> PixelMap {

        let state = state_tree.get_mut(&self.get_full_path()).unwrap()
            .as_slider_mut();
        state.size.height = 1;

        let fg_color = if state.selected {state.get_color_config().selection_foreground }
            else {state.get_color_config().foreground };
        let bg_color = if state.selected {state.get_color_config().selection_background }
            else {state.get_color_config().background };

        let mut contents = PixelMap::new();
        let value_pos =
            ((state.get_effective_size().width - 1) as f64 *
            ((state.value - state.minimum) as f64 / (state.maximum - state.minimum) as f64))
                as usize;
        for x in 0..state.get_effective_size().width {
            let symbol = if x == value_pos { "|" } else {"-"};
            contents.push(vec!(Pixel::new(symbol.to_string(), fg_color,
                                     bg_color)));
        }

        let state = state_tree.get(&self.get_full_path()).unwrap().as_slider();
        let parent_colors = state_tree.get(self.get_full_path()
            .rsplit_once('/').unwrap().0).unwrap().as_generic().get_color_config();
        contents = common::widget_functions::add_padding(
            contents, state.get_padding(), parent_colors.background,
            parent_colors.foreground);
        contents
    }

    fn handle_event(&self, event: Event, view_tree: &mut ViewTree, state_tree: &mut StateTree,
                    widget_tree: &WidgetTree, callback_tree: &mut CallbackTree,
                    scheduler: &mut Scheduler) -> bool {

        if let Event::Key(key) = event {
            if key.code == KeyCode::Left {
                self.handle_left(view_tree, state_tree, widget_tree, callback_tree, scheduler);
                return true
            } else if key.code == KeyCode::Right {
                self.handle_right(view_tree, state_tree, widget_tree, callback_tree, scheduler);
                return true
            }
        }
        false
    }

    fn on_left_mouse_click(&self, view_tree: &mut ViewTree, state_tree: &mut StateTree,
                           widget_tree: &WidgetTree, callback_tree: &mut CallbackTree,
                           scheduler: &mut Scheduler, mouse_pos: Coordinates) -> bool {

        let state = state_tree.get_mut(&self.path).unwrap().as_slider_mut();
        let ratio = ((state.maximum - state.minimum) as f64 / state.get_effective_size().width as f64);
        let mut value =
            (ratio * mouse_pos.x as f64) as isize + state.minimum + 1;
        value = min(value, state.maximum);
        state.set_value(value);

        if let Some(ref mut i ) = callback_tree
            .get_mut(&self.get_full_path()).unwrap().on_value_change {
            i(EzContext::new(self.get_full_path(),
                             view_tree, state_tree, widget_tree, scheduler));
        }
        true
    }
}
impl Slider {

    /// Initialize an instance of this object using the passed config coming from [ez_parser]
    pub fn from_config(config: Vec<String>) -> Self {
        let mut obj = Slider::default();
        obj.load_ez_config(config).unwrap();
        obj
    }

    fn handle_left(&self, view_tree: &mut ViewTree, state_tree: &mut StateTree,
                   widget_tree: &WidgetTree, callback_tree: &mut CallbackTree,
                   scheduler: &mut Scheduler) {
        let state = state_tree.get_mut(&self.path).unwrap().as_slider_mut();
        if state.value == state.minimum { return }
        state.set_value(state.get_value() - state.get_step() as isize);
        if let Some(ref mut i ) = callback_tree
            .get_mut(&self.get_full_path()).unwrap().on_value_change {
            i(EzContext::new(self.get_full_path(),
                             view_tree, state_tree, widget_tree, scheduler));
        }
    }
    fn handle_right(&self, view_tree: &mut ViewTree, state_tree: &mut StateTree,
                   widget_tree: &WidgetTree, callback_tree: &mut CallbackTree,
                   scheduler: &mut Scheduler) {

        let state = state_tree.get_mut(&self.path).unwrap().as_slider_mut();
        if state.value == state.maximum { return }
        state.set_value(state.get_value() + state.get_step() as isize);
        if let Some(ref mut i ) = callback_tree
            .get_mut(&self.get_full_path()).unwrap().on_value_change {
            i(EzContext::new(self.get_full_path(),
                             view_tree, state_tree, widget_tree, scheduler));
        }
    }
}
