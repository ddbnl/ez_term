use crate::states;
use crate::states::state::GenericState;


/// [State] implementation.
#[derive(Clone)]
pub struct CanvasState {

    /// Position of this widget relative to its' parent [Layout]
    pub position: states::definitions::Coordinates,

    /// Absolute position of this widget on screen. Automatically propagated, do not set manually
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

    /// Object containing colors to be used by this widget in different situations
    pub colors: states::definitions::ColorConfig,

    /// Bool representing if state has changed. Triggers widget redraw.
    pub changed: bool,

    /// If true this forces a global screen redraw on the next frame. Screen redraws are diffed
    /// so this can be called when needed without degrading performance. If only screen positions
    /// that fall within this widget must be redrawn, call [EzObject.redraw] instead.
    pub force_redraw: bool,
}
impl Default for CanvasState {
    fn default() -> Self {
        CanvasState{
            position: states::definitions::Coordinates::default(),
            absolute_position: states::definitions::Coordinates::default(),
            pos_hint: states::definitions::PosHint::default(),
            size: states::definitions::Size::default(),
            size_hint: states::definitions::SizeHint::default(),
            auto_scale: states::definitions::AutoScale::default(),
            padding: states::definitions::Padding::default(),
            border_config: states::definitions::BorderConfig::default(),
            colors: states::definitions::ColorConfig::default(),
            halign: states::definitions::HorizontalAlignment::Left,
            valign: states::definitions::VerticalAlignment::Top,
            changed: false,
            force_redraw: false,
        }
    }
}
impl GenericState for CanvasState {

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

    fn get_size(&self) -> &states::definitions::Size { &self.size  }

    fn get_size_mut(&mut self) -> &mut states::definitions::Size { &mut self.size }

    fn set_position(&mut self, position: states::definitions::Coordinates) { self.position = position; }

    fn get_position(&self) -> states::definitions::Coordinates { self.position }

    fn set_absolute_position(&mut self, pos: states::definitions::Coordinates) {
        if self.absolute_position != pos { self.changed = true }
        self.absolute_position = pos;
    }

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

    fn set_force_redraw(&mut self, redraw: bool) {
        self.force_redraw = redraw;
        self.changed = true;
    }

    fn get_force_redraw(&self) -> bool { self.force_redraw }

}