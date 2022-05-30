use crossterm::event::{Event, KeyCode, MouseButton, MouseEventKind};
use crate::states::state::{self, GenericState};
use crate::common;
use crate::common::KeyMap;


/// [State] implementation.
pub struct DropdownState {

    /// Position of this widget relative to its' parent [Layout]
    pub position: state::Coordinates,

    /// Absolute position of this widget on screen. Automatically propagated, do not set manually
    pub absolute_position: state::Coordinates,

    /// Relative height/width of this widget to parent layout
    pub size_hint: state::SizeHint,

    /// Pos hint of this widget
    pub pos_hint: state::PosHint,

    /// size of this widget
    pub size: state::Size,

    /// Automatically adjust size of widget to content
    pub auto_scale: state::AutoScale,

    /// Amount of space to leave between sides of the widget and other widgets
    pub padding: state::Padding,

    /// Horizontal alignment of this widget
    pub halign: state::HorizontalAlignment,

    /// Vertical alignment of this widget
    pub valign: state::VerticalAlignment,

    /// Bool representing whether this widget is currently focussed. If so, it gets the first
    /// chance to consume all events
    pub focussed: bool,

    /// Bool representing whether this widget is currently selected.
    pub selected: bool,

    /// List of options this dropdown will display
    pub options: Vec<String>,

    /// Bool representing whether an empty value should be shown to choose from
    pub allow_none: bool,

    /// The currently active choice of the dropdown.
    pub choice: String,

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
impl Default for DropdownState {
    fn default() -> Self {
       DropdownState {
           position: state::Coordinates::default(),
           absolute_position: state::Coordinates::default(),
           size_hint: state::SizeHint::default(),
           auto_scale: state::AutoScale::default(),
           pos_hint: state::PosHint::default(),
           size: state::Size::new(0, 3),
           padding: state::Padding::default(),
           halign: state::HorizontalAlignment::Left,
           valign: state::VerticalAlignment::Top,
           focussed: false,
           selected: false,
           options: Vec::new(),
           allow_none: true,
           choice: String::new(),
           border: true,
           border_config: state::BorderConfig::default(),
           colors: state::ColorConfig::default(),
           changed: false,
           callbacks: state::CallbackConfig::default(),
           keymap: common::KeyMap::new(),
           force_redraw: false
       }
    }
}
impl GenericState for DropdownState {

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

    fn get_size(&self) -> &state::Size { &self.size }

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
impl state::SelectableState for DropdownState {

    fn set_selected(&mut self, state: bool) {
        if self.selected != state { self.changed = true }
        self.selected = state;
    }
    fn get_selected(&self) -> bool { self.selected }
}
impl DropdownState {

    pub fn set_choice(&mut self, choice: String) {
        if self.choice != choice { self.changed = true }
        self.choice = choice;
    }

    pub fn get_choice(&self) -> String { self.choice.clone() }

    pub fn set_options(&mut self, options: Vec<String>) { self.options = options }

    pub fn get_options(&self) -> Vec<String> { self.options.clone() }

    pub fn set_focussed(&mut self, focussed: bool) {
        if self.focussed != focussed { self.changed = true }
        self.focussed = focussed;
    }

    pub fn get_focussed(&self) -> bool { self.focussed }

    pub fn set_allow_none(&mut self, allow_none: bool) {
        if self.allow_none != allow_none { self.changed = true }
        self.allow_none = allow_none;
    }

    pub fn get_allow_none(&self) -> bool { self.allow_none }

    /// Return the total amount of options in this dropdown including the empty option if it is
    /// allowed.
    pub fn total_options(&self) -> usize { self.options.len() + if self.allow_none {1} else {0} }

}


/// [State] implementation.
pub struct DroppedDownMenuState {

    /// Widget path of the [Dropdown] that spawned this menu.
    pub parent_path: String,

    /// Position of this widget relative to its' parent [Layout]
    pub position: state::Coordinates,

    /// Absolute position of this widget on screen. Automatically propagated, do not set manually
    pub absolute_position: state::Coordinates,

    /// Relative height/width of this widget to parent layout
    pub size_hint: state::SizeHint,

    /// Pos hint of this widget
    pub pos_hint: state::PosHint,

    /// size of this widget
    pub size: state::Size,

    /// Automatically adjust size of widget to content
    pub auto_scale: state::AutoScale,

    /// List of options this dropdown will display
    pub options: Vec<String>,

    pub allow_none: bool,

    /// The currently active choice of the dropdown.
    pub choice: String,

