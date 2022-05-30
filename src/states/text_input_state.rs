use std::time::Duration;
use crossterm::event::{Event, KeyCode};
use crate::states::state::{self, GenericState};
use crate::widgets::widget::{EzWidget};
use crate::scheduler;
use crate::common;


/// [State] implementation.
pub struct TextInputState {
    
    /// Text currently being displayed by the text input
    pub text: String,

    /// Position of this widget relative to its' parent [Layout]
    pub position: state::Coordinates,

    /// Absolute position of this widget on screen. Automatically propagated, do not set manually
    pub absolute_position: state::Coordinates,

    /// size of this widget
    pub size: state::Size,

    /// Relative height/width of this widget to parent layout
    pub size_hint: state::SizeHint,

    /// Pos hint of this widget
    pub pos_hint: state::PosHint,

    /// Automatically adjust size of widget to content
    pub auto_scale: state::AutoScale,

    /// Amount of space to leave between sides of the widget and other widgets
    pub padding: state::Padding,

    /// Horizontal alignment of this widget
    pub halign: state::HorizontalAlignment,

    /// Vertical alignment of this widget
    pub valign: state::VerticalAlignment,
    
    /// Position of cursor relative to this widget
    pub cursor_pos: state::Coordinates,

    /// Bool representing whether we have a blinking scheduled task running
    pub active_blink_task: bool,

    /// Switch for blinking. When true displays [cursor_color] on the [cursor_pos]
    pub blink_switch: bool,

    /// If text is larger than the widget, only a part of the text can be displayed. This is the
    /// index of where to start viewing the text.
    pub view_start: usize,

    /// Bool representing whether this widget is currently selected.
    pub selected: bool,

    /// How many characters [text] may hold
    pub max_length: usize,

    /// Bool representing whether this layout should have a surrounding border
    pub border: bool,

    /// [BorderConfig] object that will be used to draw the border if enabled
    pub border_config: state::BorderConfig,

    /// Object containing colors to be used by this widget in different situations
    pub colors: state::ColorConfig,

    /// Bool representing if state has changed. Triggers widget redraw.
    pub changed: bool,

    /// [CallbackConfig] containing callbacks to be called in different situations
    pub callbacks: state::CallbackConfig,

    /// A Key to callback function lookup used to store keybinds for this widget. See
    /// [KeyboardCallbackFunction] type for callback function signature.
    pub keymap: common::KeyMap,

    /// If true this forces a global screen redraw on the next frame. Screen redraws are diffed
    /// so this can be called when needed without degrading performance. If only screen positions
    /// that fall within this widget must be redrawn, call [EzObject.redraw] instead.
    pub force_redraw: bool,
}
impl Default for TextInputState {

    fn default() -> Self {
       TextInputState {
           position: state::Coordinates::default(),
           absolute_position: state::Coordinates::default(),
           size_hint: state::SizeHint::default(),
           auto_scale: state::AutoScale::default(),
           pos_hint: state::PosHint::default(),
           padding: state::Padding::default(),
           halign: state::HorizontalAlignment::Left,
           valign: state::VerticalAlignment::Top,
           size: state::Size::default(),
           cursor_pos: state::Coordinates::default(),
           active_blink_task: false,
           blink_switch: false,
           view_start: 0,
           selected: false,
           text: String::new(),
           max_length: 10000,
           border: false,
           border_config: state::BorderConfig::default(),
           colors: state::ColorConfig::default(),
           changed: false,
           callbacks: state::CallbackConfig::default(),
           keymap: common::KeyMap::new(),
           force_redraw: false
       }
    }
}


impl state::GenericState for TextInputState {

    fn set_changed(&mut self, changed: bool) { self.changed = changed }

    fn get_changed(&self) -> bool { self.changed }

    fn set_size_hint(&mut self, size_hint: state::SizeHint) {
        if self.size_hint != size_hint { self.changed = true }
        self.size_hint = size_hint;
    }

    fn get_size_hint(&self) -> &state::SizeHint { &self.size_hint }

    fn set_pos_hint(&mut self, pos_hint: state::PosHint) {
        if self.pos_hint != pos_hint { self.changed = true }
        self.pos_hint = pos_hint;
    }

    fn get_pos_hint(&self) -> &state::PosHint { &self.pos_hint }

    fn set_auto_scale(&mut self, auto_scale: state::AutoScale) {
        if self.auto_scale != auto_scale { self.changed = true }
        self.auto_scale = auto_scale;
    }

    fn get_auto_scale(&self) -> &state::AutoScale { &self.auto_scale }

    fn set_size(&mut self, size: state::Size) {
        self.size = size;
    }

    fn get_size(&self) -> &state::Size { &self.size  }

    fn set_position(&mut self, position: state::Coordinates) {
        self.position = position;
    }

    fn get_position(&self) -> state::Coordinates { self.position }

    fn set_absolute_position(&mut self, pos: state::Coordinates) { self.absolute_position = pos }

