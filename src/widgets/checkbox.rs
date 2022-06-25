//! # Checkbox Widget
//! Widget which is either on or off and implements an on_value_change callback.
use crate::widgets::ez_object::{EzObject};
use crate::states::checkbox_state::CheckboxState;
use crate::states::ez_state::{EzState, GenericState};
use crate::parser::load_properties::load_common_property;
use crate::parser::load_base_properties::load_ez_bool_property;
use crate::property::ez_values::EzValues;
use crate::run::definitions::{CallbackTree, Pixel, PixelMap, StateTree, WidgetTree};
use crate::run::tree::ViewTree;
use crate::scheduler::scheduler::Scheduler;
use crate::widgets::helper_functions::{add_border, add_padding};

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

    fn load_active_property(&mut self, parameter_value: &str, scheduler: &mut Scheduler) {

        let path = self.path.clone();
        self.state.set_active(load_ez_bool_property(
            parameter_value.trim(), scheduler, path.clone(),
            Box::new(move |state_tree: &mut StateTree, val: EzValues| {
                let state = state_tree.get_by_path_mut(&path).as_checkbox_mut();
                state.set_active(val.as_bool().to_owned());
                path.clone()
            })))
    }
}


impl EzObject for Checkbox {

    fn load_ez_parameter(&mut self, parameter_name: String, parameter_value: String,
                         scheduler: &mut Scheduler) {

        let consumed = load_common_property(
            &parameter_name, parameter_value.clone(),self, scheduler);
        if consumed { return }
        match parameter_name.as_str() {
            "active" => self.load_active_property(parameter_value.trim(), scheduler),
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
        let active_symbol = { if state.active.value {self.active_symbol}
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
            contents = add_border(contents, state.get_border_config());
        }
        let parent_colors = state.get_color_config();
        contents = add_padding(
            contents, state.get_padding(),parent_colors.background.value,
            parent_colors.foreground.value);
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
        state.set_active(!state.get_active().value);
        state.update(scheduler);
        self.on_value_change_callback(view_tree, state_tree, widget_tree, callback_tree,
                                      scheduler);
    }
}
