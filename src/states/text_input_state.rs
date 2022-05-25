use crate::states::state::{GenericState, SelectableState, HorizontalAlignment, VerticalAlignment,
                           HorizontalPositionHint, VerticalPositionHint, BorderConfig,
                           ColorConfig, Coordinates};


/// [State] implementation.
#[derive(Clone)]
pub struct TextInputState {
    
    /// Text currently being displayed by the text input
    pub text: String,

    /// Position of this widget relative to its' parent [Layout]
    pub position: Coordinates,

    /// Absolute position of this widget on screen. Automatically propagated, do not set manually
    pub absolute_position: Coordinates,

    /// Width of this widget
    pub size_hint_x: Option<f64>,

    /// Pos hint for x position of this widget
    pub pos_hint_x: Option<(HorizontalPositionHint, f64)>,

    /// Automatically adjust width of widget to content
    pub auto_scale_width: bool,

    /// Automatically adjust width of widget to content
    pub auto_scale_height: bool,

    /// Pos hint for y position of this widget
    pub pos_hint_y: Option<(VerticalPositionHint, f64)>,

    /// Amount of space to leave between top edge and content
    pub padding_top: usize,

    /// Amount of space to leave between bottom edge and content
    pub padding_bottom: usize,

    /// Amount of space to leave between left edge and content
    pub padding_left: usize,

    /// Amount of space to leave between right edge and content
    pub padding_right: usize,

    /// Width of this widget
    pub width: usize,

    /// Height of this widget
    pub height: usize,

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

    /// Bool representing whether this widget is currently selected.
    pub selected: bool,

    /// How many characters [text] may hold
    pub max_length: usize,

    /// Bool representing whether this layout should have a surrounding border
    pub border: bool,

    /// [BorderConfig] object that will be used to draw the border if enabled
    pub border_config: BorderConfig,

    /// Object containing colors to be used by this widget in different situations
    pub colors: ColorConfig,

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
           position: Coordinates::default(),
           absolute_position: Coordinates::default(),
           size_hint_x: Some(1.0),
           auto_scale_width: false,
           auto_scale_height: true,
           pos_hint_x: None,
           pos_hint_y: None,
           padding_top: 0,
           padding_bottom: 0,
           padding_left: 0,
           padding_right: 0,
           halign: HorizontalAlignment::Left,
           valign: VerticalAlignment::Top,
           width: 0,
           height: 1,
           cursor_pos: Coordinates::default(),
           active_blink_task: false,
           blink_switch: false,
           view_start: 0,
           selected: false,
           text: String::new(),
           max_length: 10000,
           border: false,
           border_config: BorderConfig::default(),
           colors: ColorConfig::default(),
           changed: false,
           force_redraw: false
       }
    }
}


impl GenericState for TextInputState {

    fn set_changed(&mut self, changed: bool) { self.changed = changed }

    fn get_changed(&self) -> bool { self.changed }

    fn set_size_hint_x(&mut self, size_hint: Option<f64>) {
        self.size_hint_x = size_hint;
        self.changed = true;
    }

    fn get_size_hint_x(&self) -> Option<f64> { self.size_hint_x }

    fn get_size_hint_y(&self) -> Option<f64> { None }
    
    fn set_pos_hint_x(&mut self, pos_hint: Option<(HorizontalPositionHint, f64)>) {
        self.pos_hint_x = pos_hint;
        self.changed = true;
    }

    fn get_pos_hint_x(&self) -> &Option<(HorizontalPositionHint, f64)> { &self.pos_hint_x }

    fn set_pos_hint_y(&mut self, pos_hint: Option<(VerticalPositionHint, f64)>) {
        self.pos_hint_y = pos_hint;
        self.changed = true;
    }

    fn get_pos_hint_y(&self) -> &Option<(VerticalPositionHint, f64)> { &self.pos_hint_y }

    fn set_auto_scale_width(&mut self, auto_scale: bool) { self.auto_scale_width = auto_scale }

    fn get_auto_scale_width(&self) -> bool { self.auto_scale_width }

    fn set_auto_scale_height(&mut self, auto_scale: bool) { self.auto_scale_height = auto_scale }

    fn get_auto_scale_height(&self) -> bool { self.auto_scale_height }
    
    fn set_width(&mut self, width: usize) { self.width = width; self.changed = true; }

    fn get_width(&self) -> usize { self.width }

    fn set_height(&mut self, height: usize) { self.height = height; self.changed = true }

    fn get_height(&self) -> usize { self.height }

    fn set_position(&mut self, position: Coordinates) {
        self.position = position;
        self.changed = true;
    }

    fn get_position(&self) -> Coordinates { self.position }

    fn set_absolute_position(&mut self, pos: Coordinates) { self.absolute_position = pos }

    fn get_absolute_position(&self) -> Coordinates { self.absolute_position }

    fn set_horizontal_alignment(&mut self, alignment: HorizontalAlignment) {
        self.halign = alignment;
        self.changed = true;
    }

    fn get_horizontal_alignment(&self) -> HorizontalAlignment { self.halign }

    fn set_vertical_alignment(&mut self, alignment: VerticalAlignment) {
        self.valign = alignment;
        self.changed = true;
    }

    fn get_vertical_alignment(&self) -> VerticalAlignment { self.valign }

    fn set_padding_top(&mut self, padding: usize) {
        self.padding_top = padding;
        self.changed = true;
    }

    fn get_padding_top(&self) -> usize { self.padding_top }

    fn set_padding_bottom(&mut self, padding: usize) {
        self.padding_bottom = padding;
        self.changed = true;
    }

    fn get_padding_bottom(&self) -> usize { self.padding_bottom }

    fn set_padding_left(&mut self, padding: usize) {
        self.padding_left = padding;
        self.changed = true;
    }

    fn get_padding_left(&self) -> usize { self.padding_left }

    fn set_padding_right(&mut self, padding: usize) {
        self.padding_right = padding;
        self.changed = true;
    }

    fn get_padding_right(&self) -> usize { self.padding_right }

    fn has_border(&self) -> bool { self.border }

    fn set_border(&mut self, enabled: bool) { self.border = enabled }

    fn set_border_config(&mut self, config: BorderConfig) { self.border_config = config }

    fn get_border_config(&self) -> &BorderConfig { &self.border_config  }

    fn set_colors(&mut self, config: ColorConfig) { self.colors = config }

    fn get_colors(&self) -> &ColorConfig { &self.colors }

    fn set_force_redraw(&mut self, redraw: bool) {
        self.force_redraw = redraw;
        self.changed = true;
    }

    fn get_force_redraw(&self) -> bool { self.force_redraw }
}
impl SelectableState for TextInputState {

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

    pub fn set_cursor_pos(&mut self, cursor_pos: Coordinates) {
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

    pub fn get_cursor_pos(&self) -> Coordinates { self.cursor_pos }

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
