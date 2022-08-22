use crossterm::style::Color;
use crate::property::ez_property::EzProperty;
use crate::property::ez_values::EzValues;
use crate::run::definitions::{Coordinates, IsizeCoordinates};
use crate::scheduler::scheduler::SchedulerFrontend;
use crate::scheduler::scheduler_funcs::clean_up_property;
use crate::states::definitions::{StateCoordinates, SizeHint, PosHint, StateSize, AutoScale, Padding, HorizontalAlignment, VerticalAlignment, BorderConfig, ColorConfig, InfiniteSize};
use crate::states::ez_state::{EzState, GenericState};


/// [State] implementation.
#[derive(Clone, Debug)]
pub struct TextInputState {

    /// Path to the widget to which this state belongs
    pub path: String,

    /// Text currently being displayed by the text input
    pub text: EzProperty<String>,

    /// Position of this widget relative to its' parent [layout]
    pub position: StateCoordinates,

    /// Absolute position of this widget on screen. Automatically propagated, do not set manually
    absolute_position: IsizeCoordinates,

    /// size of this widget
    pub size: StateSize,

    /// Infinite size of this widget for x and y axes, used in scrolling
    infinite_size: InfiniteSize,

    /// Relative height/width of this widget to parent layout
    pub size_hint: SizeHint,

    /// Pos hint of this widget
    pub pos_hint: PosHint,

    /// Automatically adjust size of widget to content
    pub auto_scale: AutoScale,

    /// Amount of space to leave between sides of the widget and other widgets
    pub padding: Padding,

    /// Horizontal alignment of this widget
    pub halign: EzProperty<HorizontalAlignment>,

    /// Vertical alignment of this widget
    pub valign: EzProperty<VerticalAlignment>,

    /// How many characters [text] may hold
    pub max_length: EzProperty<usize>,

    /// [BorderConfig] object that will be used to draw the border if enabled
    pub border_config: BorderConfig,

    /// Object containing colors to be used by this widget in different situations
    pub colors: ColorConfig,

    /// Bool representing whether widget is disabled, i.e. cannot be interacted with
    pub disabled: EzProperty<bool>,

    /// Global order number in which this widget will be selection when user presses down/up keys
    pub selection_order: EzProperty<usize>,

    /// Bool representing whether this widget is currently selected. Internal only.
    selected: bool,

    /// Position of cursor relative to this widget. Internal only
    cursor_pos: Coordinates,

    /// Bool representing whether we have a blinking scheduled task running. Internal only
    active_blink_task: bool,

    /// Switch for blinking. When true displays [cursor_color] on the [cursor_pos]. Internal only
    blink_switch: bool,

    /// If text is larger than the widget, only a part of the text can be displayed. This is the
    /// index of where to start viewing the text. Internal only.
    view_start: usize,
}
impl TextInputState {

    pub fn new(path: String, scheduler: &mut SchedulerFrontend) -> Self {

        let mut state = TextInputState {
            path: path.clone(),
            position: StateCoordinates::new(0, 0, path.clone(), scheduler),
            absolute_position: IsizeCoordinates::default(),
            size: StateSize::new(0, 0, path.clone(), scheduler),
            infinite_size: InfiniteSize::default(),
            size_hint: SizeHint::new(Some(1.0), Some(1.0), path.clone(), scheduler),
            auto_scale: AutoScale::new(false, false, path.clone(), scheduler),
            pos_hint: PosHint::new(None, None, path.clone(), scheduler),
            padding: Padding::new(0, 0, 0, 0, path.clone(), scheduler),
            halign: scheduler.new_horizontal_alignment_property(
                format!("{}/halign", path).as_str(), HorizontalAlignment::Left),
            valign: scheduler.new_vertical_alignment_property(
                format!("{}/valign", path).as_str(), VerticalAlignment::Top),
            cursor_pos: Coordinates::default(),
            active_blink_task: false,
            blink_switch: false,
            view_start: 0,
            disabled: scheduler.new_bool_property(
                format!("{}/disabled", path).as_str(),false),
            selected: false,
            selection_order: scheduler.new_usize_property(
                format!("{}/selection_order", path).as_str(), 0),
            text: scheduler.new_string_property(format!("{}/text", path).as_str(),
                                                String::new()),
            max_length: scheduler.new_usize_property(
                format!("{}/max_length", path).as_str(), 0),
            border_config: BorderConfig::new(false, path.clone(), scheduler),
            colors: ColorConfig::new(path, scheduler),
        };
        state.colors.set_bg_color(Color::Blue);
        state
    }
}


