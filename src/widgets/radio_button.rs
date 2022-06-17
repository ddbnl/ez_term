//! # Radio button Widget
//! Widget which can only be turned on. It should be in a group of other radio buttons using the
//! same 'group' field value for all. The radio buttons in a group are mutually exlusive, so when
//! one is selected the others are deselected. Supports on_value_change callback, which is only
//! called for the radio button that became active.
use crate::common;
use crate::common::definitions::{CallbackTree, StateTree, ViewTree, WidgetTree};
use crate::states::radio_button_state::RadioButtonState;
use crate::states::state::{EzState, GenericState};
use crate::widgets::widget::{Pixel, EzObject};
use crate::ez_parser;
use crate::scheduler::Scheduler;


#[derive(Clone)]
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

impl Default for RadioButton {
    fn default() -> Self {
        RadioButton {
            id: "".to_string(),
            path: String::new(),
            active_symbol: 'X',
            inactive_symbol: ' ',
            state: RadioButtonState::default(),
        }
    }
}


impl EzObject for RadioButton {

    fn load_ez_parameter(&mut self, parameter_name: String, parameter_value: String) {

        let consumed = ez_parser::load_common_parameters(
            &parameter_name, parameter_value.clone(),Box::new(self));
        if consumed { return }
        match parameter_name.as_str() {
            "group" => {
                let group = parameter_value.trim();
                if group.is_empty() { panic!("Radio button widget must have a group.") }
                self.state.group = group.to_string();
            },
            "active" =>
                self.state.active = ez_parser::load_bool_parameter(parameter_value.trim()),
            "active_symbol" => self.active_symbol = parameter_value.chars().last().unwrap(),
            "inactive_symbol" => self.inactive_symbol = parameter_value.chars().last().unwrap(),
            _ => panic!("Invalid parameter name for radio button {}", parameter_name)
        }
    }

    fn set_id(&mut self, id: String) { self.id = id }

    fn get_id(&self) -> String { self.id.clone() }

    fn set_full_path(&mut self, path: String) { self.path = path }

    fn get_full_path(&self) -> String { self.path.clone() }

    fn get_state(&self) -> EzState { EzState::RadioButton(self.state.clone()) }
    
    fn get_state_mut(&mut self) -> Box<&mut dyn GenericState>{ Box::new(&mut self.state) }
    
    fn get_contents(&self, state_tree: &mut StateTree) -> common::definitions::PixelMap {

        let state = state_tree
            .get_mut(&self.get_full_path()).unwrap().as_radio_button();
        let active_symbol = { if state.active {self.active_symbol}
                                    else {self.inactive_symbol} };
        let fg_color = if state.selected {state.get_color_config().selection_foreground }
        else {state.get_color_config().foreground };
        let bg_color = if state.selected {state.get_color_config().selection_background }
        else {state.get_color_config().background };
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
        if state.get_border_config().enabled {
            contents = common::widget_functions::add_border(
                contents, state.get_border_config());
        }
        let state = state_tree
            .get(&self.get_full_path()).unwrap().as_radio_button();
        let parent_colors = state_tree.get(self.get_full_path()
            .rsplit_once('/').unwrap().0).unwrap().as_generic().get_color_config();
        contents = common::widget_functions::add_padding(
            contents, state.get_padding(), parent_colors.background,
            parent_colors.foreground);
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
    pub fn from_config(config: Vec<String>) -> Self {
        let mut obj = RadioButton::default();
        obj.load_ez_config(config).unwrap();
        obj
    }

    /// Function that handles this RadioButton being pressed (mouse clicked/keyboard entered).
    fn handle_press(&self, view_tree: &mut ViewTree, state_tree: &mut StateTree,
                    widget_tree: &WidgetTree, callback_tree: &mut CallbackTree,
                    scheduler: &mut Scheduler) {

        // Find all other radio buttons in same group and make them inactive (mutual exclusivity)
        for (path, state) in state_tree.iter_mut() {
            if let EzState::RadioButton(ref mut i) = state {
                if i.get_group() == state.as_radio_button().group && path != &self.get_full_path() {
                    state.as_radio_button_mut().set_active(false);
                }
            }
        }
        // Set entered radio button to active and select it
        let state = state_tree.get_mut(&self.get_full_path()).unwrap()
            .as_radio_button_mut();
        if !state.active {
            state.set_active(true);
            if let Some(ref mut i) = callback_tree
                .get_mut(&self.get_full_path()).unwrap().on_value_change {
                let context = common::definitions::EzContext::new(
                    self.get_full_path(), view_tree, state_tree, widget_tree,
                    scheduler);
                i(context);
            }
        } else {
            return // Nothing to do
        }
    }
}
