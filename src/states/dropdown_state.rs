use crate::property::ez_values::EzValues;
use crate::run::definitions::IsizeCoordinates;
use crate::scheduler::scheduler::SchedulerFrontend;
use crate::scheduler::scheduler_funcs::clean_up_property;
use crate::states::definitions::{
    AutoScale, BorderConfig, ColorConfig, HorizontalAlignment, InfiniteSize, Padding, PosHint,
    SizeHint, StateCoordinates, StateSize, VerticalAlignment,
};
use crate::states::ez_state::{EzState, GenericState};
use crate::EzProperty;

/// [State] implementation.
#[derive(Clone, Debug)]
pub struct DropdownState {
    /// Path to the widget to which this state belongs
    pub path: String,

    /// Position of this widget relative to its' parent [layout]
    pub position: StateCoordinates,

    /// Absolute position of this widget on screen. Automatically propagated, do not set manually
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

    /// Bool representing whether widget is disabled, i.e. cannot be interacted with
    pub disabled: EzProperty<bool>,

    /// Global order number in which this widget will be selection when user presses down/up keys
    pub selection_order: EzProperty<usize>,

    /// List of options this dropdown will display
    options: Vec<String>,

    /// Bool representing whether an empty value should be shown to choose from
    pub allow_none: EzProperty<bool>,

    /// The currently active choice of the dropdown.
    pub choice: EzProperty<String>,

    /// [BorderConfig] object that will be used to draw the border if enabled
    pub border_config: BorderConfig,

    /// Object containing colors to be used by this widget in different situations
    pub colors: ColorConfig,

    /// Bool representing whether this widget is currently selected. Internal only.
    selected: bool,
}

impl DropdownState {
    pub fn new(path: String, scheduler: &mut SchedulerFrontend) -> Self {
        DropdownState {
            path: path.clone(),
            position: StateCoordinates::new(0, 0, path.clone(), scheduler),
            absolute_position: IsizeCoordinates::default(),
            size_hint: SizeHint::new(Some(1.0), Some(1.0), path.clone(), scheduler),
            auto_scale: AutoScale::new(false, false, path.clone(), scheduler),
            pos_hint: PosHint::new(None, None, path.clone(), scheduler),
            size: StateSize::new(0, 0, path.clone(), scheduler),
            infinite_size: InfiniteSize::default(),
            padding: Padding::new(0, 0, 0, 0, path.clone(), scheduler),
            halign: scheduler.new_horizontal_alignment_property(
                format!("{}/halign", path).as_str(),
                HorizontalAlignment::Left,
            ),
            valign: scheduler.new_vertical_alignment_property(
                format!("{}/valign", path).as_str(),
                VerticalAlignment::Top,
            ),
            disabled: scheduler.new_bool_property(format!("{}/disabled", path).as_str(), false),
            selected: false,
            selection_order: scheduler
                .new_usize_property(format!("{}/selection_order", path).as_str(), 0),
            options: Vec::new(),
            allow_none: scheduler.new_bool_property(format!("{}/allow_none", path).as_str(), true),
            choice: scheduler
                .new_string_property(format!("{}/choice", path).as_str(), String::new()),
            border_config: BorderConfig::new(true, path.clone(), scheduler),
            colors: ColorConfig::new(path, scheduler),
        }
    }
}

impl GenericState for DropdownState {
    fn get_path(&self) -> &String {
        &self.path
    }

