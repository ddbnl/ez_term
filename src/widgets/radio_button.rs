//! # Radio button Widget
//! Widget which can only be turned on. It should be in a group of other radio buttons using the
//! same 'group' field value for all. The radio buttons in a group are mutually exlusive, so when
//! one is selected the others are deselected. Supports on_value_change callback, which is only
//! called for the radio button that became active.
use std::io::{Error, ErrorKind};

use crate::parser::load_base_properties;
use crate::parser::load_common_properties::load_common_property;
use crate::run::definitions::{CallbackTree, Coordinates, Pixel, PixelMap, StateTree};
use crate::scheduler::definitions::CustomDataMap;
use crate::scheduler::scheduler::SchedulerFrontend;
use crate::states::ez_state::{EzState, GenericState};
use crate::states::radio_button_state::RadioButtonState;
use crate::widgets::ez_object::{EzObject, EzObjects};
use crate::widgets::helper_functions::{add_border, add_padding};

#[derive(Clone, Debug)]
pub struct RadioButton {
    /// ID of the widget, used to construct [path]
    pub id: String,

    /// Full path to this widget, e.g. "/root_layout/layout_2/THIS_ID"
    pub path: String,

    /// Runtime state of this widget, see [RadioButtonState] and [State]
    pub state: RadioButtonState,
}

impl RadioButton {
    pub fn new(id: String, path: String, scheduler: &mut SchedulerFrontend) -> Self {
        RadioButton {
            id,
            path: path.clone(),

            state: RadioButtonState::new(path, scheduler),
        }
    }

    pub fn from_state(
        id: String,
        path: String,
        _scheduler: &mut SchedulerFrontend,
        state: EzState,
    ) -> Self {
        RadioButton {
            id,
            path: path.clone(),
            state: state.as_radio_button().to_owned(),
        }
    }
}

impl EzObject for RadioButton {
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
            "group" => {
                let group = parameter_value.trim();
                if group.is_empty() {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        format!("Radio button widget must have a group."),
                    ));
                }
                load_base_properties::load_string_property(
                    parameter_value.trim(),
                    scheduler,
                    self.path.clone(),
                    &parameter_name,
                    self.get_state_mut(),
                )?;
            }
            "active" => load_base_properties::load_bool_property(
                parameter_value.trim(),
                scheduler,
                self.path.clone(),
                &parameter_name,
                self.get_state_mut(),
            )?,
            "active_symbol" => load_base_properties::load_string_property(
                parameter_value.trim(),
                scheduler,
                self.path.clone(),
                &parameter_name,
                self.get_state_mut(),
            )?,
            "inactive_symbol" => load_base_properties::load_string_property(
                parameter_value.trim(),
                scheduler,
                self.path.clone(),
                &parameter_name,
                self.get_state_mut(),
            )?,
            _ => panic!(
                "Invalid parameter name for radio button: {}",
                parameter_name
            ),
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
        EzState::RadioButton(self.state.clone())
    }

    fn get_state_mut(&mut self) -> &mut dyn GenericState {
        &mut self.state
    }

    fn get_contents(&self, state_tree: &mut StateTree) -> PixelMap {
        let state = state_tree.get_mut(&self.get_path()).as_radio_button_mut();
        state.set_width(5);
        state.set_height(1);
        let active_symbol = if state.get_active() {
            state.get_active_symbol()
        } else {
            state.get_inactive_symbol()
        };
        let (fg_color, bg_color) = state.get_context_colors();
        let mut contents = vec![
            vec![Pixel::new(
                "(".to_string(),
                fg_color,bg_color)],
            vec![Pixel::new(
                " ".to_string(),
                fg_color,bg_color)],
            vec![Pixel::new(
                active_symbol.to_string(),
                fg_color,bg_color)],
            vec![Pixel::new(
                " ".to_string(),
                fg_color,bg_color)],
            vec![Pixel::new(
                ")".to_string(),
                fg_color,
                bg_color)],
        ];
        if state.get_border_config().get_border() {
            contents = add_border(
                contents,
                state.get_border_config(),
                state.get_color_config(),
            );
        }
        let state = state_tree.get(&self.get_path()).as_radio_button();
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

    fn on_press(
        &self,
        state_tree: &mut StateTree,
        callback_tree: &mut CallbackTree,
        scheduler: &mut SchedulerFrontend,
        custom_data: &mut CustomDataMap,
    ) -> bool {
        let consumed = self.on_press_callback(state_tree, callback_tree, scheduler, custom_data);
        if consumed {
            return consumed;
        }
        self.handle_press(state_tree, callback_tree, scheduler, custom_data);
        true
    }

    fn on_hover(
        &self,
        state_tree: &mut StateTree,
        callback_tree: &mut CallbackTree,
        scheduler: &mut SchedulerFrontend,
        mouse_pos: Coordinates,
        custom_data: &mut CustomDataMap,
    ) -> bool {
        let consumed = self.on_hover_callback(state_tree, callback_tree, scheduler, mouse_pos,
                                              custom_data);
        if consumed {
            return consumed;
        }
        scheduler.set_selected_widget(&self.path, Some(mouse_pos));
        true
    }

    fn get_clone(&self, scheduler: &mut SchedulerFrontend) -> EzObjects {
        let mut clone = self.clone();
        let mut new_state = RadioButtonState::new(self.path.clone(), scheduler);
        new_state.copy_state_values(self.get_state());
        clone.state = new_state;
        EzObjects::RadioButton(clone)
    }
}
impl RadioButton {
    /// Initialize an instance of this object using the passed config coming from [ez_parser]
    pub fn from_config(
        config: Vec<String>,
        id: String,
        path: String,
        scheduler: &mut SchedulerFrontend,
        file: String,
        line: usize,
    ) -> Self {
        let mut obj = RadioButton::new(id, path, scheduler);
        obj.load_ez_config(config, scheduler, file, line).unwrap();
        obj
    }

    /// Function that handles this RadioButton being pressed (mouse clicked/keyboard entered).
    fn handle_press(
        &self,
        state_tree: &mut StateTree,
        callback_tree: &mut CallbackTree,
        scheduler: &mut SchedulerFrontend,
        custom_data: &mut CustomDataMap,
    ) {
        // Find all other radio buttons in same group and make them inactive (mutual exclusivity)
        let group_name = state_tree.get(&self.path).as_radio_button().get_group();
        for state in state_tree.get_all_mut().iter_mut() {
            if let EzState::RadioButton(ref mut i) = state {
                if i.get_group() == group_name && i.get_path() != &self.get_path() {
                    i.set_active(false);
                }
            }
        }
        // Set entered radio button to active and select it
        let state = state_tree.get_mut(&self.get_path()).as_radio_button_mut();
        if !state.get_active() {
            state.set_active(true);
            state.update(scheduler);
            self.on_value_change_callback(state_tree, callback_tree, scheduler, custom_data);
        } else {
            return; // Nothing to do
        }
    }
}
