use crate::scheduler::scheduler::SchedulerFrontend;
use crate::property::ez_property::EzProperty;
use crate::run::definitions::{IsizeCoordinates};
use crate::scheduler::scheduler_funcs::clean_up_property;
use crate::states::definitions::{AutoScale, BorderConfig, ColorConfig, HorizontalAlignment, Padding, PosHint, StateSize, SizeHint, StateCoordinates, VerticalAlignment, InfiniteSize};
use crate::states::ez_state::GenericState;


/// [State] implementation for [Button].
#[derive(Clone, Debug)]
pub struct SliderState {

    /// Path to the widget to which this state belongs
    pub path: String,

    /// Current value of the slider
    pub value: EzProperty<usize>,

    /// Low boundary of the slider
    pub minimum: EzProperty<usize>,

    /// Upper boundary of the slider
    pub maximum: EzProperty<usize>,

    /// Amount to change value by when moved one step
    pub step: EzProperty<usize>,

    /// Position of this widget relative to its' parent [layout]
    pub position: StateCoordinates,

    /// Absolute position of this layout on screen. Automatically propagated, do not set manually
    absolute_position: IsizeCoordinates,

    /// Relative height/width of this widget to parent layout
    pub size_hint: SizeHint,

    /// Pos hint of this widget
    pub pos_hint: PosHint,

    /// size of this widget
    pub size: StateSize,

    /// Infinite size of this widget for x and y axes, used in scrolling
    infinite_size: InfiniteSize,

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
    selected: bool,
}
impl SliderState {

    pub fn new(path: String, scheduler: &mut SchedulerFrontend) -> Self {

       SliderState {
           path: path.clone(),
           value: scheduler.new_usize_property(format!("{}/value", path).as_str(), 0),
           minimum: scheduler.new_usize_property(format!("{}/minimum", path).as_str(), 0),
           maximum: scheduler.new_usize_property(format!("{}/maximum", path).as_str(), 100),
           step: scheduler.new_usize_property(format!("{}/step", path).as_str(), 1),
           position: StateCoordinates::new(0, 0, path.clone(), scheduler),
           absolute_position: IsizeCoordinates::default(),
           size: StateSize::new(0, 0, path.clone(), scheduler),
           infinite_size: InfiniteSize::default(),
           size_hint: SizeHint::new(Some(1.0), Some(1.0), path.clone(), scheduler),
           pos_hint: PosHint::new(None, None, path.clone(), scheduler),
           auto_scale: AutoScale::new(false, false, path.clone(), scheduler),
           padding: Padding::new(0, 0, 0, 0, path.clone(), scheduler),
           halign: scheduler.new_horizontal_alignment_property(
                format!("{}/halign", path).as_str(), HorizontalAlignment::Left),
           valign: scheduler.new_vertical_alignment_property(
                format!("{}/valign", path).as_str(), VerticalAlignment::Top),
           disabled: scheduler.new_bool_property(
                format!("{}/disabled", path).as_str(),false),
           selected: false,
           selection_order: scheduler.new_usize_property(
                format!("{}/selection_order", path).as_str(), 0),
           border_config: BorderConfig::new(false, path.clone(), scheduler),
           colors: ColorConfig::new(path, scheduler),
       }
    }
}
impl GenericState for SliderState {

    fn get_path(&self) -> &String { &self.path }

    fn get_size_hint(&self) -> &SizeHint { &self.size_hint }

    fn get_size_hint_mut(&mut self) -> &mut SizeHint { &mut self.size_hint }

    fn get_pos_hint(&self) -> &PosHint { &self.pos_hint }

    fn get_pos_hint_mut(&mut self) -> &mut PosHint { &mut self.pos_hint }

    fn get_auto_scale(&self) -> &AutoScale { &self.auto_scale }

    fn get_auto_scale_mut(&mut self) -> &mut AutoScale { &mut self.auto_scale }

    fn get_size(&self) -> &StateSize { &self.size }

    fn get_size_mut(&mut self) -> &mut StateSize { &mut self.size }

    fn get_infinite_size(&self) -> &InfiniteSize { &self.infinite_size }