    fn get_property(&self, name: &str) -> EzValues {
        match name {
            "x" => EzValues::Usize(self.position.x.value),
            "y" => EzValues::Usize(self.position.y.value),
            "height" => EzValues::Usize(self.size.height.value),
            "width" => EzValues::Usize(self.size.width.value),
            "size_hint_x" => EzValues::SizeHint(self.size_hint.size_hint_x.value),
            "size_hint_y" => EzValues::SizeHint(self.size_hint.size_hint_y.value),
            "pos_hint_x" => EzValues::HorizontalPosHint(self.pos_hint.pos_hint_x.value),
            "pos_hint_y" => EzValues::VerticalPosHint(self.pos_hint.pos_hint_y.value),
            "auto_scale_width" => EzValues::Bool(self.auto_scale.auto_scale_width.value),
            "auto_scale_height" => EzValues::Bool(self.auto_scale.auto_scale_height.value),
            "padding_top" => EzValues::Usize(self.padding.padding_top.value),
            "padding_bottom" => EzValues::Usize(self.padding.padding_bottom.value),
            "padding_left" => EzValues::Usize(self.padding.padding_left.value),
            "padding_right" => EzValues::Usize(self.padding.padding_right.value),
            "halign" => EzValues::HorizontalAlignment(self.halign.value),
            "valign" => EzValues::VerticalAlignment(self.valign.value),
            "disabled" => EzValues::Bool(self.disabled.value),
            "selection_order" => EzValues::Usize(self.selection_order.value),
            "border" => EzValues::Bool(self.border_config.border.value),
            "horizontal_symbol" => {
                EzValues::String(self.border_config.horizontal_symbol.value.to_string())
            }
            "vertical_symbol" => {
                EzValues::String(self.border_config.vertical_symbol.value.to_string())
            }
            "top_left_symbol" => {
                EzValues::String(self.border_config.top_left_symbol.value.to_string())
            }
            "top_right_symbol" => {
                EzValues::String(self.border_config.top_right_symbol.value.to_string())
            }
            "bottom_left_symbol" => {
                EzValues::String(self.border_config.bottom_left_symbol.value.to_string())
            }
            "bottom_right_symbol" => {
                EzValues::String(self.border_config.bottom_right_symbol.value.to_string())
            }
            "fg_color" => EzValues::Color(self.colors.fg_color.value),
            "bg_color" => EzValues::Color(self.colors.bg_color.value),
            "selection_fg_color" => EzValues::Color(self.colors.selection_fg_color.value),
            "selection_bg_color" => EzValues::Color(self.colors.selection_bg_color.value),
            "disabled_fg_color" => EzValues::Color(self.colors.disabled_fg_color.value),
            "disabled_bg_color" => EzValues::Color(self.colors.disabled_bg_color.value),
            "border_fg_color" => EzValues::Color(self.colors.border_fg_color.value),
            "border_bg_color" => EzValues::Color(self.colors.border_bg_color.value),
            "choice" => EzValues::String(self.choice.value.to_string()),
            "allow_none" => EzValues::Bool(self.allow_none.value),
            _ => panic!("Invalid property name for button state: {}", name),
        }
    }

    fn update_property(&mut self, name: &str, value: EzValues) -> bool {
        match name {
            "x" => self.position.x.set_from_ez_value(value),
            "y" => self.position.y.set_from_ez_value(value),
            "height" => self.size.height.set_from_ez_value(value),
            "width" => self.size.width.set_from_ez_value(value),
            "size_hint_x" => self.size_hint.size_hint_x.set_from_ez_value(value),
            "size_hint_y" => self.size_hint.size_hint_y.set_from_ez_value(value),
            "pos_hint_x" => self.pos_hint.pos_hint_x.set_from_ez_value(value),
            "pos_hint_y" => self.pos_hint.pos_hint_y.set_from_ez_value(value),
            "auto_scale_width" => self.auto_scale.auto_scale_width.set_from_ez_value(value),
            "auto_scale_height" => self.auto_scale.auto_scale_height.set_from_ez_value(value),
            "padding_top" => self.padding.padding_top.set_from_ez_value(value),
            "padding_bottom" => self.padding.padding_bottom.set_from_ez_value(value),
            "padding_left" => self.padding.padding_left.set_from_ez_value(value),
            "padding_right" => self.padding.padding_right.set_from_ez_value(value),
            "halign" => self.halign.set_from_ez_value(value),
            "valign" => self.valign.set_from_ez_value(value),
            "disabled" => self.disabled.set_from_ez_value(value),
            "selection_order" => self.selection_order.set_from_ez_value(value),
            "border" => self.border_config.border.set_from_ez_value(value),
            "horizontal_symbol" => self
                .border_config
                .horizontal_symbol
                .set_from_ez_value(value),
            "vertical_symbol" => self.border_config.vertical_symbol.set_from_ez_value(value),
            "top_left_symbol" => self.border_config.top_left_symbol.set_from_ez_value(value),
            "top_right_symbol" => self.border_config.top_right_symbol.set_from_ez_value(value),
            "bottom_left_symbol" => self
                .border_config
                .bottom_left_symbol
                .set_from_ez_value(value),
            "bottom_right_symbol" => self
                .border_config
                .bottom_right_symbol
                .set_from_ez_value(value),
            "fg_color" => self.colors.fg_color.set_from_ez_value(value),
            "bg_color" => self.colors.bg_color.set_from_ez_value(value),
            "selection_fg_color" => self.colors.selection_fg_color.set_from_ez_value(value),
            "selection_bg_color" => self.colors.selection_bg_color.set_from_ez_value(value),
            "disabled_fg_color" => self.colors.disabled_fg_color.set_from_ez_value(value),
            "disabled_bg_color" => self.colors.disabled_bg_color.set_from_ez_value(value),
            "border_fg_color" => self.colors.border_fg_color.set_from_ez_value(value),
            "border_bg_color" => self.colors.border_bg_color.set_from_ez_value(value),
            "allow_none" => self.allow_none.set_from_ez_value(value),
            "choice" => self.choice.set_from_ez_value(value),
            _ => panic!("Invalid property name for dropdown state: {}", name),
        }
    }

