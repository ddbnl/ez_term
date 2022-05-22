//! # Layout
//! Module implementing the Layout struct.

use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use crossterm::style::Color;
use crate::ez_parser::{load_bool_parameter, load_color_parameter, load_size_hint,
                       load_halign_parameter, load_valign_parameter};
use crate::widgets::widget::{Pixel, EzObject, EzObjects};
use crate::widgets::state::{State, GenericState, HorizontalAlignment, VerticalAlignment};
use crate::common::{self, PixelMap, StateTree, WidgetTree, Coordinates};


/// Used with Box mode, determines whether widgets are placed below or above each other.
pub enum LayoutOrientation {
    Horizontal,
    Vertical
}


/// Different modes determining how widgets are placed in this layout.
pub enum LayoutMode {
    /// # Box mode:
    /// Widgets are placed next to each other or under one another depending on orientation.
    /// In horizontal orientation widgets always use the full height of the layout, and in
    /// vertical position they always use the full with.
    Box,
    /// Widgets are placed in their hardcoded XY positions.
    Float,
    // todo table
    // todo stack
}


/// A layout is where widgets live. They implements methods for hardcoding widget placement or
/// placing them automatically in various ways.
pub struct Layout {

    /// ID of the layout, used to construct [path]
    pub id: String,

    /// Full path to this layout, e.g. "/root_layout/layout_2/THIS_ID"
    pub path: String,

    /// Layout mode enum, see [LayoutMode] for options
    pub mode: LayoutMode,

    /// Orientation enum, see [LayoutOrientation] for options
    pub orientation: LayoutOrientation,

    /// Absolute position of this layout on screen. Automatically propagated, do not set manually
    pub absolute_position: Coordinates,

    /// List of children widgets and/or layouts
    pub children: Vec<EzObjects>,

    /// Child ID to index in [children] lookup. Used to get widgets by [id] and [path]
    pub child_lookup: HashMap<String, usize>,

    /// Runtime state of this Layout, see [LayoutState] and [State]
    pub state: LayoutState,
}


impl Default for Layout {
    fn default() -> Self {
        Layout {
            id: "".to_string(),
            path: String::new(),
            orientation: LayoutOrientation::Horizontal,
            mode: LayoutMode::Box,
            absolute_position: (0, 0),
            children: Vec::new(),
            child_lookup: HashMap::new(),
            state: LayoutState::default(),
        }
    }
}


/// [State] implementation.
#[derive(Clone)]
pub struct LayoutState {

    /// Horizontal position of this widget relative to its' parent [Layout]
    pub x: usize,

    /// Vertical position of this widget relative to its' parent [Layout]
    pub y: usize,

    /// Width of this widget
    pub size_hint_x: Option<f64>,

    /// Width of this widget
    pub size_hint_y: Option<f64>,

    /// Width of this widget
    pub width: usize,

    /// Height of this layout
    pub height: usize,

    /// Amount of space to leave between top edge and content
    pub padding_top: usize,

    /// Amount of space to leave between bottom edge and content
    pub padding_bottom: usize,

    /// Amount of space to leave between left edge and content
    pub padding_left: usize,

    /// Amount of space to leave between right edge and content
    pub padding_right: usize,

    /// Horizontal alignment of this widget
    pub halign: HorizontalAlignment,

    /// Vertical alignment of this widget
    pub valign: VerticalAlignment,

    /// Bool representing whether this layout should be filled with [filler_symbol] in positions
    /// where it does not get other content from [get_contents]
    pub fill: bool,

    /// The [Pixel.Symbol] to use for filler pixels if [fill] is true
    pub filler_symbol: String,

    /// Bool representing whether this layout should have a surrounding border
    pub border: bool,

    /// The [Pixel.symbol] to use for the horizontal border if [border] is true
    pub border_horizontal_symbol: String,

    /// The [Pixel.symbol] to use for the vertical border if [border] is true
    pub border_vertical_symbol: String,

    /// The [Pixel.symbol] to use for the top left border if [border] is true
    pub border_top_left_symbol: String,

    /// The [Pixel.symbol] to use for the top right border if [border] is true
    pub border_top_right_symbol: String,

    /// The [Pixel.symbol] to use for the bottom left border if [border] is true
    pub border_bottom_left_symbol: String,

    /// The [Pixel.symbol] to use for the bottom right border if [border] is true
    pub border_bottom_right_symbol: String,
    /// The [Pixel.foreground_color] to use for filler pixels if [fill] is true
    pub filler_foreground_color: Color,

