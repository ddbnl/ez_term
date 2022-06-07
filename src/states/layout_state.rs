use std::collections::HashMap;
use crate::common;
use crate::states;
use crate::states::state::GenericState;
use crate::widgets::widget::EzObjects;


/// [State] implementation.
#[derive(Clone)]
pub struct LayoutState {

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

    /// Bool representing whether this layout should be filled with [filler_symbol] in positions
    /// where it does not get other content from [get_contents]
    pub fill: bool,

    /// The [Pixel.Symbol] to use for filler pixels if [fill] is true
    pub filler_symbol: String,

    /// [BorderConfig] object that will be used to draw the border if enabled
    pub border_config: states::definitions::BorderConfig,

    /// Object containing colors to be used by this widget in different situations
    pub colors: states::definitions::ColorConfig,

    /// [ScrollingConfig] of this layout.
    pub scrolling_config: states::definitions::ScrollingConfig,

    /// A list of open modals. Modals are widgets that overlap other content; in other words, they
    /// open 'in front of' other content. Only one can be shown at a time (the first on in the
    /// list).
    pub open_modals: Vec<EzObjects>,

    /// A hashmap of 'Template Name > [EzWidgetDefinition]'. Used to instantiate widget templates
    /// at runtime. E.g. when spawning popups.
    pub templates: common::definitions::Templates,

    /// Global order number in which this widget will be selection when user presses down/up keys
    pub selection_order: usize,

    /// Bool representing whether this widget is currently selected.
    pub selected: bool,

    /// Bool representing if state has changed. Triggers widget redraw.
    pub changed: bool,

    /// If true this forces a global screen redraw on the next frame. Screen redraws are diffed
    /// so this can be called when needed without degrading performance. If only screen positions
    /// that fall within this widget must be redrawn, call [EzObject.redraw] instead.
    pub force_redraw: bool,
}
impl Default for LayoutState {
    fn default() -> Self {
        LayoutState {
            position: states::definitions::Coordinates::default(),
            absolute_position: states::definitions::Coordinates::default(),
            size_hint: states::definitions::SizeHint::default(),
            pos_hint: states::definitions::PosHint::default(),
            size: states::definitions::Size::default(),
            auto_scale: states::definitions::AutoScale::default(),
            padding: states::definitions::Padding::default(),
            halign: states::definitions::HorizontalAlignment::Left,
            valign: states::definitions::VerticalAlignment::Top,
            fill: false,
            filler_symbol: String::new(),
            scrolling_config: states::definitions::ScrollingConfig::default(),
            border_config: states::definitions::BorderConfig::default(),
            colors: states::definitions::ColorConfig::default(),
            changed: false,
            open_modals: Vec::new(),
            templates: HashMap::new(),
            selected: false,
            selection_order: 0,
            force_redraw: false
        }
    }
}
impl GenericState for LayoutState {

    fn set_changed(&mut self, changed: bool) { self.changed = changed }

    fn get_changed(&self) -> bool { self.changed }

    fn set_size_hint(&mut self, size_hint: states::definitions::SizeHint) { self.size_hint = size_hint; }

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

    fn get_size(&self) -> &states::definitions::Size { &self.size  }

    fn get_size_mut(&mut self) -> &mut states::definitions::Size { &mut self.size }

    fn get_effective_size(&self) -> states::definitions::Size {

        let mut size_copy = self.get_size().clone();
        let width_result: isize = size_copy.width as isize
            -if self.get_border_config().enabled {2} else {0}
            -if self.scrolling_config.enable_y {1} else {0}
            -self.get_padding().left as isize - self.get_padding().right as isize;
        let width = if width_result < 0 {0} else { width_result as usize};
        let height_result: isize = size_copy.height as isize
            -if self.get_border_config().enabled {2} else {0}
            -if self.scrolling_config.enable_x {1} else {0}
            -self.get_padding().top as isize - self.get_padding().bottom as isize;
        let height = if height_result < 0 {0} else { height_result as usize};
        size_copy.width = width;
        size_copy.height = height;
        size_copy
    }

    /// Set the how much width you want the actual content inside this widget to have. Width for
    /// e.g. border and padding will be added to this automatically.
    fn set_effective_width(&mut self, width: usize) {
        self.get_size_mut().width = width
            +if self.get_border_config().enabled {2} else {0}
            +if self.scrolling_config.enable_y {1} else {0}
            +self.get_padding().left + self.get_padding().right
    }

