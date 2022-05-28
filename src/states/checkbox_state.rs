use crate::states::state::{self, SizeHint};


/// [State] implementation.
#[derive(Clone)]
pub struct CheckboxState {

    /// Bool representing whether this widget is currently active (i.e. checkbox is checked)
    pub active: bool,

    /// Bool representing whether this widget is currently selected.
    pub selected: bool,

    /// Position of this widget relative to its' parent [Layout]
    pub position: state::Coordinates,

    /// Absolute position of this widget on screen. Automatically propagated, do not set manually
    pub absolute_position: state::Coordinates,

    /// size of this widget
    pub size: state::Size,

    /// Cannot be set, checkbox is always 5,1
    pub size_hint: SizeHint,

    /// Automatically adjust size of widget to content
    pub auto_scale: state::AutoScale,

    /// Pos hint of this widget
    pub pos_hint: state::PosHint,

    /// Amount of space to leave between sides of the widget and other widgets
    pub padding: state::Padding,

    /// Horizontal alignment of this widget
    pub halign: state::HorizontalAlignment,

    /// Vertical alignment of this widget
    pub valign: state::VerticalAlignment,

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
impl Default for CheckboxState {
    fn default() -> Self {
       CheckboxState {
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
           border: false,
           border_config: state::BorderConfig::default(),
           colors: state::ColorConfig::default(),
           changed: false,
           force_redraw: false
       }
    }
}
impl state::GenericState for CheckboxState {

    fn set_changed(&mut self, changed: bool) { self.changed = changed }

    fn get_changed(&self) -> bool { self.changed }

    fn set_size_hint(&mut self, _size_hint: state::SizeHint) { }

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

    fn set_size(&mut self, mut size: state::Size) {
        size.width = 5;
        size.height = 1;
        self.size = size;
        self.changed = true;
    }

    fn get_size(&self) -> &state::Size { &self.size  }

    fn set_position(&mut self, position: state::Coordinates) {
        self.position = position;
        self.changed = true;
    }

    fn get_position(&self) -> state::Coordinates { self.position }

    fn set_absolute_position(&mut self, pos: state::Coordinates) {
        self.absolute_position = pos;
        self.changed = true;
    }

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
impl state::SelectableState for CheckboxState {

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
