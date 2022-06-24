use crate::scheduler::Scheduler;
use crate::common::definitions::Coordinates;
use crate::EzProperty;
use crate::states::definitions::{AutoScale, BorderConfig, ColorConfig, StateCoordinates,
                                 HorizontalAlignment, Padding, PosHint, StateSize, SizeHint,
                                 VerticalAlignment};
use crate::states::state::GenericState;


/// [State] implementation for [ProgressBar].
#[derive(Clone, Debug)]
pub struct ProgressBarState {

    /// Path to the widget to which this state belongs
    pub path: String,

    /// Max value of the slider (i.e. when it's finished)
    pub max: EzProperty<usize>,

    /// Current value of the slider
    pub value: EzProperty<usize>,

    /// Position of this widget relative to its' parent [Layout]
    pub position: StateCoordinates,

    /// Absolute position of this layout on screen. Automatically propagated, do not set manually
    pub absolute_position: Coordinates,

    /// Relative height/width of this widget to parent layout
    pub size_hint: SizeHint,

    /// Pos hint of this widget
    pub pos_hint: PosHint,

    /// size of this widget
    pub size: StateSize,

    /// Automatically adjust size of widget to content
    pub auto_scale: AutoScale,

    /// Amount of space to leave between sides of the widget and other widgets
    pub padding: Padding,

    /// Horizontal alignment of this widget
    pub halign: EzProperty<HorizontalAlignment>,

    /// Vertical alignment of this widget
    pub valign: EzProperty<VerticalAlignment>,

    /// [BorderConfig] object that will be used to draw the border if enabled
    pub border_config: BorderConfig,

    /// Object containing colors to be used by this widget in different situations
    pub colors: ColorConfig,

    /// Bool representing whether widget is disabled, i.e. cannot be interacted with
    pub disabled: EzProperty<bool>,

    /// Global order number in which this widget will be selection when user presses down/up keys
    pub selection_order: EzProperty<usize>,

    /// Bool representing whether this widget is currently selected.
    pub selected: bool,
}
impl ProgressBarState {

    pub fn new(path: String, scheduler: &mut Scheduler) -> Self {

       ProgressBarState {
           path: path.clone(),
           max: scheduler.new_usize_property(format!("{}/progress_max", path),
                                             100),
           value: scheduler.new_usize_property(format!("{}/progress_value", path),
                                               0),
           position: StateCoordinates::new(0, 0, path.clone(), scheduler),
           absolute_position: Coordinates::default(),
           size: StateSize::new(0, 0, path.clone(), scheduler),
           size_hint: SizeHint::new(Some(1.0), Some(1.0), path.clone(), scheduler),
           pos_hint: PosHint::new(None, None, path.clone(), scheduler),
           auto_scale: AutoScale::new(false, false, path.clone(), scheduler),
           padding: Padding::new(0, 0, 0, 0, path.clone(), scheduler),
           halign: scheduler.new_horizontal_alignment_property(
                format!("{}/halign", path), HorizontalAlignment::Left),
           valign: scheduler.new_vertical_alignment_property(
                format!("{}/valign", path), VerticalAlignment::Top),
           selected: false,
           disabled: scheduler.new_bool_property(
               format!("{}/disabled", path),false),
           selection_order: scheduler.new_usize_property(
                format!("{}/selection_order", path), 0),
           border_config: BorderConfig::new(false, path.clone(), scheduler),
            colors: ColorConfig::new(path, scheduler),
       }
    }
}
impl GenericState for ProgressBarState {

    fn get_path(&self) -> &String {
        &self.path
    }

    fn get_size_hint(&self) -> &SizeHint { &self.size_hint }

    fn get_size_hint_mut(&mut self) -> &mut SizeHint { &mut self.size_hint }

    fn get_pos_hint(&self) -> &PosHint { &self.pos_hint }

    fn get_pos_hint_mut(&mut self) -> &mut PosHint { &mut self.pos_hint }

    fn get_auto_scale(&self) -> &AutoScale { &self.auto_scale }

    fn get_auto_scale_mut(&mut self) -> &mut AutoScale { &mut self.auto_scale }

    fn get_size(&self) -> &StateSize { &self.size }

    fn get_size_mut(&mut self) -> &mut StateSize { &mut self.size }

    fn get_position(&self) -> &StateCoordinates { &self.position }

    fn get_position_mut(&mut self) -> &mut StateCoordinates {
        &mut self.position
    }

    fn set_absolute_position(&mut self, pos: Coordinates) { self.absolute_position = pos }

    fn get_absolute_position(&self) -> Coordinates { self.absolute_position }

    fn set_horizontal_alignment(&mut self, alignment: HorizontalAlignment) {
        self.halign.set(alignment);
    }

    fn get_horizontal_alignment(&self) -> &EzProperty<HorizontalAlignment> { &self.halign }

    fn set_vertical_alignment(&mut self, alignment: VerticalAlignment) {
        self.valign.set(alignment);
    }

    fn get_vertical_alignment(&self) -> &EzProperty<VerticalAlignment> { &self.valign }

    fn get_padding(&self) -> &Padding { &self.padding }

    fn get_padding_mut(&mut self) -> &mut Padding { &mut self.padding }

    fn get_border_config(&self) -> &BorderConfig { &self.border_config  }

    fn get_border_config_mut(&mut self) -> &mut BorderConfig {  &mut self.border_config  }

    fn get_color_config(&self) -> &ColorConfig { &self.colors }

    fn get_colors_config_mut(&mut self) -> &mut ColorConfig { &mut self.colors }

    fn is_selectable(&self) -> bool { true }

    fn set_disabled(&mut self, disabled: bool) {
        self.disabled.set(disabled)
    }

    fn get_disabled(&self) -> &EzProperty<bool> { &self.disabled }

    fn get_selection_order(&self) -> &EzProperty<usize> { &self.selection_order }

    fn set_selection_order(&mut self, order: usize) {
        self.selection_order.set(order);
    }

    fn set_selected(&mut self, state: bool) {
        self.selected = state;
    }

    fn get_selected(&self) -> bool { self.selected }
}
impl ProgressBarState {

    pub fn set_value(&mut self, mut value: usize) {
        value = if value > self.max.value {self.max.value} else { value };
        self.value.set(value);
    }

    pub fn get_value(&self) -> &EzProperty<usize> { &self.value }

    pub fn set_max_value(&mut self, max_value: usize) {
        if self.value > max_value {
            self.set_value(max_value)
        }
        self.max.set(max_value);
    }

    pub fn get_max_value(&self) -> &EzProperty<usize> { &self.max }

    /// Convenience func. Set the progress bar based on two numbers: progress and total. For
    /// example when tracking progress of copying files, pass the number of copied files as
    /// 'progress' and the total number of files as 'total'. These values will be normalized to a
    /// number between 0 and 1 and passed to [set_value].
    pub fn get_normalized_value(&mut self) -> f64 { self.value.value as f64 / self.max.value as f64 }
}