    /// The [Pixel.background_color] to use for filler pixels if [fill] is true
    pub filler_background_color: Color,

    /// The[Pixel.foreground_color]  to use for the border if [border] is true
    pub border_foreground_color: Color,

    /// The [Pixel.background_color] to use for the border if [border] is true
    pub border_background_color: Color,

    /// The [Pixel.foreground_color] to use for this widgets' content
    pub content_foreground_color: Color,

    /// The [Pixel.background_color] to use for this widgets' content
    pub content_background_color: Color,

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
            x: 0,
            y: 0,
            size_hint_x: Some(1.0),
            size_hint_y: Some(1.0),
            width: 0,
            height: 0,
            padding_top: 0,
            padding_bottom: 0,
            padding_left: 0,
            padding_right: 0,
            halign: HorizontalAlignment::Left,
            valign: VerticalAlignment::Top,
            fill: false,
            filler_symbol: String::new(),
            border: false,
            border_horizontal_symbol: "━".to_string(),
            border_vertical_symbol: "│".to_string(),
            border_top_left_symbol: "┌".to_string(),
            border_top_right_symbol: "┐".to_string(),
            border_bottom_left_symbol: "└".to_string(),
            border_bottom_right_symbol: "┘".to_string(),
            filler_background_color: Color::Black,
            filler_foreground_color: Color::White,
            border_foreground_color: Color::White,
            border_background_color: Color::Black,
            content_background_color: Color::Black,
            content_foreground_color: Color::White,
            changed: false,
            force_redraw: false
        }
    }
}
impl GenericState for LayoutState {

    fn set_changed(&mut self, changed: bool) { self.changed = changed }

    fn get_changed(&self) -> bool { self.changed }

    fn set_size_hint_x(&mut self, size_hint: Option<f64>) {
        self.size_hint_x = size_hint;
        self.changed = true;
    }

    fn get_size_hint_x(&self) -> Option<f64> { self.size_hint_x }

    fn set_size_hint_y(&mut self, size_hint: Option<f64>) {
        self.size_hint_y = size_hint;
        self.changed = true;
    }

    fn get_size_hint_y(&self) -> Option<f64> { self.size_hint_y }

    fn set_width(&mut self, width: usize) { self.width = width; self.changed = true; }

    fn get_width(&self) -> usize { self.width }

    fn get_effective_width(&self) -> usize {
        self.get_width()
            -if self.has_border() {2} else {0} - self.padding_left - self.padding_right
    }

    fn set_height(&mut self, height: usize) { self.height = height; self.changed = true; }

    fn get_height(&self) -> usize { self.height }

    fn get_effective_height(&self) -> usize {
        self.get_height()
            -if self.has_border() {2} else {0} - self.padding_top - self.padding_bottom
    }

    fn set_position(&mut self, position: Coordinates) {
        self.x = position.0;
        self.y = position.1;
        self.changed = true;
    }

    fn get_position(&self) -> Coordinates { (self.x, self.y) }

    fn get_effective_position(&self) -> Coordinates {
        (self.x +if self.has_border() {2} else {0},
         self.y +if self.has_border() {2} else {0})
    }

    fn set_horizontal_alignment(&mut self, alignment: HorizontalAlignment) {
        self.halign = alignment;
        self.changed = true;
    }

    fn get_horizontal_alignment(&self) -> HorizontalAlignment { self.halign }

    fn set_vertical_alignment(&mut self, alignment: VerticalAlignment) {
        self.valign = alignment;
        self.changed = true;
    }

    fn get_vertical_alignment(&self) -> VerticalAlignment { self.valign }

    fn set_force_redraw(&mut self, redraw: bool) {
        self.force_redraw = redraw;
        self.changed = true;
    }

    fn get_force_redraw(&self) -> bool { self.force_redraw }
}
impl LayoutState {

    pub fn set_padding_top(&mut self, padding: usize) {
        self.padding_top = padding;
        self.changed = true;
    }

    pub fn get_padding_top(&self) -> usize { self.padding_top }

    pub fn set_padding_bottom(&mut self, padding: usize) {
        self.padding_bottom = padding;
        self.changed = true;
    }

    pub fn get_padding_bottom(&self) -> usize { self.padding_bottom }

    pub fn set_padding_left(&mut self, padding: usize) {
        self.padding_left = padding;
        self.changed = true;
    }

    pub fn get_padding_left(&self) -> usize { self.padding_left }

