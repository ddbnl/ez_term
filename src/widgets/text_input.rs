//! # Text input Widget
//! A widget implementing a field in which the user can input characters. Supports on_value_change
//! and on_keyboard_enter callbacks.
use std::collections::HashMap;
use std::io::{Error, ErrorKind, stdout, Write};
use std::time::Duration;
use crossterm::{cursor, QueueableCommand};
use crossterm::event::{Event, KeyCode};
use crossterm::style::{PrintStyledContent, Stylize};
use crate::states::text_input_state::TextInputState;
use crate::states::state::{EzState, GenericState, SelectableState};
use crate::widgets::widget::{EzWidget, Pixel, EzObject};
use crate::common::{KeyboardCallbackFunction, Coordinates, StateTree, WidgetTree,
                    PixelMap, GenericEzFunction, MouseCallbackFunction, EzContext, KeyMap};
use crate::ez_parser::{load_color_parameter, load_size_hint_parameter, load_halign_parameter,
                       load_valign_parameter, load_pos_hint_x_parameter, load_pos_hint_y_parameter};
use crate::scheduler::Scheduler;

pub struct TextInput {

    /// ID of the widget, used to construct [path]
    pub id: String,

    /// Full path to this widget, e.g. "/root_layout/layout_2/THIS_ID"
    pub path: String,

    /// Global order number in which this widget will be selection when user presses down/up keys
    pub selection_order: usize,

    /// Optional function to call when this widget is selected via keyboard up/down or mouse hover,
    /// see [set_bind_on_select] for examples.
    pub bound_on_select: Option<fn(context: EzContext, mouse_position: Option<Coordinates>)>,

    /// Optional function to call when this widget is right clicked, see
    /// [MouseCallbackFunction] for the callback fn type, or [set_right_left_click] for
    /// examples.
    pub bound_on_deselect: Option<GenericEzFunction>,

    /// Optional function to call when this widget is keyboard entered, see
    /// [KeyboardCallbackFunction] for the callback fn type, or [set_bind_left_click] for
    /// examples.
    pub bound_keyboard_enter: Option<GenericEzFunction>,

    /// Optional function to call when this widget is right clicked, see
    /// [MouseCallbackFunction] for the callback fn type, or [set_bind_right_click] for
    /// examples.
    pub bound_right_mouse_click: Option<MouseCallbackFunction>,

    /// Optional function to call when the value of this widget changes, see
    /// [ValueChangeCallbackFunction] for the callback fn type, or [set_bind_on_value_change] for
    /// examples.
    pub bound_on_value_change: Option<GenericEzFunction>,

    /// A Key to callback function lookup used to store keybinds for this widget. See
    /// [KeyboardCallbackFunction] type for callback function signature.
    pub keymap: KeyMap,

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
            "x" => self.state.x = parameter_value.trim().parse().unwrap(),
            "y" => self.state.y = parameter_value.trim().parse().unwrap(),
            "size_hint_x" => self.state.size_hint_x =
                load_size_hint_parameter(parameter_value.trim()).unwrap(),
            "pos_hint_x" => self.state.set_pos_hint_x(
                load_pos_hint_x_parameter(parameter_value.trim()).unwrap()),
            "pos_hint_y" => self.state.set_pos_hint_y(
                load_pos_hint_y_parameter(parameter_value.trim()).unwrap()),
            "width" => self.state.width = parameter_value.trim().parse().unwrap(),
            "halign" =>
                self.state.halign =  load_halign_parameter(parameter_value.trim()).unwrap(),
            "valign" =>
                self.state.valign =  load_valign_parameter(parameter_value.trim()).unwrap(),
            "max_length" => self.state.max_length = parameter_value.trim().parse().unwrap(),
            "selection_order" => {
                let order = parameter_value.trim().parse().unwrap();
                if order == 0 {
                    return Err(Error::new(ErrorKind::InvalidData,
                                          "selection_order must be higher than 0."))
                }
                self.selection_order = order;
            },
            "fg_color" =>
                self.state.content_foreground_color = load_color_parameter(parameter_value).unwrap(),
            "bg_color" =>
                self.state.content_background_color = load_color_parameter(parameter_value).unwrap(),
            "selection_fg_color" =>
                self.state.selection_foreground_color = load_color_parameter(parameter_value).unwrap(),
            "selection_bg_color" =>
                self.state.selection_background_color = load_color_parameter(parameter_value).unwrap(),
            "cursor_color" =>
                self.state.cursor_color = load_color_parameter(parameter_value).unwrap(),
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

