use crate::states::state::{self, SizeHint};


/// [State] implementation.
#[derive(Clone)]
pub struct RadioButtonState {

    /// Group this radio button belongs to. Set the same group value for a number of radio buttons
    /// to make them mutually exclusive.
    pub group: String,

    /// Position of this widget relative to its' parent [Layout]
    pub position: state::Coordinates,

    /// Absolute position of this widget on screen. Automatically propagated, do not set manually
    pub absolute_position: state::Coordinates,

    /// size of this widget
    pub size: state::Size,

    /// Automatically adjust size of widget to content
    pub auto_scale: state::AutoScale,

    /// Cannot be set for Radio Button, size is always 5,1
    pub size_hint: SizeHint,

    /// Pos hint of this widget
    pub pos_hint: state::PosHint,

    /// Amount of space to leave between sides of the widget and other widgets
    pub padding: state::Padding,

    /// Horizontal alignment of this widget
    pub halign: state::HorizontalAlignment,

    /// Vertical alignment of this widget
    pub valign: state::VerticalAlignment,

    /// Bool representing whether this widget is currently active (i.e. checkbox is checked)
    pub active: bool,

    /// [BorderConfig] object that will be used to draw the border if enabled
    pub border_config: state::BorderConfig,

    /// Object containing colors to be used by this widget in different situations
    pub colors: state::ColorConfig,

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
impl Default for RadioButtonState {
    fn default() -> Self {
       RadioButtonState {
           group: String::new(),
           position: state::Coordinates::default(),
           absolute_position: state::Coordinates::default(),
           size: state::Size::new(5, 1),
           auto_scale: state::AutoScale::default(),
           size_hint: SizeHint::new(None, None),
           pos_hint: state::PosHint::default(),
           padding: state::Padding::default(),
           halign: state::HorizontalAlignment::Left,
           valign: state::VerticalAlignment::Top,
           active: false,
           selected: false,
           selection_order: 0,
           border_config: state::BorderConfig::default(),
           colors: state::ColorConfig::default(),
           changed: false,
           force_redraw: false
       }
    }
}
impl state::GenericState for RadioButtonState {

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

    fn set_size(&mut self, mut size: state::Size) {
        size.width = 5;
        size.height = 1;
        self.size = size;
    }

    fn get_size(&self) -> &state::Size { &self.size  }

    fn get_size_mut(&mut self) -> &mut state::Size { &mut self.size }

    fn set_position(&mut self, position: state::Coordinates) { self.position = position; }

    fn get_position(&self) -> state::Coordinates { self.position }

    fn set_absolute_position(&mut self, pos: state::Coordinates) {
        if self.absolute_position != pos { self.changed = true }
        self.absolute_position = pos;
    }

    fn get_absolute_position(&self) -> state::Coordinates { self.absolute_position }

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

    fn set_border_config(&mut self, config: state::BorderConfig) {
        if self.border_config != config { self.changed = true }
        self.border_config = config;
    }

    fn get_border_config(&self) -> &state::BorderConfig { &self.border_config  }

    fn get_border_config_mut(&mut self) -> &mut state::BorderConfig {
        self.changed = true;
        &mut self.border_config
    }


    fn set_color_config(&mut self, config: state::ColorConfig) {
        if self.colors != config { self.changed = true }
        self.colors = config;
    }

    fn get_color_config(&self) -> &state::ColorConfig { &self.colors }

    fn get_colors_config_mut(&mut self) -> &mut state::ColorConfig {
        self.changed = true;
        &mut self.colors
    }

    fn is_selectable(&self) -> bool { true}

    fn get_selection_order(&self) -> usize { self.selection_order }

    fn set_force_redraw(&mut self, redraw: bool) {
        self.force_redraw = redraw;
        self.changed = true;
    }

    fn get_force_redraw(&self) -> bool { self.force_redraw }

    fn set_selected(&mut self, state: bool) {
        if self.selected != state { self.changed = true }
        self.selected = state;
    }

    fn get_selected(&self) -> bool { self.selected }
}
impl RadioButtonState {

    /// Get the group this radio button belongs to. Radio buttons that share a group are
    /// mutually exclusive.
    pub fn get_group(&self) -> String { self.group.clone() }

    pub fn set_active(&mut self, active: bool) {
        if self.active != active { self.changed = true }
        self.active = active;
    }

    pub fn get_active(&self) -> bool { self.active }

}