    pub fn set_padding_right(&mut self, padding: usize) {
        self.padding_right = padding;
        self.changed = true;
    }

    pub fn get_padding_right(&self) -> usize { self.padding_right }

    /// Set [filler_symbol]
    pub fn set_filler_symbol(&mut self, symbol: String) { self.filler_symbol = symbol; }

    /// Get [filler_symbol]
    pub fn get_filler_symbol(&self) -> String { self.filler_symbol.clone() }

    pub fn set_border_horizontal_symbol(&mut self, symbol: String) {
        self.border_horizontal_symbol = symbol }

    pub fn get_border_horizontal_symbol(&self) -> String { self.border_horizontal_symbol.clone() }

    pub fn set_border_vertical_symbol(&mut self, symbol: String) {
        self.border_vertical_symbol = symbol }

    pub fn get_border_vertical_symbol(&self) -> String { self.border_vertical_symbol.clone() }

    pub fn set_border_bottom_left_symbol(&mut self, symbol: String) {
        self.border_bottom_left_symbol = symbol }

    pub fn get_border_bottom_left_symbol(&self) -> String { self.border_bottom_left_symbol.clone() }

    pub fn set_border_bottom_right_symbol(&mut self, symbol: String) {
        self.border_bottom_right_symbol = symbol }

    pub fn get_border_bottom_right_symbol(&self) -> String { self.border_bottom_right_symbol.clone() }

    pub fn set_border_top_left_symbol(&mut self, symbol: String) {
        self.border_top_left_symbol = symbol }

    pub fn get_border_top_left_symbol(&self) -> String { self.border_top_left_symbol.clone() }

    pub fn set_border_top_right_symbol(&mut self, symbol: String) {
        self.border_top_right_symbol = symbol }

    pub fn get_border_top_right_symbol(&self) -> String { self.border_top_right_symbol.clone() }

    pub fn set_border(&mut self, enabled: bool) { self.border = enabled }

    pub fn has_border(&self) -> bool { self.border }

    pub fn set_border_foreground_color(&mut self, color: Color) {
        self.border_foreground_color = color;
        self.changed = true;
    }

    pub fn get_border_foreground_color(&self) -> Color { self.border_foreground_color }

    pub fn set_border_background_color(&mut self, color: Color) {
        self.border_background_color = color;
        self.changed = true;
    }

    pub fn get_border_background_color(&self) -> Color { self.border_background_color }

    pub fn set_content_foreground_color(&mut self, color: Color) {
        self.content_foreground_color = color;
        self.changed = true;
    }

    pub fn get_content_foreground_color(&self) -> Color { self.content_foreground_color }

    pub fn set_content_background_color(&mut self, color: Color) {
        self.content_background_color = color;
        self.changed = true;
    }

    pub fn get_content_background_color(&self) -> Color { self.content_background_color }

    pub fn set_filler_foreground_color(&mut self, color: Color) {
        self.filler_foreground_color = color;
        self.changed = true;
    }

    pub fn get_filler_foreground_color(&self) -> Color { self.filler_foreground_color }

    pub fn set_filler_background_color(&mut self, color: Color) {
        self.filler_background_color = color;
        self.changed = true;
    }

    pub fn get_filler_background_color(&self) -> Color { self.filler_background_color }
}

impl EzObject for Layout {

