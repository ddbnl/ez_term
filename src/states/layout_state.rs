use std::collections::HashMap;

use crate::{EzObject, EzProperty};
use crate::parser::ez_definition::Templates;
use crate::property::ez_values::EzValues;
use crate::run::definitions::{IsizeCoordinates, Size};
use crate::scheduler::scheduler::SchedulerFrontend;
use crate::scheduler::scheduler_funcs::clean_up_property;
use crate::states::definitions::{AutoScale, BorderConfig, ColorConfig, HorizontalAlignment,
                                 InfiniteSize, LayoutMode, LayoutOrientation, Padding, PosHint,
                                 ScrollingConfig, SizeHint, StateCoordinates, StateSize,
                                 TableConfig, VerticalAlignment};
use crate::states::ez_state::{EzState, GenericState};
use crate::widgets::ez_object::EzObjects;

/// [State] implementation.
#[derive(Clone, Debug)]
pub struct LayoutState {

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

    /// layout mode enum, see [LayoutMode] for options
    pub mode: EzProperty<LayoutMode>,

    /// Orientation enum, see [LayoutOrientation] for options
    pub orientation: EzProperty<LayoutOrientation>,

    /// Amount of space to leave between sides of the widget and other widgets
    pub padding: Padding,

    /// Horizontal alignment of this layout
    pub halign: EzProperty<HorizontalAlignment>,

    /// Vertical alignment of this layout
    pub valign: EzProperty<VerticalAlignment>,

    /// [TableConfig] of this layout
    pub table_config: TableConfig,

    /// ID of the child that is the active screen (i.e. its content is visible)
    pub active_screen: EzProperty<String>,

    /// Name shown for tab if [is_tab] is true and parent [is_tabbed]
    pub tab_name: EzProperty<String>,

    /// ID of the active tab (i.e. its content is visible)
    pub active_tab: EzProperty<String>,

    /// Id of the active tab header button
    selected_tab_header: String,

    /// Bool representing whether this layout should be filled with [filler_symbol] in positions
    /// where it does not get other content from [get_contents]
    pub fill: EzProperty<bool>,

    /// The [Pixel.Symbol] to use for filler pixels if [fill] is true
    pub filler_symbol: EzProperty<String>,

    /// [BorderConfig] object that will be used to draw the border if enabled
    pub border_config: BorderConfig,

    /// Object containing colors to be used by this widget in different situations
    pub colors: ColorConfig,

    /// See [ScrollingConfig]
    pub scrolling_config: ScrollingConfig,

    /// Whether this layout can be dragged around if it is a modal
    pub can_drag: EzProperty<bool>,

    /// A list of open modals. Modals are widgets that overlap other content; in other words, they
    /// open 'in front of' other content. Only one can be shown at a time (the first on in the
    /// list).
    open_modal: Option<Box<EzObjects>>,

    /// A hashmap of 'Template Name > [EzWidgetDefinition]'. Used to instantiate widget templates
    /// at runtime. E.g. when spawning popups.
    templates: Templates,

    /// Bool representing whether widget is disabled, i.e. cannot be interacted with
    pub disabled: EzProperty<bool>,

    /// Global order number in which this widget will be selection when user presses down/up keys
    pub selection_order: EzProperty<usize>,

    /// Bool representing whether this widget is currently selected.
    selected: bool,
}
impl LayoutState {