    /// If dropped down, this represents which row of the dropdown is being hovered with the mouse,
    /// or has been selected with the keyboard using up/down.
    pub dropped_down_selected_row: usize,

    /// [BorderConfig] object that will be used to draw the border if enabled
    pub border_config: state::BorderConfig,

    /// Object containing colors to be used by this widget in different situations
    pub colors: state::ColorConfig,

    /// Bool representing if state has changed. Triggers widget redraw.
    pub changed: bool,

    /// A Key to callback function lookup used to store keybinds for this widget. See
    /// [KeyboardCallbackFunction] type for callback function signature.
    pub keymap: common::KeyMap,

    /// If true this forces a global screen redraw on the next frame. Screen redraws are diffed
    /// so this can be called when needed without degrading performance. If only screen positions
    /// that fall within this widget must be redrawn, call [EzObject.redraw] instead.
    pub force_redraw: bool,
}
impl Default for DroppedDownMenuState {
    fn default() -> Self {
        DroppedDownMenuState {
            parent_path: String::new(),
            position: state::Coordinates::default(),
            absolute_position: state::Coordinates::default(),
            size_hint: state::SizeHint::default(),
            auto_scale: state::AutoScale::default(),
            pos_hint: state::PosHint::default(),
            size: state::Size::new(0, 3),
            options: Vec::new(),
            allow_none: true,
            dropped_down_selected_row:0,
            choice: String::new(),
            border_config: state::BorderConfig::default(),
            colors: state::ColorConfig::default(),
            changed: false,
            keymap: KeyMap::new(),
            force_redraw: false
        }
    }
}
impl GenericState for DroppedDownMenuState {

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

    fn set_size(&mut self, size: state::Size) { self.size = size; }

    fn get_size(&self) -> &state::Size { &self.size }

    fn set_position(&mut self, position: state::Coordinates) { self.position = position; }

    fn get_position(&self) -> state::Coordinates { self.position }

    fn set_absolute_position(&mut self, pos: state::Coordinates) { self.absolute_position = pos }

    fn get_absolute_position(&self) -> state::Coordinates { self.absolute_position }

    fn set_callbacks(&mut self, config: state::CallbackConfig) { }

    fn get_callbacks(&self) -> &state::CallbackConfig {
        panic!("Callbacks not implemented for modal")
    }

    fn get_callbacks_mut(&mut self) -> &mut state::CallbackConfig {
        panic!("Callbacks not implemented for modal")
    }

    fn get_key_map(&self) -> &common::KeyMap { &self.keymap }

    fn bind_key(&mut self, key: KeyCode, func: common::KeyboardCallbackFunction) {
        self.keymap.insert(key, func);
    }

    fn handle_event(&self, event: Event, context: common::EzContext) -> bool {
        match event {
            Event::Key(key) => {
                match key.code {
                    KeyCode::Enter => {
                        self.handle_enter(context);
                        return true
                    }
                    KeyCode::Down => {
                        self.handle_down();
                        return true
                    },
                    KeyCode::Up => {
                        self.handle_up();
                        return true
                    },
                    _ => ()
                }
            }
            Event::Mouse(event) => {
                let mouse_position = state::Coordinates::new(event.column as usize,
                                                             event.row as usize);
                if let MouseEventKind::Down(button) = event.kind {
                    if button == MouseButton::Left {
                        self.handle_left_click(context, mouse_position);
                        return true
                    }
                } else if event.kind == MouseEventKind::Moved &&
                    self.collides(mouse_position) {
                    return self.handle_hover(mouse_position)
                }
            },
            _ => ()
        }
        false
    }
    fn set_horizontal_alignment(&mut self, alignment: state::HorizontalAlignment) {
    }

    fn get_horizontal_alignment(&self) -> state::HorizontalAlignment {
        panic!("Alignment not implemented for modal")
    }

    fn set_vertical_alignment(&mut self, alignment: state::VerticalAlignment) {
    }

    fn get_vertical_alignment(&self) -> state::VerticalAlignment {
        panic!("Alignment not implemented for modal")
    }

    fn set_padding(&mut self, padding: state::Padding) { }

    fn get_padding(&self) -> &state::Padding { panic!("Padding not implemented for modal")}

    fn has_border(&self) -> bool { true }

    fn set_border(&mut self, enabled: bool) { }

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

impl DroppedDownMenuState {

    pub fn set_choice(&mut self, choice: String) {
        if self.choice != choice { self.changed = true }
        self.choice = choice;
    }

