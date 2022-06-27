//! # Radio button Widget
//! Widget which can only be turned on. It should be in a group of other radio buttons using the
//! same 'group' field value for all. The radio buttons in a group are mutually exlusive, so when
//! one is selected the others are deselected. Supports on_value_change callback, which is only
//! called for the radio button that became active.
use std::io::{Error, ErrorKind};
use crate::states::radio_button_state::RadioButtonState;
use crate::states::ez_state::{EzState, GenericState};
use crate::widgets::ez_object::{EzObject};
use crate::parser::load_properties::load_common_property;
use crate::parser::load_base_properties::{load_ez_bool_property, load_ez_string_property};
use crate::property::ez_values::EzValues;
use crate::run::definitions::{CallbackTree, Pixel, PixelMap, StateTree, WidgetTree};
use crate::run::tree::ViewTree;
use crate::scheduler::scheduler::Scheduler;
use crate::widgets::helper_functions::{add_border, add_padding};


#[derive(Clone, Debug)]
pub struct RadioButton {

    /// ID of the widget, used to construct [path]
    pub id: String,

    /// Full path to this widget, e.g. "/root_layout/layout_2/THIS_ID"
    pub path: String,

    /// [Pixel.symbol] used when the Checkbox is active
    pub active_symbol: char,

    /// [Pixel.symbol] used when the Checkbox is not active
    pub inactive_symbol: char,

    /// Runtime state of this widget, see [RadioButtonState] and [State]
    pub state: RadioButtonState,
}

impl RadioButton {
    fn new(id: String, path: String, scheduler: &mut Scheduler) -> Self {
        RadioButton {
            id,
            path: path.clone(),
            active_symbol: 'X',
            inactive_symbol: ' ',
            state: RadioButtonState::new(path, scheduler),
        }
    }

    fn load_group_property(&mut self, parameter_value: &str, scheduler: &mut Scheduler)
        -> Result<(), Error> {

        let path = self.path.clone();
        self.state.group.set(load_ez_string_property(
            parameter_value.trim(), scheduler, path.clone(),
            Box::new(move |state_tree: &mut StateTree, val: EzValues| {
                let state = state_tree.get_by_path_mut(&path)
                    .as_radio_button_mut();
                state.group.set(val.as_string().to_owned());
                path.clone()
            }))?);
        Ok(())
    }

    fn load_active_property(&mut self, parameter_value: &str, scheduler: &mut Scheduler)
        -> Result<(), Error> {

        let path = self.path.clone();
        self.state.set_active(load_ez_bool_property(
            parameter_value.trim(), scheduler, path.clone(),
            Box::new(move |state_tree: &mut StateTree, val: EzValues| {
                let state = state_tree.get_by_path_mut(&path)
                    .as_radio_button_mut();
                state.set_active(val.as_bool().to_owned());
                path.clone()
            }))?);
        Ok(())
    }
}


impl EzObject for RadioButton {

    fn load_ez_parameter(&mut self, parameter_name: String, parameter_value: String,
                         scheduler: &mut Scheduler) -> Result<(), Error> {

        let consumed = load_common_property(
            &parameter_name, parameter_value.clone(),self, scheduler)?;
        if consumed { return Ok(())}
        match parameter_name.as_str() {
            "group" => {
                let group = parameter_value.trim();
                if group.is_empty() {
                    return Err(Error::new(ErrorKind::InvalidData,
                                          format!("Radio button widget must have a group.")))
                }
                self.load_group_property(group, scheduler)?;
            },
            "active" => self.load_active_property(parameter_value.trim(), scheduler)?,
            "active_symbol" => self.active_symbol = match parameter_value.chars().last() {
                Some(i) => i,
                None => return Err(
                    Error::new(ErrorKind::InvalidData,
                               format!("Invalid value for active_symbol: \"{}\". \
                               Required format is \"active_symbol: x\"", parameter_value)))
            },
            "inactive_symbol" => self.inactive_symbol =
                match parameter_value.chars().last() {
                    Some(i) => i,
                    None => return Err(
                        Error::new(ErrorKind::InvalidData,
                                   format!("Invalid value for inactive symbol: \"{}\". \
                                   Required format \
                                   is \"inactive_symbol: -\"", parameter_value)))
                },
            _ => panic!("Invalid parameter name for radio button: {}", parameter_name)
        }
        Ok(())
    }

