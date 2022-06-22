use crate::common::definitions::Coordinates;
use crate::property::EzProperty;
use crate::scheduler::Scheduler;
use crate::states::definitions::{StateCoordinates, SizeHint, PosHint, StateSize, AutoScale, Padding,
                                 HorizontalAlignment, VerticalAlignment, BorderConfig, ColorConfig};
use crate::states::state::GenericState;


/// [State] implementation.
#[derive(Clone, Debug)]
pub struct TextInputState {

    /// Path to the widget to which this state belongs
    pub path: String,

    /// Text currently being displayed by the text input
    pub text: EzProperty<String>,

    /// Position of this widget relative to its' parent [Layout]
    pub position: StateCoordinates,

    /// Absolute position of this widget on screen. Automatically propagated, do not set manually
    pub absolute_position: Coordinates,

    /// size of this widget
    pub size: StateSize,

    /// Relative height/width of this widget to parent layout
    pub size_hint: SizeHint,

    /// Pos hint of this widget
    pub pos_hint: PosHint,

    /// Automatically adjust size of widget to content
    pub auto_scale: AutoScale,

    /// Amount of space to leave between sides of the widget and other widgets
    pub padding: Padding,

    /// Horizontal alignment of this widget
    pub halign: HorizontalAlignment,

    /// Vertical alignment of this widget
    pub valign: VerticalAlignment,
    
    /// Position of cursor relative to this widget
    pub cursor_pos: Coordinates,

    /// Bool representing whether we have a blinking scheduled task running
    pub active_blink_task: bool,

    /// Switch for blinking. When true displays [cursor_color] on the [cursor_pos]
    pub blink_switch: bool,

    /// If text is larger than the widget, only a part of the text can be displayed. This is the
    /// index of where to start viewing the text.
    pub view_start: usize,

    /// How many characters [text] may hold
    pub max_length: usize,

    /// [BorderConfig] object that will be used to draw the border if enabled
    pub border_config: BorderConfig,

    /// Object containing colors to be used by this widget in different situations
    pub colors: ColorConfig,

    /// Bool representing whether widget is disabled, i.e. cannot be interacted with
    pub disabled: bool,

    /// Global order number in which this widget will be selection when user presses down/up keys
    pub selection_order: usize,

    /// Bool representing whether this widget is currently selected.
    pub selected: bool,

    /// Bool representing if state has changed. Triggers widget redraw.
    pub changed: bool,

    /// If true this forces a global screen redraw on the next frame. Screen redraws are diffed
    /// so this can be called when needed without degrading performance. If only screen positions
    /// that fall within this widget must be redrawn, call [EzObject.redraw] instead.
    pub force_redraw: bool,
}
impl TextInputState {

    pub fn new(path: String, scheduler: &mut Scheduler) -> Self {

        TextInputState {
            path: path.clone(),
            position: StateCoordinates::new(0, 0, path.clone(), scheduler),
            absolute_position: Coordinates::default(),
            size: StateSize::new(0, 0, path.clone(), scheduler),
            size_hint: SizeHint::default(),
            auto_scale: AutoScale::default(),
            pos_hint: PosHint::default(),
            padding: Padding::new(0, 0, 0, 0, path.clone(), scheduler),
            halign: HorizontalAlignment::Left,
            valign: VerticalAlignment::Top,
            cursor_pos: Coordinates::default(),
            active_blink_task: false,
            blink_switch: false,
            view_start: 0,
            disabled: false,
            selected: false,
            selection_order: 0,
            text: scheduler.new_string_property(format!("{}/text", path.clone()),
                                                String::new()),
            max_length: 10000,
            border_config: BorderConfig::default(),
            colors: ColorConfig::default(),
            changed: false,
            force_redraw: false
        }
    }
}


impl GenericState for TextInputState {

    fn get_path(&self) -> &String {
        &self.path
    }

    fn set_size_hint(&mut self, size_hint: SizeHint) {
        if self.size_hint != size_hint { self.changed = true }
        self.size_hint = size_hint;
    }

    fn get_size_hint(&self) -> &SizeHint { &self.size_hint }

    fn set_pos_hint(&mut self, pos_hint: PosHint) {
        if self.pos_hint != pos_hint { self.changed = true }
        self.pos_hint = pos_hint;
    }

    fn get_pos_hint(&self) -> &PosHint { &self.pos_hint }

    fn set_auto_scale(&mut self, auto_scale: AutoScale) {
        if self.auto_scale != auto_scale { self.changed = true }
        self.auto_scale = auto_scale;
    }

    fn get_auto_scale(&self) -> &AutoScale { &self.auto_scale }

    fn get_size(&self) -> &StateSize { &self.size  }

    fn get_size_mut(&mut self) -> &mut StateSize { &mut self.size }

    fn get_position(&self) -> &StateCoordinates { &self.position }

    fn get_position_mut(&mut self) -> &mut StateCoordinates {
        self.changed = true;
        &mut self.position
    }

    fn set_absolute_position(&mut self, pos: Coordinates) { self.absolute_position = pos }

    fn get_absolute_position(&self) -> Coordinates { self.absolute_position }

    fn set_horizontal_alignment(&mut self, alignment: HorizontalAlignment) {
        if self.halign != alignment { self.changed = true }
        self.halign = alignment;
    }

    fn get_horizontal_alignment(&self) -> HorizontalAlignment { self.halign }

    fn set_vertical_alignment(&mut self, alignment: VerticalAlignment) {
        if self.valign != alignment { self.changed = true }
        self.valign = alignment;
    }

    fn get_vertical_alignment(&self) -> VerticalAlignment { self.valign }

    fn get_padding(&self) -> &Padding { &self.padding }

    fn get_padding_mut(&mut self) -> &mut Padding { &mut self.padding }

    fn set_border_config(&mut self, config: BorderConfig) {
        if self.border_config != config { self.changed = true }
        self.border_config = config;
    }

    fn get_border_config(&self) -> &BorderConfig { &self.border_config  }

    fn get_border_config_mut(&mut self) -> &mut BorderConfig {
        self.changed = true;
        &mut self.border_config
    }


    fn set_color_config(&mut self, config: ColorConfig) {
        if self.colors != config { self.changed = true }
        self.colors = config;
    }

    fn get_color_config(&self) -> &ColorConfig { &self.colors }

    fn get_colors_config_mut(&mut self) -> &mut ColorConfig {
        self.changed = true;
        &mut self.colors
    }

    fn is_selectable(&self) -> bool { true }

    fn set_disabled(&mut self, disabled: bool) {
        if self.disabled != disabled { self.changed = true }
        self.disabled = disabled
    }

    fn get_disabled(&self) -> bool { self.disabled }

    fn get_selection_order(&self) -> usize { self.selection_order }

    fn set_selection_order(&mut self, order: usize) {
        if self.selection_order != order { self.changed = true };
        self.selection_order = order;
    }

    fn set_selected(&mut self, state: bool) {
        if self.selected != state { self.changed = true }
        self.selected = state;
    }

    fn get_selected(&self) -> bool { self.selected }
}
impl TextInputState {

    pub fn get_text(&self) -> &EzProperty<String> { &self.text }

    pub fn get_text_mut(&mut self) -> &mut EzProperty<String> { &mut self.text }

    pub fn set_cursor_pos(&mut self, cursor_pos: Coordinates) {
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

    pub fn get_cursor_pos(&self) -> Coordinates { self.cursor_pos }

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