    fn load_ez_parameter(&mut self, parameter_name: String, parameter_value: String)
        -> Result<(), Error> {

        match parameter_name.as_str() {
            "x" => self.state.x = parameter_value.trim().parse().unwrap(),
            "y" => self.state.y = parameter_value.trim().parse().unwrap(),
            "size_hint_x" => self.state.size_hint_x =
                load_size_hint(parameter_value.trim()).unwrap(),
            "size_hint_y" => self.state.size_hint_y =
                load_size_hint(parameter_value.trim()).unwrap(),
            "width" => self.state.width = parameter_value.trim().parse().unwrap(),
            "height" => self.state.height = parameter_value.trim().parse().unwrap(),
            "halign" =>
                self.state.halign =  load_halign_parameter(parameter_value.trim()).unwrap(),
            "valign" =>
                self.state.valign =  load_valign_parameter(parameter_value.trim()).unwrap(),
            "padding_top" => self.state.padding_top = parameter_value.trim().parse().unwrap(),
            "padding_bottom" => self.state.padding_bottom = parameter_value.trim().parse().unwrap(),
            "padding_left" => self.state.padding_left = parameter_value.trim().parse().unwrap(),
            "padding_right" => self.state.padding_right = parameter_value.trim().parse().unwrap(),
            "mode" => {
                match parameter_value.trim() {
                    "box" => self.set_mode(LayoutMode::Box),
                    "float" => self.set_mode(LayoutMode::Float),
                    _ => return Err(Error::new(ErrorKind::InvalidData,
                                        format!("Invalid parameter value for mode {}",
                                                parameter_value)))
                }
            },
            "orientation" => {
                match parameter_value.trim() {
                    "horizontal" => self.set_orientation(LayoutOrientation::Horizontal),
                    "vertical" => self.set_orientation(LayoutOrientation::Vertical),
                    _ => return Err(Error::new(ErrorKind::InvalidData,
                                        format!("Invalid parameter value for orientation {}",
                                                parameter_value)))
                }
            },
            "border" => self.state.set_border(load_bool_parameter(parameter_value.trim())?),
            "border_horizontal_symbol" => self.state.border_horizontal_symbol =
                parameter_value.trim().to_string(),
            "border_vertical_symbol" => self.state.border_vertical_symbol =
                parameter_value.trim().to_string(),
            "border_top_right_symbol" => self.state.border_top_right_symbol =
                parameter_value.trim().to_string(),
            "border_top_left_symbol" => self.state.border_top_left_symbol =
                parameter_value.trim().to_string(),
            "border_bottom_left_symbol" => self.state.border_bottom_left_symbol =
                parameter_value.trim().to_string(),
            "border_bottom_right_symbol" => self.state.border_bottom_right_symbol =
                parameter_value.trim().to_string(),
            "border_fg_color" =>
                self.state.border_foreground_color = load_color_parameter(parameter_value).unwrap(),
            "border_bg_color" =>
                self.state.border_background_color = load_color_parameter(parameter_value).unwrap(),
            "filler_fg_color" =>
                self.state.filler_foreground_color = load_color_parameter(parameter_value).unwrap(),
            "filler_bg_color" =>
                self.state.filler_background_color = load_color_parameter(parameter_value).unwrap(),
            "fill" => self.state.fill = load_bool_parameter(parameter_value.trim())?,
            "filler_symbol" => self.state.set_filler_symbol(parameter_value.trim().to_string()),
            _ => return Err(Error::new(ErrorKind::InvalidData,
                                format!("Invalid parameter name for layout {}",
                                        parameter_name)))
        }
        Ok(())
    }
    fn set_id(&mut self, id: String) { self.id = id; }

    fn get_id(&self) -> String { self.id.clone() }

    fn set_full_path(&mut self, path: String) { self.path = path }

    fn get_full_path(&self) -> String { self.path.clone() }

    fn update_state(&mut self, new_state: &State) {
        let state = new_state.as_layout();
        self.state = state.clone();
        self.state.changed = false;
        self.state.force_redraw = false;
    }

    fn get_state(&self) -> State {
        State::Layout(self.state.clone())
    }

    fn get_contents(&self, state_tree: &mut StateTree) -> PixelMap {

        let mut merged_content = PixelMap::new();
        match self.get_mode() {
            LayoutMode::Box => {
                match self.get_orientation() {
                    LayoutOrientation::Horizontal => {
                        merged_content = self.get_box_mode_horizontal_orientation_contents(
                            merged_content, state_tree);
                    },
                    LayoutOrientation::Vertical => {
                        merged_content = self.get_box_mode_vertical_orientation_contents(
                            merged_content, state_tree);
                    },
                }
            },
            LayoutMode::Float => {
                merged_content = self.get_float_mode_contents(merged_content, state_tree);
            }
        }
        let state = state_tree.get_mut(&self.get_full_path()).unwrap()
            .as_layout_mut();
        // Fill empty spaces with user defined filling
        if state.fill {
            merged_content = self.fill(merged_content, state);
        }
        // If widget still doesn't fill its' height and/or width fill it with empty pixels
        while merged_content.len() < state.get_effective_width() {
            merged_content.push(Vec::new());
        }
        for x in merged_content.iter_mut() {
            while x.len() < state.get_effective_height() {
                x.push(Pixel { symbol: " ".to_string(),
                    foreground_color: state.content_foreground_color,
                    background_color: state.content_background_color,
                    underline: false});
            }
        }
        // Put padding around content if set
        merged_content = common::add_padding(merged_content, state.padding_top,
                                             state.padding_bottom,
                                             state.padding_left, state.padding_right,
                                      state.content_background_color,
                                      state.content_foreground_color);
        // Put border around content if border if set
        if state.border {
            merged_content = common::add_border(merged_content,
                                          state.border_horizontal_symbol.clone(),
                                          state.border_vertical_symbol.clone(),
                                          state.border_top_left_symbol.clone(),
                                          state.border_top_right_symbol.clone(),
                                          state.border_bottom_left_symbol.clone(),
                                          state.border_bottom_right_symbol.clone(),
                                          state.border_background_color,
                                          state.border_foreground_color);
        }
        merged_content
    }

