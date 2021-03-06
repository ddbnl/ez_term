use crate::EzProperty;
use crate::run::definitions::{IsizeCoordinates};
use crate::scheduler::scheduler::SchedulerFrontend;
use crate::scheduler::scheduler_funcs::clean_up_property;
use crate::states::definitions::{StateCoordinates, SizeHint, PosHint, StateSize, AutoScale, Padding,
                                 HorizontalAlignment, VerticalAlignment, BorderConfig, ColorConfig};
use crate::states::ez_state::GenericState;


/// [State] implementation.
#[derive(Clone, Debug)]
pub struct RadioButtonState {

    /// Path to the widget to which this state belongs
    pub path: String,

    /// Group this radio button belongs to. Set the same group value for a number of radio buttons
    /// to make them mutually exclusive.
    group: EzProperty<String>,

    /// Position of this widget relative to its' parent [layout]
    position: StateCoordinates,

    /// Absolute position of this widget on screen. Internal only.
    absolute_position: IsizeCoordinates,

    /// size of this widget
    size: StateSize,

    /// Automatically adjust size of widget to content
    auto_scale: AutoScale,

    /// Cannot be set for Radio Button, size is always 5,1
    size_hint: SizeHint,

    /// Pos hint of this widget
    pos_hint: PosHint,

    /// Amount of space to leave between sides of the widget and other widgets
    padding: Padding,

    /// Horizontal alignment of this widget
    halign: EzProperty<HorizontalAlignment>,

    /// Vertical alignment of this widget
    valign: EzProperty<VerticalAlignment>,

    /// Bool representing whether this widget is currently active (i.e. checkbox is checked)
    active: EzProperty<bool>,

    /// [Pixel.symbol] used when the Checkbox is active
    active_symbol: EzProperty<String>,

    /// [Pixel.symbol] used when the Checkbox is not active
    inactive_symbol: EzProperty<String>,

    /// [BorderConfig] object that will be used to draw the border if enabled
    border_config: BorderConfig,

    /// Object containing colors to be used by this widget in different situations
    colors: ColorConfig,

    /// Bool representing whether widget is disabled, i.e. cannot be interacted with
    disabled: EzProperty<bool>,

    /// Global order number in which this widget will be selection when user presses down/up keys
    selection_order: EzProperty<usize>,

    /// Bool representing whether this widget is currently selected.
    selected: bool,
}
impl RadioButtonState {

    pub fn new(path: String, scheduler: &mut SchedulerFrontend) -> Self {

       RadioButtonState {
           path: path.clone(),
           group: scheduler.new_string_property(format!("{}/group", path).as_str(),
                                                String::new()),
           position: StateCoordinates::new(0, 0, path.clone(), scheduler),
           absolute_position: IsizeCoordinates::default(),
           size: StateSize::new(0, 0, path.clone(), scheduler),
           auto_scale: AutoScale::new(false, false, path.clone(), scheduler),
           size_hint: SizeHint::new(Some(1.0), Some(1.0), path.clone(), scheduler),
           pos_hint: PosHint::new(None, None, path.clone(), scheduler),
           padding: Padding::new(0, 0, 0, 0, path.clone(), scheduler),
           halign: scheduler.new_horizontal_alignment_property(
                format!("{}/halign", path).as_str(), HorizontalAlignment::Left),
           valign: scheduler.new_vertical_alignment_property(
                format!("{}/valign", path).as_str(), VerticalAlignment::Top),
           active: scheduler.new_bool_property(
               format!("{}/active", path).as_str(),false),
           active_symbol: scheduler.new_string_property(format!("{}/active_symbol",
                                                                path).as_str(),"X".to_string()),
           inactive_symbol: scheduler.new_string_property(format!("{}/active_symbol",
                                                                  path).as_str(),"-".to_string()),
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
impl GenericState for RadioButtonState {
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

    fn get_position_mut(&mut self) -> &mut StateCoordinates { &mut self.position }

    fn set_absolute_position(&mut self, pos: IsizeCoordinates) { self.absolute_position = pos; }

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

    fn is_selectable(&self) -> bool { true}

    fn set_disabled(&mut self, disabled: bool) {
        self.disabled.set(disabled)
    }

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
        clean_up_property(scheduler, &self.disabled.name);
        clean_up_property(scheduler, &self.active.name);
        clean_up_property(scheduler, &self.selection_order.name);
        self.border_config.clean_up_properties(scheduler);
        self.colors.clean_up_properties(scheduler);
    }
}
impl RadioButtonState {

    /// Set the group this radio button belongs to. Radio buttons that share a group are
    /// mutually exclusive.
    pub fn set_group(&mut self, group: String) { self.group.set(group) }

    /// Get the group this radio button belongs to. Radio buttons that share a group are
    /// mutually exclusive.
    pub fn get_group(&self) -> String { self.group.value.clone() }

    pub fn set_active(&mut self, active: bool) {
        self.active.set(active);
    }

    pub fn get_active(&self) -> bool { self.active.value }

    pub fn set_active_symbol(&mut self, symbol: String) { self.active_symbol.set(symbol); }

    pub fn get_active_symbol(&self) -> String { self.active_symbol.value.clone() }

    pub fn set_inactive_symbol(&mut self, symbol: String) { self.inactive_symbol.set(symbol); }

    pub fn get_inactive_symbol(&self) -> String { self.inactive_symbol.value.clone() }
}
