use crate::states::state::GenericState;
use crate::states;


/// [State] implementation.
#[derive(Clone)]
pub struct DropdownState {

    /// Position of this widget relative to its' parent [Layout]
    pub position: states::definitions::Coordinates,

    /// Absolute position of this widget on screen. Automatically propagated, do not set manually
    pub absolute_position: states::definitions::Coordinates,

    /// Relative height/width of this widget to parent layout
    pub size_hint: states::definitions::SizeHint,

    /// Pos hint of this widget
    pub pos_hint: states::definitions::PosHint,

    /// size of this widget
    pub size: states::definitions::Size,

    /// Automatically adjust size of widget to content
    pub auto_scale: states::definitions::AutoScale,

    /// Amount of space to leave between sides of the widget and other widgets
    pub padding: states::definitions::Padding,

    /// Horizontal alignment of this widget
    pub halign: states::definitions::HorizontalAlignment,

    /// Vertical alignment of this widget
    pub valign: states::definitions::VerticalAlignment,

    /// Bool representing whether this widget is currently focussed. If so, it gets the first
    /// chance to consume all events
    pub focussed: bool,

    /// Global order number in which this widget will be selection when user presses down/up keys
    pub selection_order: usize,
    
    /// Bool representing whether this widget is currently selected.
    pub selected: bool,

    /// List of options this dropdown will display
    pub options: Vec<String>,

    /// Bool representing whether an empty value should be shown to choose from
    pub allow_none: bool,

    /// The currently active choice of the dropdown.
    pub choice: String,

    /// [BorderConfig] object that will be used to draw the border if enabled
    pub border_config: states::definitions::BorderConfig,

    /// Object containing colors to be used by this widget in different situations
    pub colors: states::definitions::ColorConfig,

    /// Bool representing if state has changed. Triggers widget redraw.
    pub changed: bool,

    /// If true this forces a global screen redraw on the next frame. Screen redraws are diffed
    /// so this can be called when needed without degrading performance. If only screen positions
    /// that fall within this widget must be redrawn, call [EzObject.redraw] instead.
    pub force_redraw: bool,
}
impl Default for DropdownState {
    fn default() -> Self {
       DropdownState {
           position: states::definitions::Coordinates::default(),
           absolute_position: states::definitions::Coordinates::default(),
           size_hint: states::definitions::SizeHint::default(),
           auto_scale: states::definitions::AutoScale::default(),
           pos_hint: states::definitions::PosHint::default(),
           size: states::definitions::Size::new(0, 3),
           padding: states::definitions::Padding::default(),
           halign: states::definitions::HorizontalAlignment::Left,
           valign: states::definitions::VerticalAlignment::Top,
           focussed: false,
           selected: false,
           selection_order: 0,
           options: Vec::new(),
           allow_none: true,
           choice: String::new(),
           border_config: states::definitions::BorderConfig::default(),
           colors: states::definitions::ColorConfig::default(),
           changed: false,
           force_redraw: false
       }
    }
}
impl GenericState for DropdownState {

    fn set_changed(&mut self, changed: bool) { self.changed = changed }

    fn get_changed(&self) -> bool { self.changed }

    fn set_size_hint(&mut self, size_hint: states::definitions::SizeHint) {
        if self.size_hint != size_hint { self.changed = true }
        self.size_hint = size_hint;
    }

    fn get_size_hint(&self) -> &states::definitions::SizeHint { &self.size_hint }

    fn set_pos_hint(&mut self, pos_hint: states::definitions::PosHint) {
        if self.pos_hint != pos_hint { self.changed = true }
        self.pos_hint = pos_hint;
    }

    fn get_pos_hint(&self) -> &states::definitions::PosHint { &self.pos_hint }

    fn set_auto_scale(&mut self, auto_scale: states::definitions::AutoScale) {
        if self.auto_scale != auto_scale { self.changed = true }
        self.auto_scale = auto_scale;
    }

    fn get_auto_scale(&self) -> &states::definitions::AutoScale { &self.auto_scale }

    fn set_size(&mut self, size: states::definitions::Size) { self.size = size; }

    fn get_size(&self) -> &states::definitions::Size { &self.size }

    fn get_size_mut(&mut self) -> &mut states::definitions::Size { &mut self.size }

    fn set_position(&mut self, position: states::definitions::Coordinates) { self.position = position; }

    fn get_position(&self) -> states::definitions::Coordinates { self.position }

    fn set_absolute_position(&mut self, pos: states::definitions::Coordinates) { self.absolute_position = pos }

    fn get_absolute_position(&self) -> states::definitions::Coordinates { self.absolute_position }

    fn set_horizontal_alignment(&mut self, alignment: states::definitions::HorizontalAlignment) {

        if self.halign != alignment { self.changed = true }
        self.halign = alignment;
    }

    fn get_horizontal_alignment(&self) -> states::definitions::HorizontalAlignment { self.halign }

    fn set_vertical_alignment(&mut self, alignment: states::definitions::VerticalAlignment) {
        if self.valign != alignment { self.changed = true }
        self.valign = alignment;
    }

