use crate::common::definitions::Coordinates;
use crate::scheduler::Scheduler;
use crate::states::definitions::{StateCoordinates, SizeHint, PosHint, Size, AutoScale, Padding,
                                 HorizontalAlignment, VerticalAlignment, BorderConfig, ColorConfig};
use crate::states::state::GenericState;


/// [State] implementation.
#[derive(Clone, Debug)]
pub struct CheckboxState {

    /// Path to the widget to which this state belongs
    pub path: String,

    /// Bool representing whether this widget is currently active (i.e. checkbox is checked)
    pub active: bool,

    /// Position of this widget relative to its' parent [Layout]
    pub position: StateCoordinates,

    /// Absolute position of this widget on screen. Automatically propagated, do not set manually
    pub absolute_position: Coordinates,

    /// size of this widget
    pub size: Size,

    /// Cannot be set, checkbox is always 5,1
    pub size_hint: SizeHint,

    /// Automatically adjust size of widget to content
    pub auto_scale: AutoScale,

    /// Pos hint of this widget
    pub pos_hint: PosHint,

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
impl CheckboxState {
    
    pub fn new(path: String, scheduler: &mut Scheduler) -> Self {
    
       CheckboxState {
           path: path.clone(),
           position: StateCoordinates::new(0, 0, path ,scheduler),
           absolute_position: Coordinates::default(),
           size: Size::new(5, 1),
           auto_scale: AutoScale::default(),
           size_hint: SizeHint::new(None, None),
           pos_hint: PosHint::default(),
           padding: Padding::default(),
           halign: HorizontalAlignment::Left,
           valign: VerticalAlignment::Top,
           active: false,
           disabled: false,
           selected: false,
           selection_order: 0,
           border_config: BorderConfig::default(),
           colors: ColorConfig::default(),
           changed: false,
           force_redraw: false
       }
    }
}
impl GenericState for CheckboxState {

    fn get_path(&self) -> &String {
        &self.path
    }

    fn set_size_hint(&mut self, _size_hint: SizeHint) { }

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

    fn set_size(&mut self, mut size: Size) {
        size.width = 5;
        size.height = 1;
        self.size = size;
    }

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
impl CheckboxState {

    pub fn set_active(&mut self, active: bool) {
        if self.active != active { self.changed = true }
        self.active = active;
    }

    pub fn get_active(&self) -> bool { self.active }
}
