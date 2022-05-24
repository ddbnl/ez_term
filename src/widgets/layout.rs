//! # Layout
//! Module implementing the Layout struct.
use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use crossterm::style::Color;
use crate::ez_parser::{load_bool_parameter, load_color_parameter, load_size_hint_parameter,
                       load_halign_parameter, load_valign_parameter, load_pos_hint_x_parameter,
                       load_pos_hint_y_parameter};
use crate::widgets::widget::{Pixel, EzObject, EzObjects};
use crate::states::layout_state::LayoutState;
use crate::states::state::{EzState, GenericState, HorizontalAlignment, VerticalAlignment,
                           HorizontalPositionHint, VerticalPositionHint};
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
            children: Vec::new(),
            child_lookup: HashMap::new(),
            state: LayoutState::default(),
        }
    }
}


impl EzObject for Layout {

    fn load_ez_parameter(&mut self, parameter_name: String, parameter_value: String)
        -> Result<(), Error> {

        match parameter_name.as_str() {
            "x" => self.state.x = parameter_value.trim().parse().unwrap(),
            "y" => self.state.y = parameter_value.trim().parse().unwrap(),
            "size_hint_x" => self.state.size_hint_x =
                load_size_hint_parameter(parameter_value.trim()).unwrap(),
            "size_hint_y" => self.state.size_hint_y =
                load_size_hint_parameter(parameter_value.trim()).unwrap(),
            "pos_hint_x" => self.state.set_pos_hint_x(
                load_pos_hint_x_parameter(parameter_value.trim()).unwrap()),
            "pos_hint_y" => self.state.set_pos_hint_y(
                load_pos_hint_y_parameter(parameter_value.trim()).unwrap()),
            "width" => self.state.width = parameter_value.trim().parse().unwrap(),
            "height" => self.state.height = parameter_value.trim().parse().unwrap(),
            "auto_scale_width" =>
                self.state.set_auto_scale_width(load_bool_parameter(parameter_value.trim())?),
            "auto_scale_height" =>
                self.state.set_auto_scale_height(load_bool_parameter(parameter_value.trim())?),
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

    fn update_state(&mut self, new_state: &EzState) {
        let state = new_state.as_layout();
        self.state = state.clone();
        self.state.changed = false;
        self.state.force_redraw = false;
    }

    fn get_state(&self) -> EzState {
        EzState::Layout(self.state.clone())
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

        // Fill empty spaces with user defined filling if any
        if state.fill {
            merged_content = self.fill(merged_content, state);
        }

        // If user wants to autoscale width we set width to length of content
        if state.get_auto_scale_width() {
            let auto_scale_width = merged_content.len();
            if auto_scale_width < state.get_effective_width() {
                state.set_effective_width(auto_scale_width);
            }
        }
        // If user wants to autoscale height we set height to the highest column
        if state.get_auto_scale_height() {
            let auto_scale_height = merged_content.iter()
                .map(|x| x.len()).max().unwrap_or(0);
            if auto_scale_height < state.get_effective_height() {
                state.set_effective_height(auto_scale_height);
            }
        }

        // If widget doesn't fill its' height and/or width at this point fill it with empty pixels
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
        if merged_content.is_empty() { return merged_content } // Empty widget
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
            if content.len() > own_state.get_effective_width() {
                // Widget added more content than was allowed, crop it and return as we're full
                content = content[0..own_state.get_effective_width()].iter()
                    .map(|x| x[0..own_state.get_effective_height()].to_vec()).collect()
            }
            let generic_child= child.as_ez_object();
            let state = state_tree.get_mut(&generic_child.get_full_path())
                .unwrap().as_generic_mut();

            let width_left = own_state.get_effective_width() - content.len();
            // If autoscaling is enabled set child size to max width. It is then expected to scale
            // itself according to its' content
            if state.get_auto_scale_width() {
                state.set_width(width_left)
            }
            if state.get_auto_scale_height() {
                state.set_height(own_state.get_effective_height())
            }
            // Scale down child to remaining size in the case that the child is too large, rather
            // panicking.
            if state.get_width() > width_left {
                state.set_width(width_left);
            }
            if state.get_height() > own_state.get_effective_height() {
                state.set_height(own_state.get_effective_height());
            }

            let valign = state.get_vertical_alignment();
            state.set_position(position);
            let child_content = generic_child.get_contents(state_tree);
            if child_content.is_empty() { continue }  // handle empty widget
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
        for _ in 0..own_state.get_effective_width() {
            content.push(Vec::new())
        }
        let mut position: Coordinates = (0, 0);
        for child in self.get_children() {
            if content.is_empty() { return content }  // No space left in widget
            if content.len() > own_state.get_effective_width() ||
                content[0].len() > own_state.get_effective_height() {
                // Widget added more content than was allowed, crop it and return as we're full
                content = content[0..own_state.get_effective_width()].iter()
                    .map(|x| x[0..own_state.get_effective_height()].to_vec()).collect()
            }

            let generic_child= child.as_ez_object();
            let state = state_tree.get_mut(&generic_child.get_full_path())
                .unwrap().as_generic_mut();

            let height_left = own_state.get_effective_height() - content[0].len();
            if height_left == 0 {
                return content
            }
            // If autoscaling is enabled set child size to max width. It is then expected to scale
            // itself according to its' content
            if state.get_auto_scale_width() {
                state.set_width(own_state.get_effective_width())
            }
            if state.get_auto_scale_height() {
                state.set_height(height_left)
            }
            // Scale down child to remaining size in the case that the child is too large, rather
            // panicking.
            if state.get_height() > height_left {
                state.set_height(height_left);
            }
            if state.get_width() > own_state.get_effective_width() {
                state.set_width(own_state.get_effective_width());
            }

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
        let own_height = own_state.get_effective_height();
        let own_width = own_state.get_effective_width();

        // Fill self with background first. Then overlay widgets.
        for _ in 0..own_width {
            content.push(Vec::new());
            for _ in 0..own_height {
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
            if content.is_empty() { return content }  // No space left in widget

            let generic_child = child.as_ez_widget();
            let state = state_tree.get_mut(
                &generic_child.get_full_path()).unwrap().as_generic_mut();

            // If autoscaling is enabled set child size to max width. It is then expected to scale
            // itself according to its' content
            if state.get_auto_scale_width() {
                state.set_width(own_width)
            }
            if state.get_auto_scale_height() {
                state.set_height(own_height)
            }
            // Scale down child to remaining size in the case that the child is too large, rather
            // panicking.
            if state.get_height() > own_height {
                state.set_height(own_height);
            }
            if state.get_width() > own_width {
                state.set_width(own_width);
            }

            let child_content = generic_child.get_contents(state_tree);
            let state = state_tree.get_mut(
                &generic_child.get_full_path()).unwrap().as_generic_mut(); // re-borrow
            self.set_child_position(own_width, own_height, state);
            let (child_x, child_y) = state.get_position();
            let (child_width, child_height) = (state.get_width(), state.get_height());
            for width in 0..child_width {
                for height in 0..child_height {
                    if child_x + width < content.len() && child_y + height < content[0].len() {
                        content[child_x + width][child_y + height] =
                            child_content[width][height].clone();
                    }
                }
            }
        }
        content
    }

    /// Set the sizes of children that use size_hint(s) using own proportions.
    pub fn set_child_sizes(&self, state_tree: &mut StateTree) {

        let own_state = state_tree.get_mut(&self.get_full_path())
            .unwrap().as_layout();
        let own_width = own_state.get_effective_width();
        let own_height = own_state.get_effective_height();

        // Check if there are multiple children who ALL have size_hint=1, and in
        // that case give them '1 / number_of_children'. That way the user can add
        // multiple children in a Box layout and have them distributed equally automatically. Any
        // kind of asymmetry breaks this behavior.
        if self.children.len() > 1 {
            let (all_default_size_hint_x, all_default_size_hint_y) =
                self.check_default_size_hints(state_tree);
            if all_default_size_hint_x {
                for child in self.get_children() {
                    let generic_child = child.as_ez_object();
                    let state = state_tree.get_mut(&generic_child.get_full_path())
                        .unwrap().as_generic_mut();
                    state.set_size_hint_x(Some(1.0 / (self.children.len() as f64)));
                }
            }
            if all_default_size_hint_y {
                for child in self.get_children() {
                    let generic_child = child.as_ez_object();
                    let state = state_tree.get_mut(&generic_child.get_full_path())
                        .unwrap().as_generic_mut();
                    state.set_size_hint_y(Some(1.0 / (self.children.len() as f64)));
                }
            }
        }
        // Now calculate actual sizes.
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

    /// Check if all chrildren employ default size_hints (i.e. size_hint=1) for x and y
    /// separately.
    fn check_default_size_hints (&self, state_tree: &StateTree) -> (bool, bool){

        let mut all_default_size_hint_x = true;
        let mut all_default_size_hint_y = true;
        for child in self.get_children() {
            if !all_default_size_hint_x && !all_default_size_hint_y {
                break
            }
            let generic_child = child.as_ez_object();
            let state = state_tree.get(&generic_child.get_full_path())
                .unwrap().as_generic();
            if let LayoutOrientation::Horizontal = self.orientation {
                if let Some(size_hint_x) = state.get_size_hint_x()
                {
                    if size_hint_x != 1.0 || state.get_auto_scale_width() ||
                        state.get_auto_scale_height() || state.get_width() > 0 {
                        all_default_size_hint_x = false;
                    }
                } else {
                    all_default_size_hint_x = false;
                }
            } else {
                all_default_size_hint_x = false;
            }
            if let LayoutOrientation::Vertical = self.orientation {
                if let Some(size_hint_y) = state.get_size_hint_y() {
                    if size_hint_y != 1.0 || state.get_auto_scale_height() ||
                        state.get_auto_scale_width() || state.get_height() > 0 {
                        all_default_size_hint_y = false;
                    }
                } else {
                    all_default_size_hint_y = false;
                }
            } else {
                all_default_size_hint_y = false;
            }
        }
        (all_default_size_hint_x, all_default_size_hint_y)
    }

    /// Set the positions of children that use pos_hint(s) using own proportions and position.
    pub fn set_child_position(&self, parent_width: usize, parent_height: usize,
                              child_state: &mut dyn GenericState) {

        // Set x by pos_hint if any
        if let Some((keyword, fraction)) = child_state.get_pos_hint_x() {
            let initial_pos = match keyword {
                HorizontalPositionHint::Left => 0,
                HorizontalPositionHint::Right => parent_width - child_state.get_width(),
                HorizontalPositionHint::Center =>
                    (parent_width as f64 / 2.0).round() as usize -
                        (child_state.get_width() as f64 / 2.0).round() as usize,
            };
            let x = (initial_pos as f64 * fraction).round() as usize;
            child_state.set_position((x, child_state.get_position().1));
        }
        // Set y by pos hint if any
        if let Some((keyword, fraction)) = child_state.get_pos_hint_y() {
            let initial_pos = match keyword {
                VerticalPositionHint::Top => 0,
                VerticalPositionHint::Bottom => parent_height - child_state.get_height(),
                VerticalPositionHint::Middle =>
                    (parent_height as f64 / 2.0).round() as usize -
                        (child_state.get_height() as f64 / 2.0).round() as usize,
            };
            let y = (initial_pos as f64 * fraction).round() as usize;
            child_state.set_position((child_state.get_position().0, y));
        }
    }
    /// Takes [absolute_position] of this layout and adds the [x][y] of children to calculate and
    /// set their [absolute_position]. Then calls this method on children, thus recursively setting
    /// [absolute_position] for all children. Use on root layout to propagate all absolute positions.
    pub fn propagate_absolute_positions(&self, state_tree: &mut StateTree) {

        let absolute_position = state_tree.get(&self.path).unwrap().as_generic()
            .get_effective_absolute_position();
        for child in self.get_children() {
            if let EzObjects::Layout(i) = child {
                let child_state =
                    state_tree.get_mut(&i.get_full_path()).unwrap().as_generic_mut();
                let pos = child_state.get_position();
                let new_absolute_position = (absolute_position.0 + pos.0,
                                                        absolute_position.1 + pos.1);
                child_state.set_absolute_position(new_absolute_position);
                i.propagate_absolute_positions(state_tree);
            } else {
                let child_state = state_tree.get_mut(
                    &child.as_ez_widget().get_full_path()).unwrap().as_generic_mut();
                let pos = child_state.get_position();
                child_state.set_absolute_position((absolute_position.0 + pos.0,
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

        if parent_state.get_effective_height() > new[0].len() {
            let offset;
            (new, offset) = common::align_content_vertically(
                new, valign, parent_state.get_effective_height(),
                parent_state.content_foreground_color,
                parent_state.content_background_color);
            let old_pos = state.get_position();
            state.set_position((old_pos.0, old_pos.1 + offset));
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
        merged_content
    }

    /// Take a [PixelMap] and merge it vertically with another [PixelMap]
    pub fn merge_vertical_contents(&self, mut merged_content: PixelMap, mut new: PixelMap,
                                   parent_state: &LayoutState, state: &mut dyn GenericState,
                                   halign: HorizontalAlignment) -> PixelMap {

        if new.is_empty() {
            return merged_content
        }

        let offset;
        if parent_state.get_effective_width() > new.len() {
            (new, offset) = common::align_content_horizontally(
                new, halign, parent_state.get_effective_width(),
            parent_state.content_foreground_color,
                parent_state.content_background_color);

            let old_pos = state.get_position();
            state.set_position((old_pos.0 + offset, old_pos.1));
        }

        for x in 0..parent_state.get_effective_width() {
            for y in 0..new[0].len() {
                if x < new.len() && y < new[x].len() {
                    merged_content[x].push(new[x][y].clone())
                } else {
                    merged_content[x].push(Pixel { symbol: " ".to_string(),
                        foreground_color: parent_state.content_foreground_color,
                        background_color: parent_state.content_background_color,
                        underline: false});
                }
            }
        }

        merged_content
    }
}