    fn update_state(&mut self, new_state: &EzState) {
        let state = new_state.as_text_input();
        self.state = state.clone();
        self.state.changed = false;
        self.state.force_redraw = false;
    }
    fn get_state(&self) -> EzState { EzState::TextInput(self.state.clone()) }

    fn get_contents(&self, state_tree: &mut StateTree) -> PixelMap {

        let state = state_tree.get_mut(&self.get_full_path()).unwrap().as_text_input();
        let fg_color = if state.selected {state.selection_foreground_color}
                           else {state.content_foreground_color};
        let bg_color = if state.selected {state.selection_background_color}
                             else {state.content_background_color};
        let mut text = state.text.clone();
        if text.len() > state.width - 1 {
            let remains = text.len() - state.view_start;
            let view_end =
                if remains > (state.width - 1) {
                    state.view_start + (state.width - 1)
                } else {
                    text.len()
                };
            text = text[state.view_start..view_end].to_string();
        }
        let mut contents = Vec::new();
        text = text.chars().rev().collect::<String>();
        for x in 0..state.get_effective_width() {
            let mut new_y = Vec::new();
            for _ in 0..state.get_effective_height() {
                if !text.is_empty() {
                    new_y.push(Pixel{
                        symbol: text.pop().unwrap().to_string(),
                        foreground_color: fg_color,
                        background_color: if state.get_blink_switch() &&
                            x == state.get_cursor_pos().0 {state.get_cursor_color()}
                        else {bg_color},
                        underline: true})
                } else {
                    new_y.push(Pixel{
                        symbol: " ".to_string(),
                        foreground_color: fg_color,
                        background_color: if state.get_blink_switch() &&
                            x == state.get_cursor_pos().0 {state.get_cursor_color()}
                            else {bg_color},
                        underline: true})
                }
            }
            contents.push(new_y);
        }
        contents
    }
}

impl EzWidget for TextInput {

    fn is_selectable(&self) -> bool { true }

    fn is_selected(&self) -> bool { self.state.selected }

    fn get_selection_order(&self) -> usize { self.selection_order }

    fn get_key_map(&self) -> &KeyMap { &self.keymap }

    fn bind_key(&mut self, key: KeyCode, func: KeyboardCallbackFunction) {
        self.keymap.insert(key, func);
    }