    pub fn new(path: String, scheduler: &mut SchedulerFrontend) -> Self {

        LayoutState {
            path: path.clone(),
            position: StateCoordinates::new(0, 0, path.clone(), scheduler),
            absolute_position: IsizeCoordinates::default(),
            size_hint: SizeHint::new(Some(1.0), Some(1.0), path.clone(), scheduler),
            pos_hint: PosHint::new(None, None, path.clone(), scheduler),
            size: StateSize::new(0, 0, path.clone(), scheduler),
            infinite_size: InfiniteSize::default(),
            auto_scale: AutoScale::new(false, false, path.clone(), scheduler),
            mode: scheduler.new_layout_mode_property(
                format!("{}/mode", path).as_str(), LayoutMode::Box),
            orientation: scheduler.new_layout_orientation_property(
                format!("{}/orientation", path).as_str(), LayoutOrientation::Horizontal),
            padding: Padding::new(0, 0, 0, 0, path.clone(), scheduler),
            halign: scheduler.new_horizontal_alignment_property(
                format!("{}/halign", path).as_str(), HorizontalAlignment::Left),
            valign: scheduler.new_vertical_alignment_property(
                format!("{}/valign", path).as_str(), VerticalAlignment::Top),
            table_config: TableConfig::new(path.clone(), scheduler),
            active_screen: scheduler.new_string_property(
                format!("{}/active_screen", path).as_str(), String::new()),
            tab_name: scheduler.new_string_property(
                format!("{}/tab_name", path).as_str(), String::new()),
            active_tab: scheduler.new_string_property(
                format!("{}/active_tab", path).as_str(), String::new()),
            selected_tab_header: String::new(),
            fill: scheduler.new_bool_property(format!("{}/fill", path).as_str(),false),
            filler_symbol: scheduler.new_string_property(
                format!("{}/filler_symbol", path).as_str(), String::new()),
            scrolling_config: ScrollingConfig::new(false, false, 0.0, 0.0,
                                                   path.clone(), scheduler),
            border_config: BorderConfig::new(false, path.clone(), scheduler),
            colors: ColorConfig::new(path.clone(), scheduler),
            can_drag: scheduler.new_bool_property(
                format!("{}/can_drag", path).as_str(), true),
            open_modal: None,
            templates: HashMap::new(),
            disabled: scheduler.new_bool_property(
                format!("{}/disabled", path).as_str(),false),
            selected: false,
            selection_order: scheduler.new_usize_property(
                format!("{}/selection_order", path).as_str(), 0),
        }
    }
}
impl GenericState for LayoutState {

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
            "active_fg_color" => self.colors.active_fg_color.set_from_ez_value(value),
            "active_bg_color" => self.colors.active_bg_color.set_from_ez_value(value),
            "flash_fg_color" => self.colors.flash_fg_color.set_from_ez_value(value),
            "flash_bg_color" => self.colors.flash_bg_color.set_from_ez_value(value),
            "filler_fg_color" => self.colors.filler_fg_color.set_from_ez_value(value),
            "filler_bg_color" => self.colors.filler_bg_color.set_from_ez_value(value),
            "tab_fg_color" => self.colors.tab_fg_color.set_from_ez_value(value),
            "tab_bg_color" => self.colors.tab_bg_color.set_from_ez_value(value),
            "tab_border_fg_color" => self.colors.tab_border_fg_color.set_from_ez_value(value),
            "tab_border_bg_color" => self.colors.tab_border_bg_color.set_from_ez_value(value),
            "border_fg_color" => self.colors.border_fg_color.set_from_ez_value(value),
            "border_bg_color" => self.colors.border_bg_color.set_from_ez_value(value),
            "cursor_color" => self.colors.cursor_color.set_from_ez_value(value),
            "orientation" => self.orientation.set_from_ez_value(value),
            "mode" => self.mode.set_from_ez_value(value),
            "active_screen" => self.active_screen.set_from_ez_value(value),
            "active_tab" => self.active_tab.set_from_ez_value(value),
            "tab_name" => self.tab_name.set_from_ez_value(value),
            "can_drag" => self.can_drag.set_from_ez_value(value),
            "fill" => self.fill.set_from_ez_value(value),
            "filler_symbol" => self.filler_symbol.set_from_ez_value(value),
            "scroll_x" => self.scrolling_config.scroll_x.set_from_ez_value(value),
            "scroll_y" => self.scrolling_config.scroll_y.set_from_ez_value(value),
            "view_start_x" => self.scrolling_config.view_start_x.set_from_ez_value(value),
            "view_start_y" => self.scrolling_config.view_start_y.set_from_ez_value(value),
            "rows" => self.table_config.rows.set_from_ez_value(value),
            "cols" => self.table_config.cols.set_from_ez_value(value),
            "col_default_width" => self.table_config.col_default_width.set_from_ez_value(value),
            "force_default_col_width" => self.table_config.force_default_col_width.set_from_ez_value(value),
            "row_default_height" => self.table_config.row_default_height.set_from_ez_value(value),
            "force_default_row_height" => self.table_config.force_default_row_height.set_from_ez_value(value),
            _ => panic!("Invalid property name for button state: {}", name),
        }
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

    fn get_effective_size(&self) -> Size {

        let width_result: isize = self.size.get_width() as isize
            -if self.get_border_config().get_border() {2} else {0}
            -if self.scrolling_config.get_scroll_y() {1} else {0}
            -self.get_padding().get_padding_left() as isize - self.get_padding().get_padding_right() as isize;
        let width = if width_result < 0 {0} else { width_result};
        let height_result: isize = self.size.get_height() as isize
            -if self.get_border_config().get_border() {2} else {0}
            -if self.scrolling_config.get_scroll_x() {1} else {0}
            -self.get_padding().get_padding_top() as isize - self.get_padding().get_padding_bottom() as isize;
        let height = if height_result < 0 {0} else { height_result};
        Size::new(width as usize, height as usize)
    }

    /// Set the how much width you want the actual content inside this widget to have. Width for
    /// e.g. border and padding will be added to this automatically.
    fn set_effective_width(&mut self, width: usize) {
        let offset = if self.get_border_config().get_border() {2} else {0}
            + if self.scrolling_config.get_scroll_y() {1} else {0}
            + self.get_padding().get_padding_left() + self.get_padding().get_padding_right();
        self.get_size_mut().set_width(width + offset);
    }

    /// Set the how much height you want the actual content inside this widget to have. Height for
    /// e.g. border and padding will be added to this automatically.
    fn set_effective_height(&mut self, height: usize) {

        let offset = if self.get_border_config().get_border() {2} else {0}
            + if self.scrolling_config.get_scroll_x() {1} else {0}
            + self.get_padding().get_padding_top() + self.get_padding().get_padding_bottom();
        self.get_size_mut().set_height(height + offset);
    }

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

    fn get_border_config_mut(&mut self) -> &mut BorderConfig {
        &mut self.border_config
    }

    fn get_color_config(&self) -> &ColorConfig { &self.colors }

    fn get_color_config_mut(&mut self) -> &mut ColorConfig {
        &mut self.colors
    }

    fn is_selectable(&self) -> bool { self.get_scrolling_config().get_is_scrolling_x()
        || self.get_scrolling_config().get_is_scrolling_y() || self.mode.value == LayoutMode::Tab
    }

    fn set_disabled(&mut self, disabled: bool) {
        self.disabled.set(disabled);
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
        clean_up_property(scheduler, &self.mode.name);
        clean_up_property(scheduler, &self.orientation.name);
        clean_up_property(scheduler, &self.halign.name);
        clean_up_property(scheduler, &self.valign.name);
        clean_up_property(scheduler, &self.active_screen.name);
        clean_up_property(scheduler, &self.active_tab.name);
        clean_up_property(scheduler, &self.tab_name.name);
        clean_up_property(scheduler, &self.fill.name);
        clean_up_property(scheduler, &self.filler_symbol.name);
        clean_up_property(scheduler, &self.disabled.name);
        clean_up_property(scheduler, &self.selection_order.name);
        self.border_config.clean_up_properties(scheduler);
        self.colors.clean_up_properties(scheduler);
    }
}
impl LayoutState {

