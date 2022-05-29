use crate::states::state::{self};
use crate::widgets::widget;
use crate::widgets::widget::EzObjects;


/// [State] implementation.
#[derive(Clone)]
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
            force_redraw: false
        }
    }
}
impl state::GenericState for LayoutState {

    fn set_changed(&mut self, changed: bool) { self.changed = changed }

    fn get_changed(&self) -> bool { self.changed }

    fn set_size_hint(&mut self, size_hint: state::SizeHint) {
        self.size_hint = size_hint;
        self.changed = true;
    }

    fn get_size_hint(&self) -> &state::SizeHint { &self.size_hint }

    fn set_pos_hint(&mut self, pos_hint: state::PosHint) {
        self.pos_hint = pos_hint;
        self.changed = true;
    }

    fn get_pos_hint(&self) -> &state::PosHint { &self.pos_hint }

    fn set_auto_scale(&mut self, auto_scale: state::AutoScale) {
        self.auto_scale = auto_scale;
        self.changed = true;
    }

    fn get_auto_scale(&self) -> &state::AutoScale { &self.auto_scale }

    fn set_size(&mut self, size: state::Size) {
        self.size = size;
        self.changed = true;
    }

    fn get_size(&self) -> &state::Size { &self.size  }

    fn set_position(&mut self, position: state::Coordinates) {
        self.position = position;
        self.changed = true;
    }

    fn get_position(&self) -> state::Coordinates { self.position }

    fn set_absolute_position(&mut self, pos: state::Coordinates) {
        self.absolute_position = pos;
        self.changed = true;
    }

    fn get_absolute_position(&self) -> state::Coordinates { self.absolute_position }

    fn set_horizontal_alignment(&mut self, alignment: state::HorizontalAlignment) {
        self.halign = alignment;
        self.changed = true;
    }

    fn get_horizontal_alignment(&self) -> state::HorizontalAlignment { self.halign }

    fn set_vertical_alignment(&mut self, alignment: state::VerticalAlignment) {
        self.valign = alignment;
        self.changed = true;
    }

    fn get_vertical_alignment(&self) -> state::VerticalAlignment { self.valign }

    fn set_padding(&mut self, padding: state::Padding) {
        self.padding = padding;
        self.changed = true;
    }

    fn get_padding(&self) -> &state::Padding { &self.padding }

    fn has_border(&self) -> bool { self.border }

    fn set_border(&mut self, enabled: bool) {
        self.border = enabled;
        self.changed = true;
    }

    fn set_border_config(&mut self, config: state::BorderConfig) {
        self.border_config = config;
        self.changed = true;
    }

    fn get_border_config(&self) -> &state::BorderConfig { &self.border_config  }

    fn set_colors(&mut self, config: state::ColorConfig) {
        self.colors = config;
        self.changed = true;
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
        self.filler_symbol = symbol; 
        self.changed = true;
    }

    /// Get [filler_symbol]
    pub fn get_filler_symbol(&self) -> String { self.filler_symbol.clone() }
    
    /// Open a new modal
    pub fn open_modal(&mut self, modal: EzObjects) {
        self.open_modals.push(modal);
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
}