    fn get_size_hint(&self) -> &SizeHint {
        &self.size_hint
    }
    fn get_size_hint_mut(&mut self) -> &mut SizeHint {
        &mut self.size_hint
    }

    fn get_pos_hint(&self) -> &PosHint {
        &self.pos_hint
    }

    fn get_pos_hint_mut(&mut self) -> &mut PosHint {
        &mut self.pos_hint
    }

    fn get_auto_scale(&self) -> &AutoScale {
        &self.auto_scale
    }

    fn get_auto_scale_mut(&mut self) -> &mut AutoScale {
        &mut self.auto_scale
    }

    fn get_size(&self) -> &StateSize {
        &self.size
    }

    fn get_size_mut(&mut self) -> &mut StateSize {
        &mut self.size
    }

    fn get_infinite_size(&self) -> &InfiniteSize {
        &self.infinite_size
    }

    fn get_infinite_size_mut(&mut self) -> &mut InfiniteSize {
        &mut self.infinite_size
    }

    fn get_position(&self) -> &StateCoordinates {
        &self.position
    }

    fn get_position_mut(&mut self) -> &mut StateCoordinates {
        &mut self.position
    }

    fn set_absolute_position(&mut self, pos: IsizeCoordinates) {
        self.absolute_position = pos
    }

    fn get_absolute_position(&self) -> IsizeCoordinates {
        self.absolute_position
    }

    fn set_halign(&mut self, alignment: HorizontalAlignment) {
        self.halign.set(alignment);
    }

    fn get_halign(&self) -> HorizontalAlignment {
        self.halign.value
    }

    fn set_valign(&mut self, alignment: VerticalAlignment) {
        self.valign.set(alignment);
    }

    fn get_valign(&self) -> VerticalAlignment {
        self.valign.value
    }

    fn get_padding(&self) -> &Padding {
        &self.padding
    }

    fn get_padding_mut(&mut self) -> &mut Padding {
        &mut self.padding
    }

    fn get_border_config(&self) -> &BorderConfig {
        &self.border_config
    }

    fn get_border_config_mut(&mut self) -> &mut BorderConfig {
        &mut self.border_config
    }

    fn get_color_config(&self) -> &ColorConfig {
        &self.colors
    }

    fn get_color_config_mut(&mut self) -> &mut ColorConfig {
        &mut self.colors
    }

    fn is_selectable(&self) -> bool {
        true
    }

    fn set_disabled(&mut self, disabled: bool) {
        self.disabled.set(disabled);
    }

    fn get_disabled(&self) -> bool {
        self.disabled.value
    }

    fn get_selection_order(&self) -> usize {
        self.selection_order.value
    }