    fn set_absolute_position(&mut self, pos: Coordinates) { self.absolute_position = pos }

    fn get_absolute_position(&self) -> Coordinates { self.absolute_position }

    fn get_effective_absolute_position(&self) -> Coordinates {
        let (x, y) = self.get_absolute_position();
        (x +if self.state.has_border() {1} else {0}, y +if self.state.has_border() {1} else {0})
    }
}

impl Layout {

    /// Initialize an instance of a Layout with the passed config parsed by [ez_parser]
    pub fn from_config(config: Vec<&str>, id: String) -> Self {
        let mut obj = Layout::default();
        obj.set_id(id);
        obj.load_ez_config(config).unwrap();
        obj
    }

    /// Used by [get_contents] when the [LayoutMode] is set to [Box] and [LayoutOrientation] is
    /// set to [Horizontal]. Merges contents of sub layouts and/or widgets horizontally, using
    /// own [height] for each.
    fn get_box_mode_horizontal_orientation_contents(&self, mut content: PixelMap,
        state_tree: &mut StateTree) -> PixelMap {

        let own_state = state_tree.get_mut(&self.get_full_path())
            .unwrap().as_layout().clone();
        let mut position: Coordinates = (0, 0);
        if own_state.has_border() {
            position = (1, 1);
        }

        for child in self.get_children() {
            let generic_child= child.as_ez_object();
            let state = state_tree.get_mut(&generic_child.get_full_path())
                .unwrap().as_generic_mut();
            let valign = state.get_vertical_alignment();
            state.set_position(position);
            let child_content = generic_child.get_contents(state_tree);
            content = self.merge_horizontal_contents(
                content, child_content,
                &own_state,
                state_tree.get_mut(&generic_child.get_full_path()).unwrap().as_generic_mut(),
                valign);
            position = (content.len(), 0);
        }
        content
    }

    /// Used by [get_contents] when the [LayoutMode] is set to [Box] and [LayoutOrientation] is
    /// set to [Vertical]. Merges contents of sub layouts and/or widgets vertically, using
    /// own [width] for each.
    fn get_box_mode_vertical_orientation_contents(&self, mut content: PixelMap,
        state_tree: &mut StateTree) -> PixelMap {

        let own_state = state_tree.get(&self.get_full_path()).unwrap().as_layout().clone();
        let own_width = own_state.get_effective_width();
        for _ in 0..own_width {
            content.push(Vec::new())
        }
        let mut position: Coordinates = (0, 0);
        for child in self.get_children() {
            let generic_child= child.as_ez_object();
            let state = state_tree.get_mut(&generic_child.get_full_path())
                .unwrap().as_generic_mut();
            state.set_position(position);
            let halign = state.get_horizontal_alignment();
            let child_content = generic_child.get_contents(state_tree);
            content = self.merge_vertical_contents(
                content, child_content,&own_state,
                state_tree.get_mut(&generic_child.get_full_path()).unwrap().as_generic_mut(),
                halign);
            position = (0, content[0].len());
        }
        content
    }

    /// Used by [get_contents] when the [LayoutMode] is set to [Float]. Places each child in the
    /// XY coordinates defined by that child, relative to itself, and uses
    /// childs' [width] and [height].
    fn get_float_mode_contents(&self, mut content: PixelMap, state_tree: &mut StateTree)
        -> PixelMap {

        let own_state = state_tree.get_mut(&self.get_full_path()).unwrap().as_layout();
        for _ in 0..own_state.get_effective_width() {
            content.push(Vec::new());
            for _ in 0..own_state.get_effective_height() {
                if own_state.fill {
                    content.last_mut().unwrap().push(self.get_filler());
                } else {
                    content.last_mut().unwrap().push(Pixel{symbol:" ".to_string(),
                            background_color: Color::Black, foreground_color: Color::White,
                            underline: false});
                }
            }
        }
        for child in self.get_children() {
            let generic_child = child.as_ez_widget();
            let child_state = state_tree.get(
                &generic_child.get_full_path()).unwrap().as_generic_state();

            let (child_x, child_y) = child_state.get_position();
            let (child_width, child_height) = (child_state.get_width(),
                                                                   child_state.get_height());

            let child_content = generic_child.get_contents(state_tree);
            for width in 0..child_width {
                for height in 0..child_height {
                    content[child_x + width][child_y + height] =
                        child_content[width][height].clone();
                }
            }
        }
        content
    }

