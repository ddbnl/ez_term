//! # Checkbox Widget
//! Widget which is either on or off and implements an on_value_change callback.
use std::io::{Error, ErrorKind};

use crate::parser::load_base_properties::{load_ez_bool_property, load_ez_string_property};
use crate::parser::load_common_properties::load_common_property;
use crate::property::ez_values::EzValues;
use crate::run::definitions::{CallbackTree, Coordinates, Pixel, PixelMap, StateTree};
use crate::scheduler::scheduler::SchedulerFrontend;
use crate::states::checkbox_state::CheckboxState;
use crate::states::ez_state::{EzState, GenericState};
use crate::widgets::ez_object::EzObject;
use crate::widgets::helper_functions::{add_border, add_padding};


#[derive(Clone, Debug)]
pub struct Checkbox {

    /// ID of the widget, used to construct [path]
    pub id: String,

    /// Full path to this widget, e.g. "/root_layout/layout_2/THIS_ID"
    pub path: String,

    /// Runtime state of this widget, see [CheckboxState] and [State]
    pub state: CheckboxState,
}

impl Checkbox {

    pub fn new(id: String, path: String, scheduler: &mut SchedulerFrontend) -> Self {

        Checkbox {
            id,
            path: path.clone(),
            state: CheckboxState::new(path, scheduler),
        }
    }

    pub fn from_state(id: String, path: String, _scheduler: &mut SchedulerFrontend, state: EzState) -> Self {
        Checkbox {
            id,
            path: path.clone(),
            state: state.as_checkbox().to_owned(),
        }
    }


    fn load_active_property(&mut self, parameter_value: &str, scheduler: &mut SchedulerFrontend)
        -> Result<(), Error>{

        let path = self.path.clone();
        self.state.set_active(load_ez_bool_property(
            parameter_value.trim(), scheduler, path.clone(),
            Box::new(move |state_tree: &mut StateTree, val: EzValues| {
                let state = state_tree.get_by_path_mut(&path).as_checkbox_mut();
                state.set_active(val.as_bool().to_owned());
                path.clone()
            }))?);
        Ok(())
    }

    fn load_active_symbol_property(&mut self, parameter_value: &str, scheduler: &mut SchedulerFrontend)
        -> Result<(), Error>{

        let path = self.path.clone();
        self.state.set_active_symbol(load_ez_string_property(
            parameter_value.trim(), scheduler, path.clone(),
            Box::new(move |state_tree: &mut StateTree, val: EzValues| {
                let state = state_tree.get_by_path_mut(&path).as_checkbox_mut();
                state.set_active_symbol(val.as_string().to_owned());
                path.clone()
            }))?);
        Ok(())
    }

    fn load_inactive_symbol_property(&mut self, parameter_value: &str, scheduler: &mut SchedulerFrontend)
                                   -> Result<(), Error>{

        let path = self.path.clone();
        self.state.set_inactive_symbol(load_ez_string_property(
            parameter_value.trim(), scheduler, path.clone(),
            Box::new(move |state_tree: &mut StateTree, val: EzValues| {
                let state = state_tree.get_by_path_mut(&path).as_checkbox_mut();
                state.set_inactive_symbol(val.as_string().to_owned());
                path.clone()
            }))?);
        Ok(())
    }
}


impl EzObject for Checkbox {

    fn load_ez_parameter(&mut self, parameter_name: String, parameter_value: String,
                         scheduler: &mut SchedulerFrontend) -> Result<(), Error> {

        let consumed = load_common_property(
            &parameter_name, parameter_value.clone(),self, scheduler)?;
        if consumed { return Ok(())}
        match parameter_name.as_str() {
            "active" => self.load_active_property(parameter_value.trim(), scheduler)?,
            "active_symbol" => self.load_active_symbol_property(
                &parameter_value.chars().last().unwrap().to_string(), scheduler)?,
            "inactive_symbol" => self.load_inactive_symbol_property(
                &parameter_value.chars().last().unwrap().to_string(), scheduler)?,
            _ => return Err(
                Error::new(ErrorKind::InvalidData,
                           format!("Invalid parameter name for checkbox: {}", parameter_name)))
        }
        Ok(())
    }

    fn set_id(&mut self, id: String) { self.id = id }

    fn get_id(&self) -> String { self.id.clone() }

    fn set_full_path(&mut self, path: String) { self.path = path }

    fn get_full_path(&self) -> String { self.path.clone() }

    fn get_state(&self) -> EzState { EzState::Checkbox(self.state.clone()) }

    fn get_state_mut(&mut self) -> &mut dyn GenericState{ &mut self.state }

    fn get_contents(&self, state_tree: &mut StateTree) -> PixelMap {

        let state = state_tree.get_by_path_mut(&self.get_full_path())
            .as_checkbox_mut();
        state.set_width(5);
        state.set_height(1);
        let active_symbol =
            if state.get_active() { state.get_active_symbol() }
            else { state.get_inactive_symbol() };

        let (fg_color, bg_color) = state.get_context_colors();
        let mut contents = vec!(
            vec!(Pixel {symbol: "[".to_string(), foreground_color: fg_color,
                background_color: bg_color, underline: false}),
            vec!(Pixel {symbol: " ".to_string(), foreground_color: fg_color,
                background_color: bg_color, underline: false}),
            vec!(Pixel { symbol: active_symbol.to_string(), foreground_color: fg_color,
                background_color: bg_color, underline: false}),
            vec!(Pixel { symbol: " ".to_string(), foreground_color: fg_color,
                background_color: bg_color, underline: false}),
            vec!(Pixel { symbol: "]".to_string(), foreground_color: fg_color,
                background_color: bg_color, underline: false}),
        );
        if state.get_border_config().get_enabled() {
            contents = add_border(contents, state.get_border_config(),
                            state.get_color_config());
        }
        let parent_colors = state.get_color_config();
        contents = add_padding(
            contents, state.get_padding(),parent_colors.get_background(),
            parent_colors.get_foreground());
        contents
    }

    fn on_press(&self, state_tree: &mut StateTree, callback_tree: &mut CallbackTree,
                scheduler: &mut SchedulerFrontend) -> bool {

        self.handle_toggle(state_tree, callback_tree, scheduler);
        true
    }

    fn on_hover(&self, state_tree: &mut StateTree, callback_tree: &mut CallbackTree,
                scheduler: &mut SchedulerFrontend, mouse_pos: Coordinates) -> bool {

        scheduler.set_selected_widget(&self.path, Some(mouse_pos));
        self.on_hover_callback(state_tree, callback_tree, scheduler, mouse_pos);
        true
    }
}
impl Checkbox {

    /// Initialize an instance of this object using the passed config coming from [ez_parser]
    pub fn from_config(config: Vec<String>, id: String, path: String, scheduler: &mut SchedulerFrontend,
                       file: String, line: usize) -> Self {

        let mut obj = Checkbox::new(id, path, scheduler);
        obj.load_ez_config(config, scheduler, file, line).unwrap();
        obj
    }

    fn handle_toggle(&self, state_tree: &mut StateTree, callback_tree: &mut CallbackTree,
                     scheduler: &mut SchedulerFrontend) {

        let state = state_tree.get_by_path_mut(&self.get_full_path())
            .as_checkbox_mut();
        state.set_active(!state.get_active());
        state.update(scheduler);
        self.on_value_change_callback(state_tree, callback_tree, scheduler);
    }
}
