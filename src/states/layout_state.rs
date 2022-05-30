use std::collections::HashMap;
use crossterm::event::KeyCode;
use crate::common;
use crate::common::StateTree;
use crate::states::state::{self};
use crate::widgets::widget::EzObjects;


/// [State] implementation.
pub struct LayoutState {

    /// Position of this widget relative to its' parent [Layout]
    pub position: state::Coordinates,

    /// Absolute position of this widget on screen. Automatically propagated, do not set manually
    pub absolute_position: state::Coordinates,

    /// Relative height/width of this widget to parent layout
    pub size_hint: state::SizeHint,

    /// Pos hint of this widget
    pub pos_hint: state::PosHint,

    /// size of this widget
    pub size: state::Size,

    /// Automatically adjust size of widget to content
    pub auto_scale: state::AutoScale,

    /// Amount of space to leave between sides of the widget and other widgets
    pub padding: state::Padding,

    /// Horizontal alignment of this widget
    pub halign: state::HorizontalAlignment,

    /// Vertical alignment of this widget
    pub valign: state::VerticalAlignment,

    /// Bool representing whether this layout should be filled with [filler_symbol] in positions
    /// where it does not get other content from [get_contents]
    pub fill: bool,

    /// The [Pixel.Symbol] to use for filler pixels if [fill] is true
    pub filler_symbol: String,

    /// Bool representing whether this layout should have a surrounding border
    pub border: bool,

    /// [BorderConfig] object that will be used to draw the border if enabled
    pub border_config: state::BorderConfig,

    /// Object containing colors to be used by this widget in different situations
    pub colors: state::ColorConfig,

    /// Bool representing if state has changed. Triggers widget redraw.
    pub changed: bool,

    /// A list of open modals. Modals are widgets that overlap other content; in other words, they
    /// open 'in front of' other content. Only one can be shown at a time (the first on in the
    /// list).
    pub open_modals: Vec<EzObjects>,

    /// A hashmap of 'Template Name > [EzWidgetDefinition]'. Used to instantiate widget templates
    /// at runtime. E.g. when spawning popups.
    pub templates: common::Templates,

    /// [CallbackConfig] containing callbacks to be called in different situations
    pub callbacks: state::CallbackConfig,

    /// A Key to callback function lookup used to store keybinds for this widget. See
    /// [KeyboardCallbackFunction] type for callback function signature.
    pub keymap: common::KeyMap,

    /// If true this forces a global screen redraw on the next frame. Screen redraws are diffed
    /// so this can be called when needed without degrading performance. If only screen positions
    /// that fall within this widget must be redrawn, call [EzObject.redraw] instead.
    pub force_redraw: bool,
}
impl Default for LayoutState {
    fn default() -> Self {
        LayoutState {
            position: state::Coordinates::default(),
            absolute_position: state::Coordinates::default(),
            size_hint: state::SizeHint::default(),
            pos_hint: state::PosHint::default(),
            size: state::Size::default(),
            auto_scale: state::AutoScale::default(),
            padding: state::Padding::default(),
            halign: state::HorizontalAlignment::Left,
            valign: state::VerticalAlignment::Top,
            fill: false,
            filler_symbol: String::new(),
            border: false,
            border_config: state::BorderConfig::default(),
            colors: state::ColorConfig::default(),
            changed: false,
            open_modals: Vec::new(),
            templates: HashMap::new(),
            callbacks: state::CallbackConfig::default(),
            keymap: common::KeyMap::new(),
            force_redraw: false
        }
    }
}
impl state::GenericState for LayoutState {

    fn set_changed(&mut self, changed: bool) { self.changed = changed }

    fn get_changed(&self) -> bool { self.changed }

    fn set_size_hint(&mut self, size_hint: state::SizeHint) {
        self.size_hint = size_hint;
    }

    fn get_size_hint(&self) -> &state::SizeHint { &self.size_hint }

    fn set_pos_hint(&mut self, pos_hint: state::PosHint) {
        if self.pos_hint != pos_hint { self.changed = true }
        self.pos_hint = pos_hint;
    }

    fn get_pos_hint(&self) -> &state::PosHint { &self.pos_hint }

    fn set_auto_scale(&mut self, auto_scale: state::AutoScale) {
        if self.auto_scale != auto_scale { self.changed = true }
        self.auto_scale = auto_scale;
    }

    fn get_auto_scale(&self) -> &state::AutoScale { &self.auto_scale }

    fn set_size(&mut self, size: state::Size) {
        self.size = size;
    }

    fn get_size(&self) -> &state::Size { &self.size  }

