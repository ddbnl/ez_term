//! A widget that displays text non-interactively.
use std::time::Duration;
use crate::states::state::{EzState, GenericState};
use crate::states::definitions::{HorizontalAlignment, VerticalAlignment};
use crate::states::button_state::ButtonState;
use crate::common;
use crate::common::definitions::{CallbackTree, PixelMap, StateTree, ViewTree, WidgetTree};
use crate::widgets::widget::{Pixel, EzObject};
use crate::parser::load_widget_properties::load_common_property;
use crate::parser::load_base_properties::load_ez_string_property;
use crate::property::EzValues;
use crate::scheduler::Scheduler;

#[derive(Clone, Debug)]
pub struct Button {

    /// ID of the widget, used to construct [path]
    pub id: String,

    /// Full path to this widget, e.g. "/root_layout/layout_2/THIS_ID"
    pub path: String,

    /// Runtime state of this widget, see [LabelState] and [State]
    pub state: ButtonState,
}

impl Button {

    pub fn new(id: String, path: String, scheduler: &mut Scheduler) -> Self {
        Button {
            id,
            path: path.clone(),
            state: ButtonState::new(path, scheduler),
        }
    }
}


impl EzObject for Button {

    fn load_ez_parameter(&mut self, parameter_name: String, parameter_value: String,
                         scheduler: &mut Scheduler) {
        
        let consumed = load_common_property(
            &parameter_name, parameter_value.clone(),self,
            scheduler);
        if consumed { return }
        match parameter_name.as_str() {
            "text" => {
                let path = self.path.clone();
                self.state.text.set(load_ez_string_property(
                    parameter_value.trim(), scheduler, path.clone(),
                    Box::new(move |state_tree: &mut StateTree, val: EzValues| {
                        let state = state_tree.get_by_path_mut(&path)
                            .as_button_mut();
                        state.text.set(val.as_string().clone());
                        path.clone()
                    })))
            },
            _ => panic!("Invalid parameter name for button {}", parameter_name)
        }
    }


    fn set_id(&mut self, id: String) { self.id = id }

    fn get_id(&self) -> String { self.id.clone() }

    fn set_full_path(&mut self, path: String) { self.path = path }

    fn get_full_path(&self) -> String { self.path.clone() }

    fn get_state(&self) -> EzState { EzState::Button(self.state.clone()) }

    fn get_state_mut(&mut self) -> &mut dyn GenericState { &mut self.state }

    fn get_contents(&self, state_tree: &mut StateTree) -> PixelMap {

        let state = state_tree.get_by_path_mut(&self.get_full_path())
            .as_button_mut();

        let (fg_color, bg_color) =
            if state.flashing {(state.get_color_config().flash_foreground.value,
                                state.get_color_config().flash_background.value)}
            else { state.get_context_colors() };

        let text = state.text.value.clone();

        let write_width = if state.get_size().infinite_width ||
            state.get_auto_scale().width.value { text.len() + 1 }
            else {state.get_effective_size().width };
        let content_lines = common::widget_functions::wrap_text(text, write_width);
        let write_height =
            if state.get_size().infinite_height || state.get_auto_scale().height.value
            { content_lines.len() }
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
        if state.get_auto_scale().width.value {
            state.set_effective_width(contents.len());
        }
        if state.get_auto_scale().height.value {
            state.set_effective_height(contents[0].len());
        }
        (contents, _) = common::widget_functions::align_content_horizontally(
            contents,HorizontalAlignment::Center,
            state.get_effective_size().width, fg_color, bg_color);
        (contents, _) = common::widget_functions::align_content_vertically(
            contents,VerticalAlignment::Middle,
            state.get_effective_size().height, fg_color, bg_color);
        contents = common::widget_functions::add_border(
            contents, state.get_border_config());
        let state = state_tree.get_by_path(&self.get_full_path()).as_button();
        let parent_colors = state_tree.get_by_path(self.get_full_path()
            .rsplit_once('/').unwrap().0).as_generic().get_color_config();
        contents = common::widget_functions::add_padding(
            contents, state.get_padding(), parent_colors.background.value,
            parent_colors.foreground.value);
        contents
    }

    fn on_press(&self, view_tree: &mut ViewTree, state_tree: &mut StateTree,
                widget_tree: &WidgetTree, callback_tree: &mut CallbackTree,
                scheduler: &mut Scheduler) -> bool {

        let consumed = self.on_press_callback(view_tree, state_tree, widget_tree,
                                                        callback_tree, scheduler);
        if !consumed {
            self.handle_on_press(state_tree, scheduler);
            return true
        }
        false
    }
}
impl Button {

    /// Initialize an instance of this object using the passed config coming from [ez_parser]
    pub fn from_config(config: Vec<String>, id: String, path: String, scheduler: &mut Scheduler)
        -> Self {

        let mut obj = Button::new(id, path, scheduler);
        obj.load_ez_config(config, scheduler).unwrap();
        obj
    }

    pub fn handle_on_press(&self, state_tree: &mut StateTree, scheduler: &mut Scheduler) {

        let state = state_tree.get_by_path_mut(&self.get_full_path()).as_button_mut();
        state.set_flashing(true);
        state.update(scheduler);
        let scheduled_func =
            | context: common::definitions::EzContext | {
                if !context.state_tree.objects.contains_key(&context.widget_path) { return false }
                let state = context.state_tree.get_by_path_mut(&context.widget_path)
                    .as_button_mut();
                state.set_flashing(false);
                state.update(context.scheduler);
                true
            };
        scheduler.schedule_once(self.get_full_path(),Box::new(scheduled_func),
                                        Duration::from_millis(50));
    }
}
