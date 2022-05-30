//! # Text input Widget
//! A widget implementing a field in which the user can input characters. Supports on_value_change
//! and on_keyboard_enter callbacks.
use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::time::Duration;
use crossterm::event::{Event, KeyCode};
use crate::ez_parser;
use crate::states::text_input_state::TextInputState;
use crate::states::state::{self, EzState, GenericState, SelectableState};
use crate::widgets::widget::{EzWidget, Pixel, EzObject};
use crate::common;
use crate::common::{EzContext, StateTree, WidgetTree};
use crate::scheduler::Scheduler;

pub struct TextInput {

    /// ID of the widget, used to construct [path]
    pub id: String,

    /// Full path to this widget, e.g. "/root_layout/layout_2/THIS_ID"
    pub path: String,

    /// Global order number in which this widget will be selection when user presses down/up keys
    pub selection_order: usize,

    /// Runtime state of this widget, see [TextInputState] and [State]
    pub state: TextInputState,

}

impl Default for TextInput {
    fn default() -> Self {
        let mut obj = TextInput {
            id: "".to_string(),
            path: String::new(),
            selection_order: 0,
            state: TextInputState::default(),
        };
        obj.bind_key(KeyCode::Backspace, Box::new(
            |context: common::EzContext, keycode: KeyCode|
                { handle_backspace(context, keycode) }));
        obj.bind_key(KeyCode::Delete, Box::new(
            |context: common::EzContext, keycode: KeyCode|
                { handle_delete(context, keycode) }));
        obj.bind_key(KeyCode::Left, Box::new(
            |context: common::EzContext, keycode: KeyCode|
                { handle_left(context, keycode) }));
        obj.bind_key(KeyCode::Backspace, Box::new(
            |context: common::EzContext, keycode: KeyCode|
                { handle_right(context, keycode) }));
        obj
    }
}


impl EzObject for TextInput {

    fn load_ez_parameter(&mut self, parameter_name: String, mut parameter_value: String)
                         -> Result<(), Error> {
        
        match parameter_name.as_str() {
            "id" => self.set_id(parameter_value.trim().to_string()),
            "x" => self.state.set_x(parameter_value.trim().parse().unwrap()),
            "y" => self.state.set_y(parameter_value.trim().parse().unwrap()),
            "pos" => self.state.set_position(
                ez_parser::load_pos_parameter(parameter_value.trim()).unwrap()),
            "size_hint" => self.state.set_size_hint(
                ez_parser::load_full_size_hint_parameter(parameter_value.trim()).unwrap()),
            "size_hint_x" => self.state.set_size_hint_x(
                ez_parser::load_size_hint_parameter(parameter_value.trim()).unwrap()),
            "pos_hint" => self.state.set_pos_hint(
                ez_parser::load_full_pos_hint_parameter(parameter_value.trim()).unwrap()),
            "pos_hint_x" => self.state.set_pos_hint_x(
                ez_parser::load_pos_hint_x_parameter(parameter_value.trim()).unwrap()),
            "pos_hint_y" => self.state.set_pos_hint_y(
                ez_parser::load_pos_hint_y_parameter(parameter_value.trim()).unwrap()),
            "auto_scale_width" =>
                self.state.set_auto_scale_width(ez_parser::load_bool_parameter(parameter_value.trim())?),
            "width" => self.state.set_width(parameter_value.trim().parse().unwrap()),
            "padding" => self.state.set_padding(ez_parser::load_full_padding_parameter(
                parameter_value.trim())?),
            "padding_x" => self.state.set_padding(ez_parser::load_padding_x_parameter(
                parameter_value.trim())?),
            "padding_y" => self.state.set_padding(ez_parser::load_padding_y_parameter(
                parameter_value.trim())?),
            "padding_top" => self.state.set_padding_top(parameter_value.trim().parse().unwrap()),
            "padding_bottom" => self.state.set_padding_bottom(parameter_value.trim().parse().unwrap()),
            "padding_left" => self.state.set_padding_left(parameter_value.trim().parse().unwrap()),
            "padding_right" => self.state.set_padding_right(parameter_value.trim().parse().unwrap()),
            "halign" =>
                self.state.halign =  ez_parser::load_halign_parameter(parameter_value.trim()).unwrap(),
            "valign" =>
                self.state.valign =  ez_parser::load_valign_parameter(parameter_value.trim()).unwrap(),
            "max_length" => self.state.set_max_length(parameter_value.trim().parse().unwrap()),
            "selection_order" => {
                let order = parameter_value.trim().parse().unwrap();
                if order == 0 {
                    return Err(Error::new(ErrorKind::InvalidData,
                                          "selection_order must be higher than 0."))
                }
                self.selection_order = order;
            },
            "border" => self.state.set_border(ez_parser::load_bool_parameter(parameter_value.trim())?),
            "border_horizontal_symbol" => self.state.border_config.horizontal_symbol =
                parameter_value.trim().to_string(),
            "border_vertical_symbol" => self.state.border_config.vertical_symbol =
                parameter_value.trim().to_string(),
            "border_top_right_symbol" => self.state.border_config.top_right_symbol =
                parameter_value.trim().to_string(),
            "border_top_left_symbol" => self.state.border_config.top_left_symbol =
                parameter_value.trim().to_string(),
            "border_bottom_left_symbol" => self.state.border_config.bottom_left_symbol =
                parameter_value.trim().to_string(),
            "border_bottom_right_symbol" => self.state.border_config.bottom_right_symbol =
                parameter_value.trim().to_string(),
            "border_fg_color" =>
                self.state.border_config.fg_color = ez_parser::load_color_parameter(parameter_value).unwrap(),
            "border_bg_color" =>
                self.state.border_config.bg_color = ez_parser::load_color_parameter(parameter_value).unwrap(),
            "fg_color" =>
                self.state.colors.foreground = ez_parser::load_color_parameter(parameter_value).unwrap(),
            "bg_color" =>
                self.state.colors.background = ez_parser::load_color_parameter(parameter_value).unwrap(),
            "selection_fg_color" =>
                self.state.colors.selection_foreground =
                    ez_parser::load_color_parameter(parameter_value).unwrap(),
            "selection_bg_color" =>
                self.state.colors.selection_background =
                    ez_parser::load_color_parameter(parameter_value).unwrap(),
            "cursor_color" =>
                self.state.colors.cursor = ez_parser::load_color_parameter(parameter_value).unwrap(),
            "text" => {
                if parameter_value.starts_with(' ') {
                    parameter_value = parameter_value.strip_prefix(' ').unwrap().to_string();
                }
                self.state.text = parameter_value
            },
            _ => return Err(Error::new(ErrorKind::InvalidData,
                                format!("Invalid parameter name for text input {}",
                                        parameter_name)))
        }
        Ok(())
    }