    /// Set the sizes of children that use size_hint(s) using own proportions.
    pub fn set_child_sizes(&self, state_tree: &mut StateTree) {

        let own_state =state_tree.get_mut(&self.get_full_path())
            .unwrap().as_layout();
        let own_width = own_state.get_effective_width();
        let own_height = own_state.get_effective_height();
        for child in self.get_children() {
            let generic_child = child.as_ez_object();
            let state = state_tree.get_mut(&generic_child.get_full_path())
                .unwrap().as_generic_mut();

            if let Some(size_hint_x) = state.get_size_hint_x() {
                let raw_child_size = own_width as f64 * size_hint_x;
                let child_size = raw_child_size.round() as usize;
                state.set_width(child_size);
            }

            if let Some(size_hint_y) = state.get_size_hint_y() {
                let raw_child_size = own_height as f64 * size_hint_y;
                let child_size = raw_child_size.round() as usize;
                state.set_height(child_size);
            }
        }
        for child in self.get_children() {
            if let EzObjects::Layout(i) = child {
                i.set_child_sizes(state_tree)
            }
        }
    }

    /// Takes [absolute_position] of this layout and adds the [x][y] of children to calculate and
    /// set their [absolute_position]. Then calls this method on children, thus recursively setting
    /// [absolute_position] for all children. Use on root layout to propagate all absolute positions.
    pub fn propagate_absolute_positions(&mut self) {

        let absolute_position = self.get_effective_absolute_position();
        for child in self.get_children_mut() {
            if let EzObjects::Layout(i) = child {
                let pos = i.state.get_position();
                let new_absolute_position =
                    (absolute_position.0 + pos.0,
                     absolute_position.1 + pos.1);
                i.set_absolute_position(new_absolute_position);
                i.propagate_absolute_positions();
            } else {
                let generic_child = child.as_ez_object_mut();
                let pos = generic_child.get_state().as_generic_state().get_position();
                generic_child.set_absolute_position((absolute_position.0 + pos.0,
                                                     absolute_position.1 + pos.1));
            }
        }
    }

    /// Takes full [path] of this layout and adds the [id] of children to create and set
    /// their [path]. Then calls this method on children, thus recursively setting
    /// [path] for all children. Use on root layout to propagate all absolute positions.
    pub fn propagate_paths(&mut self) {
        let path = self.get_full_path();
        for child in self.get_children_mut() {
            if let EzObjects::Layout(i) = child {
                i.set_full_path(path.clone() + format!("/{}", i.get_id()).as_str());
                i.propagate_paths();
            } else {
                let generic_child = child.as_ez_object_mut();
                generic_child.set_full_path(path.clone() +
                    format!("/{}", generic_child.get_id()).as_str());
            }
        }
    }

    /// Get a filler pixel using symbol and color settings set for this layout.
    pub fn get_filler(&self) -> Pixel {
        Pixel{symbol: self.state.get_filler_symbol(),
            foreground_color: self.state.get_filler_foreground_color(),
            background_color: self.state.get_filler_background_color(),
            underline: false}
    }

    /// Add a child ([Layout] or [EzWidget]) to this Layout.
    pub fn add_child(&mut self, child: EzObjects) {
        let generic_child = child.as_ez_object();
        self.child_lookup.insert(generic_child.get_id(), self.children.len());
        self.children.push(child);
    }

    /// Get the State for each child [EzWidget] and return it in a <[path], [State]>
    /// HashMap.
    pub fn get_state_tree(&mut self) -> StateTree {
        let mut state_tree = HashMap::new();
        for (child_path, child) in self.get_widgets_recursive() {
            state_tree.insert(child_path, child.as_ez_object().get_state());
        }
        state_tree.insert(self.get_full_path(), self.get_state());
        state_tree
    }

    /// Get an EzWidget trait object for each child [EzWidget] and return it in a
    /// <[path], [EzObject]> HashMap.
    pub fn get_widget_tree(&self) -> WidgetTree {
        let mut widget_tree = WidgetTree::new();
        for (child_path, child) in self.get_widgets_recursive() {
            widget_tree.insert(child_path, child);
        }
        widget_tree

    }
    /// Get a list of children non-recursively. Can be [Layout] or [EzWidget]
    pub fn get_children(&self) -> &Vec<EzObjects> { &self.children }

