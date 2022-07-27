use crossterm::style::Color;
use crate::property::ez_property::EzProperty;
use crate::run::definitions::{Coordinates, IsizeCoordinates};
use crate::scheduler::scheduler::SchedulerFrontend;
use crate::scheduler::scheduler_funcs::clean_up_property;
use crate::states::definitions::{StateCoordinates, SizeHint, PosHint, StateSize, AutoScale, Padding,
                                 HorizontalAlignment, VerticalAlignment, BorderConfig, ColorConfig};
use crate::states::ez_state::GenericState;


/// [State] implementation.
#[derive(Clone, Debug)]
pub struct TextInputState {

    /// Path to the widget to which this state belongs
    pub path: String,

    /// Text currently being displayed by the text input
    text: EzProperty<String>,

    /// Position of this widget relative to its' parent [layout]
    position: StateCoordinates,

    /// Absolute position of this widget on screen. Automatically propagated, do not set manually
    absolute_position: IsizeCoordinates,

    /// size of this widget
    size: StateSize,

    /// Relative height/width of this widget to parent layout
    size_hint: SizeHint,

    /// Pos hint of this widget
    pos_hint: PosHint,

    /// Automatically adjust size of widget to content
    auto_scale: AutoScale,

    /// Amount of space to leave between sides of the widget and other widgets
    padding: Padding,

    /// Horizontal alignment of this widget
    halign: EzProperty<HorizontalAlignment>,

    /// Vertical alignment of this widget
    valign: EzProperty<VerticalAlignment>,

    /// How many characters [text] may hold
    max_length: EzProperty<usize>,

    /// [BorderConfig] object that will be used to draw the border if enabled
    border_config: BorderConfig,

    /// Object containing colors to be used by this widget in different situations
    colors: ColorConfig,

    /// Bool representing whether widget is disabled, i.e. cannot be interacted with
    disabled: EzProperty<bool>,

    /// Global order number in which this widget will be selection when user presses down/up keys
    selection_order: EzProperty<usize>,

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
        state.colors.set_background(Color::Blue);
        state
    }
}


impl GenericState for TextInputState {

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
    
    pub fn set_text(&mut self, text: String) { self.get_text_mut().set(text) }

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