    fn handle_event(&self, event: Event, context: EzContext) -> bool {
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

    fn set_bind_on_value_change(&mut self, func: GenericEzFunction) {
        self.bound_on_value_change = Some(func)
    }

    fn get_bind_on_value_change(&self) -> Option<GenericEzFunction> {
        self.bound_on_value_change }

    fn set_bind_keyboard_enter(&mut self, func: GenericEzFunction) {
        self.bound_keyboard_enter = Some(func)
    }

    fn get_bind_keyboard_enter(&self) -> Option<GenericEzFunction> {
        self.bound_keyboard_enter
    }

    fn set_bind_right_click(&mut self, func: MouseCallbackFunction) {
        self.bound_right_mouse_click = Some(func)
    }

    fn get_bind_right_click(&self) -> Option<MouseCallbackFunction> { self.bound_right_mouse_click}

    fn set_bind_on_select(&mut self, func: fn(EzContext, Option<Coordinates>)) {
        self.bound_on_select = Some(func);
    }

    fn get_bind_on_select(&self) -> Option<fn(EzContext, Option<Coordinates>)> {
        self.bound_on_select
    }

    fn on_select(&self, context: EzContext, mouse_pos: Option<Coordinates>) {

        let state = context.state_tree.get_mut(&self.get_full_path())
            .unwrap().as_text_input_mut();
        state.set_selected(true);

        // Handle blinking of cursor
        let target_pos;
        // Handle this widget being selected from mouse, follow user click position
        if let Some(pos) = mouse_pos {
            let (mut x, y) = (pos.0, pos.1);
            if pos.0 > state.text.len() {x = state.text.len()};
            target_pos = (x, y);
            if !state.active_blink_task {
                start_cursor_blink(target_pos, state, context.scheduler, self.get_full_path());
            } else {
                state.set_cursor_pos(target_pos);
                state.set_blink_switch(true);
            }
            // Handle this widget being selected from keyboard. We choose the position.
        } else {
            // If text fills the widget move to end of widget. If not, move to end of text.
            let target_pos_in_widget = if state.text.len() > (state.get_effective_width() - 1)
            {state.get_effective_width() - 1} else {state.text.len()};
            target_pos = (target_pos_in_widget, 0);
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
    pub fn from_config(config: Vec<&str>, id: String) -> Self {
        let mut obj = TextInput::default();
        obj.set_id(id);
        obj.load_ez_config(config).unwrap();
        obj
    }
}

/// Start blink the position on which the cursor is currently located. This is a custom cursor not
/// the actual terminal cursor. This is because in dynamic interfaces with scheduled tasks changing
/// visual content, the crossterm cursor is constantly jumping around, which cannot seem to be
/// resolved using the Hide/Show/SavePosition/RestorePosition methods.
fn start_cursor_blink(target_pos: Coordinates, state: &mut TextInputState,
                      scheduler: &mut Scheduler, name: String) {

    state.set_cursor_pos(target_pos);
    state.set_active_blink_task(true);
    let blink_func = move | context: EzContext | {
        let state = context.state_tree.get_mut(&context.widget_path).unwrap()
            .as_text_input_mut();
        if !state.selected {
            state.set_blink_switch(false);
            state.set_active_blink_task(false);
            return false
        };
        state.set_blink_switch(!state.get_blink_switch());
        true
    };
    scheduler.schedule_interval(name, Box::new(blink_func),
                                Duration::from_millis(500));
}


/// Get the widget object, absolute position, cursor position, and position of the cursor
/// relative to the widget. These are commonly used data for keyboard callbcks for text input
/// widgets.
fn prepare_handle_function<'a>(widget_name: String, widget_tree: &'a WidgetTree,
                               state: &TextInputState)
    -> (&'a dyn EzWidget, Coordinates, Coordinates) {

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
pub fn handle_right(context: EzContext, _key: KeyCode) {

    let state = context.state_tree.get_mut(&context.widget_path).unwrap()
        .as_text_input_mut();
    let (_widget_obj, widget_pos, cursor_pos) =
        prepare_handle_function(context.widget_path.clone(), context.widget_tree, state);

    // Text does not fit in widget, advance view
    if state.get_text().len() > state.get_width() - 1 && cursor_pos.0 >= state.get_width() - 2 &&
             state.get_text().len() - state.get_view_start() > (state.get_width() - 1) {
        if state.get_text().len() - state.get_view_start() - state.get_width() >= 4 {
            state.set_view_start(state.get_view_start() + 4);
            state.set_cursor_pos((state.get_cursor_pos().0 - 3, state.get_cursor_pos().1));
        } else {
            state.set_view_start(state.get_text().len() - state.get_width() + 1);
            state.set_cursor_pos((state.get_width() - 1, state.get_cursor_pos().1));
        }
    // Text does not fit in widget but can't move further
    } else if state.get_text().len() > state.get_width() - 1 &&
        cursor_pos.0 == state.get_width() - 1 {} // Max view, nothing to do
    // Text fits in widget, handle normally
    else if cursor_pos.0 < state.get_text().len() {
        state.set_cursor_pos((state.get_cursor_pos().0 + 1, state.get_cursor_pos().1));
    }
}

/// Handle a left arrow button press by user. Move cursor to the left or move the
/// view if the cursor was at the edge of the widget.
pub fn handle_left(context: EzContext, _key: KeyCode) {

    let state = context.state_tree.get_mut(&context.widget_path).unwrap()
        .as_text_input_mut();
    let (_widget_obj, _widget_pos, cursor_pos) =
        prepare_handle_function(context.widget_path.clone(), context.widget_tree, state);

    // Text does not fit in widget and cursor at 0, move view to left if not at 0 already
    if state.get_text().len() > state.get_width() - 1 &&
            cursor_pos.0 <= 1 && state.get_view_start() > 0 {
        if state.get_view_start() >= 4 {
            state.set_view_start(state.get_view_start() - 4 );
            state.set_cursor_pos((cursor_pos.0 + 4, cursor_pos.1));
        } else {
            state.set_view_start(0);
            state.set_cursor_pos((4, cursor_pos.1));
        }
    // Text fits in widget or cursor pos is not at 0, move cursor normally
    } else if cursor_pos.0 > 0 {
        state.set_cursor_pos((state.get_cursor_pos().0 - 1, state.get_cursor_pos().1));
    }
}

/// Handle a delete button press by user. Delete character to the right of the widget. Move the
/// view as necessary.
pub fn handle_delete(context: EzContext, _key: KeyCode) {

    let state = context.state_tree.get_mut(&context.widget_path).unwrap()
        .as_text_input_mut();
    let (widget_obj, _widget_pos, cursor_pos) =
        prepare_handle_function(context.widget_path.clone(), context.widget_tree, state);
    // Check if text does not fit in widget, then we have to delete on a view
    if state.get_text().len() > state.get_width() - 1 {
        // Get the view on the string, as well pre- and post to reconstruct it later
        let (pre_view_text, mut view_text, mut post_view_text) =
            get_view_parts(state.get_text(), state.get_view_start(),
                           state.get_width());

        if cursor_pos.0 == state.get_width() - 1 && post_view_text.is_empty() {
            return
        }
        // Check if deleting in the last position, i.e. deleting out of view
        if cursor_pos.0 > view_text.len() - 1 {
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
            view_text = format!("{}{}", view_text[0..cursor_pos.0 as usize].to_string(),
                                view_text[(cursor_pos.0 + 1) as usize..view_text.len()].to_string());
        }

        // Reconstruct text with backspace view
        state.set_text(format!("{}{}{}", pre_view_text, view_text, post_view_text));
        // If we're viewing the start of a string then delete should move the view
        // forward if it's not already at the end
    }
    // Check if text fits in widget, then delete text as normal
    else {
        // Check if cursor is ahead of text, i.e. nothing to delete
        if cursor_pos.0 == state.get_text().len() {
            return
        }
        let mut text = state.get_text();
        text = format!("{}{}", text[0..cursor_pos.0 as usize].to_string(),
                       text[(cursor_pos.0 + 1) as usize..text.len()].to_string());
        state.set_text(text);
    }
    // Write changes to screen
    widget_obj.on_value_change(context);
}

/// Handle a backspace button press by user. Delete character to the left of the widget. Move the
/// cursor and/or view as necessary.
pub fn handle_backspace(context: EzContext, _key: KeyCode) {

    let state = context.state_tree.get_mut(&context.widget_path).unwrap()
        .as_text_input_mut();
    let (widget_obj, _widget_pos, cursor_pos) =
        prepare_handle_function(context.widget_path.clone(),
                                context.widget_tree, state);
    let mut text = state.get_text();

    // Check if text does not fit in widget, then we have to backspace on a view
    if state.get_text().len() > state.get_width() - 1 {
        // Check if cursor is at start of text, i.e. nothing to backspace
        if cursor_pos.0 == 0 && state.get_view_start() == 0 {
            return
        }
        let (mut pre_view_text, mut view_text, post_view_text) =
            get_view_parts(state.get_text(), state.get_view_start(),
                           state.get_width());
        // Perform backspace on the text view
        if cursor_pos.0 == 0 {
            // Backspace out of view
            pre_view_text = pre_view_text[0..pre_view_text.len() - 1].to_string();
        } else {
            // Backspace in view
            view_text = format!("{}{}",
                                view_text[0..(cursor_pos.0 - 1) as usize].to_string(),
                                view_text[cursor_pos.0 as usize..view_text.len()].to_string());
        }
        // Reconstruct text with backspace view
        state.set_text(format!("{}{}{}", pre_view_text, view_text, post_view_text));

        // Backspace should move the view back if it's not already at the start
        if state.view_start > 1 && post_view_text.is_empty() {
            state.set_view_start(state.get_view_start() - 1);
        }
        // If backspacing out of view move back view 2 times if possible
        if state.view_start > 1 && cursor_pos.0 == 0 && !pre_view_text.is_empty() {
            state.set_view_start(state.get_view_start() - 1);
        }

        if (cursor_pos.0 > 0 && pre_view_text.is_empty()) ||
            (cursor_pos.0 == state.get_width() - 1 && !post_view_text.is_empty()) {
            state.set_cursor_pos((state.get_cursor_pos().0 - 1, state.get_cursor_pos().1));
        }
    }
    // Check if text fits in widget, then backspace text as normal
    else {
        // Check if cursor is at start of text, i.e. nothing to backspace
        if cursor_pos.0 == 0 {
            return
        }
        // Perform backspace on text
        text = format!("{}{}", text[0..(cursor_pos.0 - 1) as usize].to_string(),
                       text[cursor_pos.0 as usize..text.len()].to_string());
        state.set_text(text);
        state.set_cursor_pos((state.get_cursor_pos().0 - 1, state.get_cursor_pos().1));
    }
    // Write changes to screen
    widget_obj.on_value_change(context);
}

/// Handle a char button press by user. insert the char at the cursor and move the cursor and/or
/// view where necessary.
pub fn handle_char(context: EzContext, char: char) {

    let state = context.state_tree.get_mut(&context.widget_path).unwrap().
        as_text_input_mut();
    if state.get_text().len() >= state.get_max_length() {
        return
    }
    let (widget_obj, _widget_pos, cursor_pos) =
        prepare_handle_function(context.widget_path.clone(),
                                context.widget_tree, &state);
    let mut text;

    // Text still fits in widget, add char as normal
    if state.get_text().len() < (state.get_width() - 3) {
        // Get position of cursor in text for validation and deleting
        text = state.get_text();
        text = format!("{}{}{}", text[0..cursor_pos.0 as usize].to_string(), char,
                       text[(cursor_pos.0) as usize..text.len()].to_string());
        state.set_text(text);
    }
    // Text does not fit in widget, add char to view
    else {
        let (pre_view_text, mut view_text, post_view_text) =
            get_view_parts(state.get_text(), state.get_view_start(),
                           state.get_width());
        view_text = format!("{}{}{}",
                            view_text[0..cursor_pos.0 as usize].to_string(), char,
                            view_text[(cursor_pos.0) as usize..view_text.len()].to_string());
        let new_text = format!("{}{}{}", pre_view_text, view_text, post_view_text);
        state.set_text(new_text);
    }

    if state.get_text().len() < (state.get_width() - 1){
        state.set_cursor_pos((state.get_cursor_pos().0 + 1, state.get_cursor_pos().1));
    } else if cursor_pos.0 >= state.get_width() - 2 {
        state.set_view_start(state.get_view_start() + 1);
    }
    widget_obj.on_value_change(context);
}
