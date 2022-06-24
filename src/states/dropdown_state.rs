use crate::states::state::GenericState;
use crate::common::definitions::{Coordinates};
use crate::EzProperty;
use crate::scheduler::scheduler_funcs::clean_up_property;
use crate::scheduler::scheduler::Scheduler;
use crate::states::definitions::{StateCoordinates, SizeHint, PosHint, StateSize, AutoScale, Padding,
                                 HorizontalAlignment, VerticalAlignment, BorderConfig, ColorConfig};


/// [State] implementation.
#[derive(Clone, Debug)]
pub struct DropdownState {

    /// Path to the widget to which this state belongs
    pub path: String,

    /// Position of this widget relative to its' parent [Layout]
    pub position: StateCoordinates,

    /// Absolute position of this widget on screen. Automatically propagated, do not set manually
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

    /// Bool representing whether widget is disabled, i.e. cannot be interacted with
    pub disabled: EzProperty<bool>,

    /// Global order number in which this widget will be selection when user presses down/up keys
    pub selection_order: EzProperty<usize>,

    /// List of options this dropdown will display
    pub options: Vec<String>,

    /// Bool representing whether an empty value should be shown to choose from
    pub allow_none: EzProperty<bool>,

    /// The currently active choice of the dropdown.
    pub choice: EzProperty<String>,

    /// [BorderConfig] object that will be used to draw the border if enabled
    pub border_config: BorderConfig,

    /// Object containing colors to be used by this widget in different situations
    pub colors: ColorConfig,

    /// Bool representing whether this widget is currently selected. Internal only.
    pub selected: bool,
}
impl DropdownState {
    pub fn new(path: String, scheduler: &mut Scheduler) -> Self {

       DropdownState {
           path: path.clone(),
           position: StateCoordinates::new(0, 0, path.clone(), scheduler),
           absolute_position: Coordinates::default(),
           size_hint: SizeHint::new(Some(1.0), Some(1.0), path.clone(), scheduler),
           auto_scale: AutoScale::new(false, false, path.clone(), scheduler),
           pos_hint: PosHint::new(None, None, path.clone(), scheduler),
           size: StateSize::new(0, 0, path.clone(), scheduler),
           padding: Padding::new(0, 0, 0, 0, path.clone(), scheduler),
           halign: scheduler.new_horizontal_alignment_property(
                format!("{}/halign", path), HorizontalAlignment::Left),
           valign: scheduler.new_vertical_alignment_property(
                format!("{}/valign", path), VerticalAlignment::Top),
           disabled: scheduler.new_bool_property(format!("{}/disabled", path),false),
           selected: false,
           selection_order: scheduler.new_usize_property(
                format!("{}/selection_order", path), 0),
           options: Vec::new(),
           allow_none: scheduler.new_bool_property(format!("{}/allow_none", path),true),
           choice: scheduler.new_string_property(format!("{}/choice", path),
                                                 String::new()),
           border_config: BorderConfig::new(false, path.clone(), scheduler),
           colors: ColorConfig::new(path, scheduler),
       }
    }

}
impl GenericState for DropdownState {

    fn get_path(&self) -> &String { &self.path }

    fn get_size_hint(&self) -> &SizeHint { &self.size_hint }

    fn get_size_hint_mut(&mut self) -> &mut SizeHint { &mut self.size_hint }

    fn get_pos_hint(&self) -> &PosHint { &self.pos_hint }

    fn get_pos_hint_mut(&mut self) -> &mut PosHint { &mut self.pos_hint }

    fn get_auto_scale(&self) -> &AutoScale { &self.auto_scale }

    fn get_auto_scale_mut(&mut self) -> &mut AutoScale { &mut self.auto_scale }

    fn get_size(&self) -> &StateSize { &self.size }

    fn get_size_mut(&mut self) -> &mut StateSize { &mut self.size }

    fn get_position(&self) -> &StateCoordinates { &self.position }

    fn get_position_mut(&mut self) -> &mut StateCoordinates { &mut self.position }

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

    fn set_disabled(&mut self, disabled: bool) { self.disabled.set(disabled) }

    fn get_disabled(&self) -> &EzProperty<bool> { &self.disabled }

