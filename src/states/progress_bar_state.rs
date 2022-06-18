use crate::states::definitions::{AutoScale, BorderConfig, ColorConfig, Coordinates,
                                 HorizontalAlignment, Padding, PosHint, Size, SizeHint,
                                 VerticalAlignment};
use crate::states::state::GenericState;


/// [State] implementation for [ProgressBar].
#[derive(Clone)]
pub struct ProgressBarState {

    /// Current value of the slider
    pub value: f64,

    /// Position of this widget relative to its' parent [Layout]
    pub position: Coordinates,

    /// Absolute position of this layout on screen. Automatically propagated, do not set manually
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

    /// Bool representing if state has changed. Triggers widget redraw.
    pub changed: bool,

    /// Object containing colors to be used by this widget in different situations
    pub colors: ColorConfig,

    /// Global order number in which this widget will be selection when user presses down/up keys
    pub selection_order: usize,

    /// Bool representing whether this widget is currently selected.
    pub selected: bool,

    /// If true this forces a global screen redraw on the next frame. Screen redraws are diffed
    /// so this can be called when needed without degrading performance. If only screen positions
    /// that fall within this widget must be redrawn, call [EzObject.redraw] instead.
    pub force_redraw: bool,
}
impl Default for ProgressBarState {
    fn default() -> Self {

       ProgressBarState {
           value: 0.0,
           position: Coordinates::default(),
           absolute_position: Coordinates::default(),
           size: Size::default(),
           size_hint: SizeHint::default(),
           pos_hint: PosHint::default(),
           auto_scale: AutoScale::default(),
           padding: Padding::default(),
           halign: HorizontalAlignment::Left,
           valign: VerticalAlignment::Top,
           selected: false,
           selection_order: 0,
           border_config: BorderConfig::default(),
           colors: ColorConfig::default(),
           changed: false,
           force_redraw: false,
       }
    }
}
impl GenericState for ProgressBarState {

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

    fn get_size(&self) -> &Size { &self.size }

    fn get_size_mut(&mut self) -> &mut Size { &mut self.size }

    fn set_position(&mut self, position: Coordinates) { self.position = position; }

    fn get_position(&self) -> Coordinates { self.position }

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

    fn is_selectable(&self) -> bool { true }

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
impl ProgressBarState {

    /// Set value of the progress bar. Must be a number between 0 and 1.
    pub fn set_value(&mut self, value: f64) {
        if value < 0.0 || value > 1.0 {panic!("Progress bar value must be between 0 and 1")}
        if self.value != value { self.changed = true }
        self.value = value;
    }

    pub fn get_value(&self) -> f64 { self.value }

    /// Convenience func. Set the progress bar based on two numbers: progress and total. For
    /// example when tracking progress of copying files, pass the number of copied files as
    /// 'progress' and the total number of files as 'total'. These values will be normalized to a
    /// number between 0 and 1 and passed to [set_value].
    pub fn set_value_from_progress(&mut self, progress: usize, total: usize) {
        self.set_value(progress as f64 / total as f64)
    }
}