    /// Set [LayoutMode]
    pub fn set_mode(&mut self, mode: LayoutMode) { self.mode.set(mode); }

    /// Get [LayoutMode]
    pub fn get_mode(&self) -> &LayoutMode { &self.mode.value }

    /// Set [LayoutOrientation]
    pub fn set_orientation(&mut self, orientation: LayoutOrientation) {
        self.orientation.set(orientation);
    }

    /// Get [LayoutOrientation]
    pub fn get_orientation(&self) -> &LayoutOrientation { &self.orientation.value }

    /// Set the ID of the child that is the currently active screen (i.e. content is showing)
    pub fn set_active_screen(&mut self, id: &str) { self.active_screen.set(id.to_string()); }

    /// Get the ID of the child that is the currently active screen (i.e. content is showing)
    pub fn get_active_screen(&self) -> String { self.active_screen.value.clone() }

    /// Set the id of the layout that is currently active as the current tab (i.e. content is
    /// showing)
    pub fn set_active_tab(&mut self, tab: &str) { self.active_tab.set(tab.to_string()); }

    /// Get the id of the layout that is currently active as a tab (i.e. content is showing)
    pub fn get_active_tab(&self) -> String { self.active_tab.value.clone() }

    pub fn set_tab_name(&mut self, name: &str) { self.tab_name.set(name.to_string()); }