    pub fn get_choice(&self) -> String { self.choice.clone() }

    pub fn set_options(&mut self, options: Vec<String>) { self.options = options }

    pub fn get_options(&self) -> Vec<String> { self.options.clone() }

    pub fn set_allow_none(&mut self, allow_none: bool) {
        if self.allow_none != allow_none { self.changed = true }
        self.allow_none = allow_none;
    }

    pub fn get_allow_none(&self) -> bool { self.allow_none }

    pub fn set_dropped_down_selected_row(&mut self, dropped_down_selected_row: usize) {
        if self.dropped_down_selected_row != dropped_down_selected_row { self.changed = true }
        self.dropped_down_selected_row = dropped_down_selected_row;
    }

    pub fn get_dropped_down_selected_row(&self) -> usize { self.dropped_down_selected_row }

    /// Return the total amount of options in this dropdown including the empty option if it is
    /// allowed.
    pub fn total_options(&self) -> usize { self.options.len() + if self.allow_none {1} else {0} }


    /// Called when this widget is already dropped down and enter is pressed
    fn handle_enter(&self, mut context: common::EzContext) {
        let choice = self.get_dropped_down_options()[self.dropped_down_selected_row].clone();
        context.state_tree.get_mut(&self.parent_path).unwrap()
            .as_dropdown_mut().set_choice(choice);
        context.state_tree
            .get_mut(format!("/{}", self.parent_path.split('/').nth(1).unwrap()).as_str())
            .unwrap().as_layout_mut().dismiss_modal();
        context.widget_path = self.parent_path.clone();  // Change widget to parent for on_value_change callback
        context.widget_tree.get(&self.parent_path).unwrap().as_ez_widget()
            .on_value_change(context);
    }

    /// Called when this widget is already dropped down and up is pressed
    fn handle_up(&self, state: &mut DroppedDownMenuState) {

        if state.dropped_down_selected_row == 0 {
            state.set_dropped_down_selected_row(self.total_options() - 1);
        } else {
            state.set_dropped_down_selected_row(self.dropped_down_selected_row - 1);
        }
    }

    /// Called when this widget is already dropped down and down is pressed
    fn handle_down(&self, state: &mut DroppedDownMenuState) {
        if state.dropped_down_selected_row == self.total_options() - 1 {
            state.set_dropped_down_selected_row(0);
        } else {
            state.set_dropped_down_selected_row(self.dropped_down_selected_row + 1);
        }
    }

    /// Called when this widget is already dropped down and widget is left clicked
    fn handle_left_click(&self, mut context: common::EzContext, pos: state::Coordinates) {

        if self.collides(pos) {
            let clicked_row = pos.y - self.absolute_position.y;
            // Check if not click on border
            if clicked_row != 0 && clicked_row <= self.get_effective_size().height {
                let choice = self.get_dropped_down_options()[clicked_row - 1]
                    .clone();
                context.state_tree.get_mut(&self.parent_path).unwrap()
                    .as_dropped_down_menu_mut().set_choice(choice);
                context.state_tree.get_mut("/root").unwrap().as_layout_mut().dismiss_modal();
                context.widget_path = self.parent_path.clone();  // Change widget to parent for on_value_change callback
                context.widget_tree.get(&self.parent_path).unwrap().as_ez_widget()
                    .on_value_change(context);
            }
        } else {
            context.state_tree.get_mut("/root").unwrap().as_layout_mut().dismiss_modal();
        }
    }

    /// Called when this widget is already dropped down and widget is hovered
    fn handle_hover(&mut self, pos: state::Coordinates) -> bool {
        let hovered_row = pos.y - self.absolute_position.y;
        // Check if not hover on border
        if hovered_row - 1 != self.dropped_down_selected_row &&
            hovered_row != 0 && hovered_row <= self.get_dropped_down_options().len() {
            self.set_dropped_down_selected_row(hovered_row - 1);
            return true
        }
        false
    }
    /// Get an ordered list of options, including the empty option if it was allowed. Order is:
    /// - Active choice
    /// - Empty (if allowed)
    /// - Rest of the options in user defined order
    fn get_dropped_down_options(&self) -> Vec<String> {
        let mut options = vec!(self.choice.clone());
        if self.allow_none && !self.choice.is_empty() {
            options.push("".to_string())
        }
        for option in self.options.iter().filter(|x| x.to_string() != self.choice) {
            options.push(option.to_string());
        }
        options
    }

}