    fn set_position(&mut self, position: state::Coordinates) {
        self.position = position;
    }

    fn get_position(&self) -> state::Coordinates { self.position }

    fn set_absolute_position(&mut self, pos: state::Coordinates) {
        if self.absolute_position != pos { self.changed = true }
        self.absolute_position = pos;
    }

    fn get_absolute_position(&self) -> state::Coordinates { self.absolute_position }

    fn set_callbacks(&mut self, config: state::CallbackConfig) {
        self.callbacks = config;
    }

    fn get_callbacks(&self) -> &state::CallbackConfig { &self.callbacks }

    fn get_callbacks_mut(&mut self) -> &mut state::CallbackConfig {
        &mut self.callbacks
    }

    fn get_key_map(&self) -> &common::KeyMap { &self.keymap }

    fn bind_key(&mut self, key: KeyCode, func: common::KeyboardCallbackFunction) {
        self.keymap.insert(key, func);
    }

    fn set_horizontal_alignment(&mut self, alignment: state::HorizontalAlignment) {
        if self.halign != alignment { self.changed = true }
        self.halign = alignment;
    }

    fn get_horizontal_alignment(&self) -> state::HorizontalAlignment { self.halign }

    fn set_vertical_alignment(&mut self, alignment: state::VerticalAlignment) {
        if self.valign != alignment { self.changed = true }
        self.valign = alignment;
    }

    fn get_vertical_alignment(&self) -> state::VerticalAlignment { self.valign }

    fn set_padding(&mut self, padding: state::Padding) {
        if self.padding != padding { self.changed = true }
        self.padding = padding;
    }

    fn get_padding(&self) -> &state::Padding { &self.padding }

    fn has_border(&self) -> bool { self.border }

    fn set_border(&mut self, enabled: bool) {
        if self.border != enabled { self.changed = true }
        self.border = enabled;
    }

    fn set_border_config(&mut self, config: state::BorderConfig) {
        if self.border_config != config { self.changed = true }
        self.border_config = config;
    }

    fn get_border_config(&self) -> &state::BorderConfig { &self.border_config  }

    fn set_colors(&mut self, config: state::ColorConfig) {
        if self.colors != config { self.changed = true }
        self.colors = config;
    }

    fn get_colors(&self) -> &state::ColorConfig { &self.colors }

    fn get_colors_mut(&mut self) -> &mut state::ColorConfig {
        self.changed = true;
        &mut self.colors
    }

    fn set_force_redraw(&mut self, redraw: bool) {
        self.force_redraw = redraw;
        self.changed = true;
    }

    fn get_force_redraw(&self) -> bool { self.force_redraw }
}
impl LayoutState {

    /// Set [filler_symbol]
    pub fn set_filler_symbol(&mut self, symbol: String) {
        if self.filler_symbol != symbol { self.changed = true }
        self.filler_symbol = symbol;
    }

    /// Get [filler_symbol]
    pub fn get_filler_symbol(&self) -> String { self.filler_symbol.clone() }

    /// Open a popup based on a template defined in the Ez file. Returns the state of the new popup
    pub fn open_popup(&mut self, template: String) -> (String, StateTree) {
        let mut popup = self.templates.get_mut(&template).unwrap().clone();
        let init_popup = popup.parse(&mut self.templates);
        self.force_redraw = true;
        self.open_modal(init_popup)
    }
    
    /// Open a new modal. Returns the state of the new modal.
    pub fn open_modal(&mut self, mut modal: EzObjects) -> (String, StateTree) {
        let modal_path = format!("/modal/{}", modal.as_ez_object().get_id());
        modal.as_ez_object_mut().set_full_path(modal_path.clone());
        let mut extra_state_tree;
        if let EzObjects::Layout(ref mut i) = modal {
            i.propagate_paths();
            extra_state_tree = common::get_state_tree(i);
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
    pub fn get_modals(&self) -> &Vec<EzObjects> {
        &self.open_modals
    }
    
    /// Get mutable reference to all open modals
    pub fn get_modals_mut(&mut self) -> &mut Vec<EzObjects> {
        self.changed = true;
        &mut self.open_modals
    }

    /// Set templates. Used by [ez_parser] on the root layout to keep a hold of all templates
    /// defined by the user. They can be used to instantiate e.g. popups at runtime.
    pub fn set_templates(&mut self, templates: common::Templates) {
        self.templates = templates
    }

    /// Get templates. Use on the root layout to get all templates defined by the user.
    /// They can be used to instantiate e.g. popups at runtime.
    pub fn get_templates(&self) -> &common::Templates { &self.templates }

}
