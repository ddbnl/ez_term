use crate::property::ez_property::EzProperty;
use crate::run::definitions::{IsizeCoordinates};
use crate::scheduler::scheduler::SchedulerFrontend;
use crate::scheduler::scheduler_funcs::clean_up_property;
use crate::states::definitions::{StateCoordinates, SizeHint, PosHint, StateSize, AutoScale, Padding,
                                 HorizontalAlignment, VerticalAlignment, BorderConfig, ColorConfig};
use crate::states::ez_state::GenericState;


/// [State] implementation.
#[derive(Clone, Debug)]
pub struct LabelState {

    /// Path to the widget to which this state belongs
    pub path: String,

    /// Optional file path to retrieve text from
    from_file: EzProperty<String>,

    /// Text currently being displayed by the label
    text: EzProperty<String>,

    /// Position of this widget relative to its' parent [layout]
    position: StateCoordinates,

    /// Absolute position of this widget on screen. Automatically propagated, do not set manually
    absolute_position: IsizeCoordinates,

    /// Relative height/width of this widget to parent layout
    size_hint: SizeHint,

    /// Pos hint of this widget
    pos_hint: PosHint,

    /// size of this widget
    size: StateSize,

    /// Automatically adjust size of widget to content
    auto_scale: AutoScale,

    /// Amount of space to leave between sides of the widget and other widgets
    padding: Padding,

    /// Horizontal alignment of this widget
    halign: EzProperty<HorizontalAlignment>,

    /// Vertical alignment of this widget
    valign: EzProperty<VerticalAlignment>,

    /// [BorderConfig] object that will be used to draw the border if enabled
    border_config: BorderConfig,

    /// Object containing colors to be used by this widget in different situations
    colors: ColorConfig,

    /// Bool representing whether widget is disabled, i.e. cannot be interacted with
    disabled: EzProperty<bool>,

    /// Global order number in which this widget will be selection when user presses down/up keys
    selection_order: EzProperty<usize>,
}
impl LabelState {

    pub fn new(path: String, scheduler: &mut SchedulerFrontend) -> Self {

       LabelState {
           path: path.clone(),
           from_file: scheduler.new_string_property(format!("{}/from_file", path).as_str(),
                                               String::new()),
           text: scheduler.new_string_property(format!("{}/text", path).as_str(),
                                               String::new()),
           position: StateCoordinates::new(0, 0, path.clone(), scheduler),
           absolute_position: IsizeCoordinates::default(),
           size_hint: SizeHint::new(Some(1.0), Some(1.0), path.clone(), scheduler),
           pos_hint: PosHint::new(None, None, path.clone(), scheduler),
           auto_scale: AutoScale::new(false, false, path.clone(), scheduler),
           size: StateSize::new(0, 0, path.clone(), scheduler),
           padding: Padding::new(0, 0, 0, 0, path.clone(), scheduler),
           halign: scheduler.new_horizontal_alignment_property(
                format!("{}/halign", path).as_str(), HorizontalAlignment::Left),
           valign: scheduler.new_vertical_alignment_property(
                format!("{}/valign", path).as_str(), VerticalAlignment::Top),
           border_config: BorderConfig::new(false, path.clone(), scheduler),
           colors: ColorConfig::new(path.clone(), scheduler),
           disabled: scheduler.new_bool_property(format!("{}/disabled", path).as_str(),false),
           selection_order: scheduler.new_usize_property(
               format!("{}/selection_order", path).as_str(), 0),
       }
    }
}
impl GenericState for LabelState {

    fn get_path(&self) -> &String { &self.path }

    fn get_size_hint(&self) -> &SizeHint { &self.size_hint }

    fn get_size_hint_mut(&mut self) -> &mut SizeHint { &mut self.size_hint }

    fn get_pos_hint(&self) -> &PosHint { &self.pos_hint }

    fn get_pos_hint_mut(&mut self) -> &mut PosHint { &mut self.pos_hint }

    fn get_auto_scale(&self) -> &AutoScale { &self.auto_scale }

    fn get_auto_scale_mut(&mut self) -> &mut AutoScale { &mut self.auto_scale }

    fn get_size(&self) -> &StateSize { &self.size  }

    fn get_size_mut(&mut self) -> &mut StateSize { &mut self.size }

    fn get_position(&self) -> &StateCoordinates { &self.position }

    fn get_position_mut(&mut self) -> &mut StateCoordinates {
        &mut self.position
    }

    fn set_absolute_position(&mut self, pos: IsizeCoordinates) {
        self.absolute_position = pos;
    }

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

    fn set_disabled(&mut self, disabled: bool) {
        self.disabled.set(disabled)
    }

    fn get_disabled(&self) -> bool { self.disabled.value }

    fn get_selection_order(&self) -> usize { self.selection_order.value }

    fn set_selection_order(&mut self, order: usize) { self.selection_order.set(order); }

    fn clean_up_properties(&self, scheduler: &mut SchedulerFrontend) {

        self.position.clean_up_properties(scheduler);
        self.size.clean_up_properties(scheduler);
        self.size_hint.clean_up_properties(scheduler);
        self.pos_hint.clean_up_properties(scheduler);
        self.auto_scale.clean_up_properties(scheduler);
        self.padding.clean_up_properties(scheduler);
        clean_up_property(scheduler, &self.halign.name);
        clean_up_property(scheduler, &self.valign.name);
        clean_up_property(scheduler, &self.disabled.name);
        clean_up_property(scheduler, &self.text.name);
        clean_up_property(scheduler, &self.selection_order.name);
        self.border_config.clean_up_properties(scheduler);
        self.colors.clean_up_properties(scheduler);
    }
}
impl LabelState {

    pub fn set_from_file(&mut self, fp: String) { self.from_file.set(fp); }

    pub fn get_from_file(&self) -> String { self.from_file.value.clone() }

    pub fn get_text(&self) -> String { self.text.value.clone() }

    pub fn get_text_mut(&mut self) -> &mut EzProperty<String> { &mut self.text }
    
    pub fn set_text(&mut self, text: String) { self.get_text_mut().set(text) }
}