    fn get_vertical_alignment(&self) -> states::definitions::VerticalAlignment { self.valign }

    fn set_padding(&mut self, padding: states::definitions::Padding) {
        if self.padding != padding { self.changed = true }
        self.padding = padding;
    }

    fn get_padding(&self) -> &states::definitions::Padding { &self.padding }

    fn set_border_config(&mut self, config: states::definitions::BorderConfig) {
        if self.border_config != config { self.changed = true }
        self.border_config = config;
    }

    fn get_border_config(&self) -> &states::definitions::BorderConfig { &self.border_config  }

    fn get_border_config_mut(&mut self) -> &mut states::definitions::BorderConfig {
        self.changed = true;
        &mut self.border_config
    }

    fn set_color_config(&mut self, config: states::definitions::ColorConfig) {
        if self.colors != config { self.changed = true }
        self.colors = config;
    }

    fn get_color_config(&self) -> &states::definitions::ColorConfig { &self.colors }

    fn get_colors_config_mut(&mut self) -> &mut states::definitions::ColorConfig {
        self.changed = true;
        &mut self.colors
    }

    fn is_selectable(&self) -> bool { true }

    fn get_selection_order(&self) -> usize { self.selection_order }

    fn set_force_redraw(&mut self, redraw: bool) {
        self.force_redraw = redraw;
        self.changed = true;
    }

    fn get_force_redraw(&self) -> bool { self.force_redraw }

    fn set_selected(&mut self, state: bool) {
        if self.selected != state { self.changed = true }
        self.selected = state;
    }
    fn get_selected(&self) -> bool { self.selected }
}
impl DropdownState {

    pub fn set_choice(&mut self, choice: String) {
        if self.choice != choice { self.changed = true }
        self.choice = choice;
    }

    pub fn get_choice(&self) -> String { self.choice.clone() }

    pub fn set_options(&mut self, options: Vec<String>) { self.options = options }

    pub fn get_options(&self) -> Vec<String> { self.options.clone() }

    pub fn set_focussed(&mut self, focussed: bool) {
        if self.focussed != focussed { self.changed = true }
        self.focussed = focussed;
    }

    pub fn get_focussed(&self) -> bool { self.focussed }

    pub fn set_allow_none(&mut self, allow_none: bool) {
        if self.allow_none != allow_none { self.changed = true }
        self.allow_none = allow_none;
    }

    pub fn get_allow_none(&self) -> bool { self.allow_none }

    /// Return the total amount of options in this dropdown including the empty option if it is
    /// allowed.
    pub fn total_options(&self) -> usize { self.options.len() + if self.allow_none {1} else {0} }

}


/// [State] implementation.
#[derive(Clone)]
pub struct DroppedDownMenuState {

    /// Widget path of the [Dropdown] that spawned this menu.
    pub parent_path: String,

    /// Position of this widget relative to its' parent [Layout]
    pub position: states::definitions::Coordinates,

    /// Absolute position of this widget on screen. Automatically propagated, do not set manually
    pub absolute_position: states::definitions::Coordinates,

    /// Relative height/width of this widget to parent layout
    pub size_hint: states::definitions::SizeHint,

    /// Pos hint of this widget
    pub pos_hint: states::definitions::PosHint,

    /// size of this widget
    pub size: states::definitions::Size,

    /// Automatically adjust size of widget to content
    pub auto_scale: states::definitions::AutoScale,

    pub padding: states::definitions::Padding,

    /// List of options this dropdown will display
    pub options: Vec<String>,

    pub allow_none: bool,

    /// The currently active choice of the dropdown.
    pub choice: String,

    /// If dropped down, this represents which row of the dropdown is being hovered with the mouse,
    /// or has been selected with the keyboard using up/down.
    pub dropped_down_selected_row: usize,

    /// [BorderConfig] object that will be used to draw the border if enabled
    pub border_config: states::definitions::BorderConfig,

    /// Object containing colors to be used by this widget in different situations
    pub colors: states::definitions::ColorConfig,

    /// Bool representing if state has changed. Triggers widget redraw.
    pub changed: bool,

    /// If true this forces a global screen redraw on the next frame. Screen redraws are diffed
    /// so this can be called when needed without degrading performance. If only screen positions
    /// that fall within this widget must be redrawn, call [EzObject.redraw] instead.
    pub force_redraw: bool,
}
impl Default for DroppedDownMenuState {
    fn default() -> Self {
        DroppedDownMenuState {
            parent_path: String::new(),
            position: states::definitions::Coordinates::default(),
            absolute_position: states::definitions::Coordinates::default(),
            size_hint: states::definitions::SizeHint::default(),
            auto_scale: states::definitions::AutoScale::default(),
            pos_hint: states::definitions::PosHint::default(),
            size: states::definitions::Size::new(0, 3),
            padding: states::definitions::Padding::new(0, 0, 0, 0),
            options: Vec::new(),
            allow_none: true,
            dropped_down_selected_row:0,
            choice: String::new(),
            border_config: states::definitions::BorderConfig::default(),
            colors: states::definitions::ColorConfig::default(),
            changed: false,
            force_redraw: false
        }
    }
}
impl GenericState for DroppedDownMenuState {