    fn get_infinite_size_mut(&mut self) -> &mut InfiniteSize { &mut self.infinite_size }

    fn get_position(&self) -> &StateCoordinates { &self.position }

    fn get_position_mut(&mut self) -> &mut StateCoordinates { &mut self.position }

    fn set_absolute_position(&mut self, pos: IsizeCoordinates) { self.absolute_position = pos }

    fn get_absolute_position(&self) -> IsizeCoordinates { self.absolute_position }

    fn set_horizontal_alignment(&mut self, alignment: HorizontalAlignment) {
        self.halign.set(alignment);
    }

    fn get_horizontal_alignment(&self) -> HorizontalAlignment { self.halign.value }

    fn set_vertical_alignment(&mut self, alignment: VerticalAlignment) {
        self.valign.set(alignment);
    }

    fn get_vertical_alignment(&self) -> VerticalAlignment { self.valign.value }

    fn get_padding(&self) -> &Padding { &self.padding }

    fn get_padding_mut(&mut self) -> &mut Padding { &mut self.padding }

    fn get_border_config(&self) -> &BorderConfig { &self.border_config  }

    fn get_border_config_mut(&mut self) -> &mut BorderConfig {  &mut self.border_config  }

    fn get_color_config(&self) -> &ColorConfig { &self.colors }

    fn get_color_config_mut(&mut self) -> &mut ColorConfig { &mut self.colors }

    fn is_selectable(&self) -> bool { true }

    fn set_disabled(&mut self, disabled: bool) { self.disabled.set(disabled) }

    fn get_disabled(&self) -> bool { self.disabled.value }

    fn get_selection_order(&self) -> usize { self.selection_order.value }

    fn set_selection_order(&mut self, order: usize) { self.selection_order.set(order); }

    fn set_selected(&mut self, state: bool) { self.selected = state; }

    fn get_selected(&self) -> bool { self.selected }

    fn clean_up_properties(&self, scheduler: &mut SchedulerFrontend) {

        self.position.clean_up_properties(scheduler);
        self.size.clean_up_properties(scheduler);
        self.size_hint.clean_up_properties(scheduler);
        self.pos_hint.clean_up_properties(scheduler);
        self.auto_scale.clean_up_properties(scheduler);
        self.padding.clean_up_properties(scheduler);
        clean_up_property(scheduler, &self.halign.name);
        clean_up_property(scheduler, &self.valign.name);
        clean_up_property(scheduler, &self.value.name);
        clean_up_property(scheduler, &self.minimum.name);
        clean_up_property(scheduler, &self.maximum.name);
        clean_up_property(scheduler, &self.step.name);
        clean_up_property(scheduler, &self.disabled.name);
        clean_up_property(scheduler, &self.selection_order.name);
        self.border_config.clean_up_properties(scheduler);
        self.colors.clean_up_properties(scheduler);
    }
}
impl SliderState {

    pub fn set_value(&mut self, value: usize) {
        self.value.set(value);
    }

    pub fn get_value(&self) -> usize { self.value.value }

    pub fn set_minimum(&mut self, minimum: usize) {
        self.minimum.set(minimum);
    }
    pub fn get_minimum(&self) -> usize { self.minimum.value }

    pub fn set_maximum(&mut self, maximum: usize) {
        self.maximum.set(maximum);
    }

    pub fn get_maximum(&self) -> usize { self.maximum.value }

    pub fn set_step(&mut self, step: usize) {
        self.step.set(step);
    }

    pub fn get_step(&self) -> usize { self.step.value }

    pub fn validate(&self) {
        if self.minimum.value >= self.maximum.value {panic!("Slider minimum must be lower than maximum")}
        if self.minimum.value % self.step.value != 0 {panic!("Slider minimum must be a multiple of step")}
        if self.maximum.value % self.step.value != 0 {panic!("Slider maximum must be a multiple of step")}
        if self.step.value < 1 {panic!("Step value must be larger than 0")}
        if self.value.value % self.step.value != 0 {panic!("Slider value must be a multiple of step")}
    }
}