    /// Get a mutable list of children non-recursively. Can be [Layout] or [EzWidget]
    pub fn get_children_mut(&mut self) -> &mut Vec<EzObjects> { &mut self.children }

    /// Get a specific child ref by its' [id]
    pub fn get_child(&self, id: & str) -> &EzObjects {
        let index = self.child_lookup.get(id)
            .unwrap_or_else(|| panic!("No child: {} in {}", id, self.get_id()));
        self.children.get(*index).unwrap()
    }

    /// Get a specific child mutable ref by its'[id]
    pub fn get_child_mut(&mut self, id: & str) -> &mut EzObjects {
        let index = self.child_lookup.get(id)
            .unwrap_or_else(|| panic!("No child: {} in {}", id, self.get_id()));
        self.children.get_mut(*index).unwrap()
    }

    /// Get a specific child ref by its' [path]. Call on root Layout to find any EzObject that
    /// exists
    pub fn get_child_by_path(&self, path: &str) -> Option<&EzObjects> {

        let mut paths: Vec<&str> = path.split('/').filter(|x| !x.is_empty()).collect();
        // If user passed a path starting with this layout, take it off first.
        if *paths.first().unwrap() == self.get_id() {
            paths.remove(0);
        }
        paths.reverse();
        let mut root = self.get_child(paths.pop().unwrap());
        while !paths.is_empty() {
            if let EzObjects::Layout(i) = root {
                root = i.get_child(paths.pop().unwrap());
            }
        }
        Some(root)
    }
    /// Get a specific child mutable ref by its' [path]. Call on root Layout to find any
    /// [EzObject] that exists
    pub fn get_child_by_path_mut(&mut self, path: &str) -> Option<&mut EzObjects> {

        let mut paths: Vec<&str> = path.split('/').filter(|x| !x.is_empty()).collect();
        if paths.first().unwrap() == &self.get_id() {
            paths.remove(0);
        }
        paths.reverse();
        let mut root = self.get_child_mut(paths.pop().unwrap());
        while !paths.is_empty() {
            if let EzObjects::Layout(i) = root {
                root = i.get_child_mut(paths.pop().unwrap());
            }
        }
        Some(root)
    }

    /// Get a list of all children refs recursively. Call on root [Layout] for all [EzWidgets] that
    /// exist.
    pub fn get_widgets_recursive(&self) -> HashMap<String, &EzObjects> {

        let mut results = HashMap::new();
        for child in self.get_children() {
            if let EzObjects::Layout(i) = child {
                for (sub_child_path, sub_child) in i.get_widgets_recursive() {
                    results.insert(sub_child_path, sub_child);
                }
                results.insert(child.as_ez_object().get_full_path(), child);
            } else {
                results.insert(child.as_ez_object().get_full_path(), child);
            }
        }
        results
    }

    /// Set [LayoutMode]
    pub fn set_mode(&mut self, mode: LayoutMode) { self.mode = mode }

    /// Get [LayoutMode]
    pub fn get_mode(&self) -> &LayoutMode { &self.mode }

    /// Set [LayoutOrientation]
    pub fn set_orientation(&mut self, orientation: LayoutOrientation) { self.orientation = orientation }

    /// Get [LayoutOrientation]
    pub fn get_orientation(&self) -> &LayoutOrientation { &self.orientation  }

    /// Fill any empty positions with [Pixel] from [get_filler]
    pub fn fill(&self, mut contents: PixelMap, state: &mut LayoutState) -> PixelMap {

        for x in 0..(state.get_effective_width()) {
            for y in contents[x].iter_mut() {
                if y.symbol.is_empty() || y.symbol == " ".to_string() {
                    y.symbol = self.get_filler().symbol;
                }
            }
            while contents[x].len() < (state.get_effective_height()) {
                contents[x].push(self.get_filler().clone());
            }
        }
        while contents.len() < state.get_effective_width() {
            let mut new_x = Vec::new();
            for _ in 0..state.get_effective_height() {
                new_x.push(self.get_filler().clone());
            }
            contents.push(new_x);
        }
        contents
    }