    /// Set the how much height you want the actual content inside this widget to have. Height for
    /// e.g. border and padding will be added to this automatically.
    fn set_effective_height(&mut self, height: usize) {
        self.get_size_mut().height = height
            +if self.get_border_config().enabled {2} else {0}
            +if self.scrolling_config.enable_x {1} else {0}
            +self.get_padding().top + self.get_padding().bottom
    }

    fn set_position(&mut self, position: states::definitions::Coordinates) { self.position = position; }

    fn get_position(&self) -> states::definitions::Coordinates { self.position }

    fn set_absolute_position(&mut self, pos: states::definitions::Coordinates) { self.absolute_position = pos; }

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

    fn is_selectable(&self) -> bool { self.get_scrolling_config().is_scrolling_x
        || self.get_scrolling_config().is_scrolling_y }

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
impl LayoutState {

    pub fn set_scrolling_config(&mut self, config: states::definitions::ScrollingConfig) {
        if self.scrolling_config != config { self.changed = true }
        self.scrolling_config = config;
    }

    pub fn get_scrolling_config(&self) -> &states::definitions::ScrollingConfig { &self.scrolling_config }

    pub fn get_scrolling_config_mut(&mut self) -> &mut states::definitions::ScrollingConfig {
        self.changed = true;
        &mut self.scrolling_config
    }

    /// Set [filler_symbol]
    pub fn set_filler_symbol(&mut self, symbol: String) {
        if self.filler_symbol != symbol { self.changed = true }
        self.filler_symbol = symbol;
    }

    /// Get [filler_symbol]
    pub fn get_filler_symbol(&self) -> String { self.filler_symbol.clone() }

    /// Open a popup based on a template defined in the Ez file. Returns the state of the new popup
    pub fn open_popup(&mut self, template: String) -> (String, common::definitions::StateTree) {
        let mut popup = self.templates.get_mut(&template).unwrap().clone();
        let init_popup = popup.parse(&mut self.templates);
        self.open_modal(init_popup)
    }
    
    /// Open a new modal. Returns the state of the new modal.
    pub fn open_modal(&mut self, mut modal: EzObjects) -> (String, common::definitions::StateTree) {

        if modal.as_ez_object().get_id().is_empty() {
            modal.as_ez_object_mut().set_id(self.open_modals.len().to_string());
        }
        let modal_path = format!("/modal/{}", modal.as_ez_object().get_id());
        modal.as_ez_object_mut().set_full_path(modal_path.clone());

        // State tree must be appended with the new states
        let mut extra_state_tree;
        if let EzObjects::Layout(ref mut i) = modal {
            i.propagate_paths();
            extra_state_tree = common::screen_functions::initialize_state_tree(i);
        } else {
            extra_state_tree = HashMap::new();
            extra_state_tree.insert(modal_path.clone(),modal.as_ez_object().get_state());
        }
        self.open_modals.push(modal);
        self.changed = true;
        (modal_path, extra_state_tree)
    }
    
    /// Dismiss the current modal
    pub fn dismiss_modal(&mut self) {
        self.open_modals.remove(0);
        self.changed = true;
        self.force_redraw = true;
    }

    /// Dismiss all modals, clearing the entire stack
    pub fn dismiss_all_modals(&mut self) {

        self.open_modals.clear();
        self.changed = true;
        self.force_redraw = true;
    }
    
    /// Get reference to all open modals
    pub fn get_modals(&self) -> &Vec<EzObjects> { &self.open_modals }
    
    /// Get mutable reference to all open modals
    pub fn get_modals_mut(&mut self) -> &mut Vec<EzObjects> {
        self.changed = true;
        &mut self.open_modals
    }

    /// Set templates. Used by [ez_parser] on the root layout to keep a hold of all templates
    /// defined by the user. They can be used to instantiate e.g. popups at runtime.
    pub fn set_templates(&mut self, templates: common::definitions::Templates) {
        self.templates = templates
    }

    /// Get templates. Use on the root layout to get all templates defined by the user.
    /// They can be used to instantiate e.g. popups at runtime.
    pub fn get_templates(&self) -> &common::definitions::Templates { &self.templates }

}