    fn set_selection_order(&mut self, order: usize) {
        self.selection_order.set(order);
    }

    fn set_selected(&mut self, state: bool) {
        self.selected = state;
    }

    fn get_selected(&self) -> bool {
        self.selected
    }

    fn copy_state_values(&mut self, other: EzState) {
        let other = other.as_dropdown();
        self.position.x.copy_from(&other.position.x);
        self.position.y.copy_from(&other.position.y);
        self.size.height.copy_from(&other.size.height);
        self.size.width.copy_from(&other.size.width);
        self.size_hint
            .size_hint_x
            .copy_from(&other.size_hint.size_hint_x);
        self.size_hint
            .size_hint_y
            .copy_from(&other.size_hint.size_hint_y);
        self.pos_hint
            .pos_hint_x
            .copy_from(&other.pos_hint.pos_hint_x);
        self.pos_hint
            .pos_hint_y
            .copy_from(&other.pos_hint.pos_hint_y);
        self.auto_scale
            .auto_scale_width
            .copy_from(&other.auto_scale.auto_scale_width);
        self.auto_scale
            .auto_scale_height
            .copy_from(&other.auto_scale.auto_scale_height);
        self.padding
            .padding_top
            .copy_from(&other.padding.padding_top);
        self.padding
            .padding_bottom
            .copy_from(&other.padding.padding_bottom);
        self.padding
            .padding_left
            .copy_from(&other.padding.padding_left);
        self.padding
            .padding_right
            .copy_from(&other.padding.padding_right);
        self.padding
            .padding_right
            .copy_from(&other.padding.padding_right);
        self.halign.copy_from(&other.halign);
        self.valign.copy_from(&other.valign);
        self.disabled.copy_from(&other.disabled);
        self.selection_order.copy_from(&other.selection_order);
        self.border_config
            .border
            .copy_from(&other.border_config.border);
        self.border_config
            .horizontal_symbol
            .copy_from(&other.border_config.horizontal_symbol);
        self.border_config
            .vertical_symbol
            .copy_from(&other.border_config.vertical_symbol);
        self.border_config
            .top_left_symbol
            .copy_from(&other.border_config.top_left_symbol);
        self.border_config
            .top_right_symbol
            .copy_from(&other.border_config.top_right_symbol);
        self.border_config
            .bottom_left_symbol
            .copy_from(&other.border_config.bottom_left_symbol);
        self.border_config
            .bottom_right_symbol
            .copy_from(&other.border_config.bottom_right_symbol);
        self.colors.fg_color.copy_from(&other.colors.fg_color);
        self.colors.bg_color.copy_from(&other.colors.bg_color);
        self.colors
            .selection_fg_color
            .copy_from(&other.colors.selection_fg_color);
        self.colors
            .selection_bg_color
            .copy_from(&other.colors.selection_bg_color);
        self.colors
            .disabled_fg_color
            .copy_from(&other.colors.disabled_fg_color);
        self.colors
            .disabled_bg_color
            .copy_from(&other.colors.disabled_bg_color);
        self.colors
            .border_fg_color
            .copy_from(&other.colors.border_fg_color);
        self.colors
            .border_bg_color
            .copy_from(&other.colors.border_bg_color);
        self.colors
            .cursor_color
            .copy_from(&other.colors.cursor_color);
        self.choice.copy_from(&other.choice);
        self.allow_none.copy_from(&other.allow_none);
        self.options = other.options.clone();
    }

    fn clean_up_properties(&self, scheduler: &mut SchedulerFrontend) {
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
    pub fn set_choice(&mut self, choice: &str) {
        self.choice.set(choice.to_string());
    }

    pub fn get_choice(&self) -> String {
        self.choice.value.clone()
    }

    pub fn set_options(&mut self, options: Vec<String>) {
        self.options = options
    }

    pub fn get_options(&self) -> Vec<String> {
        self.options.clone()
    }

    pub fn set_allow_none(&mut self, allow_none: bool) {
        self.allow_none.set(allow_none);
    }

    pub fn get_allow_none(&self) -> bool {
        self.allow_none.value
    }

    /// Return the total amount of options in this dropdown including the empty option if it is
    /// allowed.
    pub fn total_options(&self) -> usize {
        self.options.len() + if self.allow_none.value { 1 } else { 0 }
    }
}

/// [State] implementation.
#[derive(Clone, Debug)]
pub struct DroppedDownMenuState {
    /// Path to the widget to which this state belongs
    pub path: String,