    fn set_id(&mut self, id: String) { self.id = id }

    fn get_id(&self) -> String { self.id.clone() }

    fn set_full_path(&mut self, path: String) { self.path = path }

    fn get_full_path(&self) -> String { self.path.clone() }

    fn get_state(&self) -> EzState { EzState::TextInput(TextInputState::default()) }

    fn get_contents(&self, state_tree: &mut common::StateTree, widget_tree: &common::WidgetTree) -> common::PixelMap {

        let state = state_tree
            .get_mut(&self.get_full_path()).unwrap().as_text_input_mut();
        let fg_color = if state.get_selected() {state.get_colors().selection_foreground }
                           else {state.get_colors().foreground };
        let bg_color = if state.get_selected() {state.get_colors().selection_background }
                             else {state.get_colors().background };
        let mut text = state.get_text();
        if text.len() > state.get_size().width - 1 {
            let remains = text.len() - state.get_view_start();
            let view_end =
                if remains > (state.get_size().width - 1) {
                    state.get_view_start() + (state.get_size().width - 1)
                } else {
                    text.len()
                };
            text = text[state.get_view_start()..view_end].to_string();
        }
        let mut contents = Vec::new();
        text = text.chars().rev().collect::<String>();
        for x in 0..state.get_effective_size().width {
            let mut new_y = Vec::new();
            for _ in 0..if state.get_effective_size().height >= 1 {1} else {0} {
                if !text.is_empty() {
                    new_y.push(Pixel{
                        symbol: text.pop().unwrap().to_string(),
                        foreground_color: fg_color,
                        background_color: if state.get_blink_switch() &&
                            x == state.get_cursor_pos().x {state.get_colors().cursor }
                        else {bg_color},
                        underline: true})
                } else {
                    new_y.push(Pixel{
                        symbol: " ".to_string(),
                        foreground_color: fg_color,
                        background_color: if state.get_blink_switch() &&
                            x == state.get_cursor_pos().x {state.get_colors().cursor }
                            else {bg_color},
                        underline: true})
                }
            }
            contents.push(new_y);
        }
        if state.get_auto_scale().width {
            state.set_effective_width(contents.len());
        }
        if state.get_auto_scale().height {
            state.set_effective_height(1);
        }
        if state.has_border() {
            contents = common::add_border(contents, state.get_border_config());
        }
        let state = state_tree.get(&self.get_full_path()).unwrap().as_text_input();
        let parent_colors = state_tree.get(self.get_full_path()
            .rsplit_once('/').unwrap().0).unwrap().as_generic().get_colors();
        contents = common::add_padding(
            contents, state.get_padding(),parent_colors.background,
            parent_colors.foreground);
        contents
    }
}

impl EzWidget for TextInput {

    fn is_selectable(&self) -> bool { true }

    fn get_selection_order(&self) -> usize { self.selection_order }

    fn on_select(&self, context: common::EzContext, mouse_pos: Option<state::Coordinates>) {

        let state = context.state_tree.get_mut(&self.get_full_path())
            .unwrap().as_text_input_mut();
        state.set_selected(true);

        // Handle blinking of cursor
        let mut target_pos;
        // Handle this widget being selected from mouse, follow user click position
        if let Some(pos) = mouse_pos {
            target_pos = state::Coordinates::new(pos.x, pos.y);
            if pos.x > state.text.len() {target_pos.x = state.text.len()};
            if !state.active_blink_task {
                start_cursor_blink(target_pos, state, context.scheduler, self.get_full_path());
            } else {
                state.set_cursor_pos(target_pos);
                state.set_blink_switch(true);
            }
            // Handle this widget being selected from keyboard. We choose the position.
        } else {
            // If text fills the widget move to end of widget. If not, move to end of text.
            let target_x = if state.text.len() > (state.get_effective_size().width - 1)
                                 {state.get_effective_size().width - 1} else {state.text.len()};
            target_pos = state::Coordinates::new(target_x, state.get_position().y);
            start_cursor_blink(target_pos, state, context.scheduler, self.get_full_path());
        }

        // Call user callback if any
        let new_context = common::EzContext::new(context.widget_path,
        context.view_tree, context.state_tree, context.widget_tree, context.scheduler);
        if let Some(ref func) = state.callbacks.on_select {
            func(context, mouse_pos);
        }
    }

}


impl TextInput {

    /// Initialize an instance of this object using the passed config coming from [ez_parser]
    pub fn from_config(config: Vec<String>) -> Self {
        let mut obj = TextInput::default();
        obj.load_ez_config(config).unwrap();
        obj
    }
}
