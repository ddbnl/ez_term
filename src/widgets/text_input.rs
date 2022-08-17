//! # Text input Widget
//! A widget implementing a field in which the user can input characters. Supports on_value_change
//! and on_keyboard_enter callbacks.
use std::cmp::min;
use std::io::{Error, ErrorKind};
use std::time::Duration;

use crossterm::event::{Event, KeyCode};

use crate::Context;
use crate::parser::load_base_properties::load_string_property;
use crate::parser::load_common_properties::load_common_property;
use crate::property::ez_values::EzValues;
use crate::run::definitions::{CallbackTree, Coordinates, Pixel, PixelMap, StateTree};
use crate::scheduler::scheduler::SchedulerFrontend;
use crate::states::ez_state::{EzState, GenericState};
use crate::states::text_input_state::TextInputState;
use crate::widgets::ez_object::EzObject;
use crate::widgets::helper_functions::{add_border, add_padding};


#[derive(Clone, Debug)]
pub struct TextInput {

    /// ID of the widget, used to construct [path]
    pub id: String,

    /// Full path to this widget, e.g. "/root_layout/layout_2/THIS_ID"
    pub path: String,

    /// Runtime state of this widget, see [TextInputState] and [State]
    pub state: TextInputState,

}

impl TextInput {
    pub fn new(id: String, path: String, scheduler: &mut SchedulerFrontend) -> Self {
        TextInput {
            id,
            path: path.clone(),
            state: TextInputState::new(path, scheduler),
        }
    }

    pub fn from_state(id: String, path: String, _scheduler: &mut SchedulerFrontend, state: EzState) -> Self {
        TextInput {
            id,
            path: path.clone(),
            state: state.as_text_input().to_owned(),
        }
    }

}


impl EzObject for TextInput {

    fn load_ez_parameter(&mut self, parameter_name: String, parameter_value: String,
                         scheduler: &mut SchedulerFrontend) -> Result<(), Error>{

        let consumed = load_common_property(
            &parameter_name, parameter_value.clone(),self, scheduler)?;
        if consumed { return Ok(()) }
        match parameter_name.as_str() {
            "max_length" => {
                let length = match parameter_value.trim().parse() {
                    Ok(i) => i,
                    Err(_) => return Err(
                        Error::new(ErrorKind::InvalidData,
                                   format!("Invalid value for max_length: \"{}\". Required \
                                   format is \"max_length: 10\"", parameter_value)))
                };
                self.state.set_max_length(length);
            },
            "text" => {
                let path = self.path.clone();
                self.state.set_text(load_string_property(
                    parameter_value.trim(), scheduler, path.clone(),
                    Box::new(move |state_tree: &mut StateTree, val: EzValues| {
                        let state = state_tree.get_mut(&path)
                            .as_text_input_mut();
                        state.set_text(val.as_string().clone());
                        path.clone()
                    }))?)
            }
            _ => return Err(
                Error::new(ErrorKind::InvalidData,
                           format!("Invalid parameter name for text input: {}",
                                   parameter_name)))
        }
        Ok(())
    }
    fn set_id(&mut self, id: &str) { self.id = id.to_string() }

    fn get_id(&self) -> String { self.id.clone() }

    fn set_path(&mut self, id: &str) { self.id = id.to_string() }

    fn get_path(&self) -> String { self.path.clone() }

    fn get_state(&self) -> EzState { EzState::TextInput(self.state.clone()) }
    
    fn get_state_mut(&mut self) -> &mut dyn GenericState { &mut self.state }

