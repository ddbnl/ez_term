//! # Text input Widget
//! A widget implementing a field in which the user can input characters. Supports on_value_change
//! and on_keyboard_enter callbacks.
use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use std::time::Duration;
use crossterm::event::{Event, KeyCode};
use crate::ez_parser;
use crate::states::text_input_state::TextInputState;
use crate::states::state::{self, GenericState, SelectableState};
use crate::widgets::widget::{EzWidget, Pixel, EzObject};
use crate::common;
use crate::scheduler::Scheduler;

#[derive(Clone)]
pub struct TextInput {

    /// ID of the widget, used to construct [path]
    pub id: String,

    /// Full path to this widget, e.g. "/root_layout/layout_2/THIS_ID"
    pub path: String,

    /// Global order number in which this widget will be selection when user presses down/up keys
    pub selection_order: usize,

    /// Optional function to call when this widget is selected via keyboard up/down or mouse hover,
    /// see [set_bind_on_select] for examples.
    pub bound_on_select: Option<fn(context: common::EzContext, mouse_position: Option<state::Coordinates>)>,

    /// Optional function to call when this widget is right clicked, see
    /// [MouseCallbackFunction] for the callback fn type, or [set_right_left_click] for
    /// examples.
    pub bound_on_deselect: Option<common::GenericEzFunction>,

    /// Optional function to call when this widget is keyboard entered, see
    /// [KeyboardCallbackFunction] for the callback fn type, or [set_bind_left_click] for
    /// examples.
    pub bound_keyboard_enter: Option<common::GenericEzFunction>,

    /// Optional function to call when this widget is right clicked, see
    /// [MouseCallbackFunction] for the callback fn type, or [set_bind_right_click] for
    /// examples.
    pub bound_right_mouse_click: Option<common::MouseCallbackFunction>,

    /// Optional function to call when the value of this widget changes, see
    /// [ValueChangeCallbackFunction] for the callback fn type, or [set_bind_on_value_change] for
    /// examples.
    pub bound_on_value_change: Option<common::GenericEzFunction>,

    /// A Key to callback function lookup used to store keybinds for this widget. See
    /// [KeyboardCallbackFunction] type for callback function signature.
    pub keymap: common::KeyMap,

    /// Runtime state of this widget, see [TextInputState] and [State]
    pub state: TextInputState,

}