    fn get_selection_order(&self) -> &EzProperty<usize> { &self.selection_order }

    fn set_selection_order(&mut self, order: usize) {
        self.selection_order.set(order);
    }

    fn set_selected(&mut self, state: bool) {
        self.selected = state;
    }

    fn get_selected(&self) -> bool { self.selected }
    fn clean_up_properties(&self, scheduler: &mut Scheduler) {

        self.position.clean_up_properties(scheduler);
        self.size.clean_up_properties(scheduler);
        self.size_hint.clean_up_properties(scheduler);
        self.pos_hint.clean_up_properties(scheduler);
        self.auto_scale.clean_up_properties(scheduler);
        self.padding.clean_up_properties(scheduler);
        clean_up_property(scheduler, &self.halign.name);
        clean_up_property(scheduler, &self.valign.name);
        clean_up_property(scheduler, &self.allow_none.name);
        clean_up_property(scheduler, &self.choice.name);
        clean_up_property(scheduler, &self.disabled.name);
        clean_up_property(scheduler, &self.selection_order.name);
        self.border_config.clean_up_properties(scheduler);
        self.colors.clean_up_properties(scheduler);
    }
}
impl DropdownState {

    pub fn set_choice(&mut self, choice: String) { self.choice.set(choice); }

    pub fn get_choice(&self) -> &EzProperty<String> { &self.choice }

    pub fn set_options(&mut self, options: Vec<String>) { self.options = options }

    pub fn get_options(&self) -> Vec<String> { self.options.clone() }

    pub fn set_allow_none(&mut self, allow_none: bool) { self.allow_none.set(allow_none); }

    pub fn get_allow_none(&self) -> &EzProperty<bool> { &self.allow_none }

    /// Return the total amount of options in this dropdown including the empty option if it is
    /// allowed.
    pub fn total_options(&self) -> usize { self.options.len() +
        if self.allow_none.value {1} else {0} }

}


/// [State] implementation.
#[derive(Clone, Debug)]
pub struct DroppedDownMenuState {

    /// Path to the widget to which this state belongs
    pub path: String,

    /// Widget path of the [Dropdown] that spawned this menu.
    pub parent_path: String,

    /// Position of this widget relative to its' parent [Layout]
    pub position: StateCoordinates,

    /// Absolute position of this widget on screen. Automatically propagated, do not set manually
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

    /// List of options this dropdown will display
    pub options: Vec<String>,

    pub allow_none: EzProperty<bool>,

    /// The currently active choice of the dropdown.
    pub choice: EzProperty<String>,

    /// [BorderConfig] object that will be used to draw the border if enabled
    pub border_config: BorderConfig,

    /// Object containing colors to be used by this widget in different situations
    pub colors: ColorConfig,

    /// Bool representing whether widget is disabled, i.e. cannot be interacted with
    pub disabled: EzProperty<bool>,

    /// Global order number in which this widget will be selection when user presses down/up keys
    pub selection_order: EzProperty<usize>,

    /// If dropped down, this represents which row of the dropdown is being hovered with the mouse,
    /// or has been selected with the keyboard using up/down. Internal only.
    pub dropped_down_selected_row: usize,

}
impl DroppedDownMenuState {

