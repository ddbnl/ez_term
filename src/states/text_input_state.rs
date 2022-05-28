use crate::states::state::{self};


/// [State] implementation.
#[derive(Clone)]
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
           force_redraw: false
       }
    }
}


impl state::GenericState for TextInputState {

    fn set_changed(&mut self, changed: bool) { self.changed = changed }

    fn get_changed(&self) -> bool { self.changed }

    fn set_size_hint(&mut self, size_hint: state::SizeHint) {
        self.size_hint = size_hint;
        self.changed = true;
    }

    fn get_size_hint(&self) -> &state::SizeHint { &self.size_hint }

    fn set_pos_hint(&mut self, pos_hint: state::PosHint) {
        self.pos_hint = pos_hint;
        self.changed = true;
    }

    fn get_pos_hint(&self) -> &state::PosHint { &self.pos_hint }

    fn set_auto_scale(&mut self, auto_scale: state::AutoScale) {
        self.auto_scale = auto_scale;
        self.changed = true;
    }

    fn get_auto_scale(&self) -> &state::AutoScale { &self.auto_scale }

    fn set_size(&mut self, size: state::Size) {
        self.size = size;
        self.changed = true;
    }

    fn get_size(&self) -> &state::Size { &self.size  }

    fn set_position(&mut self, position: state::Coordinates) {
        self.position = position;
        self.changed = true;
    }

    fn get_position(&self) -> state::Coordinates { self.position }

    fn set_absolute_position(&mut self, pos: state::Coordinates) { self.absolute_position = pos }

    fn get_absolute_position(&self) -> state::Coordinates { self.absolute_position }

    fn set_horizontal_alignment(&mut self, alignment: state::HorizontalAlignment) {
        self.halign = alignment;
        self.changed = true;
    }

    fn get_horizontal_alignment(&self) -> state::HorizontalAlignment { self.halign }

    fn set_vertical_alignment(&mut self, alignment: state::VerticalAlignment) {
        self.valign = alignment;
        self.changed = true;
    }

    fn get_vertical_alignment(&self) -> state::VerticalAlignment { self.valign }

    fn set_padding(&mut self, padding: state::Padding) {
        self.padding = padding;
        self.changed = true;
    }

    fn get_padding(&self) -> &state::Padding { &self.padding }

    fn has_border(&self) -> bool { self.border }

    fn set_border(&mut self, enabled: bool) {
        self.border = enabled;
        self.changed = true;
    }

    fn set_border_config(&mut self, config: state::BorderConfig) {
        self.border_config = config;
        self.changed = true;
    }

    fn get_border_config(&self) -> &state::BorderConfig { &self.border_config  }

    fn set_colors(&mut self, config: state::ColorConfig) {
        self.colors = config;
        self.changed = true;
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
        self.selected = state;
        self.changed = true;
    }

    fn get_selected(&self) -> bool { self.selected }
}
impl TextInputState {


    pub fn set_text(&mut self, text: String) {
        self.text = text;
        self.changed = true;
    }

    pub fn get_text(&self) -> String { self.text.clone() }

    pub fn set_cursor_pos(&mut self, cursor_pos: state::Coordinates) {
        self.cursor_pos = cursor_pos;
        self.changed = true;
    }

    pub fn set_cursor_x(&mut self, pos: usize) {
        self.cursor_pos.x = pos;
        self.changed = true;
    }

    pub fn set_cursor_y(&mut self, pos: usize) {
        self.cursor_pos.y = pos;
        self.changed = true;
    }

    pub fn get_cursor_pos(&self) -> state::Coordinates { self.cursor_pos }

    pub fn set_active_blink_task(&mut self, active: bool) {
        self.active_blink_task = active;
        self.changed = true;
    }

    pub fn get_active_blink_task(&self) -> bool { self.active_blink_task }

    pub fn set_blink_switch(&mut self, active: bool) {
        self.blink_switch = active;
        self.changed = true;
    }

    pub fn get_blink_switch(&self) -> bool { self.blink_switch }

    pub fn set_view_start(&mut self, view_start: usize) {
        self.view_start = view_start;
        self.changed = true;
    }

    pub fn get_view_start(&self) -> usize { self.view_start }

    pub fn set_max_length(&mut self, max_length: usize) {
        self.max_length = max_length;
        self.changed = true;
    }

    pub fn get_max_length(&self) -> usize { self.max_length }
}