    /// Take a [PixelMap] and merge it horizontally with another [PixelMap]
    pub fn merge_horizontal_contents(&self, mut merged_content: PixelMap, mut new: PixelMap,
                                     parent_state: &LayoutState, state: &mut dyn GenericState,
                                     valign: VerticalAlignment) -> PixelMap {

        let empty_pixel = Pixel { symbol: " ".to_string(),
            foreground_color: parent_state.content_foreground_color,
            background_color: parent_state.content_background_color, underline: false};

        let mut new_position = state.get_position();
        match valign {
            VerticalAlignment::Top => {
                // We align top by filling out empty space to the bottom
                for x in new.iter_mut() {
                    for _ in 0..parent_state.get_effective_height() - x.len() {
                        x.push(empty_pixel.clone());
                    }
                }
            },
            VerticalAlignment::Bottom => {
                // We align bottom by filling out empty space to the top
                for (i, x) in new.iter_mut().enumerate() {
                    for _ in 0..parent_state.get_effective_height() - x.len() {
                        x.insert(0, empty_pixel.clone());
                        if i == 0 {
                            new_position = (new_position.0, new_position.1 + 1);
                        }
                    }
                }
            },
            VerticalAlignment::Middle => {
                // We align in the middle by filling out empty space alternating top and bottom
                let mut switch = true;
                for (i, x) in new.iter_mut().enumerate() {
                    for _ in 0..parent_state.get_effective_height() - x.len() {
                        if switch {
                            x.push(empty_pixel.clone());
                            switch = !switch
                        } else {
                            x.insert(0, empty_pixel.clone());
                            if i == 0 {
                                new_position = (new_position.0, new_position.1 + 1);
                            }
                            switch = !switch
                        }
                    }
                }
            }
        }
        for x in 0..new.len() {
            merged_content.push(new[x].clone());
            let last = merged_content.last_mut().unwrap();
            while last.len() < parent_state.get_effective_height() {
                last.push(Pixel { symbol: " ".to_string(),
                    foreground_color: parent_state.content_foreground_color,
                    background_color: parent_state.content_background_color,
                    underline: false});
            }
        }
        state.set_position(new_position);
        merged_content
    }

    /// Take a [PixelMap] and merge it vertically with another [PixelMap]
    pub fn merge_vertical_contents(&self, mut merged_content: PixelMap, mut new: PixelMap,
                                   parent_state: &LayoutState, state: &mut dyn GenericState,
                                   halign: HorizontalAlignment) -> PixelMap {

        if new.is_empty() {
            return merged_content
        }

        let empty_pixel = Pixel { symbol: " ".to_string(),
            foreground_color: parent_state.content_foreground_color,
            background_color: parent_state.content_background_color, underline: false};

        let mut new_position = state.get_position();
        match halign {
            HorizontalAlignment::Left => {
                // We align left by filling out empty space to the right
                for _ in 0..parent_state.get_effective_width() - new.len() {
                    new.push(Vec::new());
                    for _ in 0..new[0].len() {
                        new.last_mut().unwrap().push(empty_pixel.clone());
                    }
                }
            },
            HorizontalAlignment::Right => {
                // We align right by filling out empty space from the left
                for _ in 0..parent_state.get_effective_width() - new.len() {
                    new.insert(0, Vec::new());
                    new_position = (new_position.0 + 1, new_position.1);
                    for _ in 0..new.last().unwrap().len() {
                        new.first_mut().unwrap().push(empty_pixel.clone());
                    }
                }
            },
            HorizontalAlignment::Center => {
                // We align in the center by filling out empty space alternating left and right
                let mut switch = true;
                for _ in 0..parent_state.get_effective_width() - new.len() {
                    if switch {
                        new.push(Vec::new());
                        for _ in 0..new[0].len() {
                            new.last_mut().unwrap().push(empty_pixel.clone());
                        }
                        switch = !switch;
                    } else {
                        new.insert(0, Vec::new());
                        new_position = (new_position.0 + 1, new_position.1);
                        for _ in 0..new.last().unwrap().len() {
                            new.first_mut().unwrap().push(empty_pixel.clone());
                        }
                        switch = !switch;
                    }
                }
            }
        }
        for x in 0..parent_state.get_effective_width() {
            for y in 0..new[0].len() {
                if x < new.len() {
                    merged_content[x].push(new[x][y].clone())
                } else {
                    merged_content[x].push(Pixel { symbol: " ".to_string(),
                        foreground_color: parent_state.content_foreground_color,
                        background_color: parent_state.content_background_color,
                        underline: false});
                }
            }
        }
        state.set_position(new_position);
        merged_content
    }
}