    fn set_id(&mut self, id: String) { self.id = id }

    fn get_id(&self) -> String { self.id.clone() }

    fn set_full_path(&mut self, path: String) { self.path = path }

    fn get_full_path(&self) -> String { self.path.clone() }

    fn get_state(&self) -> EzState { EzState::RadioButton(self.state.clone()) }
    
    fn get_state_mut(&mut self) -> &mut dyn GenericState { &mut self.state }
    
    fn get_contents(&self, state_tree: &mut StateTree) -> PixelMap {

        let state = state_tree
            .get_by_path_mut(&self.get_full_path()).as_radio_button_mut();
        state.set_width(5);
        state.set_height(1);
        let active_symbol = { if state.active.value {self.active_symbol}
                                    else {self.inactive_symbol} };
        let (fg_color, bg_color) = state.get_context_colors();
        let mut contents = vec!(
            vec!(Pixel {symbol: "(".to_string(), foreground_color: fg_color,
                background_color: bg_color, underline: false}),
            vec!(Pixel {symbol: " ".to_string(), foreground_color: fg_color,
                background_color: bg_color, underline: false}),
            vec!(Pixel {symbol: active_symbol.to_string(), foreground_color: fg_color,
                background_color: bg_color, underline: false}),
            vec!(Pixel {symbol: " ".to_string(), foreground_color: fg_color,
                background_color: bg_color, underline: false}),
            vec!(Pixel {symbol: ")".to_string(), foreground_color: fg_color,
                background_color: bg_color, underline: false}),
        );
        if state.get_border_config().enabled.value {
            contents = add_border(contents, state.get_border_config());
        }
        let state = state_tree
            .get_by_path(&self.get_full_path()).as_radio_button();
        let parent_colors = state_tree.get_by_path(self.get_full_path()
            .rsplit_once('/').unwrap().0).as_generic().get_color_config();
        contents = add_padding(
            contents, state.get_padding(), parent_colors.background.value,
            parent_colors.foreground.value);
        contents
    }

    fn on_press(&self, view_tree: &mut ViewTree, state_tree: &mut StateTree,
                widget_tree: &WidgetTree, callback_tree: &mut CallbackTree,
                scheduler: &mut Scheduler) -> bool {
        self.handle_press(view_tree, state_tree, widget_tree, callback_tree, scheduler);
        true
    }
}
impl RadioButton {

    /// Initialize an instance of this object using the passed config coming from [ez_parser]
    pub fn from_config(config: Vec<String>, id: String, path: String, scheduler: &mut Scheduler,
                       file: String, line: usize) -> Self {

        let mut obj = RadioButton::new(id, path, scheduler);
        obj.load_ez_config(config, scheduler, file, line).unwrap();
        obj
    }

    /// Function that handles this RadioButton being pressed (mouse clicked/keyboard entered).
    fn handle_press(&self, view_tree: &mut ViewTree, state_tree: &mut StateTree,
                    widget_tree: &WidgetTree, callback_tree: &mut CallbackTree,
                    scheduler: &mut Scheduler) {

        // Find all other radio buttons in same group and make them inactive (mutual exclusivity)
        let group_name =
            state_tree.get_by_path(&self.path).as_radio_button().group.value.clone();
        for (path, state) in state_tree.objects.iter_mut() {
            if let EzState::RadioButton(ref mut i) = state {
                if i.get_group().value == group_name && path != &self.get_full_path() {
                    state.as_radio_button_mut().set_active(false);
                }
            }
        }
        // Set entered radio button to active and select it
        let state = state_tree.get_by_path_mut(&self.get_full_path())
            .as_radio_button_mut();
        if !state.active.value {
            state.set_active(true);
            state.update(scheduler);
            self.on_value_change_callback(view_tree, state_tree, widget_tree, callback_tree,
                                          scheduler);
        } else {
            return // Nothing to do
        }
    }
}
