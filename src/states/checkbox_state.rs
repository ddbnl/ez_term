use crate::states::state::{GenericState, SelectableState, VerticalAlignment, HorizontalAlignment,
                           HorizontalPositionHint, VerticalPositionHint, BorderConfig,
                           ColorConfig, Coordinates};


/// [State] implementation.
#[derive(Clone)]
pub struct CheckboxState {

    /// Bool representing whether this widget is currently active (i.e. checkbox is checked)
    pub active: bool,

    /// Bool representing whether this widget is currently selected.
    pub selected: bool,

    /// Position of this widget relative to its' parent [Layout]
    pub position: Coordinates,

    /// Absolute position of this widget on screen. Automatically propagated, do not set manually
    pub absolute_position: Coordinates,

    /// Pos hint for x position of this widget
    pub pos_hint_x: Option<(HorizontalPositionHint, f64)>,

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

    /// Horizontal alignment of this widget
    pub halign: HorizontalAlignment,

    /// Vertical alignment of this widget
    pub valign: VerticalAlignment,

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
impl Default for CheckboxState {
    fn default() -> Self {
       CheckboxState {
           position: Coordinates::default(),
           absolute_position: Coordinates::default(),
           pos_hint_x: None,
           pos_hint_y: None,
           padding_top: 0,
           padding_bottom: 0,
           padding_left: 0,
           padding_right: 0,
           halign: HorizontalAlignment::Left,
           valign: VerticalAlignment::Top,
           active: false,
           selected: false,
           border: false,
           border_config: BorderConfig::default(),
           colors: ColorConfig::default(),
           changed: false,
           force_redraw: false
       }
    }
}
impl GenericState for CheckboxState {

    fn set_changed(&mut self, changed: bool) { self.changed = changed }

    fn get_changed(&self) -> bool { self.changed }

    fn get_size_hint_x(&self) -> Option<f64> { None }

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
    
    fn set_width(&mut self, _width: usize) { }

    fn get_width(&self) -> usize { 5 + self.padding_left + self.padding_top }

    fn set_height(&mut self, _height: usize) { }

    fn get_height(&self) -> usize { 1 + self.padding_top + self.padding_bottom }

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
impl SelectableState for CheckboxState {

    fn set_selected(&mut self, state: bool) {
        self.selected = state;
        self.changed = true;
    }

    fn get_selected(&self) -> bool { self.selected }
}
impl CheckboxState {

    pub fn set_active(&mut self, active: bool) {
        self.active = active;
        self.changed = true;
    }

    pub fn get_active(&self) -> bool { self.active }
}
