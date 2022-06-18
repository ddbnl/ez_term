//! A widget that displays text non-interactively.
use std::time::Duration;
use crate::states::state::{EzState, GenericState};
use crate::states::button_state::ButtonState;
use crate::states;
use crate::common;
use crate::widgets::widget::{Pixel, EzObject};
use crate::ez_parser;
use crate::scheduler::Scheduler;

#[derive(Clone)]
pub struct Button {

    /// ID of the widget, used to construct [path]
    pub id: String,

    /// Full path to this widget, e.g. "/root_layout/layout_2/THIS_ID"
    pub path: String,

    /// Runtime state of this widget, see [LabelState] and [State]
    pub state: ButtonState,
}

impl Default for Button {
    fn default() -> Self {
        Button {
            id: "".to_string(),
            path: String::new(),
            state: ButtonState::default(),
        }
    }
}


impl EzObject for Button {

    fn load_ez_parameter(&mut self, parameter_name: String, parameter_value: String) {
        
        let consumed = ez_parser::load_common_parameters(
            &parameter_name, parameter_value.clone(),Box::new(self));
        if consumed { return }
        match parameter_name.as_str() {
            "text" => self.state.set_text(
                ez_parser::load_text_parameter(parameter_value.as_str())),
            _ => panic!("Invalid parameter name for button {}", parameter_name)
        }
    }


    fn set_id(&mut self, id: String) { self.id = id }

    fn get_id(&self) -> String { self.id.clone() }

    fn set_full_path(&mut self, path: String) { self.path = path }

    fn get_full_path(&self) -> String { self.path.clone() }

    fn get_state(&self) -> EzState { EzState::Button(self.state.clone()) }

    fn get_state_mut(&mut self) -> Box<&mut dyn GenericState>{ Box::new(&mut self.state) }

    fn get_contents(&self, state_tree: &mut common::definitions::StateTree) -> common::definitions::PixelMap {

        let state = state_tree.get_mut(&self.get_full_path()).unwrap()
            .as_button_mut();

        let (fg_color, bg_color) =
            if state.flashing {(state.get_color_config().flash_foreground,
                                state.get_color_config().flash_background)}
            else { state.get_context_colors() };

        let text = state.text.clone();

        let write_width = if state.get_effective_size().infinite_width ||
            state.get_auto_scale().width { text.len() + 1 }
            else {state.get_effective_size().width };
        let content_lines = common::widget_functions::wrap_text(text, write_width);
        let write_height =
            if state.get_effective_size().infinite_height || state.get_auto_scale().height { content_lines.len() }
            else {state.get_effective_size().height };

        let longest_line = content_lines.iter().map(|x| x.len()).max();
        let longest_line = if let Some(i) = longest_line { i } else { 0 };
        let mut contents = Vec::new();
        for x in 0..longest_line {
            let mut new_y = Vec::new();
            for y in 0..write_height {
                if y < content_lines.len() && x < content_lines[y].len() {
                    new_y.push(Pixel { symbol: content_lines[y][x..x+1].to_string(),
                        foreground_color: fg_color, background_color: bg_color, underline: false })
                }
            }
            contents.push(new_y);
        }
        if state.get_auto_scale().width {
            state.set_effective_width(contents.len());
        }
        if state.get_auto_scale().height {
            state.set_effective_height(contents[0].len());
        }
        (contents, _) = common::widget_functions::align_content_horizontally(
            contents,states::definitions::HorizontalAlignment::Center,
            state.get_effective_size().width, fg_color, bg_color);
        (contents, _) = common::widget_functions::align_content_vertically(
            contents,states::definitions::VerticalAlignment::Middle,
            state.get_effective_size().height, fg_color, bg_color);
        contents = common::widget_functions::add_border(
            contents, state.get_border_config());
        let state = state_tree.get(&self.get_full_path()).unwrap().as_button();
        let parent_colors = state_tree.get(self.get_full_path()
            .rsplit_once('/').unwrap().0).unwrap().as_generic().get_color_config();
        contents = common::widget_functions::add_padding(
            contents, state.get_padding(), parent_colors.background,
            parent_colors.foreground);
        contents
    }

    fn on_press(&self, view_tree: &mut common::definitions::ViewTree,
                state_tree: &mut common::definitions::StateTree,
                widget_tree: &common::definitions::WidgetTree,
                callback_tree: &mut common::definitions::CallbackTree, scheduler: &mut Scheduler)
    -> bool {

        let context = common::definitions::EzContext::new(
            self.get_full_path().clone(), view_tree, state_tree, widget_tree, scheduler);

        let mut consumed = false;
        if let Some(ref mut i) = callback_tree
            .get_mut(&self.get_full_path()).unwrap().on_press {
            consumed = i(context);
        }
        if !consumed {
            self.handle_on_press(state_tree, scheduler);
            return true
        }
        consumed
    }
}
impl Button {

    /// Initialize an instance of this object using the passed config coming from [ez_parser]
    pub fn from_config(config: Vec<String>) -> Self {
        let mut obj = Button::default();
        obj.load_ez_config(config).unwrap();
        obj.state.border_config.enabled = true;
        obj
    }

    pub fn handle_on_press(&self, state_tree: &mut common::definitions::StateTree,
                           scheduler: &mut Scheduler) {

        state_tree.get_mut(&self.get_full_path()).unwrap().as_button_mut()
            .set_flashing(true);
        let scheduled_func =
            | context: common::definitions::EzContext | {
                if !context.state_tree.contains_key(&context.widget_path) { return false }
                context.state_tree.get_mut(&context.widget_path).unwrap().as_button_mut()
                    .set_flashing(false);
                true
            };
        scheduler.schedule_once(self.get_full_path().clone(),
                                        Box::new(scheduled_func),
                                        Duration::from_millis(50));
    }
}
