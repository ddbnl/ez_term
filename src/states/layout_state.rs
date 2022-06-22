use std::collections::HashMap;
use crate::common::definitions::{Coordinates, Size, StateTree};
use crate::states::definitions::{StateCoordinates, SizeHint, PosHint, StateSize, AutoScale, Padding,
                                 HorizontalAlignment, VerticalAlignment, BorderConfig, ColorConfig,
                                 LayoutMode, LayoutOrientation, ScrollingConfig};
use crate::common;
use crate::scheduler::Scheduler;
use crate::states::state::GenericState;
use crate::widgets::widget::EzObjects;


/// [State] implementation.
#[derive(Clone, Debug)]
pub struct LayoutState {

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

    /// Layout mode enum, see [LayoutMode] for options
    pub mode: LayoutMode,

    /// Orientation enum, see [LayoutOrientation] for options
    pub orientation: LayoutOrientation,

    /// Amount of space to leave between sides of the widget and other widgets
    pub padding: Padding,

    /// Horizontal alignment of this widget
    pub halign: HorizontalAlignment,

    /// Vertical alignment of this widget
    pub valign: VerticalAlignment,

    /// ID of the child that is the active screen (i.e. its content is visible)
    pub active_screen: String,

    /// Name shown for tab if [is_tab] is true and parent [is_tabbed]
    pub tab_name: String,

    /// Path to active tab (i.e. its content is visible)
    pub active_tab: String,

    /// Path to active tab header button
    pub selected_tab_header: String,

    /// Bool representing whether this layout should be filled with [filler_symbol] in positions
    /// where it does not get other content from [get_contents]
    pub fill: bool,

    /// The [Pixel.Symbol] to use for filler pixels if [fill] is true
    pub filler_symbol: String,

    /// [BorderConfig] object that will be used to draw the border if enabled
    pub border_config: BorderConfig,

    /// Object containing colors to be used by this widget in different situations
    pub colors: ColorConfig,

    /// [ScrollingConfig] of this layout.
    pub scrolling_config: ScrollingConfig,

    /// A list of open modals. Modals are widgets that overlap other content; in other words, they
    /// open 'in front of' other content. Only one can be shown at a time (the first on in the
    /// list).
    pub open_modals: Vec<EzObjects>,

    /// A hashmap of 'Template Name > [EzWidgetDefinition]'. Used to instantiate widget templates
    /// at runtime. E.g. when spawning popups.
    pub templates: common::definitions::Templates,

    /// Bool representing whether widget is disabled, i.e. cannot be interacted with
    pub disabled: bool,

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
impl LayoutState {

    pub fn new(path: String, scheduler: &mut Scheduler) -> Self {

        LayoutState {
            path: path.clone(),
            position: StateCoordinates::new(0, 0, path.clone(), scheduler),
            absolute_position: Coordinates::default(),
            size_hint: SizeHint::default(),
            pos_hint: PosHint::default(),
            size: StateSize::new(0, 0, path.clone(), scheduler),
            auto_scale: AutoScale::default(),
            orientation: LayoutOrientation::Horizontal,
            mode: LayoutMode::Box,
            padding: Padding::new(0, 0, 0, 0, path, scheduler),
            halign: HorizontalAlignment::Left,
            valign: VerticalAlignment::Top,
            active_screen: String::new(),
            tab_name: "Tab".to_string(),
            active_tab: String::new(),
            selected_tab_header: String::new(),
            fill: false,
            filler_symbol: String::new(),
            scrolling_config: ScrollingConfig::default(),
            border_config: BorderConfig::default(),
            colors: ColorConfig::default(),
            changed: false,
            open_modals: Vec::new(),
            templates: HashMap::new(),
            disabled: false,
            selected: false,
            selection_order: 0,
            force_redraw: false
        }
    }
}
impl GenericState for LayoutState {

    fn get_path(&self) -> &String {
        &self.path
    }

    fn set_size_hint(&mut self, size_hint: SizeHint) { self.size_hint = size_hint; }

    fn get_size_hint(&self) -> &SizeHint { &self.size_hint }

    fn set_pos_hint(&mut self, pos_hint: PosHint) {
        if self.pos_hint != pos_hint { self.changed = true }
        self.pos_hint = pos_hint;
    }

    fn get_pos_hint(&self) -> &PosHint { &self.pos_hint }

    fn set_auto_scale(&mut self, auto_scale: AutoScale) {
        if self.auto_scale != auto_scale { self.changed = true }
        self.auto_scale = auto_scale;
    }

    fn get_auto_scale(&self) -> &AutoScale { &self.auto_scale }

    fn get_size(&self) -> &StateSize { &self.size  }

    fn get_size_mut(&mut self) -> &mut StateSize { &mut self.size }

    fn get_effective_size(&self) -> Size {

        let width_result: isize = self.size.width.value as isize
            -if self.get_border_config().enabled {2} else {0}
            -if self.scrolling_config.enable_y {1} else {0}
            -self.get_padding().left.value as isize - self.get_padding().right.value as isize;
        let width = if width_result < 0 {0} else { width_result};
        let height_result: isize = self.size.height.value as isize
            -if self.get_border_config().enabled {2} else {0}
            -if self.scrolling_config.enable_x {1} else {0}
            -self.get_padding().top.value as isize - self.get_padding().bottom.value as isize;
        let height = if height_result < 0 {0} else { height_result};
        Size::new(width as usize, height as usize)
    }

    /// Set the how much width you want the actual content inside this widget to have. Width for
    /// e.g. border and padding will be added to this automatically.
    fn set_effective_width(&mut self, width: usize) {
        let offset = if self.get_border_config().enabled {2} else {0}
            + if self.scrolling_config.enable_y {1} else {0}
            + self.get_padding().left.value + self.get_padding().right.value;
        self.get_size_mut().width.set(width + offset);
    }

