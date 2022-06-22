//! # Checkbox Widget
//! Widget which is either on or off and implements an on_value_change callback.
use crate::common;
use crate::common::definitions::{CallbackTree, PixelMap, StateTree, ViewTree, WidgetTree};
use crate::widgets::widget::{Pixel, EzObject};
use crate::states::checkbox_state::CheckboxState;
use crate::states::state::{EzState, GenericState};
use crate::parser;
use crate::scheduler::Scheduler;

#[derive(Clone, Debug)]
pub struct Checkbox {

    /// ID of the widget, used to construct [path]
    pub id: String,

    /// Full path to this widget, e.g. "/root_layout/layout_2/THIS_ID"
    pub path: String,

    /// [Pixel.symbol] used when the Checkbox is active
    pub active_symbol: char,

    /// [Pixel.symbol] used when the Checkbox is not active
    pub inactive_symbol: char,

    /// Runtime state of this widget, see [CheckboxState] and [State]
    pub state: CheckboxState,
}

impl Checkbox {
    pub fn new(id: String, path: String, scheduler: &mut Scheduler) -> Self {

        Checkbox {
            id,
            path: path.clone(),
            active_symbol: 'X',
            inactive_symbol: ' ',
            state: CheckboxState::new(path, scheduler),
        }
    }
}


impl EzObject for Checkbox {

    fn load_ez_parameter(&mut self, parameter_name: String, parameter_value: String,
                         scheduler: &mut Scheduler) {

        let consumed = parser::load_common_parameters(
            &parameter_name, parameter_value.clone(),self, scheduler);
        if consumed { return }
        match parameter_name.as_str() {
            "active" =>
                self.state.active = parser::load_bool_parameter(parameter_value.trim()),
            "active_symbol" => self.active_symbol = parameter_value.chars().last().unwrap(),
            "inactive_symbol" => self.inactive_symbol = parameter_value.chars().last().unwrap(),
            _ => panic!("Invalid parameter name for check box {}", parameter_name)
        }
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
        let active_symbol = { if state.active {self.active_symbol}
                              else {self.inactive_symbol} };

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
        if state.get_border_config().enabled.value {
            contents = common::widget_functions::add_border(
                contents, state.get_border_config());
        }
        let parent_colors = state.get_color_config();
        contents = common::widget_functions::add_padding(
            contents, state.get_padding(),parent_colors.background,
            parent_colors.foreground);
        contents
    }

    fn on_press(&self, view_tree: &mut ViewTree, state_tree: &mut StateTree,
                widget_tree: &WidgetTree, callback_tree: &mut CallbackTree,
                scheduler: &mut Scheduler) -> bool {
        self.handle_toggle(view_tree, state_tree, widget_tree, callback_tree, scheduler);
        true
    }
}
impl Checkbox {

    /// Initialize an instance of this object using the passed config coming from [ez_parser]
    pub fn from_config(config: Vec<String>, id: String, path: String, scheduler: &mut Scheduler)
                       -> Self {

        let mut obj = Checkbox::new(id, path, scheduler);
        obj.load_ez_config(config, scheduler).unwrap();
        obj
    }

    fn handle_toggle(&self, view_tree: &mut ViewTree, state_tree: &mut StateTree,
                           widget_tree: &WidgetTree, callback_tree: &mut CallbackTree,
                           scheduler: &mut Scheduler) {

        let state = state_tree.get_by_path_mut(&self.get_full_path())
            .as_checkbox_mut();
        state.set_active(!state.get_active());
        state.update(scheduler);
        self.on_value_change_callback(view_tree, state_tree, widget_tree, callback_tree,
                                      scheduler);
    }
}
