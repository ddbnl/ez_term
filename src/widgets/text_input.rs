//! # Text input Widget
//! A widget implementing a field in which the user can input characters. Supports on_value_change
//! and on_keyboard_enter callbacks.
use std::io::Error;
use std::time::Duration;
use crossterm::event::{Event, KeyCode};
use crate::ez_parser;
use crate::scheduler;
use crate::states;
use crate::states::text_input_state::TextInputState;
use crate::states::state::{EzState, GenericState};
use crate::widgets::widget::{Pixel, EzObject};
use crate::common;
use crate::common::definitions::{CallbackTree, PixelMap, StateTree, ViewTree, WidgetTree};
use crate::scheduler::Scheduler;
use crate::states::definitions::Coordinates;

#[derive(Clone)]
pub struct TextInput {

    /// ID of the widget, used to construct [path]
    pub id: String,

    /// Full path to this widget, e.g. "/root_layout/layout_2/THIS_ID"
    pub path: String,

    /// Runtime state of this widget, see [TextInputState] and [State]
    pub state: TextInputState,

}

impl Default for TextInput {
    fn default() -> Self {
        TextInput {
            id: "".to_string(),
            path: String::new(),
            state: TextInputState::default(),
        }
    }
}


impl EzObject for TextInput {

    fn load_ez_parameter(&mut self, parameter_name: String, mut parameter_value: String) {

        match parameter_name.as_str() {
            "id" => self.set_id(parameter_value.trim().to_string()),
            "x" => self.state.set_x(parameter_value.trim().parse().unwrap()),
            "y" => self.state.set_y(parameter_value.trim().parse().unwrap()),
            "pos" => self.state.set_position(
                ez_parser::load_pos_parameter(parameter_value.trim())),
            "size_hint" => self.state.set_size_hint(
                ez_parser::load_full_size_hint_parameter(parameter_value.trim())),
            "size_hint_x" => self.state.set_size_hint_x(
                ez_parser::load_size_hint_parameter(parameter_value.trim())),
            "pos_hint" => self.state.set_pos_hint(
                ez_parser::load_full_pos_hint_parameter(parameter_value.trim())),
            "pos_hint_x" => self.state.set_pos_hint_x(
                ez_parser::load_pos_hint_x_parameter(parameter_value.trim())),
            "pos_hint_y" => self.state.set_pos_hint_y(
                ez_parser::load_pos_hint_y_parameter(parameter_value.trim())),
            "auto_scale_width" =>
                self.state.set_auto_scale_width(
                    ez_parser::load_bool_parameter(parameter_value.trim())),
            "width" => self.state.get_size_mut().width = parameter_value.trim().parse().unwrap(),
            "padding" => self.state.set_padding(ez_parser::load_full_padding_parameter(
                parameter_value.trim())),
            "padding_x" => self.state.set_padding(ez_parser::load_padding_x_parameter(
                parameter_value.trim())),
            "padding_y" => self.state.set_padding(ez_parser::load_padding_y_parameter(
                parameter_value.trim())),
            "padding_top" =>
                self.state.set_padding_top(parameter_value.trim().parse().unwrap()),
            "padding_bottom" =>
                self.state.set_padding_bottom(parameter_value.trim().parse().unwrap()),
            "padding_left" =>
                self.state.set_padding_left(parameter_value.trim().parse().unwrap()),
            "padding_right" =>
                self.state.set_padding_right(parameter_value.trim().parse().unwrap()),
            "halign" =>
                self.state.halign =  ez_parser::load_halign_parameter(parameter_value.trim()),
            "valign" =>
                self.state.valign =  ez_parser::load_valign_parameter(parameter_value.trim()),
            "max_length" => self.state.set_max_length(parameter_value.trim().parse().unwrap()),
            "selection_order" => {
                let order = parameter_value.trim().parse().unwrap();
                if order == 0 {
                    panic!("selection_order must be higher than 0.")
                }
                self.state.selection_order = order;
            },
            "border" => self.state.border_config.enabled =
                ez_parser::load_bool_parameter(parameter_value.trim()),
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
                self.state.border_config.fg_color =
                    ez_parser::load_color_parameter(parameter_value),
            "border_bg_color" =>
                self.state.border_config.bg_color =
                    ez_parser::load_color_parameter(parameter_value),
            "fg_color" =>
                self.state.colors.foreground = ez_parser::load_color_parameter(parameter_value),
            "bg_color" =>
                self.state.colors.background = ez_parser::load_color_parameter(parameter_value),
            "selection_fg_color" =>
                self.state.colors.selection_foreground =
                    ez_parser::load_color_parameter(parameter_value),
            "selection_bg_color" =>
                self.state.colors.selection_background =
                    ez_parser::load_color_parameter(parameter_value),
            "cursor_color" =>
                self.state.colors.cursor = ez_parser::load_color_parameter(parameter_value),
            "text" => {
                if parameter_value.starts_with(' ') {
                    parameter_value = parameter_value.strip_prefix(' ').unwrap().to_string();
                }
                self.state.text = parameter_value
            },
            _ => panic!("Invalid parameter name for text input {}", parameter_name)
        }
    }
    fn set_id(&mut self, id: String) { self.id = id }