    fn get_contents(&self, state_tree: &mut StateTree) -> PixelMap {

        let state = state_tree
            .get_mut(&self.get_path()).as_text_input_mut();
        let (fg_color, bg_color) = state.get_context_colors();
        let mut text = state.get_text();
        if text.len() > state.get_effective_size().width - 1 {
            let remains = text.len() - state.get_view_start();
            let view_end =
                if remains > (state.get_effective_size().width - 1) {
                    state.get_view_start() + (state.get_effective_size().width - 1)
                } else {
                    text.len()
                };
            text = text[state.get_view_start()..view_end].to_string();
        }
        let mut contents = Vec::new();
        text = text.chars().rev().collect::<String>();

        let write_height = if !state.get_infinite_size().height {
            if state.get_effective_size().height >= 1 {1} else {0}
        } else { 1 };

        for x in 0..state.get_effective_size().width {
            let mut new_y = Vec::new();
            for _ in 0..write_height {
                if !text.is_empty() {
                    new_y.push(Pixel {
                        symbol: text.pop().unwrap().to_string(),
                        foreground_color: fg_color,
                        background_color: if state.get_blink_switch() &&
                            x == state.get_cursor_pos().x {state.get_color_config().get_cursor_color() }
                        else {bg_color},
                        underline: true})
                } else {
                    new_y.push(Pixel {
                        symbol: " ".to_string(),
                        foreground_color: fg_color,
                        background_color: if state.get_blink_switch() &&
                            x == state.get_cursor_pos().x {state.get_color_config().get_cursor_color() }
                            else {bg_color},
                        underline: true})
                }
            }
            contents.push(new_y);
        }
        state.set_effective_height(1); // Hack until multiline implementation
        if state.get_auto_scale().get_auto_scale_width() {
            state.set_effective_width(contents.len());
        }
        if state.get_auto_scale().get_auto_scale_height() {
            state.set_effective_height(1);
        }
        if state.get_border_config().get_border() {
            contents = add_border(
                contents, state.get_border_config(), state.get_color_config());
        }
        let state = state_tree.get(&self.get_path()).as_text_input();
        let parent_colors = state_tree.get(self.get_path()
            .rsplit_once('/').unwrap().0).as_generic().get_color_config();
        contents = add_padding(
            contents, state.get_padding(), parent_colors.get_bg_color(),
            parent_colors.get_fg_color());
        contents
    }

    fn handle_event(&self, event: Event, state_tree: &mut StateTree,
                    callback_tree: &mut CallbackTree, scheduler: &mut SchedulerFrontend) -> bool {

        let state = state_tree.get_mut(&self.get_path())
            .as_text_input_mut();
        let current_text = state.get_text().clone();
        if let Event::Key(key) = event {
            if key.code == KeyCode::Backspace {
                handle_backspace(state, scheduler);
                self.check_changed_text(state_tree, callback_tree, scheduler,current_text);
                return true
            }
            else if key.code == KeyCode::Delete {
                handle_delete(state, scheduler);
                self.check_changed_text(state_tree,  callback_tree, scheduler,current_text);
                return true
            }
            else if key.code == KeyCode::Left {
                handle_left(state, scheduler);
                return true
            }
            else if key.code == KeyCode::Right {
                handle_right(state, scheduler);
                return true
            }
            else if let KeyCode::Char(c) = key.code {
                handle_char(state, c, scheduler);
                self.check_changed_text(state_tree, callback_tree, scheduler,current_text);
                return true
            }
        }
        false
    }

    fn on_left_mouse_click(&self, state_tree: &mut StateTree, callback_tree: &mut CallbackTree,
                           scheduler: &mut SchedulerFrontend, mouse_pos: Coordinates) -> bool {

        let consumed = self.on_left_mouse_click_callback(
            state_tree, callback_tree, scheduler, mouse_pos);
        if consumed { return consumed}
        scheduler.deselect_widget(); // We deselect first to allow re-selecting in a different pos
        scheduler.set_selected_widget(&self.path, Some(mouse_pos));
        true
    }

    fn on_hover(&self, state_tree: &mut StateTree, callback_tree: &mut CallbackTree,
                scheduler: &mut SchedulerFrontend, mouse_pos: Coordinates) -> bool {

        let consumed = self.on_hover_callback(state_tree, callback_tree, scheduler, mouse_pos);
        if consumed { return consumed}
        true
    }

    fn on_select(&self, state_tree: &mut StateTree, callback_tree: &mut CallbackTree,
                 scheduler: &mut SchedulerFrontend, mouse_pos: Option<Coordinates>) -> bool {

        let consumed = self.on_select_callback(state_tree, callback_tree, scheduler, mouse_pos);
        if consumed { return consumed}

        let state = state_tree.get_mut(
            &self.get_path()).as_text_input_mut();
        state.set_selected(true);
        state.update(scheduler);
        // Handle blinking of cursor
        let mut target_pos;
        // Handle this widget being selected from mouse, follow user click position
        if let Some(pos) = mouse_pos {
            target_pos = Coordinates::new(pos.x, pos.y);
            if pos.x > state.get_text().len() { target_pos.x = state.get_text().len() };
            if !state.get_active_blink_task() {
                start_cursor_blink(target_pos, state, scheduler,self.get_path());
            } else {
                state.set_cursor_pos(target_pos);
                state.set_blink_switch(true);
            }
            // Handle this widget being selected from keyboard. We choose the position.
        } else {
            // If text fills the widget move to end of widget. If not, move to end of text.
            let target_x = if state.get_text().len() > (state.get_effective_size().width - 1)
            { state.get_effective_size().width - 1 } else { state.get_text().len() };
            target_pos = Coordinates::new(target_x, state.get_position().get_y());
            start_cursor_blink(target_pos, state, scheduler,
                               self.get_path());
        }
        true
    }
}


