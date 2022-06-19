use crate::common::definitions::Coordinates;
use crate::scheduler::Scheduler;
use crate::states::definitions::{StateCoordinates, SizeHint, PosHint, Size, AutoScale, Padding,
                                 HorizontalAlignment, VerticalAlignment, BorderConfig, ColorConfig};
use crate::states::state::GenericState;


/// [State] implementation.
#[derive(Clone, Debug)]
pub struct LabelState {

    /// Text currently being displayed by the label
    pub text: String,

    /// Position of this widget relative to its' parent [Layout]
    pub position: StateCoordinates,

    /// Absolute position of this widget on screen. Automatically propagated, do not set manually
    pub absolute_position: Coordinates,

    /// Relative height/width of this widget to parent layout
    pub size_hint: SizeHint,

    /// Pos hint of this widget
    pub pos_hint: PosHint,

    /// size of this widget
    pub size: Size,

    /// Automatically adjust size of widget to content
    pub auto_scale: AutoScale,

    /// Amount of space to leave between sides of the widget and other widgets
    pub padding: Padding,

    /// Horizontal alignment of this widget
    pub halign: HorizontalAlignment,

    /// Vertical alignment of this widget
    pub valign: VerticalAlignment,

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
impl LabelState {

    pub fn new(path: String, scheduler: &mut Scheduler) -> Self {

       LabelState {
           position: StateCoordinates::new(0, 0, path, scheduler),
           absolute_position: Coordinates::default(),
           size_hint: SizeHint::default(),
           pos_hint: PosHint::default(),
           auto_scale: AutoScale::default(),
           size: Size::default(),
           padding: Padding::default(),
           halign: HorizontalAlignment::Left,
           valign: VerticalAlignment::Top,
           text: String::new(),
           border_config: BorderConfig::default(),
           colors: ColorConfig::default(),
           changed: false,
           force_redraw: false
       }
    }
}
impl GenericState for LabelState {

    fn set_changed(&mut self, changed: bool) { self.changed = changed }

    fn get_changed(&self) -> bool { self.changed }

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

    fn set_size(&mut self, size: Size) { self.size = size; }

    fn get_size(&self) -> &Size { &self.size  }

    fn get_size_mut(&mut self) -> &mut Size { &mut self.size }

    fn get_position(&self) -> &StateCoordinates { &self.position }

    fn get_position_mut(&mut self) -> &mut StateCoordinates {
        self.changed = true;
        &mut self.position
    }

    fn set_absolute_position(&mut self, pos: Coordinates) {
        if self.absolute_position != pos { self.changed = true }
        self.absolute_position = pos;
    }

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

    fn set_padding(&mut self, padding: Padding) {
        if self.padding != padding { self.changed = true }
        self.padding = padding;
    }

    fn get_padding(&self) -> &Padding { &self.padding }

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

    fn set_force_redraw(&mut self, redraw: bool) {
        self.force_redraw = redraw;
        self.changed = true;
    }

    fn get_force_redraw(&self) -> bool { self.force_redraw }


}
impl LabelState {

    pub fn set_text(&mut self, text: String) {
        if self.text != text { self.changed = true }
        self.text = text;
    }

    pub fn get_text(&self) -> String { self.text.clone() }
}