    fn get_absolute_position(&self) -> state::Coordinates { self.absolute_position }

    fn set_callbacks(&mut self, config: state::CallbackConfig) {
        self.callbacks = config;
    }

    fn get_callbacks(&self) -> &state::CallbackConfig { &self.callbacks }

    fn get_callbacks_mut(&mut self) -> &mut state::CallbackConfig {
        &mut self.callbacks
    }

    fn get_key_map(&self) -> &common::KeyMap { &self.keymap }

    fn bind_key(&mut self, key: KeyCode, func: common::KeyboardCallbackFunction) {
        self.keymap.insert(key, func);
    }

    fn handle_event(&self, event: Event, context: common::EzContext) -> bool {
        if let Event::Key(key) = event {
            if self.get_key_map().contains_key(&key.code) {
                let func = self.get_key_map().get_mut(&key.code).unwrap();
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
    fn set_horizontal_alignment(&mut self, alignment: state::HorizontalAlignment) {
        if self.halign != alignment { self.changed = true }
        self.halign = alignment;
    }

    fn get_horizontal_alignment(&self) -> state::HorizontalAlignment { self.halign }

    fn set_vertical_alignment(&mut self, alignment: state::VerticalAlignment) {
        if self.valign != alignment { self.changed = true }
        self.valign = alignment;
    }

    fn get_vertical_alignment(&self) -> state::VerticalAlignment { self.valign }

    fn set_padding(&mut self, padding: state::Padding) {
        if self.padding != padding { self.changed = true }
        self.padding = padding;
    }

    fn get_padding(&self) -> &state::Padding { &self.padding }

    fn has_border(&self) -> bool { self.border }

    fn set_border(&mut self, enabled: bool) {
        if self.border != enabled { self.changed = true }
        self.border = enabled;
    }

    fn set_border_config(&mut self, config: state::BorderConfig) {
        if self.border_config != config { self.changed = true }
        self.border_config = config;
    }

    fn get_border_config(&self) -> &state::BorderConfig { &self.border_config  }

    fn set_colors(&mut self, config: state::ColorConfig) {
        if self.colors != config { self.changed = true }
        self.colors = config;
    }

    fn get_colors(&self) -> &state::ColorConfig { &self.colors }

    fn get_colors_mut(&mut self) -> &mut state::ColorConfig {
        self.changed = true;
        &mut self.colors
    }

    fn set_force_redraw(&mut self, redraw: bool) {
        self.force_redraw = redraw;
        self.changed = true;
    }

    fn get_force_redraw(&self) -> bool { self.force_redraw }
}
impl state::SelectableState for TextInputState {

    fn set_selected(&mut self, state: bool) {
        if self.selected != state { self.changed = true }
        self.selected = state;
    }

    fn get_selected(&self) -> bool { self.selected }
}
impl TextInputState {


    pub fn set_text(&mut self, text: String) {
        if self.text != text { self.changed = true }
        self.text = text;
    }

    pub fn get_text(&self) -> String { self.text.clone() }

    pub fn set_cursor_pos(&mut self, cursor_pos: state::Coordinates) {
        if self.cursor_pos != cursor_pos { self.changed = true }
        self.cursor_pos = cursor_pos;
    }

    pub fn set_cursor_x(&mut self, pos: usize) {
        if self.cursor_pos.x != pos { self.changed = true }
        self.cursor_pos.x = pos;
    }

    pub fn set_cursor_y(&mut self, pos: usize) {
        if self.cursor_pos.y != pos { self.changed = true }
        self.cursor_pos.y = pos;
    }

    pub fn get_cursor_pos(&self) -> state::Coordinates { self.cursor_pos }

    pub fn set_active_blink_task(&mut self, active: bool) {
        if self.active_blink_task != active { self.changed = true }
        self.active_blink_task = active;
    }

    pub fn get_active_blink_task(&self) -> bool { self.active_blink_task }

    pub fn set_blink_switch(&mut self, active: bool) {
        if self.blink_switch != active { self.changed = true }
        self.blink_switch = active;
    }

    pub fn get_blink_switch(&self) -> bool { self.blink_switch }

    pub fn set_view_start(&mut self, view_start: usize) {
        if self.view_start != view_start { self.changed = true }
        self.view_start = view_start;
    }

    pub fn get_view_start(&self) -> usize { self.view_start }

    pub fn set_max_length(&mut self, max_length: usize) {
        if self.max_length != max_length { self.changed = true }
        self.max_length = max_length;
    }

    pub fn get_max_length(&self) -> usize { self.max_length }
}


/// Start blink the position on which the cursor is currently located. This is a custom cursor not
/// the actual terminal cursor. This is because in dynamic interfaces with scheduled tasks changing
/// visual content, the crossterm cursor is constantly jumping around, which cannot seem to be
/// resolved using the Hide/Show/SavePosition/RestorePosition methods.
fn start_cursor_blink(target_pos: state::Coordinates, state: &mut TextInputState,
                      scheduler: &mut scheduler::Scheduler, name: String) {

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