impl GenericState for TextInputState {

    fn get_path(&self) -> &String { &self.path }

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
            "horizontal_symbol" => EzValues::String(self.border_config.horizontal_symbol.value.to_string()),
            "vertical_symbol" => EzValues::String(self.border_config.vertical_symbol.value.to_string()),
            "top_left_symbol" => EzValues::String(self.border_config.top_left_symbol.value.to_string()),
            "top_right_symbol" => EzValues::String(self.border_config.top_right_symbol.value.to_string()),
            "bottom_left_symbol" => EzValues::String(self.border_config.bottom_left_symbol.value.to_string()),
            "bottom_right_symbol" => EzValues::String(self.border_config.bottom_right_symbol.value.to_string()),
            "fg_color" => EzValues::Color(self.colors.fg_color.value),
            "bg_color" => EzValues::Color(self.colors.bg_color.value),
            "selection_fg_color" => EzValues::Color(self.colors.selection_fg_color.value),
            "selection_bg_color" => EzValues::Color(self.colors.selection_bg_color.value),
            "disabled_fg_color" => EzValues::Color(self.colors.disabled_fg_color.value),
            "disabled_bg_color" => EzValues::Color(self.colors.disabled_bg_color.value),
            "border_fg_color" => EzValues::Color(self.colors.border_fg_color.value),
            "border_bg_color" => EzValues::Color(self.colors.border_bg_color.value),
            "cursor_color" => EzValues::Color(self.colors.cursor_color.value),
            "text" => EzValues::String(self.text.value.to_string()),
            "max_length" => EzValues::Usize(self.max_length.value),
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
            "border_fg_color" => self.colors.border_fg_color.set_from_ez_value(value),
            "border_bg_color" => self.colors.border_bg_color.set_from_ez_value(value),
            "cursor_color" => self.colors.cursor_color.set_from_ez_value(value),
            "text" => self.text.set_from_ez_value(value),
            "max_length" => self.max_length.set_from_ez_value(value),
            _ => panic!("Invalid property name for text input state: {}", name),
        }
    }

    fn copy_state_values(&mut self, other: EzState) {

        let other = other.as_text_input();
        self.position.x.copy_from(&other.position.x);
        self.position.y.copy_from(&other.position.y);
        self.size.height.copy_from(&other.size.height);
        self.size.width.copy_from(&other.size.width);
        self.size_hint.size_hint_x.copy_from(&other.size_hint.size_hint_x);
        self.size_hint.size_hint_y.copy_from(&other.size_hint.size_hint_y);
        self.pos_hint.pos_hint_x.copy_from(&other.pos_hint.pos_hint_x);
        self.pos_hint.pos_hint_y.copy_from(&other.pos_hint.pos_hint_y);
        self.auto_scale.auto_scale_width.copy_from(&other.auto_scale.auto_scale_width);
        self.auto_scale.auto_scale_height.copy_from(&other.auto_scale.auto_scale_height);
        self.padding.padding_top.copy_from(&other.padding.padding_top);
        self.padding.padding_bottom.copy_from(&other.padding.padding_bottom);
        self.padding.padding_left.copy_from(&other.padding.padding_left);
        self.padding.padding_right.copy_from(&other.padding.padding_right);
        self.padding.padding_right.copy_from(&other.padding.padding_right);
        self.halign.copy_from(&other.halign);
        self.valign.copy_from(&other.valign);
        self.disabled.copy_from(&other.disabled);
        self.selection_order.copy_from(&other.selection_order);
        self.border_config.border.copy_from(&other.border_config.border);
        self.border_config.horizontal_symbol.copy_from(&other.border_config.horizontal_symbol);
        self.border_config.vertical_symbol.copy_from(&other.border_config.vertical_symbol);
        self.border_config.top_left_symbol.copy_from(&other.border_config.top_left_symbol);
        self.border_config.top_right_symbol.copy_from(&other.border_config.top_right_symbol);
        self.border_config.bottom_left_symbol.copy_from(&other.border_config.bottom_left_symbol);
        self.border_config.bottom_right_symbol.copy_from(&other.border_config.bottom_right_symbol);
        self.colors.fg_color.copy_from(&other.colors.fg_color);
        self.colors.bg_color.copy_from(&other.colors.bg_color);
        self.colors.selection_fg_color.copy_from(&other.colors.selection_fg_color);
        self.colors.selection_bg_color.copy_from(&other.colors.selection_bg_color);
        self.colors.disabled_fg_color.copy_from(&other.colors.disabled_fg_color);
        self.colors.disabled_bg_color.copy_from(&other.colors.disabled_bg_color);
        self.colors.border_fg_color.copy_from(&other.colors.border_fg_color);
        self.colors.border_bg_color.copy_from(&other.colors.border_bg_color);
        self.colors.cursor_color.copy_from(&other.colors.cursor_color);
        self.text.copy_from(&other.text);
        self.max_length.copy_from(&other.max_length);
    }
    
    fn get_size_hint(&self) -> &SizeHint { &self.size_hint }

    fn get_size_hint_mut(&mut self) -> &mut SizeHint { &mut self.size_hint }

    fn get_pos_hint(&self) -> &PosHint { &self.pos_hint }

    fn get_pos_hint_mut(&mut self) -> &mut PosHint { &mut self.pos_hint }

    fn get_auto_scale(&self) -> &AutoScale { &self.auto_scale }

    fn get_auto_scale_mut(&mut self) -> &mut AutoScale { &mut self.auto_scale }

    fn get_size(&self) -> &StateSize { &self.size  }

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

    fn get_border_config_mut(&mut self) -> &mut BorderConfig {  &mut self.border_config  }

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
        clean_up_property(scheduler, &self.max_length.name);
        clean_up_property(scheduler, &self.disabled.name);
        clean_up_property(scheduler, &self.selection_order.name);
        self.border_config.clean_up_properties(scheduler);
        self.colors.clean_up_properties(scheduler);
    }
}
impl TextInputState {

