//! A widget that displays text non-interactively.
use std::cmp::min;
use std::io::{Error, ErrorKind};

use crossterm::event::{Event, KeyCode};
use crate::Context;
use crate::parser::load_base_properties::load_usize_property;

use crate::parser::load_common_properties::load_common_property;
use crate::property::ez_values::EzValues;
use crate::run::definitions::{CallbackTree, Coordinates, Pixel, PixelMap, StateTree};
use crate::scheduler::scheduler::SchedulerFrontend;
use crate::states::ez_state::{EzState, GenericState};
use crate::states::slider_state::SliderState;
use crate::widgets::ez_object::EzObject;
use crate::widgets::helper_functions::add_padding;


#[derive(Clone, Debug)]
pub struct Slider {

    /// ID of the widget, used to construct [path]
    pub id: String,

    /// Full path to this widget, e.g. "/root_layout/layout_2/THIS_ID"
    pub path: String,

    /// Runtime state of this widget, see [LabelState] and [State]
    pub state: SliderState,
}

impl Slider {
    pub fn new(id: String, path: String, scheduler: &mut SchedulerFrontend) -> Self {
        Slider {
            id,
            path: path.clone(),
            state: SliderState::new(path, scheduler),
        }
    }

    pub fn from_state(id: String, path: String, _scheduler: &mut SchedulerFrontend, state: EzState) -> Self {
        Slider {
            id,
            path: path.clone(),
            state: state.as_slider().to_owned(),
        }
    }

    fn load_step_property(&mut self, parameter_value: &str, scheduler: &mut SchedulerFrontend)
                           -> Result<(), Error> {
        let path = self.path.clone();
        self.state.set_step(load_usize_property(
            parameter_value.trim(), scheduler, self.path.clone(),
                Box::new(move |state_tree: &mut StateTree, val: EzValues| {
                    let state = state_tree.get_mut(&path)
                        .as_slider_mut();
                    state.set_step(val.as_usize());
                    path.clone()
                }))?);
        Ok(())
    }

    fn load_min_property(&mut self, parameter_value: &str, scheduler: &mut SchedulerFrontend)
                          -> Result<(), Error> {
        let path = self.path.clone();
        self.state.set_min(load_usize_property(
            parameter_value.trim(), scheduler, self.path.clone(),
            Box::new(move |state_tree: &mut StateTree, val: EzValues| {
                let state = state_tree.get_mut(&path)
                    .as_slider_mut();
                state.set_min(val.as_usize());
                path.clone()
            }))?);
        Ok(())
    }

    fn load_max_property(&mut self, parameter_value: &str, scheduler: &mut SchedulerFrontend)
                          -> Result<(), Error> {
        let path = self.path.clone();
        self.state.set_max(load_usize_property(
            parameter_value.trim(), scheduler, self.path.clone(),
            Box::new(move |state_tree: &mut StateTree, val: EzValues| {
                let state = state_tree.get_mut(&path)
                    .as_slider_mut();
                state.set_max(val.as_usize());
                path.clone()
            }))?);
        Ok(())
    }

    fn load_value_property(&mut self, parameter_value: &str, scheduler: &mut SchedulerFrontend)
                          -> Result<(), Error> {
        let path = self.path.clone();
        self.state.set_value(load_usize_property(
            parameter_value.trim(), scheduler, self.path.clone(),
            Box::new(move |state_tree: &mut StateTree, val: EzValues| {
                let state = state_tree.get_mut(&path)
                    .as_slider_mut();
                state.set_value(val.as_usize());
                path.clone()
            }))?);
        Ok(())
    }
}


impl EzObject for Slider {

    fn load_ez_parameter(&mut self, parameter_name: String, parameter_value: String,
                         scheduler: &mut SchedulerFrontend) -> Result<(), Error> {
        let consumed = load_common_property(
            &parameter_name, parameter_value.clone(), self, scheduler)?;
        if consumed { return Ok(())}
        match parameter_name.as_str() {
            "value" => self.load_value_property(parameter_value.trim(), scheduler)?,
            "min" => self.load_min_property(parameter_value.trim(), scheduler)?,
            "max" => self.load_max_property(parameter_value.trim(), scheduler)?,
            "step" => self.load_step_property(parameter_value.trim(), scheduler)?,
            _ => return Err(
                Error::new(ErrorKind::InvalidData,
                           format!("Invalid parameter name for slider: {}", parameter_name)))
        }
        Ok(())
    }

    fn set_id(&mut self, id: &str) { self.id = id.to_string() }

    fn get_id(&self) -> String { self.id.clone() }

    fn set_path(&mut self, id: &str) { self.id = id.to_string() }

    fn get_path(&self) -> String { self.path.clone() }

    fn get_state(&self) -> EzState { EzState::Slider(self.state.clone()) }

    fn get_state_mut(&mut self) -> &mut dyn GenericState { &mut self.state }