    pub fn get_tab_name(&self) -> String {
        if self.tab_name.value.is_empty() {
            self.get_id()
        } else {
            self.tab_name.value.clone()
        }
    }

    /// Set the tab header that is currently selected
    pub fn set_selected_tab_header(&mut self, id: String) { self.selected_tab_header = id; }

    /// Get the tab header that is currently selected
    pub fn get_selected_tab_header(&self) -> String { self.selected_tab_header.clone() }

    /// Set the [ScrollingConfig] active for this layout
    pub fn set_scrolling_config(&mut self, config: ScrollingConfig) {
        self.scrolling_config = config;
    }

    /// Get a ref to the [ScrollingConfig] active for this layout
    pub fn get_scrolling_config(&self) -> &ScrollingConfig { &self.scrolling_config }

    /// Get a mutable ref to the [ScrollingConfig] active for this layout
    pub fn get_scrolling_config_mut(&mut self) -> &mut ScrollingConfig {
        &mut self.scrolling_config
    }

    /// Get a ref to the [TableConfig] active for this layout
    pub fn get_table_config(&self) -> &TableConfig { &self.table_config  }

    /// Get a mutable ref to the [TableConfig] active for this layout
    pub fn get_table_config_mut(&mut self) -> &mut TableConfig { &mut self.table_config }

    /// Set [fill]
    pub fn set_fill(&mut self, enable: bool) { self.fill.set(enable); }

    /// Get [fill]
    pub fn get_fill(&self) -> bool { self.fill.value }

    /// Set [filler_symbol]
    pub fn set_filler_symbol(&mut self, symbol: String) { self.filler_symbol.set(symbol); }

    /// Get [filler_symbol]
    pub fn get_filler_symbol(&self) -> String { self.filler_symbol.value.clone() }

    /// Open a popup based on a template defined in the Ez file. Returns the state of the new popup
    pub fn open_modal_from_template(&mut self, template: String,
                                    scheduler: &mut SchedulerFrontend) -> Vec<(String, EzState)> {
        let mut popup = self.templates.get(&template).unwrap().clone();
        let config = vec!("id: modal".to_string());
        let init_popup = popup.parse(scheduler,"/root".to_string(), 0,
                                     Some(config));
        self.open_modal(init_popup)
    }

    /// Open a new modal. Returns the state of the new modal.
    pub fn open_modal(&mut self, mut modal: EzObjects ) -> Vec<(String, EzState)> {

        // State tree must be appended with the new states
        let mut extra_state_tree = Vec::new();
        if let EzObjects::Layout(ref mut i) = modal {
            extra_state_tree.push((i.get_path(), i.get_state()));
            for widget in i.get_widgets_recursive() {
                let widget = widget.as_ez_object();
                extra_state_tree.push((widget.get_path(), widget.get_state()));
            }
        } else if let EzObjects::DroppedDownMenu(ref mut i) = modal {
            extra_state_tree.push((i.get_path(), i.get_state()));
        } else {
            panic!("Modal must be spawned from a Layout (template)")
        }
        self.open_modal = Some(Box::new(modal));
        extra_state_tree.reverse();
        extra_state_tree
    }
    
    /// Dismiss the current modal
    pub fn dismiss_modal(&mut self, scheduler: &mut SchedulerFrontend) {

        self.open_modal = None;
        self.update(scheduler);
        scheduler.deselect_widget();
        scheduler.force_redraw();
    }

    pub fn set_can_drag(&mut self, can_drag: bool) { self.can_drag.set(can_drag); }

    pub fn get_can_drag(&self) -> bool { self.can_drag.value }

    pub fn has_modal(&self) -> bool { self.open_modal.is_some() }
    
    /// Get reference to all open modals
    pub fn get_modal(&self) -> &EzObjects { self.open_modal.as_ref().unwrap() }

    /// Get reference to all open modals
    pub fn get_modal_mut(&mut self) -> &mut EzObjects { self.open_modal.as_mut().unwrap() }

    /// Set templates. Used by [ez_parser] on the root layout to keep a hold of all templates
    /// defined by the user. They can be used to instantiate e.g. popups at runtime.
    pub fn set_templates(&mut self, templates: Templates) {
        self.templates = templates
    }

    /// Get templates. Use on the root layout to get all templates defined by the user.
    /// They can be used to instantiate e.g. popups at runtime.
    pub fn get_templates(&self) -> &Templates { &self.templates }

}