    fn get_id(&self) -> String { self.id.clone() }

    fn set_full_path(&mut self, path: String) { self.path = path }

    fn get_full_path(&self) -> String { self.path.clone() }

    fn get_state(&self) -> EzState { EzState::TextInput(self.state.clone()) }

    fn get_contents(&self, state_tree: &mut common::definitions::StateTree)
        -> common::definitions::PixelMap {

        let state = state_tree
            .get_mut(&self.get_full_path()).unwrap().as_text_input_mut();
        let fg_color = if state.get_selected() {state.get_color_config().selection_foreground }
                           else {state.get_color_config().foreground };
        let bg_color = if state.get_selected() {state.get_color_config().selection_background }
                             else {state.get_color_config().background };
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

        let write_height = if !state.get_size().infinite_height {
            if state.get_effective_size().height >= 1 {1} else {0}
        } else { 1 };

        for x in 0..state.get_effective_size().width {
            let mut new_y = Vec::new();
            for _ in 0..write_height {
                if !text.is_empty() {
                    new_y.push(Pixel{
                        symbol: text.pop().unwrap().to_string(),
                        foreground_color: fg_color,
                        background_color: if state.get_blink_switch() &&
                            x == state.get_cursor_pos().x {state.get_color_config().cursor }
                        else {bg_color},
                        underline: true})
                } else {
                    new_y.push(Pixel{
                        symbol: " ".to_string(),
                        foreground_color: fg_color,
                        background_color: if state.get_blink_switch() &&
                            x == state.get_cursor_pos().x {state.get_color_config().cursor }
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
        if state.get_border_config().enabled {
            contents = common::widget_functions::add_border(
                contents, state.get_border_config());
        }
        let state = state_tree.get(&self.get_full_path()).unwrap().as_text_input();
        let parent_colors = state_tree.get(self.get_full_path()
            .rsplit_once('/').unwrap().0).unwrap().as_generic().get_color_config();
        contents = common::widget_functions::add_padding(
            contents, state.get_padding(),parent_colors.background,
            parent_colors.foreground);
        contents
    }

    fn on_left_mouse_click(&self, view_tree: &mut ViewTree, state_tree: &mut StateTree,
                           widget_tree: &WidgetTree, callback_tree: &mut CallbackTree,
                           scheduler: &mut Scheduler, mouse_pos: Coordinates) -> bool {
        true
    }

    fn handle_event(&self, event: Event, view_tree: &mut common::definitions::ViewTree,
                    state_tree: &mut common::definitions::StateTree, widget_tree: &common::definitions::WidgetTree,
                    callback_tree: &mut common::definitions::CallbackTree, scheduler: &mut Scheduler) -> bool {

        let state = state_tree.get_mut(&self.get_full_path().clone())
            .unwrap().as_text_input_mut();
        let current_text = state.text.clone();
        if let Event::Key(key) = event {
            if key.code == KeyCode::Backspace {
                handle_backspace(state);
                if state.text != current_text {
                    if let Some(ref mut i ) = callback_tree
                        .get_mut(&self.get_full_path()).unwrap().on_value_change {
                        i(common::definitions::EzContext::new(self.get_full_path().clone(),
                        view_tree, state_tree, widget_tree, scheduler));
                    }
                }
                return true
            }
            if key.code == KeyCode::Delete {
                handle_delete(state);
                if state.text != current_text {
                    if let Some(ref mut i ) = callback_tree
                        .get_mut(&self.get_full_path()).unwrap().on_value_change {
                        i(common::definitions::EzContext::new(self.get_full_path().clone(),
                                                 view_tree, state_tree, widget_tree, scheduler));
                    }
                }
                return true
            }
            if key.code == KeyCode::Left {
                handle_left(state);
                return true
            }
            if key.code == KeyCode::Right {
                handle_right(state);
                return true
            }
            if let KeyCode::Char(c) = key.code {
                handle_char(state, c);
                if state.text != current_text {
                    if let Some(ref mut i ) = callback_tree
                        .get_mut(&self.get_full_path()).unwrap().on_value_change {
                        i(common::definitions::EzContext::new(self.get_full_path().clone(),
                                                 view_tree, state_tree, widget_tree, scheduler));
                    }
                }
                return true
            }
        }
        false
    }

    fn on_select(&self, view_tree: &mut common::definitions::ViewTree,
                 state_tree: &mut common::definitions::StateTree,
                 widget_tree: &common::definitions::WidgetTree,
                 callback_tree: &mut common::definitions::CallbackTree,
                 scheduler: &mut Scheduler,
                 mouse_pos: Option<states::definitions::Coordinates>) -> bool {

        let state = state_tree.get_mut(
            &self.get_full_path()).unwrap().as_text_input_mut();
        state.set_selected(true);
        // Handle blinking of cursor
        let mut target_pos;
        // Handle this widget being selected from mouse, follow user click position
        if let Some(pos) = mouse_pos {
            target_pos = states::definitions::Coordinates::new(pos.x, pos.y);
            if pos.x > state.text.len() { target_pos.x = state.text.len() };
            if !state.active_blink_task {
                start_cursor_blink(target_pos, state, scheduler,
                                   self.get_full_path().clone());
            } else {
                state.set_cursor_pos(target_pos);
                state.set_blink_switch(true);
            }
            // Handle this widget being selected from keyboard. We choose the position.
        } else {
            // If text fills the widget move to end of widget. If not, move to end of text.
            let target_x = if state.text.len() > (state.get_effective_size().width - 1)
            { state.get_effective_size().width - 1 } else { state.text.len() };
            target_pos = states::definitions::Coordinates::new(target_x, state.get_position().y);
            start_cursor_blink(target_pos, state, scheduler,
                               self.get_full_path().clone());
        }

        // Call user callback if any
        if let Some(ref mut i) = callback_tree.
            get_mut(&self.get_full_path()).unwrap().on_select {
            let context = common::definitions::EzContext::new(
                self.get_full_path().clone(), view_tree, state_tree, widget_tree,
                scheduler);
            i(context, mouse_pos);
        };
        true
    }
}


/// Handle a char button press by user. insert the char at the cursor and move the cursor and/or
/// view where necessary.
pub fn handle_char(state: &mut TextInputState, char: char) {

    if state.get_text().len() >= state.get_max_length() {
        return
    }
    let cursor_pos = state.get_cursor_pos();
    let mut text;

    // Text still fits in widget, add char as normal
    if state.get_text().len() < (state.get_effective_size().width) {
        text = state.get_text();
        text = format!("{}{}{}", text[0..cursor_pos.x as usize].to_string(), char,
                       text[(cursor_pos.x) as usize..text.len()].to_string());
        state.set_text(text);
    }
    // Text does not fit in widget, add char to view
    else {
        let (pre_view_text, mut view_text, post_view_text) =
            get_view_parts(state.get_text(), state.get_view_start(),
                           state.get_effective_size().width);
        view_text = format!("{}{}{}",
                            view_text[0..cursor_pos.x as usize].to_string(), char,
                            view_text[(cursor_pos.x) as usize..view_text.len()].to_string());
        let new_text = format!("{}{}{}", pre_view_text, view_text, post_view_text);
        state.set_text(new_text);
    }

    if state.get_text().len() < (state.get_effective_size().width){
        state.set_cursor_x(state.get_cursor_pos().x + 1);
    } else {
        state.set_view_start(state.get_view_start() + 1);
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

/// Start blink the position on which the cursor is currently located. This is a custom cursor not
/// the actual terminal cursor. This is because in dynamic interfaces with scheduled tasks changing
/// visual content, the crossterm cursor is constantly jumping around, which cannot seem to be
/// resolved using the Hide/Show/SavePosition/RestorePosition methods.
fn start_cursor_blink(target_pos: states::definitions::Coordinates, state: &mut TextInputState,
                      scheduler: &mut scheduler::Scheduler, name: String) {

    state.set_cursor_pos(target_pos);
    state.set_active_blink_task(true);
    let mut counter = 3;
    let blink_func = move | context: common::definitions::EzContext | {
        let state = context.state_tree.get_mut(&context.widget_path).unwrap()
            .as_text_input_mut();
        if !state.selected {
            state.set_blink_switch(false);
            state.set_active_blink_task(false);
            return false
        };
        if counter >= 3 {
            counter = 0;
            state.set_blink_switch(!state.get_blink_switch());
        } else {
            counter += 1;
        }
        true
    };
    scheduler.schedule_interval(name, Box::new(blink_func),
                                Duration::from_millis(100));
}


/// Given a view, return which parts of the widget text are visible. Also return the part that
/// comes before the view and after the view. Used by keyboard callbacks to alter the view.
pub fn get_view_parts(text: String, view_start: usize, widget_with: usize) -> (String, String, String) {

    let pre_view_text = if view_start == 0 { "".to_string() }
    else { text[0..view_start].to_string() };
    let view_text =
        if text.len() - view_start <= widget_with - 2 { text[view_start..text.len()].to_string() }
        else { text[view_start..view_start + widget_with - 1].to_string() };
    let post_view_text =
        if text.len() - view_start <= (widget_with - 1) { "".to_string() }
        else { text[view_start + widget_with - 1..text.len()].to_string() };
    (pre_view_text, view_text, post_view_text)
}

/// Handle a right arrow button press by user. Move cursor to the right or move the
/// view if the cursor was at the edge of the widget.
pub fn handle_right(state: &mut TextInputState) {

    let cursor_pos = state.get_cursor_pos();
    // Text does not fit in widget, advance view
    if state.get_text().len() > state.get_effective_size().width - 1 &&
        cursor_pos.x >= state.get_effective_size().width - 2 &&
        state.get_text().len() - state.get_view_start() > (state.get_effective_size().width - 1) {
        if state.get_text().len() - state.get_view_start() - state.get_effective_size().width >= 4 {
            state.set_view_start(state.get_view_start() + 4);
            state.set_cursor_x(state.get_cursor_pos().x - 3);
        } else {
            state.set_view_start(state.get_text().len() - state.get_effective_size().width + 1);
            state.set_cursor_x(state.get_cursor_pos().x - 1);
        }
        // Text does not fit in widget but can't move further
    } else if state.get_text().len() > state.get_effective_size().width - 1 &&
        cursor_pos.x == state.get_effective_size().width - 1 {} // Max view, nothing to do
    // Text fits in widget, handle normally
    else if cursor_pos.x < state.get_text().len() {
        state.set_cursor_x(state.get_cursor_pos().x + 1);
    }
}

/// Handle a left arrow button press by user. Move cursor to the left or move the
/// view if the cursor was at the edge of the widget.
pub fn handle_left(state: &mut TextInputState) {

    let cursor_pos = state.get_cursor_pos();

    // Text does not fit in widget and cursor at 0, move view to left if not at 0 already
    if state.get_text().len() > state.get_effective_size().width - 1 &&
        cursor_pos.x <= 1 && state.get_view_start() > 0 {
        if state.get_view_start() >= 4 {
            state.set_view_start(state.get_view_start() - 4 );
            state.set_cursor_x(state.get_cursor_pos().x + 4);
        } else {
            state.set_view_start(0);
            state.set_cursor_x(4);
        }
        // Text fits in widget or cursor pos is not at 0, move cursor normally
    } else if cursor_pos.x > 0 {
        state.set_cursor_x(state.get_cursor_pos().x - 1);
    }
}

/// Handle a delete button press by user. Delete character to the right of the widget. Move the
/// view as necessary.
pub fn handle_delete(state: &mut TextInputState) {

    let cursor_pos = state.get_cursor_pos();
    // Check if text does not fit in widget, then we have to delete on a view
    if state.get_text().len() > state.get_effective_size().width - 1 {
        // Get the view on the string, as well pre- and post to reconstruct it later
        let (pre_view_text, mut view_text, mut post_view_text) =
            get_view_parts(state.get_text(), state.get_view_start(),
                           state.get_effective_size().width);

        if cursor_pos.x == state.get_effective_size().width - 1 && post_view_text.is_empty() {
            return
        }
        // Check if deleting in the last position, i.e. deleting out of view
        if cursor_pos.x > view_text.len() - 1 {
            // Deleting out of view, delete from post_view_text instead of view_text
            if !post_view_text.is_empty() {
                post_view_text = post_view_text[1..post_view_text.len()].to_string();
                // Deleting out of view but at end of text already, nothing to do
            } else {
                return
            }
            // Not deleting at end of view, delete as normal
        } else {
            // Perform delete on the text view
            view_text = format!("{}{}", view_text[0..cursor_pos.x as usize].to_string(),
                                view_text[(cursor_pos.x + 1) as usize..view_text.len()].to_string());
        }

        // Reconstruct text with backspace view
        state.set_text(format!("{}{}{}", pre_view_text, view_text, post_view_text));
        // If we're viewing the start of a string then delete should move the view
        // forward if it's not already at the end
    }
    // Check if text fits in widget, then delete text as normal
    else {
        // Check if cursor is ahead of text, i.e. nothing to delete
        if cursor_pos.x == state.get_text().len() {
            return
        }
        let mut text = state.get_text();
        text = format!("{}{}", text[0..cursor_pos.x as usize].to_string(),
                       text[(cursor_pos.x + 1) as usize..text.len()].to_string());
        state.set_text(text);
    }
}

/// Handle a backspace button press by user. Delete character to the left of the widget. Move the
/// cursor and/or view as necessary.
pub fn handle_backspace(state: &mut TextInputState) {
    let cursor_pos = state.get_cursor_pos();
    let mut text = state.get_text();

    // Check if text does not fit in widget, then we have to backspace on a view
    if state.get_text().len() > state.get_effective_size().width - 1 {
        // Check if cursor is at start of text, i.e. nothing to backspace
        if cursor_pos.x == 0 && state.get_view_start() == 0 {
            return
        }
        let (mut pre_view_text, mut view_text, post_view_text) =
            get_view_parts(state.get_text(), state.get_view_start(),
                           state.get_effective_size().width);
        // Perform backspace on the text view
        if cursor_pos.x == 0 {
            // Backspace out of view
            pre_view_text = pre_view_text[0..pre_view_text.len() - 1].to_string();
        } else {
            // Backspace in view
            view_text = format!("{}{}",
                                view_text[0..(cursor_pos.x - 1) as usize].to_string(),
                                view_text[cursor_pos.x as usize..view_text.len()].to_string());
        }
        // Reconstruct text with backspace view
        state.set_text(format!("{}{}{}", pre_view_text, view_text, post_view_text));

        // Backspace should move the view back if it's not already at the start
        if state.view_start > 1 && post_view_text.is_empty() {
            state.set_view_start(state.get_view_start() - 1);
        }
        // If backspacing out of view move back view 2 times if possible
        if state.view_start > 1 && cursor_pos.x == 0 && !pre_view_text.is_empty() {
            state.set_view_start(state.get_view_start() - 1);
        }

        if (cursor_pos.x > 0 && pre_view_text.is_empty()) ||
            (cursor_pos.x == state.get_effective_size().width - 1 && !post_view_text.is_empty()) {
            state.set_cursor_x(state.get_cursor_pos().x - 1);
        }
    }
    // Check if text fits in widget, then backspace text as normal
    else {
        // Check if cursor is at start of text, i.e. nothing to backspace
        if cursor_pos.x == 0 {
            return
        }
        // Perform backspace on text
        text = format!("{}{}", text[0..(cursor_pos.x - 1) as usize].to_string(),
                       text[cursor_pos.x as usize..text.len()].to_string());
        state.set_text(text);
        state.set_cursor_x(state.get_cursor_pos().x - 1);
    }
}