    pub fn new(path: String, scheduler: &mut Scheduler) -> Self {

        DroppedDownMenuState {
            path: path.clone(),
            parent_path: String::new(),
            position: StateCoordinates::new(0, 0, path.clone(), scheduler),
            absolute_position: Coordinates::default(),
            size_hint: SizeHint::new(Some(1.0), Some(1.0), path.clone(), scheduler),
            auto_scale: AutoScale::new(false, false, path.clone(), scheduler),
            pos_hint: PosHint::new(None, None, path.clone(), scheduler),
            size: StateSize::new(0, 3, path.clone(), scheduler),
            padding: Padding::new(0, 0, 0, 0, path.clone(), scheduler),
            halign: scheduler.new_horizontal_alignment_property(
                format!("{}/halign", path), HorizontalAlignment::Left),
            valign: scheduler.new_vertical_alignment_property(
                format!("{}/valign", path), VerticalAlignment::Top),
            options: Vec::new(),
            allow_none: scheduler.new_bool_property(format!("{}/allow_none", path),
                                                    true),
            choice: scheduler.new_string_property(format!("{}/choice", path),
                                                  String::new()),
            border_config: BorderConfig::new(false, path.clone(), scheduler),
            colors: ColorConfig::new(path.clone(), scheduler),
            disabled: scheduler.new_bool_property(
                format!("{}/disabled", path),false),
            selection_order: scheduler.new_usize_property(
                format!("{}/selection_order", path), 0),
            dropped_down_selected_row: 0,
        }
    }
}
impl GenericState for DroppedDownMenuState {

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
        self.halign.set(alignment)
    }

    fn get_horizontal_alignment(&self) -> &EzProperty<HorizontalAlignment> { &self.halign }

    fn set_vertical_alignment(&mut self, alignment: VerticalAlignment) {
        self.valign.set(alignment)
    }

    fn get_vertical_alignment(&self) -> &EzProperty<VerticalAlignment> { &self.valign }

    fn get_padding(&self) -> &Padding { &self.padding }

    fn get_padding_mut(&mut self) -> &mut Padding { &mut self.padding }

    fn get_border_config(&self) -> &BorderConfig { &self.border_config  }

    fn get_border_config_mut(&mut self) -> &mut BorderConfig {  &mut self.border_config  }

    fn get_color_config(&self) -> &ColorConfig { &self.colors }

    fn get_colors_config_mut(&mut self) -> &mut ColorConfig { &mut self.colors }

    fn set_disabled(&mut self, disabled: bool) {
        self.disabled.set(disabled)
    }

    fn get_disabled(&self) -> &EzProperty<bool> { &self.disabled }

    fn get_selection_order(&self) -> &EzProperty<usize> { &self.selection_order }

    fn set_selection_order(&mut self, order: usize) { self.selection_order.set(order); }

    fn clean_up_properties(&self, scheduler: &mut Scheduler) {

        self.position.clean_up_properties(scheduler);
        self.size.clean_up_properties(scheduler);
        self.size_hint.clean_up_properties(scheduler);
        self.pos_hint.clean_up_properties(scheduler);
        self.auto_scale.clean_up_properties(scheduler);
        self.padding.clean_up_properties(scheduler);
        clean_up_property(scheduler, &self.halign.name);
        clean_up_property(scheduler, &self.valign.name);
        clean_up_property(scheduler, &self.allow_none.name);
        clean_up_property(scheduler, &self.disabled.name);
        clean_up_property(scheduler, &self.choice.name);
        clean_up_property(scheduler, &self.selection_order.name);
        self.border_config.clean_up_properties(scheduler);
        self.colors.clean_up_properties(scheduler);
    }
}

impl DroppedDownMenuState {

    pub fn set_choice(&mut self, choice: String) { self.choice.set(choice); }

    pub fn get_choice(&self) -> &EzProperty<String> { &self.choice }

    pub fn set_options(&mut self, options: Vec<String>) { self.options = options }

    pub fn get_options(&self) -> Vec<String> { self.options.clone() }

    pub fn set_allow_none(&mut self, allow_none: bool) { self.allow_none.set(allow_none); }

    pub fn get_allow_none(&self) -> &EzProperty<bool> { &self.allow_none }

    pub fn set_dropped_down_selected_row(&mut self, dropped_down_selected_row: usize) {
        self.dropped_down_selected_row = dropped_down_selected_row;
    }

    pub fn get_dropped_down_selected_row(&self) -> usize { self.dropped_down_selected_row }

    /// Return the total amount of options in this dropdown including the empty option if it is
    /// allowed.
    pub fn total_options(&self) -> usize { self.options.len() +
        if self.allow_none.value {1} else {0} }

    /// Get an ordered list of options, including the empty option if it was allowed. Order is:
    /// - Active choice
    /// - Empty (if allowed)
    /// - Rest of the options in user defined order
    pub fn get_dropped_down_options(&self) -> Vec<String> {
        let mut options = vec!(self.choice.value.clone());
        if self.allow_none.value && !self.choice.value.is_empty() {
            options.push("".to_string())
        }
        for option in self.options.iter()
            .filter(|x| x.to_string() != self.choice.value) {
            options.push(option.to_string());
        }
        options
    }

}