    fn get_contents(&self, state_tree: &mut StateTree) -> PixelMap {

        let state = state_tree.get_mut(&self.get_path())
            .as_slider_mut();

        if state.get_effective_size().width == 0 { return PixelMap::new() }
        if state.get_value() < state.get_min() {
            state.set_value(state.get_min())
        }
        if state.get_value() > state.get_max() {
            state.set_value(state.get_max())
        }

        state.set_height(1);
        if state.get_auto_scale().get_auto_scale_width() {
            state.set_effective_width(((state.get_max() - state.get_min()) /
                state.get_step()) as usize + 1);
        }

        let mut contents = PixelMap::new();
        let value_pos =
            ((state.get_effective_size().width - 1) as f64 *
            ((state.get_value() - state.get_min()) as f64 /
                (state.get_max() - state.get_min()) as f64))
                as usize;
        for x in 0..state.get_effective_size().width {
            let fg_color =
                if state.get_disabled() {state.get_color_config().get_disabled_fg_color() }
                else if x == value_pos &&
                    state.get_selected() { state.get_color_config().get_selection_fg_color() }
                else { state.get_color_config().get_fg_color() };
            let bg_color =
                if state.get_disabled() {state.get_color_config().get_disabled_bg_color() }
                else {state.get_color_config().get_bg_color()};
            contents.push(vec!(Pixel::new(
                if x == value_pos { "🮚".to_string() } else { "━".to_string() },
                fg_color, bg_color)));
        }

        let state = state_tree.get(&self.get_path()).as_slider();
        let parent_colors = state_tree.get(self.get_path()
            .rsplit_once('/').unwrap().0).as_generic().get_color_config();
        contents = add_padding(
            contents, state.get_padding(), parent_colors.get_bg_color(),
            parent_colors.get_fg_color());
        contents
    }

    fn handle_event(&self, event: Event, state_tree: &mut StateTree,
                    callback_tree: &mut CallbackTree, scheduler: &mut SchedulerFrontend) -> bool {

        if let Event::Key(key) = event {
            if key.code == KeyCode::Left {
                self.handle_left(state_tree, callback_tree, scheduler);
                return true
            } else if key.code == KeyCode::Right {
                self.handle_right(state_tree, callback_tree, scheduler);
                return true
            }
            else if callback_tree.get(&self.get_path())
                .obj.keymap.contains(key.code, key.modifiers) {
                let func =
                    callback_tree.get_mut(&self.get_path())
                        .obj.keymap.get_mut(key.code, key.modifiers).unwrap();
                let context = Context::new(
                    self.get_path(), state_tree, scheduler);
                func(context, key.code, key.modifiers);
                return true
            }
        }
        false
    }

    fn on_left_mouse_click(&self, state_tree: &mut StateTree, callback_tree: &mut CallbackTree,
                           scheduler: &mut SchedulerFrontend, mouse_pos: Coordinates) -> bool {

        let consumed = self.on_press_callback(state_tree, callback_tree, scheduler);
        if consumed { return consumed}
        let state = state_tree.get_mut(&self.path).as_slider_mut();
        let value = self.value_from_mouse_pos(state, mouse_pos);
        state.set_value(value);
        state.update(scheduler);
        self.on_value_change_callback(state_tree, callback_tree, scheduler);
        true
    }

    fn on_hover(&self, state_tree: &mut StateTree, callback_tree: &mut CallbackTree,
                scheduler: &mut SchedulerFrontend, mouse_pos: Coordinates) -> bool {

        let consumed = self.on_hover_callback(state_tree, callback_tree, scheduler, mouse_pos);
        if consumed { return consumed}
        scheduler.set_selected_widget(&self.path, Some(mouse_pos));
        true
    }

    fn on_drag(&self, state_tree: &mut StateTree, callback_tree: &mut CallbackTree,
               scheduler: &mut SchedulerFrontend, previous_pos: Option<Coordinates>,
               mouse_pos: Coordinates) -> bool {

        let consumed = self.on_drag_callback(state_tree, callback_tree, scheduler,
                                             previous_pos, mouse_pos);
        if consumed { return consumed}
        let state = state_tree.get_mut(&self.path).as_slider_mut();
        let value = self.value_from_mouse_pos(state, mouse_pos);
        state.set_value(value);
        state.update(scheduler);
        self.on_value_change_callback(state_tree, callback_tree, scheduler);
        self.on_drag_callback(state_tree, callback_tree, scheduler, previous_pos, mouse_pos);
        true
    }
}
impl Slider {

    /// Initialize an instance of this object using the passed config coming from [ez_parser]
    pub fn from_config(config: Vec<String>, id: String, path: String, scheduler: &mut SchedulerFrontend,
                       file: String, line: usize) -> Self {

        let mut obj = Slider::new(id, path, scheduler);
        obj.load_ez_config(config, scheduler, file, line).unwrap();
        obj
    }

    fn value_from_mouse_pos(&self, state: &mut SliderState, mouse_pos: Coordinates) -> usize {
        let ratio = (state.get_max() - state.get_min()) as f64
            / state.get_effective_size().width as f64;
        let mut value = (ratio * mouse_pos.x as f64).round() as usize + state.get_min();

        // Make sure the set value is a multiple of step
        if value >= state.get_max() - state.get_step() ||
            mouse_pos.x == state.get_effective_size().width - 1 {
            value = state.get_max();
        } else if mouse_pos.x == 0 {
            value = state.get_min();
        } else {
            value -= value % state.get_step();
        }

        min(value, state.get_max())
    }

    fn handle_left(&self, state_tree: &mut StateTree, callback_tree: &mut CallbackTree,
                   scheduler: &mut SchedulerFrontend) {

        let state = state_tree.get_mut(&self.path).as_slider_mut();
        if state.get_value() == state.get_min() { return }
        state.set_value(state.get_value() - state.get_step());
        state.update(scheduler);
        self.on_value_change_callback(state_tree, callback_tree, scheduler);
    }
    fn handle_right(&self, state_tree: &mut StateTree, callback_tree: &mut CallbackTree,
                    scheduler: &mut SchedulerFrontend) {

        let state = state_tree.get_mut(&self.path).as_slider_mut();
        if state.get_value() == state.get_max() { return }
        state.set_value(state.get_value() + state.get_step());
        state.update(scheduler);
        self.on_value_change_callback(state_tree, callback_tree, scheduler);
    }
}
