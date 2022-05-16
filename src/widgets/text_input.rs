//! # Text input Widget
//! A widget implementing a field in which the user can input characters. Supports on_value_change
//! and on_keyboard_enter callbacks.
use std::collections::HashMap;
use std::io::{Error, ErrorKind, stdout, Write};
use crossterm::{cursor, QueueableCommand};
use crossterm::event::{Event, KeyCode};
use crossterm::style::{Color};
use crate::widgets::widget_state::{WidgetState, RedrawWidgetState, SelectableWidgetState};
use crate::widgets::widget::{EzWidget, Pixel, EzObject};
use crate::common::{self, KeyboardCallbackFunction, Coordinates, StateTree, ViewTree, WidgetTree,
                    PixelMap, GenericCallbackFunction};
use crate::ez_parser::{load_color_parameter};

pub struct TextInput {

    /// ID of the widget, used to construct [path]
    pub id: String,

    /// Full path to this widget, e.g. "/root_layout/layout_2/THIS_ID"
    pub path: String,

    /// Horizontal position of this widget relative to its' parent [Layout]
    pub x: usize,

    /// Vertical position of this widget relative to its' parent [Layout]
    pub y: usize,

    /// Width of this widget
    pub width: usize,

    /// Height of this widget
    pub height: usize,

    /// Absolute position of this layout on screen. Automatically propagated, do not set manually
    pub absolute_position: Coordinates,

    /// The [Pixel.foreground_color] to use for this widgets' content
    pub content_foreground_color: Color,

    /// The [Pixel.background_color] to use for this widgets' content
    pub content_background_color: Color,

    /// The [Pixel.foreground_color] to use for this widgets' content when selected
    pub selection_foreground_color: Color,

    /// The [Pixel.background_color] to use for this widgets' content when selected
    pub selection_background_color: Color,

    /// Global order number in which this widget will be selection when user presses down/up keys
    pub selection_order: usize,

    /// Optional function to call when this widget is keyboard entered, see
    /// [KeyboardCallbackFunction] for the callback fn type, or [set_bind_left_click] for
    /// examples.
    pub bound_keyboard_enter: Option<GenericCallbackFunction>,

    /// Optional function to call when this widget is left clicked, see
    /// [MouseCallbackFunction] for the callback fn type, or [set_bind_left_click] for
    /// examples.
    pub bound_right_mouse_click: Option<fn(pos: Coordinates)>,

    /// Optional function to call when the value of this widget changes, see
    /// [ValueChangeCallbackFunction] for the callback fn type, or [set_bind_on_value_change] for
    /// examples.
    pub bound_on_value_change: Option<GenericCallbackFunction>,

    /// A Key to callback function lookup used to store keybinds for this widget. See
    /// [KeyboardCallbackFunction] type for callback function signature.
    pub keymap: HashMap<KeyCode, KeyboardCallbackFunction>,

    /// Runtime state of this widget, see [TextInputState] and [WidgetState]
    pub state: TextInputState,

}

impl Default for TextInput {
    fn default() -> Self {
        let mut obj = TextInput {
            id: "".to_string(),
            path: String::new(),
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            absolute_position: (0, 0),
            content_background_color: Color::Black,
            content_foreground_color: Color::White,
            selection_background_color: Color::Blue,
            selection_foreground_color: Color::Yellow,
            selection_order: 0,
            bound_keyboard_enter: None,
            bound_right_mouse_click: None,
            bound_on_value_change: None,
            keymap: HashMap::new(),
            state: TextInputState{view_start: 0, selected: false, text: String::new(),
                max_length: 10000, force_redraw: false},
        };
        obj.bind_key(KeyCode::Backspace, handle_backspace);
        obj.bind_key(KeyCode::Delete, handle_delete);
        obj.bind_key(KeyCode::Left, handle_left);
        obj.bind_key(KeyCode::Right, handle_right);
        obj
    }
}


/// [WidgetState] implementation.
#[derive(Clone)]
pub struct TextInputState {
    /// Text currently being displayed by the text input
    pub text: String,

    /// If text is larger than the widget, only a part of the text can be displayed. This is the
    /// index of where to start viewing the text.
    pub view_start: usize,

    /// Bool representing whether this widget is currently selected.
    pub selected: bool,

    /// How many characters [text] may hold
    pub max_length: usize,