    pub fn get_text(&self) -> String { self.text.value.clone() }

    pub fn get_text_mut(&mut self) -> &mut EzProperty<String> { &mut self.text }
    
    pub fn set_text(&mut self, text: String) { self.get_text_mut().set(text); }

    pub fn set_cursor_pos(&mut self, cursor_pos: Coordinates) { self.cursor_pos = cursor_pos; }

    pub fn set_cursor_x(&mut self, pos: usize) { self.cursor_pos.x = pos; }

    pub fn set_cursor_y(&mut self, pos: usize) { self.cursor_pos.y = pos; }

    pub fn get_cursor_pos(&self) -> Coordinates { self.cursor_pos }

    pub fn set_active_blink_task(&mut self, active: bool) { self.active_blink_task = active; }

    pub fn get_active_blink_task(&self) -> bool { self.active_blink_task }

    pub fn set_blink_switch(&mut self, active: bool) { self.blink_switch = active; }

    pub fn get_blink_switch(&self) -> bool { self.blink_switch }

    pub fn set_view_start(&mut self, view_start: usize) { self.view_start = view_start; }

    pub fn get_view_start(&self) -> usize { self.view_start }

    pub fn set_max_length(&mut self, max_length: usize) { self.max_length.set(max_length); }

    pub fn get_max_length(&self) -> usize { self.max_length.value }

}

