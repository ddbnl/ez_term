//! # Layout
//! Module implementing the Layout struct.

use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use crossterm::style::Color;
use crate::ez_parser::{load_bool_parameter, load_color_parameter};
use crate::widgets::widget::{Pixel, EzObject, EzObjects};
use crate::widgets::state::{State, GenericState};
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
            fill: false,
            filler_symbol: String::new(),
            border: false,
            border_horizontal_symbol: "━".to_string(),
            border_vertical_symbol: "│".to_string(),
            border_top_left_symbol: "┌".to_string(),
            border_top_right_symbol: "┐".to_string(),
            border_bottom_left_symbol: "└".to_string(),
            border_bottom_right_symbol: "┘".to_string(),
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
    pub width: usize,

    /// Height of this layout
    pub height: usize,

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
            width: 0,
            height: 0,
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

    fn set_width(&mut self, width: usize) { self.width = width; self.changed = true; }

    fn get_width(&self) -> usize { self.width }

    fn set_height(&mut self, height: usize) { self.height = height; self.changed = true; }

    fn get_height(&self) -> usize { self.height }

    fn set_position(&mut self, position: Coordinates) {
        self.x = position.0;
        self.y = position.1;
        self.changed = true;
    }

    fn get_position(&self) -> Coordinates { (self.x, self.y) }

    fn set_force_redraw(&mut self, redraw: bool) {
        self.force_redraw = redraw;
        self.changed = true;
    }

    fn get_force_redraw(&self) -> bool { self.force_redraw }
}
impl LayoutState {

    pub fn set_border_foreground_color(&mut self, color: Color) {
        self.border_foreground_color = color;
        self.changed = true;
    }

    pub fn get_border_foreground_color(&self) -> Color {
        self.border_foreground_color
    }

    pub fn set_border_background_color(&mut self, color: Color) {
        self.border_background_color = color;
        self.changed = true;
    }

    pub fn get_border_background_color(&self) -> Color {
        self.border_background_color
    }

    pub fn set_content_foreground_color(&mut self, color: Color) {
        self.content_foreground_color = color;
        self.changed = true;
    }

    pub fn get_content_foreground_color(&self) -> Color {
        self.content_foreground_color
    }

    pub fn set_content_background_color(&mut self, color: Color) {
        self.content_background_color = color;
        self.changed = true;
    }

    pub fn get_content_background_color(&self) -> Color {
        self.content_background_color
    }

    pub fn set_filler_foreground_color(&mut self, color: Color) {
        self.filler_foreground_color = color;
        self.changed = true;
    }

    pub fn get_filler_foreground_color(&self) -> Color {
        self.filler_foreground_color
    }

    pub fn set_filler_background_color(&mut self, color: Color) {
        self.filler_background_color = color;
        self.changed = true;
    }

    pub fn get_filler_background_color(&self) -> Color {
        self.filler_background_color
    }
}

impl EzObject for Layout {