    /// Widget path of the [Dropdown] that spawned this menu.
    pub parent_path: String,

    /// Position of this widget relative to its' parent [layout]
    pub position: StateCoordinates,

    /// Absolute position of this widget on screen. Automatically propagated, do not set manually
    pub absolute_position: IsizeCoordinates,

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
    pub fn new(path: String, scheduler: &mut SchedulerFrontend) -> Self {
        DroppedDownMenuState {
            path: path.clone(),
            parent_path: String::new(),
            position: StateCoordinates::new(0, 0, path.clone(), scheduler),
            absolute_position: IsizeCoordinates::default(),
            size_hint: SizeHint::new(Some(1.0), Some(1.0), path.clone(), scheduler),
            auto_scale: AutoScale::new(false, false, path.clone(), scheduler),
            pos_hint: PosHint::new(None, None, path.clone(), scheduler),
            size: StateSize::new(0, 3, path.clone(), scheduler),
            padding: Padding::new(0, 0, 0, 0, path.clone(), scheduler),
            halign: scheduler.new_horizontal_alignment_property(
                format!("{}/halign", path).as_str(),
                HorizontalAlignment::Left,
            ),
            valign: scheduler.new_vertical_alignment_property(
                format!("{}/valign", path).as_str(),
                VerticalAlignment::Top,
            ),
            options: Vec::new(),
            allow_none: scheduler.new_bool_property(format!("{}/allow_none", path).as_str(), true),
            choice: scheduler
                .new_string_property(format!("{}/choice", path).as_str(), String::new()),
            border_config: BorderConfig::new(false, path.clone(), scheduler),
            colors: ColorConfig::new(path.clone(), scheduler),
            disabled: scheduler.new_bool_property(format!("{}/disabled", path).as_str(), false),
            selection_order: scheduler
                .new_usize_property(format!("{}/selection_order", path).as_str(), 0),
            dropped_down_selected_row: 0,
            infinite_size: InfiniteSize::default(),
        }
    }
}

impl GenericState for DroppedDownMenuState {
    fn get_path(&self) -> &String {
        &self.path
    }

    fn get_property(&self, _name: &str) -> EzValues {
        todo!()
    }

    fn update_property(&mut self, _name: &str, _value: EzValues) -> bool {
        false
    }

    fn get_size_hint(&self) -> &SizeHint {
        &self.size_hint
    }

    fn get_size_hint_mut(&mut self) -> &mut SizeHint {
        &mut self.size_hint
    }

    fn get_pos_hint(&self) -> &PosHint {
        &self.pos_hint
    }

    fn get_pos_hint_mut(&mut self) -> &mut PosHint {
        &mut self.pos_hint
    }

    fn get_auto_scale(&self) -> &AutoScale {
        &self.auto_scale
    }

    fn get_auto_scale_mut(&mut self) -> &mut AutoScale {
        &mut self.auto_scale
    }

    fn get_size(&self) -> &StateSize {
        &self.size
    }

    fn get_size_mut(&mut self) -> &mut StateSize {
        &mut self.size
    }

    fn get_infinite_size(&self) -> &InfiniteSize {
        &self.infinite_size
    }

    fn get_infinite_size_mut(&mut self) -> &mut InfiniteSize {
        &mut self.infinite_size
    }

    fn get_position(&self) -> &StateCoordinates {
        &self.position
    }

    fn get_position_mut(&mut self) -> &mut StateCoordinates {
        &mut self.position
    }

    fn set_absolute_position(&mut self, pos: IsizeCoordinates) {
        self.absolute_position = pos
    }

    fn get_absolute_position(&self) -> IsizeCoordinates {
        self.absolute_position
    }

    fn set_halign(&mut self, alignment: HorizontalAlignment) {
        self.halign.set(alignment);
    }