/// Handle a char button press by user. insert the char at the cursor and move the cursor and/or
/// view where necessary.
pub fn handle_char(state: &mut TextInputState, char: char, scheduler: &mut SchedulerFrontend) {

    if state.get_max_length() > 0 &&
        state.get_text().len() >= state.get_max_length() {
        return
    }
    let cursor_pos = state.get_cursor_pos();
    let mut text;

    // Text still fits in widget, add char as normal
    if state.get_text().len() < state.get_effective_size().width - 1 {
        text = state.get_text().clone();
        text = format!("{}{}{}", text[0..cursor_pos.x as usize].to_string(), char,
                       text[(cursor_pos.x) as usize..text.len()].to_string());
        state.get_text_mut().set(text);
        if state.get_cursor_pos().x < state.get_effective_size().width - 1 {
            state.set_cursor_x(state.get_cursor_pos().x + 1);
        } else {
            state.set_view_start(state.get_view_start() + 1);
        }
    }
    // Text does not fit in widget, add char to view
    else {
        let (pre_view_text, mut view_text, post_view_text) =
            get_view_parts(state.get_text().clone(), state.get_view_start(),
                           state.get_effective_size().width);
        if cursor_pos.x < state.get_effective_size().width - 1 {
            view_text = format!("{}{}{}",
                                view_text[0..min(cursor_pos.x, view_text.len()) as usize].to_string(), char,
                                view_text[min(cursor_pos.x as usize, view_text.len())..view_text.len()].to_string());
        } else {
            view_text = format!("{}{}", view_text, char);
        }
        let new_text = format!("{}{}{}", pre_view_text, view_text, post_view_text);
        state.get_text_mut().set(new_text);
        if state.get_cursor_pos().x < state.get_effective_size().width - 1 {
            state.set_cursor_x(state.get_cursor_pos().x + 1);
        } else {
            state.set_cursor_x(state.get_cursor_pos().x - 1);
            state.set_view_start(state.get_view_start() + 2);
        }
    }
    state.update(scheduler);
}


impl TextInput {

    /// Initialize an instance of this object using the passed config coming from [ez_parser]
    pub fn from_config(config: Vec<String>, id: String, path: String, scheduler: &mut SchedulerFrontend,
                       file: String, line: usize) -> Self {

        let mut obj = TextInput::new(id, path, scheduler);
        obj.load_ez_config(config, scheduler, file, line).unwrap();
        obj
    }

    /// Check if text has changed to call on_value_change.
    fn check_changed_text(&self, state_tree: &mut StateTree, callback_tree: &mut CallbackTree,
                          scheduler: &mut SchedulerFrontend, old_text: String) {

        let state = state_tree.get(&self.path).as_text_input();
        if state.get_text() != old_text {
            self.on_value_change_callback(state_tree,callback_tree, scheduler);
        }
    }
}