    fn load_ez_parameter(&mut self, parameter_name: String, parameter_value: String)
        -> Result<(), Error> {
        match parameter_name.as_str() {
            "x" => self.state.x = parameter_value.trim().parse().unwrap(),
            "y" => self.state.y = parameter_value.trim().parse().unwrap(),
            "width" => self.state.width = parameter_value.trim().parse().unwrap(),
            "height" => self.state.height = parameter_value.trim().parse().unwrap(),
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
            "border" => self.set_border(load_bool_parameter(parameter_value.trim())?),
            "borderHorizontalSymbol" => self.border_horizontal_symbol =
                parameter_value.trim().to_string(),
            "borderVerticalSymbol" => self.border_vertical_symbol =
                parameter_value.trim().to_string(),
            "borderTopRightSymbol" => self.border_top_right_symbol =
                parameter_value.trim().to_string(),
            "borderTopLeftSymbol" => self.border_top_left_symbol =
                parameter_value.trim().to_string(),
            "borderBottomLeftSymbol" => self.border_bottom_left_symbol =
                parameter_value.trim().to_string(),
            "borderBottomRightSymbol" => self.border_bottom_right_symbol =
                parameter_value.trim().to_string(),
            "borderForegroundColor" =>
                self.state.border_foreground_color = load_color_parameter(parameter_value).unwrap(),
            "borderBackgroundColor" =>
                self.state.border_background_color = load_color_parameter(parameter_value).unwrap(),
            "fillerForegroundColor" =>
                self.state.filler_foreground_color = load_color_parameter(parameter_value).unwrap(),
            "fillerBackgroundColor" =>
                self.state.filler_background_color = load_color_parameter(parameter_value).unwrap(),
            "fill" => self.fill = load_bool_parameter(parameter_value.trim())?,
            "fillerSymbol" => self.set_filler_symbol(parameter_value.trim().to_string()),
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
                if self.fill {
                    merged_content = self.fill(merged_content);
                }
            },
            LayoutMode::Float => {
                merged_content = self.get_float_mode_contents(merged_content, state_tree);
            }
        }
        if self.border {
            merged_content = common::add_border(merged_content,
                                          self.border_horizontal_symbol.clone(),
                                          self.border_vertical_symbol.clone(),
                                          self.border_top_left_symbol.clone(),
                                          self.border_top_right_symbol.clone(),
                                          self.border_bottom_left_symbol.clone(),
                                          self.border_bottom_right_symbol.clone(),
                                          self.state.border_background_color,
                                          self.state.border_foreground_color);
        }
        merged_content
    }

    fn set_absolute_position(&mut self, pos: Coordinates) { self.absolute_position = pos }

    fn get_absolute_position(&self) -> Coordinates { self.absolute_position }

    fn set_border_horizontal_symbol(&mut self, symbol: String) {
        self.border_horizontal_symbol = symbol }
    fn get_border_horizontal_symbol(&self) -> String { self.border_horizontal_symbol.clone() }

    fn set_border_vertical_symbol(&mut self, symbol: String) {
        self.border_vertical_symbol = symbol }

    fn get_border_vertical_symbol(&self) -> String { self.border_vertical_symbol.clone() }

    fn set_border_bottom_left_symbol(&mut self, symbol: String) {
        self.border_bottom_left_symbol = symbol }

    fn get_border_bottom_left_symbol(&self) -> String { self.border_bottom_left_symbol.clone() }

    fn set_border_bottom_right_symbol(&mut self, symbol: String) {
        self.border_bottom_right_symbol = symbol }

    fn get_border_bottom_right_symbol(&self) -> String { self.border_bottom_right_symbol.clone() }

    fn set_border_top_left_symbol(&mut self, symbol: String) {
        self.border_top_left_symbol = symbol }

    fn get_border_top_left_symbol(&self) -> String { self.border_top_left_symbol.clone() }

    fn set_border_top_right_symbol(&mut self, symbol: String) {
        self.border_top_right_symbol = symbol }

    fn get_border_top_right_symbol(&self) -> String { self.border_top_right_symbol.clone() }

    fn set_border(&mut self, enabled: bool) { self.border = enabled }

    fn has_border(&self) -> bool { self.border }
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
        
        let mut position: Coordinates = (0, 0);
        if self.has_border() {
            position = (1, 1);
        }
        for child in self.get_children() {
            let generic_child= child.as_ez_object();
            let state = state_tree.get_mut(&generic_child.get_full_path())
                .unwrap().as_generic_mut();
            state.set_position(position);
            let child_content = generic_child.get_contents(state_tree);
            content = merge_horizontal_contents( content, child_content);
            position = (content.len(), 0);
        }
        content
    }

    /// Used by [get_contents] when the [LayoutMode] is set to [Box] and [LayoutOrientation] is
    /// set to [Vertical]. Merges contents of sub layouts and/or widgets vertically, using
    /// own [width] for each.
    fn get_box_mode_vertical_orientation_contents(&self, mut content: PixelMap,
        state_tree: &mut StateTree) -> PixelMap {

        for _ in 0..self.state.get_width() {
            content.push(Vec::new())
        }
        let mut position:Coordinates = (0, 0);
        for child in self.get_children() {
            let generic_child= child.as_ez_object();
            let state = state_tree.get_mut(&generic_child.get_full_path())
                .unwrap().as_generic_mut();
            state.set_position(position);
            let child_content = generic_child.get_contents(state_tree);
            content = merge_vertical_contents(content, child_content);
            position = (0, content[0].len());
        }
        content
    }

    /// Used by [get_contents] when the [LayoutMode] is set to [Float]. Places each child in the
    /// XY coordinates defined by that child, relative to itself, and uses
    /// childs' [width] and [height].
    fn get_float_mode_contents(&self, mut content: PixelMap, state_tree: &mut StateTree)
        -> PixelMap {

        for _ in 0..self.state.get_width() {
            content.push(Vec::new());
            for _ in 0..self.state.get_height() {
                if self.fill {
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
            let (mut child_width, mut child_height) = (child_state.get_width(),
                                                                   child_state.get_height());

            let child_content = generic_child.get_contents(state_tree);
            child_width = if generic_child.has_border() { child_width + 2}
            else { child_width };
            child_height = if generic_child.has_border() { child_height + 2}
            else { child_height};
            for width in 0..child_width {
                for height in 0..child_height {
                    content[child_x + width][child_y + height] =
                        child_content[width][height].clone();
                }
            }
        }
        content
    }

    /// Takes [absolute_position] of this layout and adds the [x][y] of children to calculate and
    /// set their [absolute_position]. Then calls this method on children, thus recursively setting
    /// [absolute_position] for all children. Use on root layout to propagate all absolute positions.
    pub fn propagate_absolute_positions(&mut self) {
        let absolute_position = self.get_absolute_position();
        let border_offset = if self.has_border() {1} else {0};
        for child in self.get_children_mut() {
            if let EzObjects::Layout(i) = child {
                let pos = i.state.get_position();
                let new_absolute_position =
                    (absolute_position.0 + pos.0 + border_offset,
                     absolute_position.1 + pos.1 + border_offset);
                i.set_absolute_position(new_absolute_position);
                i.propagate_absolute_positions();
            } else {
                let generic_child = child.as_ez_object_mut();
                let pos = generic_child.get_state().as_generic_state().get_position();
                generic_child.set_absolute_position((absolute_position.0 + pos.0 + border_offset,
                                                     absolute_position.1 + pos.1 + border_offset));
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
        Pixel{symbol: self.get_filler_symbol(),
            foreground_color: self.get_filler_foreground_color(),
            background_color: self.get_filler_background_color(),
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

    /// Set [filler_symbol]
    pub fn set_filler_symbol(&mut self, symbol: String) { self.filler_symbol = symbol; }

    /// Get [filler_symbol]
    pub fn get_filler_symbol(&self) -> String { self.filler_symbol.clone() }

    /// Set [filler_background_color]
    pub fn get_filler_background_color(&self) -> Color { self.state.filler_background_color }

    /// Set [filler_foreground_color]
    pub fn get_filler_foreground_color(&self) -> Color { self.state.filler_foreground_color }

    /// Fill any empty positions with [Pixel] from [get_filler]
    pub fn fill(&self, mut contents: PixelMap) -> PixelMap {
        for x in 0..self.state.get_width() {
            while contents[x].len() < self.state.get_height() {
                contents[x].push(self.get_filler().clone());
            }
        }
        while contents.len() < self.state.get_width() {
            let mut new_x = Vec::new();
            for _ in 0..self.state.get_height() {
                new_x.push(self.get_filler().clone());
            }
            contents.push(new_x);
        }
        contents
    }
}


/// Take a [PixelMap] and merge it horizontally with another [PixelMap]
pub fn merge_horizontal_contents(mut merged_content: PixelMap, new: PixelMap) -> PixelMap {
    for x in 0..new.len() {
        merged_content.push(new[x].clone());
    }
    merged_content
}

/// Take a [PixelMap] and merge it vertically with another [PixelMap]
pub fn merge_vertical_contents(mut merged_content: PixelMap, new: PixelMap) -> PixelMap {
    if new.is_empty() {
        return merged_content
    }
    for x in 0..new.len() {
        for y in 0..new[0].len() {
            merged_content[x].push(new[x][y].clone())
        }
    }
    merged_content
}