    fn get_halign(&self) -> HorizontalAlignment {
        self.halign.value
    }

    fn set_valign(&mut self, alignment: VerticalAlignment) {
        self.valign.set(alignment);
    }

    fn get_valign(&self) -> VerticalAlignment {
        self.valign.value
    }

    fn get_padding(&self) -> &Padding {
        &self.padding
    }

    fn get_padding_mut(&mut self) -> &mut Padding {
        &mut self.padding
    }

    fn get_border_config(&self) -> &BorderConfig {
        &self.border_config
    }

    fn get_border_config_mut(&mut self) -> &mut BorderConfig {
        &mut self.border_config
    }

    fn get_color_config(&self) -> &ColorConfig {
        &self.colors
    }

    fn get_color_config_mut(&mut self) -> &mut ColorConfig {
        &mut self.colors
    }

    fn set_disabled(&mut self, disabled: bool) {
        self.disabled.set(disabled);
    }

    fn get_disabled(&self) -> bool {
        self.disabled.value
    }

    fn get_selection_order(&self) -> usize {
        self.selection_order.value
    }

    fn set_selection_order(&mut self, order: usize) {
        self.selection_order.set(order);
    }

    fn copy_state_values(&mut self, _other: EzState) {}

    fn clean_up_properties(&self, scheduler: &mut SchedulerFrontend) {
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
    pub fn create(
        path: String,
        size: StateSize,
        infinite_size: InfiniteSize,
        auto_scale: AutoScale,
        options: Vec<String>,
        allow_none: EzProperty<bool>,
        size_hint: SizeHint,
        position: StateCoordinates,
        padding: Padding,
        halign: EzProperty<HorizontalAlignment>,
        valign: EzProperty<VerticalAlignment>,
        disabled: EzProperty<bool>,
        selection_order: EzProperty<usize>,
        absolute_position: IsizeCoordinates,
        pos_hint: PosHint,
        dropped_down_selected_row: usize,
        border_config: BorderConfig,
        colors: ColorConfig,
        choice: EzProperty<String>,
        parent_path: String,
    ) -> Self {
        DroppedDownMenuState {
            path,
            size,
            infinite_size,
            auto_scale,
            options,
            allow_none,
            size_hint,
            position,
            padding,
            halign,
            valign,
            disabled,
            selection_order,
            absolute_position,
            pos_hint,
            dropped_down_selected_row,
            border_config,
            colors,
            choice,
            parent_path,
        }
    }

    pub fn set_choice(&mut self, choice: String) {
        self.choice.set(choice);
    }

    pub fn get_choice(&self) -> String {
        self.choice.value.clone()
    }

    pub fn set_options(&mut self, options: Vec<String>) {
        self.options = options
    }

    pub fn get_options(&self) -> Vec<String> {
        self.options.clone()
    }

    pub fn set_allow_none(&mut self, allow_none: bool) {
        self.allow_none.set(allow_none);
    }

    pub fn get_allow_none(&self) -> bool {
        self.allow_none.value
    }

    pub fn set_dropped_down_selected_row(&mut self, dropped_down_selected_row: usize) {
        self.dropped_down_selected_row = dropped_down_selected_row;
    }

    pub fn get_dropped_down_selected_row(&self) -> usize {
        self.dropped_down_selected_row
    }

    /// Return the total amount of options in this dropdown including the empty option if it is
    /// allowed.
    pub fn total_options(&self) -> usize {
        self.options.len() + if self.allow_none.value { 1 } else { 0 }
    }

    /// Get an ordered list of options, including the empty option if it was allowed. Order is:
    /// - Active choice
    /// - Empty (if allowed)
    /// - Rest of the options in user defined order
    pub fn get_dropped_down_options(&self) -> Vec<String> {
        let mut options = vec![self.choice.value.clone()];
        if self.allow_none.value && !self.choice.value.is_empty() {
            options.push("".to_string())
        }
        for option in self
            .options
            .iter()
            .filter(|x| x.to_string() != self.choice.value)
        {
            options.push(option.to_string());
        }
        options
    }
}
