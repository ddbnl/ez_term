use crate::property::ez_property::EzProperty;
use crate::property::ez_values::EzValues;
use crate::run::definitions::IsizeCoordinates;
use crate::scheduler::scheduler::SchedulerFrontend;
use crate::scheduler::scheduler_funcs::clean_up_property;
use crate::states::definitions::{AutoScale, BorderConfig, ColorConfig, HorizontalAlignment, InfiniteSize, Padding, PosHint, SizeHint, StateCoordinates, StateSize, VerticalAlignment};
use crate::states::ez_state::{EzState, GenericState};


/// [State] implementation for [Button].
#[derive(Clone, Debug)]
pub struct ButtonState {

    /// Path to the widget to which this state belongs
    pub path: String,

    /// Text currently being displayed by the label
    pub text: EzProperty<String>,

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

    /// Bool representing whether this widget is currently displaying it's flash color.
    flashing: bool,
}
impl ButtonState {
    
    pub fn new(path: String, scheduler: &mut SchedulerFrontend) -> Self {

        ButtonState {
            path: path.clone(),
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
            text: scheduler.new_string_property(format!("{}/text", path).as_str(),
                                                String::new()),
            disabled: scheduler.new_bool_property(
                 format!("{}/disabled", path).as_str(),false),
            selected: false,
            selection_order: scheduler.new_usize_property(
                 format!("{}/selection_order", path).as_str(), 0),
            flashing: false,
            border_config: BorderConfig::new(true, path.clone(), scheduler),
            colors: ColorConfig::new(path, scheduler),
        }
    }
}
impl GenericState for ButtonState {