    fn set_changed(&mut self, changed: bool) { self.changed = changed }

    fn get_changed(&self) -> bool { self.changed }

    fn set_size_hint(&mut self, size_hint: states::definitions::SizeHint) {
        if self.size_hint != size_hint { self.changed = true }
        self.size_hint = size_hint;
    }

    fn get_size_hint(&self) -> &states::definitions::SizeHint { &self.size_hint }

    fn set_pos_hint(&mut self, pos_hint: states::definitions::PosHint) {
        if self.pos_hint != pos_hint { self.changed = true }
        self.pos_hint = pos_hint;
    }

    fn get_pos_hint(&self) -> &states::definitions::PosHint { &self.pos_hint }

    fn set_auto_scale(&mut self, auto_scale: states::definitions::AutoScale) {
        if self.auto_scale != auto_scale { self.changed = true }
        self.auto_scale = auto_scale;
    }

    fn get_auto_scale(&self) -> &states::definitions::AutoScale { &self.auto_scale }

    fn set_size(&mut self, size: states::definitions::Size) { self.size = size; }

    fn get_size(&self) -> &states::definitions::Size { &self.size }

    fn get_size_mut(&mut self) -> &mut states::definitions::Size { &mut self.size }

    fn set_position(&mut self, position: states::definitions::Coordinates) { self.position = position; }

    fn get_position(&self) -> states::definitions::Coordinates { self.position }

    fn set_absolute_position(&mut self, pos: states::definitions::Coordinates) { self.absolute_position = pos }

    fn get_absolute_position(&self) -> states::definitions::Coordinates { self.absolute_position }

    fn set_horizontal_alignment(&mut self, _alignment: states::definitions::HorizontalAlignment) {
    }

    fn get_horizontal_alignment(&self) -> states::definitions::HorizontalAlignment {
        panic!("Alignment not implemented for modal")
    }

    fn set_vertical_alignment(&mut self, _alignment: states::definitions::VerticalAlignment) {
    }

    fn get_vertical_alignment(&self) -> states::definitions::VerticalAlignment {
        panic!("Alignment not implemented for modal")
    }

    fn set_padding(&mut self, _padding: states::definitions::Padding) { }

    fn get_padding(&self) -> &states::definitions::Padding { &self.padding }

    fn set_border_config(&mut self, config: states::definitions::BorderConfig) {
        if self.border_config != config { self.changed = true }
        self.border_config = config;
    }

    fn get_border_config(&self) -> &states::definitions::BorderConfig { &self.border_config  }

    fn get_border_config_mut(&mut self) -> &mut states::definitions::BorderConfig {
        self.changed = true;
        &mut self.border_config
    }

    fn set_color_config(&mut self, config: states::definitions::ColorConfig) {
        if self.colors != config { self.changed = true }
        self.colors = config;
    }

    fn get_color_config(&self) -> &states::definitions::ColorConfig { &self.colors }

    fn get_colors_config_mut(&mut self) -> &mut states::definitions::ColorConfig {
        self.changed = true;
        &mut self.colors
    }

    fn set_force_redraw(&mut self, redraw: bool) {
        self.force_redraw = redraw;
        self.changed = true;
    }

    fn get_force_redraw(&self) -> bool { self.force_redraw }
}

impl DroppedDownMenuState {

    pub fn set_choice(&mut self, choice: String) {
        if self.choice != choice { self.changed = true }
        self.choice = choice;
    }

    pub fn get_choice(&self) -> String { self.choice.clone() }

    pub fn set_options(&mut self, options: Vec<String>) { self.options = options }

    pub fn get_options(&self) -> Vec<String> { self.options.clone() }

    pub fn set_allow_none(&mut self, allow_none: bool) {
        if self.allow_none != allow_none { self.changed = true }
        self.allow_none = allow_none;
    }

    pub fn get_allow_none(&self) -> bool { self.allow_none }

    pub fn set_dropped_down_selected_row(&mut self, dropped_down_selected_row: usize) {
        if self.dropped_down_selected_row != dropped_down_selected_row { self.changed = true }
        self.dropped_down_selected_row = dropped_down_selected_row;
    }

    pub fn get_dropped_down_selected_row(&self) -> usize { self.dropped_down_selected_row }

    /// Return the total amount of options in this dropdown including the empty option if it is
    /// allowed.
    pub fn total_options(&self) -> usize { self.options.len() + if self.allow_none {1} else {0} }

    /// Get an ordered list of options, including the empty option if it was allowed. Order is:
    /// - Active choice
    /// - Empty (if allowed)
    /// - Rest of the options in user defined order
    pub fn get_dropped_down_options(&self) -> Vec<String> {
        let mut options = vec!(self.choice.clone());
        if self.allow_none && !self.choice.is_empty() {
            options.push("".to_string())
        }
        for option in self.options.iter().filter(|x| x.to_string() != self.choice) {
            options.push(option.to_string());
        }
        options
    }

}