/// Start blink the position on which the cursor is currently located. This is a custom cursor not
/// the actual terminal cursor. This is because in dynamic interfaces with scheduled tasks changing
/// visual content, the crossterm cursor is constantly jumping around, which cannot seem to be
/// resolved using the Hide/Show/SavePosition/RestorePosition methods.
fn start_cursor_blink(target_pos: Coordinates, state: &mut TextInputState,
                      scheduler: &mut SchedulerFrontend, name: String) {

    state.set_cursor_pos(target_pos);
    state.set_active_blink_task(true);
    state.update(scheduler);
    let path = state.path.clone();
    let mut counter = 3;
    let blink_func = move | context: Context| {
        let state = context.state_tree.get_mut(&path.clone())
            .as_text_input_mut();
        if !state.get_selected() {
            state.set_blink_switch(false);
            state.set_active_blink_task(false);
            state.update(context.scheduler);
            return false
        };
        if counter >= 3 {
            counter = 0;
            state.set_blink_switch(!state.get_blink_switch());
            state.update(context.scheduler);
        } else {
            counter += 1;
        }
        true
    };
    scheduler.schedule_recurring(name.as_str(), Box::new(blink_func),
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
pub fn handle_right(state: &mut TextInputState, scheduler: &mut SchedulerFrontend) {

    let cursor_pos = state.get_cursor_pos();
    // Text does not fit in widget, advance view
    let text = state.get_text().clone();
    if text.len() > state.get_effective_size().width - 1 &&
        cursor_pos.x >= state.get_effective_size().width - 2 &&
        text.len() - state.get_view_start() > (state.get_effective_size().width - 1) {
        if text.len() - state.get_view_start() - state.get_effective_size().width >= 4 {
            state.set_view_start(state.get_view_start() + 4);
            state.set_cursor_x(state.get_cursor_pos().x - 3);
        } else {
            state.set_view_start(text.len() - state.get_effective_size().width + 1);
            state.set_cursor_x(state.get_cursor_pos().x - 1);
        }
        // Text does not fit in widget but can't move further
    } else if text.len() > state.get_effective_size().width - 1 &&
        cursor_pos.x == state.get_effective_size().width - 1 {} // Max view, nothing to do
    // Text fits in widget, handle normally
    else if cursor_pos.x < text.len() {
        state.set_cursor_x(state.get_cursor_pos().x + 1);
    }
    state.update(scheduler);
}

/// Handle a left arrow button press by user. Move cursor to the left or move the
/// view if the cursor was at the edge of the widget.
pub fn handle_left(state: &mut TextInputState, scheduler: &mut SchedulerFrontend) {

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
    state.update(scheduler);
}

/// Handle a delete button press by user. Delete character to the right of the widget. Move the
/// view as necessary.
pub fn handle_delete(state: &mut TextInputState, scheduler: &mut SchedulerFrontend) {

    let cursor_pos = state.get_cursor_pos();
    // Check if text does not fit in widget, then we have to delete on a view
    if state.get_text().len() > state.get_effective_size().width - 1 {
        // Get the view on the string, as well pre- and post to reconstruct it later
        let (pre_view_text, mut view_text, mut post_view_text) =
            get_view_parts(state.get_text().clone(), state.get_view_start(),
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
        state.get_text_mut().set(format!("{}{}{}", pre_view_text, view_text, post_view_text));
        // If we're viewing the start of a string then delete should move the view
        // forward if it's not already at the end
    }
    // Check if text fits in widget, then delete text as normal
    else {
        // Check if cursor is ahead of text, i.e. nothing to delete
        if cursor_pos.x == state.get_text().len() {
            return
        }
        let mut text = state.get_text().clone();
        text = format!("{}{}", text[0..cursor_pos.x as usize].to_string(),
                       text[(cursor_pos.x + 1) as usize..text.len()].to_string());
        state.get_text_mut().set(text);
    }
    state.update(scheduler);
}

/// Handle a backspace button press by user. Delete character to the left of the widget. Move the
/// cursor and/or view as necessary.
pub fn handle_backspace(state: &mut TextInputState, scheduler: &mut SchedulerFrontend) {
    let cursor_pos = state.get_cursor_pos();
    let mut text = state.get_text().clone();

    // Check if text does not fit in widget, then we have to backspace on a view
    if state.get_text().len() > state.get_effective_size().width - 1 {
        // Check if cursor is at start of text, i.e. nothing to backspace
        if cursor_pos.x == 0 && state.get_view_start() == 0 {
            return
        }
        let (mut pre_view_text, mut view_text, post_view_text) =
            get_view_parts(state.get_text().clone(), state.get_view_start(),
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
        state.get_text_mut().set(format!("{}{}{}", pre_view_text, view_text, post_view_text));

        // Backspace should move the view back if it's not already at the start
        if state.get_view_start() > 1 && post_view_text.is_empty() {
            state.set_view_start(state.get_view_start() - 1);
        }
        // If backspacing out of view move back view 2 times if possible
        if state.get_view_start() > 1 && cursor_pos.x == 0 && !pre_view_text.is_empty() {
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
        state.get_text_mut().set(text);
        state.set_cursor_x(state.get_cursor_pos().x - 1);
    }
    state.update(scheduler);
}