    /// If true this forces a global screen redraw on the next frame. Screen redraws are diffed
    /// so this can be called when needed without degrading performance. If only screen positions
    /// that fall within this widget must be redrawn, call [EzObject.redraw] instead.
    pub force_redraw: bool,
}
impl RedrawWidgetState for TextInputState {
    fn set_force_redraw(&mut self, redraw: bool) { self.force_redraw = redraw }
    fn get_force_redraw(&self) -> bool { self.force_redraw }
}
impl SelectableWidgetState for TextInputState {
    fn set_selected(&mut self, state: bool) { self.selected = state }
    fn get_selected(&self) -> bool { self.selected }
}


impl EzObject for TextInput {

    fn load_ez_parameter(&mut self, parameter_name: String, mut parameter_value: String)
                         -> Result<(), Error> {
        match parameter_name.as_str() {
            "x" => self.x = parameter_value.trim().parse().unwrap(),
            "y" => self.y = parameter_value.trim().parse().unwrap(),
            "width" => self.width = parameter_value.trim().parse().unwrap(),
            "height" => self.height = parameter_value.trim().parse().unwrap(),
            "maxLength" => self.state.max_length = parameter_value.trim().parse().unwrap(),
            "selectionOrder" => {
                let order = parameter_value.trim().parse().unwrap();
                if order == 0 {
                    return Err(Error::new(ErrorKind::InvalidData,
                                          "selectionOrder must be higher than 0."))
                }
                self.selection_order = order;
            },
            "contentForegroundColor" =>
                self.content_foreground_color = load_color_parameter(parameter_value).unwrap(),
            "contentBackgroundColor" =>
                self.content_background_color = load_color_parameter(parameter_value).unwrap(),
            "selectionForegroundColor" =>
                self.selection_foreground_color = load_color_parameter(parameter_value).unwrap(),
            "selectionBackgroundColor" =>
                self.selection_background_color = load_color_parameter(parameter_value).unwrap(),
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

    fn redraw(&mut self, view_tree: &mut ViewTree) {

        let pos = self.get_absolute_position();
        let content = self.get_contents();
        common::write_to_screen(pos, content, view_tree);
        if self.state.selected {
            self.select().unwrap();
        } else {
            self.deselect().unwrap();
        }
    }

    fn get_contents(&mut self) -> PixelMap {

        let fg_color = if self.state.selected {self.get_selection_foreground_color()}
                           else {self.get_content_foreground_color()};
        let bg_color = if self.state.selected {self.get_selection_background_color()}
                             else {self.get_content_background_color()};
        let mut text = self.state.text.clone();
        if text.len() > self.width - 1 {
            let remains = text.len() - self.state.view_start;
            let view_end =
                if remains > (self.width - 1) {
                    self.state.view_start + (self.width - 1)
                } else {
                    text.len()
                };
            text = text[self.state.view_start..view_end].to_string();
        }
        let mut contents = Vec::new();
        text = text.chars().rev().collect::<String>();
        for _ in 0..self.get_width() {
            let mut new_y = Vec::new();
            for _ in 0..self.get_height() {
                if !text.is_empty() {
                    new_y.push(Pixel{
                        symbol: text.pop().unwrap().to_string(),
                        foreground_color: fg_color, background_color: bg_color, underline: true})
                } else {
                    new_y.push(Pixel{
                        symbol: " ".to_string(),
                        foreground_color: fg_color, background_color: bg_color, underline: true})
                }
            }
            contents.push(new_y);
        }
        contents
    }
    fn set_width(&mut self, width: usize) { self.width = width  }

    fn get_width(&self) -> usize { self.width }

    fn set_height(&mut self, height: usize) { self.height = height }

    fn get_height(&self) -> usize { self.height }

    fn set_position(&mut self, position: Coordinates) {
        self.x = position.0;
        self.y = position.1;
    }

    fn get_position(&self) -> Coordinates { (self.x, self.y) }

    fn set_absolute_position(&mut self, pos: Coordinates) {
       self.absolute_position = pos
    }

    fn get_absolute_position(&self) -> Coordinates {
       self.absolute_position
    }

}

impl EzWidget for TextInput {

    fn get_state(&self) -> WidgetState {
        WidgetState::TextInput(self.state.clone())
    }

    fn set_content_foreground_color(&mut self, color: Color) { self.content_foreground_color = color }

    fn get_content_foreground_color(&self) -> Color { self.content_foreground_color }

    fn set_content_background_color(&mut self, color: Color) { self.content_background_color = color }

    fn get_content_background_color(&self) -> Color { self.content_background_color }

    fn set_selection_foreground_color(&mut self, color: Color) {
        self.selection_foreground_color = color }

    fn get_selection_foreground_color(&self) -> Color { self.selection_foreground_color }

    fn set_selection_background_color(&mut self, color: Color) {
        self.selection_background_color = color }

    fn get_selection_background_color(&self) -> Color { self.selection_background_color }

    fn get_key_map(&self) -> &HashMap<KeyCode, KeyboardCallbackFunction> { &self.keymap }

    fn bind_key(&mut self, key: KeyCode, func: KeyboardCallbackFunction) {
        self.keymap.insert(key, func);
    }

    fn handle_event(&self, event: Event, view_tree: &mut ViewTree,
                    state_tree: & mut StateTree, widget_tree: &WidgetTree) -> bool {
        if let Event::Key(key) = event {
            if self.get_key_map().contains_key(&key.code) {
                let func = self.get_key_map().get(&key.code).unwrap();
                func(self.get_full_path(), key.code, view_tree, state_tree, widget_tree);
                return true
            };
            if let KeyCode::Char(c) = key.code {
                handle_char(self.get_full_path(), c, view_tree, state_tree,
                            widget_tree);
                return true
            }
        }
        false
    }

    fn is_selectable(&self) -> bool { true }

    fn is_selected(&self) -> bool { self.state.selected }

    fn get_selection_order(&self) -> usize { self.selection_order }

    fn set_bind_on_value_change(&mut self, func: GenericCallbackFunction) {
        self.bound_on_value_change = Some(func)
    }

    fn get_bind_on_value_change(&self) -> Option<GenericCallbackFunction> {
        self.bound_on_value_change }

    fn set_bind_keyboard_enter(&mut self, func: GenericCallbackFunction) {
        self.bound_keyboard_enter = Some(func)
    }

    fn get_bind_keyboard_enter(&self) -> Option<GenericCallbackFunction> {
        self.bound_keyboard_enter
    }

    /// On left click select the widget and also show the cursor at the position the user clicked
    /// in the widget.
    fn on_left_click(&self, position: Coordinates, _view_tree: &mut ViewTree,
                     state_tree: &mut StateTree, _widget_tree: &WidgetTree) {
        let mut state = state_tree.get_mut(&self.get_full_path()).unwrap().as_text_input_mut();

        let abs = self.get_absolute_position();
        let (mut x, y) = (abs.0 + position.0, abs.1 + position.1);
        if position.0 > state.text.len() {x = abs.0 + state.text.len()};
        state.selected = true;
        stdout().queue(cursor::MoveTo(x as u16, y as u16)).unwrap()
            .queue(cursor::Show).unwrap().flush().unwrap();
    }

    fn set_bind_right_click(&mut self, func: fn(Coordinates)) {
        self.bound_right_mouse_click = Some(func)
    }

    fn get_bind_right_click(&self) -> Option<fn(Coordinates)> { self.bound_right_mouse_click}

    fn state_changed(&self, other_state: &WidgetState) -> bool {
        let state = other_state.as_text_input();
        if state.selected != self.state.selected { return true };
        if state.text != self.state.text { return true };
        if state.view_start != self.state.view_start { return true };
        if state.max_length != self.state.max_length { return true }
        false

    }
    fn update_state(&mut self, new_state: &WidgetState) {
        let state = new_state.as_text_input();
        self.state = state.clone();
        self.state.force_redraw = false;
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

    /// Called when this widget is selected by keyboard. Moves the cursor to the end of the text
    /// if it was not already inside the text.
    fn select(&mut self) -> crossterm::Result<()> {
        stdout().queue(cursor::Hide)?.flush()?;
        let pos = self.get_absolute_position();
        let cursor_pos = cursor::position().unwrap();
        if !self.collides((cursor_pos.0 as usize, cursor_pos.1 as usize)) {
            let move_to_pos = if self.state.text.len() > (self.width - 1) {self.width - 1}
            else {self.state.text.len()};
            stdout().queue(cursor::MoveTo((pos.0 + move_to_pos) as u16,
                                          pos.1 as u16))?.queue(cursor::Show)?.flush()?;
        }
        stdout().queue(cursor::Show)?.flush()?;
        Ok(())
    }

    /// Called when the widget is deselected; hides the cursor again.
    fn deselect(&mut self) -> crossterm::Result<()> {
        stdout().queue(cursor::Hide)?.flush()?;
        Ok(())
    }
}


fn prepare_handle_function<'a>(widget_name: String, widget_tree: &'a WidgetTree)
    -> (&'a dyn EzWidget, Coordinates, Coordinates, Coordinates) {

    let widget_obj = widget_tree.values()
        .map(|x| x.as_ez_widget())
        .filter(|x| x.get_full_path() == widget_name).last().unwrap();
    let widget_pos = widget_obj.get_absolute_position();
    let cursor_pos = cursor::position().unwrap();
    let cursor_pos = (cursor_pos.0 as usize, cursor_pos.1 as usize);
    let text_pos = (cursor_pos.0 - (widget_pos.0), cursor_pos.1 - (widget_pos.1));
    (widget_obj, widget_pos, cursor_pos, text_pos)
}

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

pub fn handle_right(widget: String, _key: KeyCode, _view_tree: &mut ViewTree,
                    state_tree: &mut StateTree, widget_tree: &WidgetTree) {

    let (widget_obj, widget_pos, cursor_pos,
        text_pos) = prepare_handle_function(widget.clone(), widget_tree);
    let state = state_tree.get_mut(&widget).unwrap().as_text_input_mut();

    // Text does not fit in widget, advance view
    if state.text.len() > widget_obj.get_width() - 1 && text_pos.0 == widget_obj.get_width() - 1 &&
             state.text.len() - state.view_start > (widget_obj.get_width() - 1) {
        state.view_start += 1;
        state.selected = true;
    // Text does not fit in widget but can't move further
    } else if state.text.len() > widget_obj.get_width() - 1 &&
        text_pos.0 == widget_obj.get_width() - 1 { // Max view, nothing to do
    // Text fits in widget, handle normally
    } else if cursor_pos.0 < widget_pos.0 + state.text.len() {
        stdout().queue(cursor::MoveRight(1)).unwrap()
            .flush().unwrap();
    }
}
pub fn handle_left(widget: String, _key: KeyCode, _view_tree: &mut ViewTree,
                   state_tree: &mut StateTree, widget_tree: &WidgetTree) {

    let (widget_obj, _widget_pos, _cursor_pos,
        text_pos) = prepare_handle_function(widget.clone(), widget_tree);
    let state = state_tree.get_mut(&widget).unwrap().as_text_input_mut();

    // Text does not fit in widget and cursor at 0, move view to left if not at 0 already
    if state.text.len() > widget_obj.get_width() - 1 && text_pos.0 == 0 && state.view_start >  0 {
        state.view_start -= 1;
        state.selected = true;
    // Text fits in widget or cursor pos is not at 0, move cursor normally
    } else if text_pos.0 > 0 {
        stdout().queue(cursor::MoveLeft(1)).unwrap()
            .flush().unwrap();
    }
}

pub fn handle_delete(widget: String, _key: KeyCode, view_tree: &mut ViewTree,
                     state_tree: &mut StateTree, widget_tree: &WidgetTree) {

    let (widget_obj, _widget_pos, _cursor_pos,
        text_pos) = prepare_handle_function(widget.clone(), widget_tree);
    let state = state_tree.get_mut(&widget).unwrap().as_text_input_mut();

    // Check if text does not fit in widget, then we have to delete on a view
    if state.text.len() > widget_obj.get_width() - 1 {
        // Get the view on the string, as well pre- and post to reconstruct it later
        let (pre_view_text, mut view_text, mut post_view_text) =
            get_view_parts(state.text.clone(), state.view_start,
                           widget_obj.get_width());

        if text_pos.0 == widget_obj.get_width() - 1 && post_view_text.is_empty() {
            return
        }
        // Check if deleting in the last position, i.e. deleting out of view
        if text_pos.0 > view_text.len() - 1 {
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
            view_text = format!("{}{}", view_text[0..text_pos.0 as usize].to_string(),
                                view_text[(text_pos.0 + 1) as usize..view_text.len()].to_string());
        }

        // Reconstruct text with backspace view
        state.text = format!("{}{}{}", pre_view_text, view_text, post_view_text);
        // If we're viewing the start of a string then delete should move the view
        // forward if it's not already at the end
    }
    // Check if text fits in widget, then delete text as normal
    else {
        // Check if cursor is ahead of text, i.e. nothing to delete
        if text_pos.0 == state.text.len() {
            return
        }
        let mut text = state.text.clone();
        text = format!("{}{}", text[0..text_pos.0 as usize].to_string(),
                       text[(text_pos.0 + 1) as usize..text.len()].to_string());
        state.text = text;
    }
    // Write changes to screen
    state.selected = true;
    widget_obj.on_value_change(widget_obj.get_full_path(), view_tree, state_tree,
                               widget_tree);
}

pub fn handle_backspace(widget: String, _key: KeyCode, view_tree: &mut ViewTree,
                        state_tree: &mut StateTree, widget_tree: &WidgetTree) {

    let (widget_obj, _widget_pos, _cursor_pos,
        text_pos) = prepare_handle_function(widget.clone(), widget_tree);
    let state = state_tree.get_mut(&widget).unwrap().as_text_input_mut();
    let mut text = state.text.clone();

    // Check if text does not fit in widget, then we have to backspace on a view
    if state.text.len() > widget_obj.get_width() - 1 {
        // Check if cursor is at start of text, i.e. nothing to backspace
        if text_pos.0 == 0 && state.view_start == 0 {
            return
        }
        let (mut pre_view_text, mut view_text, post_view_text) =
            get_view_parts(state.text.clone(), state.view_start,
                           widget_obj.get_width());
        // Perform backspace on the text view
        if text_pos.0 == 0 {
            // Backspace out of view
            pre_view_text = pre_view_text[0..pre_view_text.len() - 1].to_string();
        } else {
            // Backspace in view
            view_text = format!("{}{}",
                                view_text[0..(text_pos.0 - 1) as usize].to_string(),
                                view_text[text_pos.0 as usize..view_text.len()].to_string());
        }
        // Reconstruct text with backspace view
        state.text = format!("{}{}{}", pre_view_text, view_text, post_view_text);

        // Backspace should move the view back if it's not already at the start
        if state.view_start > 1 && post_view_text.is_empty() {
            state.view_start -= 1;
        }
        // If backspacing out of view move back view 2 times if possible
        if state.view_start >= 2 && text_pos.0 == 0 && !pre_view_text.is_empty() {
            state.view_start -= 2
        }
        // Backspace should always move the cursor back one space in we're in the
        // middle of a view. At the start of a view we move the view, not the cursor.
        // At the end of a view, we move the cursor back if there's more content so
        // it can come into view. If no more content we don't so pre_view_content can
        // come into view from the left instead.
        if (text_pos.0 < widget_obj.get_width() - 1 && text_pos.0 != 0) ||
            (text_pos.0 == widget_obj.get_width() - 1 && !post_view_text.is_empty()) {
            stdout().queue(cursor::MoveLeft(1)).unwrap();
        }
    }
    // Check if text fits in widget, then backspace text as normal
    else {
        // Check if cursor is at start of text, i.e. nothing to backspace
        if text_pos.0 == 0 {
            return
        }
        // Perform backspace on text
        text = format!("{}{}", text[0..(text_pos.0 - 1) as usize].to_string(),
                       text[text_pos.0 as usize..text.len()].to_string());
        state.text = text;
        stdout().queue(cursor::MoveLeft(1)).unwrap();
    }
    // Write changes to screen
    state.selected = true;
    widget_obj.on_value_change(widget_obj.get_full_path(), view_tree, state_tree,
                               widget_tree);
}

pub fn handle_char(widget: String, char: char, view_tree: &mut ViewTree, state_tree: &mut StateTree,
                   widget_tree: &WidgetTree) {

    let state = state_tree.get_mut(&widget).unwrap().as_text_input_mut();
    if state.text.len() >= state.max_length {
        return
    }
    let (widget_obj, _widget_pos, _cursor_pos,
        text_pos) = prepare_handle_function(widget.clone(), widget_tree);
    let mut text;

    // Text still fits in widget, add char as normal
    if state.text.len() < (widget_obj.get_width() - 3) {
        // Get position of cursor in text for validation and deleting
        text = state.text.clone();
        text = format!("{}{}{}", text[0..text_pos.0 as usize].to_string(), char,
                       text[(text_pos.0) as usize..text.len()].to_string());
        state.text = text;
    }
    // Text does not fit in widget, add char to view
    else {
        let (pre_view_text, mut view_text, post_view_text) =
            get_view_parts(state.text.clone(), state.view_start,
                           widget_obj.get_width());
        view_text = format!("{}{}{}",
                            view_text.clone()[0..text_pos.0 as usize].to_string(), char,
                            view_text[(text_pos.0) as usize..view_text.len()].to_string());
        let new_text = format!("{}{}{}", pre_view_text, view_text, post_view_text);
        state.text = new_text;
    }

    if state.text.len() < (widget_obj.get_width() - 1){
        stdout().queue(cursor::MoveRight(1)).unwrap().flush().unwrap();
    } else if text_pos.0 >= widget_obj.get_width() - 2 {
        state.view_start += 1;
    }
    widget_obj.on_value_change(widget_obj.get_full_path(), view_tree, state_tree,
                               widget_tree);
}
