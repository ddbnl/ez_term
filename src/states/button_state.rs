use crate::states;
use crate::states::state::GenericState;


/// [State] implementation for [Button].
#[derive(Clone)]
pub struct ButtonState {

    /// Text currently being displayed by the label
    pub text: String,

    /// Position of this widget relative to its' parent [Layout]
    pub position: states::definitions::Coordinates,

    /// Absolute position of this layout on screen. Automatically propagated, do not set manually
    pub absolute_position: states::definitions::Coordinates,

    /// Relative height/width of this widget to parent layout
    pub size_hint: states::definitions::SizeHint,

    /// Pos hint of this widget
    pub pos_hint: states::definitions::PosHint,

    /// size of this widget
    pub size: states::definitions::Size,

    /// Automatically adjust size of widget to content
    pub auto_scale: states::definitions::AutoScale,

    /// Amount of space to leave between sides of the widget and other widgets
    pub padding: states::definitions::Padding,

    /// Horizontal alignment of this widget
    pub halign: states::definitions::HorizontalAlignment,

    /// Vertical alignment of this widget
    pub valign: states::definitions::VerticalAlignment,

    /// [BorderConfig] object that will be used to draw the border if enabled
    pub border_config: states::definitions::BorderConfig,

    /// Bool representing if state has changed. Triggers widget redraw.
    pub changed: bool,

    /// Object containing colors to be used by this widget in different situations
    pub colors: states::definitions::ColorConfig,

    /// Bool representing whether widget is disabled, i.e. cannot be interacted with
    pub disabled: bool,

    /// Global order number in which this widget will be selection when user presses down/up keys
    pub selection_order: usize,

    /// Bool representing whether this widget is currently selected.
    pub selected: bool,

    /// Bool representing whether this widget is currently displaying it's flash color.
    pub flashing: bool,

    /// If true this forces a global screen redraw on the next frame. Screen redraws are diffed
    /// so this can be called when needed without degrading performance. If only screen positions
    /// that fall within this widget must be redrawn, call [EzObject.redraw] instead.
    pub force_redraw: bool,
}
impl Default for ButtonState {
    fn default() -> Self {

       ButtonState {
           position: states::definitions::Coordinates::default(),
           absolute_position: states::definitions::Coordinates::default(),
           size: states::definitions::Size::default(),
           size_hint: states::definitions::SizeHint::default(),
           pos_hint: states::definitions::PosHint::default(),
           auto_scale: states::definitions::AutoScale::default(),
           padding: states::definitions::Padding::default(),
           halign: states::definitions::HorizontalAlignment::Left,
           valign: states::definitions::VerticalAlignment::Top,
           text: String::new(),
           disabled: false,
           selected: false,
           selection_order: 0,
           flashing: false,
           border_config: states::definitions::BorderConfig::default(),
           colors: states::definitions::ColorConfig::default(),
           changed: false,
           force_redraw: false,
       }
    }
}
impl GenericState for ButtonState {

    fn set_changed(&mut self, changed: bool) { self.changed = changed }

    fn get_changed(&self) -> bool { self.changed }

    fn set_size_hint(&mut self, size_hint: states::definitions::SizeHint) {
        if self.size_hint != size_hint { self.changed = true }
        self.size_hint = size_hint;
    }

    fn get_size_hint(&self) -> &states::definitions::SizeHint { &self.size_hint }

    fn set_pos_hint(&mut self, pos_hint: states::definitions::PosHint) {
        if self.pos_hint != pos_hint { self.changed = true }
        self.pos_hint = pos_hint;
    }

    fn get_pos_hint(&self) -> &states::definitions::PosHint { &self.pos_hint }

    fn set_auto_scale(&mut self, auto_scale: states::definitions::AutoScale) {
        if self.auto_scale != auto_scale { self.changed = true }
        self.auto_scale = auto_scale;
    }

    fn get_auto_scale(&self) -> &states::definitions::AutoScale { &self.auto_scale }

    fn set_size(&mut self, size: states::definitions::Size) { self.size = size; }

    fn get_size(&self) -> &states::definitions::Size { &self.size }

    fn get_size_mut(&mut self) -> &mut states::definitions::Size { &mut self.size }

    fn set_position(&mut self, position: states::definitions::Coordinates) { self.position = position; }

    fn get_position(&self) -> states::definitions::Coordinates { self.position }

    fn set_absolute_position(&mut self, pos: states::definitions::Coordinates) { self.absolute_position = pos }

    fn get_absolute_position(&self) -> states::definitions::Coordinates { self.absolute_position }

    fn set_horizontal_alignment(&mut self, alignment: states::definitions::HorizontalAlignment) {
        if self.halign != alignment { self.changed = true }
        self.halign = alignment;
    }

    fn get_horizontal_alignment(&self) -> states::definitions::HorizontalAlignment { self.halign }

    fn set_vertical_alignment(&mut self, alignment: states::definitions::VerticalAlignment) {
        if self.valign != alignment { self.changed = true }
        self.valign = alignment;
    }

    fn get_vertical_alignment(&self) -> states::definitions::VerticalAlignment { self.valign }

    fn set_padding(&mut self, padding: states::definitions::Padding) {
        if self.padding != padding { self.changed = true }
        self.padding = padding;
    }

    fn get_padding(&self) -> &states::definitions::Padding { &self.padding }

    fn set_border_config(&mut self, config: states::definitions::BorderConfig) {
        if self.border_config != config { self.changed = true }
        self.border_config = config;
    }

    fn get_border_config(&self) -> &states::definitions::BorderConfig { &self.border_config  }

    fn get_border_config_mut(&mut self) -> &mut states::definitions::BorderConfig {
        self.changed = true;
        &mut self.border_config
    }

    fn set_color_config(&mut self, config: states::definitions::ColorConfig) {
        if self.colors != config { self.changed = true }
        self.colors = config;
    }

    fn get_color_config(&self) -> &states::definitions::ColorConfig { &self.colors }

    fn get_colors_config_mut(&mut self) -> &mut states::definitions::ColorConfig {
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
impl ButtonState {

    pub fn set_text(&mut self, text: String) {
        if self.text != text { self.changed = true }
        self.text = text;
    }

    pub fn get_text(&self) -> String { self.text.clone() }

    pub fn set_flashing(&mut self, flashing: bool) {
        if self.flashing != flashing { self.changed = true }
        self.flashing = flashing;
    }

    pub fn get_flashing(&self) -> bool { self.flashing }

}