impl Default for TextInput {
    fn default() -> Self {
        let mut obj = TextInput {
            id: "".to_string(),
            path: String::new(),
            selection_order: 0,
            bound_on_select: None,
            bound_on_deselect: None,
            bound_keyboard_enter: None,
            bound_right_mouse_click: None,
            bound_on_value_change: None,
            keymap: HashMap::new(),
            state: TextInputState::default(),
        };
        obj.bind_key(KeyCode::Backspace, handle_backspace);
        obj.bind_key(KeyCode::Delete, handle_delete);
        obj.bind_key(KeyCode::Left, handle_left);
        obj.bind_key(KeyCode::Right, handle_right);
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

    fn update_state(&mut self, new_state: &state::EzState) {
        let state = new_state.as_text_input();
        self.state = state.clone();
        self.state.changed = false;
        self.state.force_redraw = false;
    }
    fn get_state(&self) -> state::EzState { state::EzState::TextInput(self.state.clone()) }

    fn get_contents(&self, state_tree: &mut common::StateTree) -> common::PixelMap {

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

    fn get_key_map(&self) -> &common::KeyMap { &self.keymap }

    fn bind_key(&mut self, key: KeyCode, func: common::KeyboardCallbackFunction) {
        self.keymap.insert(key, func);
    }

    fn handle_event(&self, event: Event, context: common::EzContext) -> bool {
        if let Event::Key(key) = event {
            if self.get_key_map().contains_key(&key.code) {
                let func = self.get_key_map().get(&key.code).unwrap();
                func(context, key.code);
                return true
            };
            if let KeyCode::Char(c) = key.code {
                handle_char(context, c);
                return true
            }
        }
        false
    }

    fn set_bind_on_value_change(&mut self, func: common::GenericEzFunction) {
        self.bound_on_value_change = Some(func)
    }

    fn get_bind_on_value_change(&self) -> Option<common::GenericEzFunction> {
        self.bound_on_value_change }

    fn set_bind_keyboard_enter(&mut self, func: common::GenericEzFunction) {
        self.bound_keyboard_enter = Some(func)
    }

    fn get_bind_keyboard_enter(&self) -> Option<common::GenericEzFunction> {
        self.bound_keyboard_enter
    }

    fn set_bind_right_click(&mut self, func: common::MouseCallbackFunction) {
        self.bound_right_mouse_click = Some(func)
    }

    fn get_bind_right_click(&self) -> Option<common::MouseCallbackFunction> { self.bound_right_mouse_click}

    fn set_bind_on_select(&mut self, func: fn(common::EzContext, Option<state::Coordinates>)) {
        self.bound_on_select = Some(func);
    }

    fn get_bind_on_select(&self) -> Option<fn(common::EzContext, Option<state::Coordinates>)> {
        self.bound_on_select
    }

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
        if let Some(func) = self.bound_on_select {
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

/// Start blink the position on which the cursor is currently located. This is a custom cursor not
/// the actual terminal cursor. This is because in dynamic interfaces with scheduled tasks changing
/// visual content, the crossterm cursor is constantly jumping around, which cannot seem to be
/// resolved using the Hide/Show/SavePosition/RestorePosition methods.
fn start_cursor_blink(target_pos: state::Coordinates, state: &mut TextInputState,
                      scheduler: &mut Scheduler, name: String) {

    state.set_cursor_pos(target_pos);
    state.set_active_blink_task(true);
    let mut counter = 3;
    let blink_func = move | context: common::EzContext | {
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


/// Get the widget object, absolute position, cursor position, and position of the cursor
/// relative to the widget. These are commonly used data for keyboard callbacks for text input
/// widgets.
fn prepare_handle_function<'a>(widget_name: String, widget_tree: &'a common::WidgetTree,
                               state: &mut TextInputState)
    -> (&'a dyn EzWidget, state::Coordinates, state::Coordinates) {

    let widget_obj = widget_tree.get(&widget_name).unwrap();
    let widget_obj = widget_obj.as_ez_widget();
    let widget_pos = state.get_absolute_position();
    let cursor_pos = state.get_cursor_pos();
    (widget_obj, widget_pos, cursor_pos)
}

/// Given a view, return which parts of the widget text are visible. Also return the part that
/// comes before the view and after the view. Used by keyboard callbacks to alter the view.
fn get_view_parts(text: String, view_start: usize, widget_with: usize) -> (String, String, String) {

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
pub fn handle_right(context: common::EzContext, _key: KeyCode) {

    let state = context.state_tree.get_mut(&context.widget_path).unwrap()
        .as_text_input_mut();
    let (_widget_obj, _widget_pos, cursor_pos) =
        prepare_handle_function(context.widget_path.clone(), context.widget_tree, state);

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
pub fn handle_left(context: common::EzContext, _key: KeyCode) {

    let state = context.state_tree.get_mut(&context.widget_path).unwrap()
        .as_text_input_mut();
    let (_widget_obj, _widget_pos, cursor_pos) =
        prepare_handle_function(context.widget_path.clone(), context.widget_tree, state);

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
pub fn handle_delete(context: common::EzContext, _key: KeyCode) {

    let state = context.state_tree.get_mut(&context.widget_path).unwrap()
        .as_text_input_mut();
    let (widget_obj, _widget_pos, cursor_pos) =
        prepare_handle_function(context.widget_path.clone(), context.widget_tree, state);
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
    // Write changes to screen
    widget_obj.on_value_change(context);
}

/// Handle a backspace button press by user. Delete character to the left of the widget. Move the
/// cursor and/or view as necessary.
pub fn handle_backspace(context: common::EzContext, _key: KeyCode) {

    let state = context.state_tree.get_mut(&context.widget_path).unwrap()
        .as_text_input_mut();
    let (widget_obj, _widget_pos, cursor_pos) =
        prepare_handle_function(context.widget_path.clone(),
                                context.widget_tree, state);
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
    // Write changes to screen
    widget_obj.on_value_change(context);
}

/// Handle a char button press by user. insert the char at the cursor and move the cursor and/or
/// view where necessary.
pub fn handle_char(context: common::EzContext, char: char) {

    let state = context.state_tree.get_mut(&context.widget_path).unwrap().
        as_text_input_mut();
    if state.get_text().len() >= state.get_max_length() {
        return
    }
    let (widget_obj, _widget_pos, cursor_pos) =
        prepare_handle_function(context.widget_path.clone(),
                                context.widget_tree, state);
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
    widget_obj.on_value_change(context);
}