    fn get_path(&self) -> &String { &self.path }

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
            "horizontal_symbol" => self.border_config.horizontal_symbol.set_from_ez_value(value),
            "vertical_symbol" => self.border_config.vertical_symbol.set_from_ez_value(value),
            "top_left_symbol" => self.border_config.top_left_symbol.set_from_ez_value(value),
            "top_right_symbol" => self.border_config.top_right_symbol.set_from_ez_value(value),
            "bottom_left_symbol" => self.border_config.bottom_left_symbol.set_from_ez_value(value),
            "bottom_right_symbol" => self.border_config.bottom_right_symbol.set_from_ez_value(value),
            "fg_color" => self.colors.fg_color.set_from_ez_value(value),
            "bg_color" => self.colors.bg_color.set_from_ez_value(value),
            "selection_fg_color" => self.colors.selection_fg_color.set_from_ez_value(value),
            "selection_bg_color" => self.colors.selection_bg_color.set_from_ez_value(value),
            "disabled_fg_color" => self.colors.disabled_fg_color.set_from_ez_value(value),
            "disabled_bg_color" => self.colors.disabled_bg_color.set_from_ez_value(value),
            "flash_fg_color" => self.colors.flash_fg_color.set_from_ez_value(value),
            "flash_bg_color" => self.colors.flash_bg_color.set_from_ez_value(value),
            "active_fg_color" => self.colors.active_fg_color.set_from_ez_value(value),
            "active_bg_color" => self.colors.active_bg_color.set_from_ez_value(value),
            "border_fg_color" => self.colors.border_fg_color.set_from_ez_value(value),
            "border_bg_color" => self.colors.border_bg_color.set_from_ez_value(value),
            "text" => self.text.set_from_ez_value(value),
            _ => panic!("Invalid property name for button state: {}", name),
        }
    }

    fn copy_state_values(&mut self, other: EzState) {

        let other = other.as_button();
        self.position.x.value = other.position.x.value;
        self.position.y.value = other.position.y.value;
        self.size.height.value = other.size.height.value;
        self.size.width.value = other.size.width.value;
        self.size_hint.size_hint_x.value = other.size_hint.size_hint_x.value;
        self.size_hint.size_hint_y.value = other.size_hint.size_hint_y.value;
        self.pos_hint.pos_hint_x.value = other.pos_hint.pos_hint_x.value;
        self.pos_hint.pos_hint_y.value = other.pos_hint.pos_hint_y.value;
        self.auto_scale.auto_scale_width.value = other.auto_scale.auto_scale_width.value;
        self.auto_scale.auto_scale_height.value = other.auto_scale.auto_scale_height.value;
        self.padding.padding_top.value = other.padding.padding_top.value;
        self.padding.padding_bottom.value = other.padding.padding_bottom.value;
        self.padding.padding_left.value = other.padding.padding_left.value;
        self.padding.padding_right.value = other.padding.padding_right.value;
        self.padding.padding_right.value = other.padding.padding_right.value;
        self.halign.value = other.halign.value;
        self.valign.value = other.valign.value;
        self.disabled.value = other.disabled.value;
        self.selection_order.value = other.selection_order.value;
        self.border_config.border.value = other.border_config.border.value;
        self.border_config.horizontal_symbol.value = other.border_config.horizontal_symbol.value.clone();
        self.border_config.vertical_symbol.value = other.border_config.vertical_symbol.value.clone();
        self.border_config.top_left_symbol.value = other.border_config.top_left_symbol.value.clone();
        self.border_config.top_right_symbol.value = other.border_config.top_right_symbol.value.clone();
        self.border_config.bottom_left_symbol.value = other.border_config.bottom_left_symbol.value.clone();
        self.border_config.bottom_right_symbol.value = other.border_config.bottom_right_symbol.value.clone();
        self.colors.fg_color.value = other.colors.fg_color.value;
        self.colors.bg_color.value = other.colors.bg_color.value;
        self.colors.selection_fg_color.value = other.colors.selection_fg_color.value;
        self.colors.selection_bg_color.value = other.colors.selection_bg_color.value;
        self.colors.disabled_fg_color.value = other.colors.disabled_fg_color.value;
        self.colors.disabled_bg_color.value = other.colors.disabled_bg_color.value;
        self.colors.border_fg_color.value = other.colors.border_fg_color.value;
        self.colors.border_bg_color.value = other.colors.border_bg_color.value;
        self.colors.cursor_color.value = other.colors.cursor_color.value;
        self.text.value = other.text.value.clone();
    }

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

    fn set_halign(&mut self, alignment: HorizontalAlignment) {
        self.halign.set(alignment);
    }

    fn get_halign(&self) -> HorizontalAlignment { self.halign.value }

    fn set_valign(&mut self, alignment: VerticalAlignment) {
        self.valign.set(alignment);
    }

    fn get_valign(&self) -> VerticalAlignment { self.valign.value }

    fn get_padding(&self) -> &Padding { &self.padding }

    fn get_padding_mut(&mut self) -> &mut Padding { &mut self.padding }

    fn get_border_config(&self) -> &BorderConfig { &self.border_config  }

    fn get_border_config_mut(&mut self) -> &mut BorderConfig { &mut self.border_config }

    fn get_color_config(&self) -> &ColorConfig { &self.colors }

    fn get_color_config_mut(&mut self) -> &mut ColorConfig { &mut self.colors }

    fn is_selectable(&self) -> bool { true }

    fn set_disabled(&mut self, disabled: bool) { self.disabled.set(disabled); }

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
        clean_up_property(scheduler, &self.text.name);
        clean_up_property(scheduler, &self.disabled.name);
        clean_up_property(scheduler, &self.selection_order.name);
        self.border_config.clean_up_properties(scheduler);
        self.colors.clean_up_properties(scheduler);
    }
}
impl ButtonState {

    pub fn get_text(&self) -> String { self.text.value.clone() }

    pub fn set_text(&mut self, text: String) { self.text.set(text); }

    pub fn set_flashing(&mut self, flashing: bool) { self.flashing = flashing; }

    pub fn get_flashing(&self) -> bool { self.flashing }

}