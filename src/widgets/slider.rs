//! A widget that displays text non-interactively.
use std::cmp::min;
use std::io::{Error, ErrorKind};
use crossterm::event::{Event, KeyCode};
use crate::EzContext;
use crate::states::ez_state::{EzState, GenericState};
use crate::widgets::ez_object::{EzObject};
use crate::parser::load_properties::load_common_property;
use crate::run::definitions::{CallbackTree, Coordinates, Pixel, PixelMap, StateTree, WidgetTree};
use crate::run::tree::ViewTree;
use crate::scheduler::scheduler::Scheduler;
use crate::states::slider_state::SliderState;
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
    fn new(id: String, path: String, scheduler: &mut Scheduler) -> Self {
        Slider {
            id,
            path: path.clone(),
            state: SliderState::new(path, scheduler),
        }
    }
}


impl EzObject for Slider {

    fn load_ez_parameter(&mut self, parameter_name: String, parameter_value: String,
                         scheduler: &mut Scheduler) -> Result<(), Error> {
        let consumed = load_common_property(
            &parameter_name, parameter_value.clone(), self, scheduler)?;
        if consumed { return Ok(())}
        match parameter_name.as_str() {
            "value" => self.state.set_value(match parameter_value.trim().parse() {
                Ok(i) => i,
                Err(_) => return Err(
                    Error::new(ErrorKind::InvalidData,
                    format!("Could not parse value parameter: \"{}\". Should be in format \
                    \"value: 10\"", parameter_value)))
            }),
            "minimum" => self.state.set_minimum(match parameter_value.trim().parse() {
                Ok(i) => i,
                Err(_) => return Err(
                    Error::new(ErrorKind::InvalidData,
                               format!("Could not parse minimum parameter: \"{}\". \
                               Should be in format \"minimum: 0\"", parameter_value)))
            }),
            "maximum" => self.state.set_maximum(match parameter_value.trim().parse() {
                Ok(i) => i,
                Err(_) => return Err(
                    Error::new(ErrorKind::InvalidData,
                               format!("Could not parse maximum parameter: \"{}\". \
                               Should be in format \"maximum: 100\"", parameter_value)))
            }),
            "step" => self.state.set_step(match parameter_value.trim().parse() {
                Ok(i) => i,
                Err(_) => return Err(
                    Error::new(ErrorKind::InvalidData,
                               format!("Could not parse step parameter: \"{}\". \
                               Should be in format \"step: 1\"", parameter_value)))
            }),
            _ => return Err(
                Error::new(ErrorKind::InvalidData,
                           format!("Invalid parameter name for slider: {}", parameter_name)))
        }
        Ok(())
    }

    fn set_id(&mut self, id: String) { self.id = id }

    fn get_id(&self) -> String { self.id.clone() }

    fn set_full_path(&mut self, path: String) { self.path = path }

    fn get_full_path(&self) -> String { self.path.clone() }

    fn get_state(&self) -> EzState { EzState::Slider(self.state.clone()) }

    fn get_state_mut(&mut self) -> &mut dyn GenericState { &mut self.state }

    fn get_contents(&self, state_tree: &mut StateTree) -> PixelMap {

        let state = state_tree.get_by_path_mut(&self.get_full_path())
            .as_slider_mut();
        state.size.height.set(1);
        if state.auto_scale.width.value {
            state.set_effective_width(((state.maximum.value - state.minimum.value) /
                state.step.value) as usize + 1);
        }

        let mut contents = PixelMap::new();
        let value_pos =
            ((state.get_effective_size().width - 1) as f64 *
            ((state.value.get() - state.minimum.value) as f64 /
                (state.maximum.value - state.minimum.value) as f64))
                as usize;
        for x in 0..state.get_effective_size().width {
            let fg_color =
                if state.disabled.value {state.get_color_config().disabled_foreground.value }
                else if x == value_pos &&
                    state.selected { state.get_color_config().selection_foreground.value }
                else { state.get_color_config().foreground.value };
            let bg_color =
                if state.disabled.value {state.get_color_config().disabled_background.value }
                else {state.get_color_config().background.value};
            contents.push(vec!(Pixel::new(
                if x == value_pos { "🮚".to_string() } else { "━".to_string() },
                fg_color, bg_color)));
        }

        let state = state_tree.get_by_path(&self.get_full_path()).as_slider();
        let parent_colors = state_tree.get_by_path(self.get_full_path()
            .rsplit_once('/').unwrap().0).as_generic().get_color_config();
        contents = add_padding(
            contents, state.get_padding(), parent_colors.background.value,
            parent_colors.foreground.value);
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

        let state = state_tree.get_by_path_mut(&self.path).as_slider_mut();
        let ratio = (state.maximum.value - state.minimum.value) as f64
            / state.get_effective_size().width as f64;
        let mut value = (ratio * mouse_pos.x as f64) as usize + state.minimum.value;
        value -= value % state.step.value;
        value = min(value, state.maximum.value);
        state.set_value(value);
        state.update(scheduler);
        self.on_value_change_callback(view_tree, state_tree, widget_tree, callback_tree,
                                      scheduler);
        true
    }
}
impl Slider {

    /// Initialize an instance of this object using the passed config coming from [ez_parser]
    pub fn from_config(config: Vec<String>, id: String, path: String, scheduler: &mut Scheduler,
                       file: String, line: usize) -> Self {

        let mut obj = Slider::new(id, path, scheduler);
        obj.load_ez_config(config, scheduler, file, line).unwrap();
        obj
    }

    fn handle_left(&self, view_tree: &mut ViewTree, state_tree: &mut StateTree,
                   widget_tree: &WidgetTree, callback_tree: &mut CallbackTree,
                   scheduler: &mut Scheduler) {
        let state = state_tree.get_by_path_mut(&self.path).as_slider_mut();
        if state.value == state.minimum { return }
        state.set_value(state.get_value().value - state.get_step().value);
        state.update(scheduler);
        if let Some(ref mut i ) = callback_tree
            .get_by_path_mut(&self.get_full_path()).on_value_change {
            i(EzContext::new(self.get_full_path(),
                             view_tree, state_tree, widget_tree, scheduler));
        }
    }
    fn handle_right(&self, view_tree: &mut ViewTree, state_tree: &mut StateTree,
                   widget_tree: &WidgetTree, callback_tree: &mut CallbackTree,
                   scheduler: &mut Scheduler) {

        let state = state_tree.get_by_path_mut(&self.path).as_slider_mut();
        if state.value == state.maximum { return }
        state.set_value(state.get_value().value + state.get_step().value);
        state.update(scheduler);
        self.on_value_change_callback(view_tree, state_tree, widget_tree, callback_tree,
                                      scheduler);
    }
}