    /// Set the how much height you want the actual content inside this widget to have. Height for
    /// e.g. border and padding will be added to this automatically.
    fn set_effective_height(&mut self, height: usize) {

        let offset = if self.get_border_config().enabled {2} else {0}
            + if self.scrolling_config.enable_x {1} else {0}
            + self.get_padding().top.value + self.get_padding().bottom.value;
        self.get_size_mut().height.set(height + offset);
    }

    fn get_position(&self) -> &StateCoordinates { &self.position }

    fn get_position_mut(&mut self) -> &mut StateCoordinates {
        self.changed = true;
        &mut self.position
    }

    fn set_absolute_position(&mut self, pos: Coordinates) { self.absolute_position = pos; }

    fn get_absolute_position(&self) -> Coordinates { self.absolute_position }

    fn set_horizontal_alignment(&mut self, alignment: HorizontalAlignment) {
        if self.halign != alignment { self.changed = true }
        self.halign = alignment;
    }

    fn get_horizontal_alignment(&self) -> HorizontalAlignment { self.halign }

    fn set_vertical_alignment(&mut self, alignment: VerticalAlignment) {
        if self.valign != alignment { self.changed = true }
        self.valign = alignment;
    }

    fn get_vertical_alignment(&self) -> VerticalAlignment { self.valign }

    fn get_padding(&self) -> &Padding { &self.padding }

    fn get_padding_mut(&mut self) -> &mut Padding { &mut self.padding }

    fn set_border_config(&mut self, config: BorderConfig) {
        if self.border_config != config { self.changed = true }
        self.border_config = config;
    }

    fn get_border_config(&self) -> &BorderConfig { &self.border_config  }

    fn get_border_config_mut(&mut self) -> &mut BorderConfig {
        self.changed = true;
        &mut self.border_config
    }

    fn set_color_config(&mut self, config: ColorConfig) {
        if self.colors != config { self.changed = true }
        self.colors = config;
    }

    fn get_color_config(&self) -> &ColorConfig { &self.colors }

    fn get_colors_config_mut(&mut self) -> &mut ColorConfig {
        self.changed = true;
        &mut self.colors
    }

    fn is_selectable(&self) -> bool { self.get_scrolling_config().is_scrolling_x
        || self.get_scrolling_config().is_scrolling_y || self.mode == LayoutMode::Tabbed }

    fn set_disabled(&mut self, disabled: bool) {
        if self.disabled != disabled { self.changed = true }
        self.disabled = disabled
    }

    fn get_disabled(&self) -> bool { self.disabled }

    fn get_selection_order(&self) -> usize { self.selection_order }

    fn set_selection_order(&mut self, order: usize) {
        if self.selection_order != order { self.changed = true };
        self.selection_order = order;
    }

    fn set_selected(&mut self, state: bool) {
        if self.selected != state { self.changed = true }
        self.selected = state;
    }

    fn get_selected(&self) -> bool { self.selected }
}
impl LayoutState {

    /// Set [LayoutMode]
    pub fn set_mode(&mut self, mode: LayoutMode) { self.mode = mode }

    /// Get [LayoutMode]
    pub fn get_mode(&self) -> &LayoutMode { &self.mode }

    /// Set [LayoutOrientation]
    pub fn set_orientation(&mut self, orientation: LayoutOrientation) {
        self.orientation = orientation
    }

    /// Get [LayoutOrientation]
    pub fn get_orientation(&self) -> &LayoutOrientation { &self.orientation }

    /// Set the ID of the child that is the currently active screen (i.e. content is showing)
    pub fn set_active_screen(&mut self, id: String) {
        if self.active_screen != id { self.changed = true }
        self.active_screen = id;
    }

    /// Get the ID of the child that is the currently active screen (i.e. content is showing)
    pub fn get_active_screen(&self) -> String { self.active_screen.clone() }

    /// Set the path to the Layout that is currently active as the current tab (i.e. content is
    /// showing)
    pub fn set_active_tab(&mut self, path: String) {
        if self.active_tab != path { self.changed = true }
        self.active_tab = path;
    }

    /// Get the [path] to the Layout that is currently active as a tab (i.e. content is showing)
    pub fn get_active_tab(&self) -> String { self.active_tab.clone() }

    /// Set the tab header that is currently selected
    pub fn set_selected_tab_header(&mut self, path: String) {
        if self.selected_tab_header != path { self.changed = true }
        self.selected_tab_header = path;
    }

    /// Get the tab header that is currently selected
    pub fn get_selected_tab_header(&self) -> String { self.selected_tab_header.clone() }

    /// Set the [ScrollingConfig] active for this Layout
    pub fn set_scrolling_config(&mut self, config: ScrollingConfig) {
        if self.scrolling_config != config { self.changed = true }
        self.scrolling_config = config;
    }

    /// Get a ref to the [ScrollingConfig] active for this Layout
    pub fn get_scrolling_config(&self) -> &ScrollingConfig { &self.scrolling_config }

    /// Get a mutable ref to the [ScrollingConfig] active for this Layout
    pub fn get_scrolling_config_mut(&mut self) -> &mut ScrollingConfig {
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
    pub fn open_popup(&mut self, template: String, scheduler: &mut Scheduler)
        -> (String, common::definitions::StateTree) {
        let mut popup = self.templates.get(&template).unwrap().clone();
        let init_popup = popup.parse(&mut self.templates, scheduler,
                                     "/modal".to_string(), 0, None);
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
            extra_state_tree = StateTree::new("state_tree".to_string());
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
