//! # Layout
//! Module implementing the Layout struct.
use std::collections::HashMap;
use crossterm::event::{Event, KeyCode};
use crate::parser;
use crate::common;
use crate::common::definitions::{CallbackTree, EzContext, PixelMap, StateTree, ViewTree, WidgetTree,
                                 Coordinates};
use crate::widgets::widget::{Pixel, EzObject, EzObjects};
use crate::states::layout_state::LayoutState;
use crate::states::state::{EzState, GenericState};
use crate::scheduler::Scheduler;
use crate::states::definitions::{AutoScale, CallbackConfig, ColorConfig, LayoutMode, Size, SizeHint,
                                 LayoutOrientation};
use crate::widgets::button::Button;


/// A layout is where widgets live. They implements methods for hardcoding widget placement or
/// placing them automatically in various ways.
#[derive(Clone, Debug)]
pub struct Layout {

    /// ID of the layout, used to construct [path]
    pub id: String,

    /// Full path to this layout, e.g. "/root_layout/layout_2/THIS_ID"
    pub path: String,

    /// List of children widgets and/or layouts
    pub children: Vec<EzObjects>,

    /// Child ID to index in [children] lookup. Used to get widgets by [id] and [path]
    pub child_lookup: HashMap<String, usize>,

    /// Runtime state of this Layout, see [LayoutState] and [State]
    pub state: LayoutState,
}


impl Layout {
    pub fn new(id: String, path: String, scheduler: &mut Scheduler) -> Self {
        Layout {
            id,
            path: path.clone(),
            children: Vec::new(),
            child_lookup: HashMap::new(),
            state: LayoutState::new(path, scheduler),
        }
    }
}


impl EzObject for Layout {

    fn load_ez_parameter(&mut self, parameter_name: String, parameter_value: String,
                         scheduler: &mut Scheduler) {

        let consumed = parser::load_common_parameters(
            &parameter_name, parameter_value.clone(),Box::new(self), scheduler);
        if consumed { return }
        match parameter_name.as_str() {
            "mode" => {
                match parameter_value.to_lowercase().trim() {
                    "box" => self.state.mode = LayoutMode::Box,
                    "float" => self.state.mode = LayoutMode::Float,
                    "screen" => self.state.mode = LayoutMode::Screen,
                    "tabbed" => self.state.mode = LayoutMode::Tabbed,
                    _ => panic!("Invalid parameter value for mode {}", parameter_value)
                }
            },
            "orientation" => {
                match parameter_value.trim() {
                    "horizontal" =>
                        self.state.orientation = LayoutOrientation::Horizontal,
                    "vertical" =>
                        self.state.orientation = LayoutOrientation::Vertical,
                    _ => panic!("Invalid parameter value for orientation {}",
                                       parameter_value)
                }
            },
            "scroll" => parser::load_full_enable_scrolling_parameter(
                parameter_value.trim(), &mut self.state.scrolling_config),
            "scroll_x" => self.state.scrolling_config.enable_x =
                parser::load_bool_parameter(parameter_value.trim()),
            "scroll_y" => self.state.scrolling_config.enable_y =
                parser::load_bool_parameter(parameter_value.trim()),
            "fill" =>
                self.state.fill = parser::load_bool_parameter(parameter_value.trim()),
            "filler_symbol" =>
                self.state.set_filler_symbol(parameter_value.trim().to_string()),
            _ => panic!("Invalid parameter name for layout {}", parameter_name)
        }
    }
    fn set_id(&mut self, id: String) { self.id = id; }

    fn get_id(&self) -> String { self.id.clone() }

    fn set_full_path(&mut self, path: String) { self.path = path }

    fn get_full_path(&self) -> String { self.path.clone() }

    fn get_state(&self) -> EzState { EzState::Layout(self.state.clone()) }

    fn get_state_mut(&mut self) -> Box<&mut dyn GenericState>{ Box::new(&mut self.state) }

    fn get_contents(&self, state_tree: &mut StateTree) -> PixelMap {

        let mut merged_content = PixelMap::new();
        let mode = state_tree.get(&self.path).unwrap().as_layout().mode.clone();
        let orientation =
            state_tree.get(&self.path).unwrap().as_layout().orientation.clone();
        match mode {
            LayoutMode::Box => {
                match orientation {
                    LayoutOrientation::Horizontal => {
                        merged_content =
                            self.get_box_mode_horizontal_orientation_contents(state_tree);
                    },
                    LayoutOrientation::Vertical => {
                        merged_content =
                            self.get_box_mode_vertical_orientation_contents(state_tree);
                    },
                }
            },
            LayoutMode::Float => {
                merged_content =
                    self.get_float_mode_contents(merged_content, state_tree);
            },
            LayoutMode::Screen => {
                merged_content = self.get_screen_mode_contents(state_tree);
            },
            LayoutMode::Tabbed => {
                merged_content = self.get_tabbed_mode_contents(state_tree);
            }
        }

        merged_content = self.add_user_filler(state_tree, merged_content);
        merged_content = self.auto_scale_to_content(state_tree, merged_content);
        merged_content = self.add_empty_filler(state_tree, merged_content);
        merged_content = self.create_horizontal_scroll_box(state_tree, merged_content);
        merged_content = self.create_vertical_scroll_box(state_tree, merged_content);
        let state = state_tree.get(&self.get_full_path()).unwrap().as_layout();

        if merged_content.is_empty() { return merged_content } // Empty widget
        // Put border around content if border if set
        if state.get_border_config().enabled {
            merged_content = common::widget_functions::add_border(merged_content,
                                                                  state.get_border_config());
        }
        // Put padding around content if set
        merged_content = common::widget_functions::add_padding(
            merged_content, state.get_padding(), state.get_color_config().background,
            state.get_color_config().foreground);
        merged_content = self.get_modal_contents(state_tree, merged_content);

        self.propagate_absolute_positions(state_tree);
        merged_content
    }

    fn handle_event(&self, event: Event, _view_tree: &mut ViewTree, state_tree: &mut StateTree,
                    _widget_tree: &WidgetTree, _callback_tree: &mut CallbackTree,
                    _scheduler: &mut Scheduler) -> bool {

        let state = state_tree.get_mut(&self.get_full_path())
            .unwrap().as_layout_mut();
        if let Event::Key(key) = event {
            if key.code == KeyCode::PageUp {
                self.handle_scroll_up(state);
                return true
            } else if key.code == KeyCode::PageDown {
                self.handle_scroll_down(state);
                return true
            } else if key.code == KeyCode::Left {
                if state.get_mode() == &LayoutMode::Tabbed {
                    self.handle_tab_left(state_tree);
                } else {
                    self.handle_scroll_left(state);
                }
                return true
            } else if key.code == KeyCode::Right {
                if state.get_mode() == &LayoutMode::Tabbed {
                    self.handle_tab_right(state_tree);
                } else {
                    self.handle_scroll_right(state);
                }
                return true
            }
        }
        false
    }

    fn on_keyboard_enter(&self, _view_tree: &mut ViewTree, state_tree: &mut StateTree,
                         _widget_tree: &WidgetTree, _callback_tree: &mut CallbackTree,
                         _scheduler: &mut Scheduler) -> bool {
        let state = state_tree.get_mut(&self.path).unwrap().as_layout_mut();
        if !state.selected_tab_header.is_empty() {
            state.set_active_tab(state.get_selected_tab_header()
                .strip_suffix("_tab_header").unwrap().to_string());
            return true
        }
        false
    }

    fn on_left_mouse_click(&self, _view_tree: &mut ViewTree, state_tree: &mut StateTree,
                           _widget_tree: &WidgetTree, _callback_tree: &mut CallbackTree,
                           _scheduler: &mut Scheduler, mouse_pos: Coordinates) -> bool {

        let state = state_tree.get_mut(&self.path).unwrap().as_layout_mut();

        if state.scrolling_config.is_scrolling_x &&
            mouse_pos.y == state.get_effective_size().height + 1{

            let (scrollbar_size, scrollbar_pos) = self.get_horizontal_scrollbar_parameters(
                state.get_scrolling_config().original_width,
                state.get_effective_size().width,
                state.get_scrolling_config().view_start_x);

            if mouse_pos.x < scrollbar_pos {
                self.handle_scroll_left(state);
                return true
            } else if mouse_pos.x > scrollbar_pos + scrollbar_size {
                self.handle_scroll_right(state);
                return true
            }
        }

        if state.scrolling_config.is_scrolling_y &&
            mouse_pos.x == state.get_size().width - 1 {

            let (scrollbar_size, scrollbar_pos) = self.get_vertical_scrollbar_parameters(
                state.get_scrolling_config().original_height,
                state.get_effective_size().height,
                state.get_scrolling_config().view_start_y);

            if mouse_pos.y < scrollbar_pos {
                self.handle_scroll_up(state);
                return true
            } else if mouse_pos.y > scrollbar_pos + scrollbar_size {
                self.handle_scroll_down(state);
                return true
            }
        }
        false
    }

    fn on_select(&self, _view_tree: &mut ViewTree, state_tree: &mut StateTree,
                 _widget_tree: &WidgetTree, _callback_tree: &mut CallbackTree,
                 _scheduler: &mut Scheduler, _mouse_pos: Option<Coordinates>) -> bool {

        for child in self.children.iter() {
            if let EzObjects::Button(i) = child {
                state_tree.get_mut(&self.path).unwrap().as_layout_mut()
                    .selected_tab_header = i.path.clone();
                return true
            }
        }
        true
    }

    fn on_deselect(&self, _view_tree: &mut ViewTree, state_tree: &mut StateTree,
                 _widget_tree: &WidgetTree, _callback_tree: &mut CallbackTree,
                 _scheduler: &mut Scheduler) -> bool {

        state_tree.get_mut(&self.path).unwrap().as_layout_mut()
            .selected_tab_header.clear();
        true
    }
}

impl Layout {

    /// Initialize an instance of a Layout with the passed config parsed by [ez_parser]
    pub fn from_config(config: Vec<String>, id: String, path: String, scheduler: &mut Scheduler)
                       -> Self {

        let mut obj = Layout::new(id, path, scheduler);
        obj.load_ez_config(config, scheduler).unwrap();
        obj
    }

    /// Scale size down to the size of the actual content of the layout.
    fn auto_scale_to_content(&self, state_tree: &mut StateTree, contents: PixelMap) -> PixelMap {
        let state = state_tree.get_mut(&self.get_full_path()).unwrap()
            .as_layout_mut();
        // If user wants to autoscale width we set width to length of content
        if state.get_auto_scale().width {
            let auto_scale_width = contents.len();
            if auto_scale_width < state.get_effective_size().width {
                state.set_effective_width(auto_scale_width);
            }
        }
        // If user wants to autoscale height we set height to the highest column
        if state.get_auto_scale().height {
            let auto_scale_height = contents.iter()
                .map(|x| x.len()).max().unwrap_or(0);
            if auto_scale_height < state.get_effective_size().height {
                state.set_effective_height(auto_scale_height);
            }
        }
        contents
    }

    /// Overwrite a PixelMap of current own content with the content of the active modal. Modals
    /// overlap all content.
    fn get_modal_contents(&self, state_tree: &mut StateTree, mut contents: PixelMap) -> PixelMap {
        if state_tree.get(&self.get_full_path()).unwrap().as_layout().get_modals().is_empty() {
            return contents
        }

        // Size modal
        let parent_size = state_tree.get(&self.get_full_path()).unwrap().as_layout()
            .get_size().clone();
        let modal = state_tree.get(&self.get_full_path()).unwrap().as_layout()
            .get_modals().first().unwrap().clone();
        let state = state_tree
            .get_mut(&modal.as_ez_object().get_full_path()).unwrap();
        common::widget_functions::resize_with_size_hint(state, parent_size.width,
                                                        parent_size.height);
        common::widget_functions::reposition_with_pos_hint(
            parent_size.width, parent_size.height,
            state.as_generic_mut());
        let x = state.as_generic().get_position().x.get();
        let y = state.as_generic().get_position().y.get();
        state.as_generic_mut().set_absolute_position(Coordinates::new(x, y));

        //Get contents
        let modal_content;
        if let EzObjects::Layout(ref i) = modal {
            i.set_child_sizes(state_tree);
            modal_content = i.get_contents(state_tree);
            i.propagate_absolute_positions(state_tree);
        } else {
            modal_content = modal.as_ez_object().get_contents(state_tree);
        }

        // Overwrite own content with modal (modal is always on top)
        let state = state_tree
            .get_mut(&state_tree.get(&self.get_full_path()).unwrap().as_layout()
            .get_modals().first().unwrap().as_ez_object()
            .get_full_path()).unwrap().as_generic();
        let start_pos = state.get_position();
        for x in 0..modal_content.len() {
            for y in 0..modal_content[x].len() {
                let write_pos = Coordinates::new(start_pos.x.get() + x,
                                                            start_pos.y.get() + y);
                if write_pos.x <= parent_size.width && write_pos.y <= parent_size.height {
                    contents[write_pos.x][write_pos.y] = modal_content[x][y].clone();
                }
            }
        }
        contents
    }

    /// Used by [get_contents] when the [LayoutMode] is set to [Float]. Places each child in the
    /// XY coordinates defined by that child, relative to itself, and uses
    /// childs' [width] and [height].
    fn get_float_mode_contents(&self, mut content: PixelMap, state_tree: &mut StateTree)
        -> PixelMap {
        let own_state = state_tree.get(&self.get_full_path()).unwrap().as_layout();
        let own_height = own_state.get_effective_size().height;
        let own_width = own_state.get_effective_size().width;


        // Fill self with background first. Then overlay widgets.
        let filler = Pixel::new(own_state.get_filler_symbol(),
                                own_state.get_color_config().filler_foreground,
                                own_state.get_color_config().filler_background);
        for _ in 0..own_width {
            content.push(Vec::new());
            for _ in 0..own_height {
                if own_state.fill {
                    content.last_mut().unwrap().push(filler.clone());
                } else {
                    content.last_mut().unwrap().push(
                        Pixel::new(
                            " ".to_string(), own_state.colors.foreground,
                            own_state.colors.background));
                }
            }
        }
        for child in self.get_children() {
            if content.is_empty() { return content }  // No space left in widget

            let generic_child = child.as_ez_object();
            let state = state_tree.get_mut(
                &generic_child.get_full_path()).unwrap().as_generic_mut();

            // If autoscaling is enabled set child size to max width. It is then expected to scale
            // itself according to its' content
            if state.get_auto_scale().width {
                state.get_size_mut().width = own_width
            }
            if state.get_auto_scale().height {
                state.get_size_mut().height = own_height
            }
            // Scale down child to remaining size in the case that the child is too large, rather
            // panicking.
            if state.get_size().height > own_height {
                state.get_size_mut().height = own_height;
            }
            if state.get_size().width > own_width {
                state.get_size_mut().width = own_width;
            }

            let child_content = generic_child.get_contents(state_tree);
            let state = state_tree.get_mut(
                &generic_child.get_full_path()).unwrap().as_generic_mut(); // re-borrow
            common::widget_functions::reposition_with_pos_hint(
                own_width, own_height,state);
            let child_pos = state.get_position();
            for width in 0..child_content.len() {
                for height in 0..child_content[width].len() {
                    if child_pos.x.get() + width <= content.len()
                        && child_pos.y.get() + height <= content[child_pos.x.get() + width].len() {
                        content[child_pos.x.get() + width][child_pos.y.get() + height] =
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
        let own_width = own_state.get_effective_size().width;
        let own_height = own_state.get_effective_size().height;

        // Check if there are multiple children who ALL have size_hint=1, and in
        // that case give them '1 / number_of_children'. That way the user can add
        // multiple children in a Box layout and have them distributed equally automatically. Any
        // kind of asymmetry breaks this behavior.
        if self.children.len() > 1 && own_state.mode != LayoutMode::Tabbed &&
            own_state.mode != LayoutMode::Screen {
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
                .unwrap();
            common::widget_functions::resize_with_size_hint(
                state, own_width, own_height);
        }
        for child in self.get_children() {
            if let EzObjects::Layout(i) = child {
                i.set_child_sizes(state_tree)
            }
        }
    }

    /// Check if all chrildren employ default size_hints (i.e. size_hint=1) for x and y
    /// separately.
    fn check_default_size_hints(&self, state_tree: &StateTree) -> (bool, bool) {

        let mut all_default_size_hint_x = true;
        let mut all_default_size_hint_y = true;
        let own_orientation = state_tree.get(&self.path).unwrap()
            .as_layout().orientation.clone();
        for child in self.get_children() {
            if !all_default_size_hint_x && !all_default_size_hint_y {
                break
            }
            let generic_child = child.as_ez_object();
            let state = state_tree.get(&generic_child.get_full_path())
                .unwrap().as_generic();
            if let LayoutOrientation::Horizontal = own_orientation {
                if let Some(size_hint_x) = state.get_size_hint().x
                {
                    if size_hint_x != 1.0 || state.get_auto_scale().width ||
                        state.get_auto_scale().height || state.get_size().width > 0 {
                        all_default_size_hint_x = false;
                    }
                } else {
                    all_default_size_hint_x = false;
                }
            } else {
                all_default_size_hint_x = false;
            }
            if let LayoutOrientation::Vertical = own_orientation {
                if let Some(size_hint_y) = state.get_size_hint().y {
                    if size_hint_y != 1.0 || state.get_auto_scale().height ||
                        state.get_auto_scale().width || state.get_size().height > 0 {
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

    /// Takes [absolute_position] of this layout and adds the [x][y] of children to calculate and
    /// set their [absolute_position]. Then calls this method on children, thus recursively setting
    /// [absolute_position] for all children. Use on root layout to propagate all absolute positions.
    pub fn propagate_absolute_positions(&self, state_tree: &mut StateTree) {
        let absolute_position = state_tree.get(&self.path).unwrap().as_generic()
            .get_effective_absolute_position();
        let size = state_tree.get(&self.path).unwrap().as_layout()
            .get_size().clone();
        let scrolling = state_tree.get(&self.path).unwrap().as_layout()
            .get_scrolling_config().clone();
        for child in self.get_children() {
            if let EzObjects::Layout(i) = child {
                let child_state =
                    state_tree.get_mut(&i.get_full_path()).unwrap().as_generic_mut();
                let pos = child_state.get_position();
                let mut new_absolute_position = Coordinates::new(
                    absolute_position.x + pos.x.get(), absolute_position.y + pos.y.get());
                if scrolling.is_scrolling_x && size.width > 0 {
                    new_absolute_position.x -= scrolling.view_start_x % size.width;
                }
                if scrolling.is_scrolling_y && size.height > 0 {
                    new_absolute_position.y -= scrolling.view_start_y % size.height;
                }
                child_state.set_absolute_position(new_absolute_position);
                i.propagate_absolute_positions(state_tree);
            } else {
                let child_state = state_tree.get_mut(
                    &child.as_ez_object().get_full_path()).unwrap().as_generic_mut();
                let pos = child_state.get_position();
                let new_absolute_position = Coordinates::new(
                    absolute_position.x + pos.x.get(), absolute_position.y + pos.y.get());
                child_state.set_absolute_position(new_absolute_position);
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

    /// Add a child ([Layout] or [EzWidget]) to this Layout.
    pub fn add_child(&mut self, mut child: EzObjects, scheduler: &mut Scheduler) {

        let generic_child = child.as_ez_object_mut();
        let id = generic_child.get_id().clone();
        let path = generic_child.get_full_path().clone();
        let parent_path = self.path.clone();
        if self.child_lookup.contains_key(&id) {
            panic!("A layout may not contain two children with identical IDs: \"{}\"",
                    generic_child.get_id());
        }

        self.child_lookup.insert(generic_child.get_id().clone(), self.children.len());
        self.children.push(child.clone());

        if self.state.mode == LayoutMode::Tabbed {
            if let EzObjects::Layout(_) = child.clone() {
                let new_id = format!("{}_tab_header", id.clone());
                let new_path = format!("{}/{}", parent_path, new_id.clone());
                let mut tab_header = Button::new(new_id, new_path, scheduler);
                tab_header.state.size_hint = SizeHint::new(None, None);
                tab_header.state.text = id;

                let tab_on_click = move |context: EzContext| {
                    context.state_tree
                        .get_mut(&parent_path)
                        .unwrap().as_layout_mut()
                        .set_active_tab(path.clone());
                    true
                };
                let callback_config = CallbackConfig::from_on_press(
                    Box::new(tab_on_click));
                scheduler.set_callback_config(tab_header.path.clone(), callback_config);
                self.add_child(EzObjects::Button(tab_header), scheduler);
            }
        }
    }

    /// Get an EzWidget trait object for each child [EzWidget] and return it in a
    /// <[path], [EzObject]> HashMap.
    pub fn get_widget_tree(&self) -> WidgetTree {
        let mut widget_tree = WidgetTree::new();
        for (child_path, child) in self.get_widgets_recursive() {
            widget_tree.insert(child_path, child);
        }
        for i in 0..self.state.open_modals.len() {
            if let EzObjects::Layout(ref layout) = self.state.open_modals[i] {
                for (child_path, child) in layout.get_widgets_recursive() {
                    widget_tree.insert(child_path, child);
                }
            }
            widget_tree.insert(
                self.state.open_modals[i].as_ez_object().get_full_path(),
                &self.state.open_modals[i]);
        }
        widget_tree
    }
    /// Get a list of children non-recursively. Can be [Layout] or [EzWidget]
    pub fn get_children(&self) -> &Vec<EzObjects> { &self.children }

    /// Get a mutable list of children non-recursively. Can be [Layout] or [EzWidget]
    pub fn get_children_mut(&mut self) -> &mut Vec<EzObjects> { &mut self.children }

    /// Get a specific child ref by its' [id]
    pub fn get_child(&self, id: &str) -> &EzObjects {
        let index = self.child_lookup.get(id)
            .unwrap_or_else(|| panic!("No child: {} in {}", id, self.get_id()));
        self.children.get(*index).unwrap()
    }

    /// Get a specific child mutable ref by its'[id]
    pub fn get_child_mut(&mut self, id: &str) -> &mut EzObjects {
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

    /// Fill any empty positions with [Pixel] from [get_filler]
    pub fn add_user_filler(&self, state_tree: &mut StateTree, mut contents: PixelMap) -> PixelMap {
        let state = state_tree.get_mut(&self.get_full_path()).unwrap()
            .as_layout_mut();
        if !state.fill { return contents }

        let filler = Pixel::new(state.get_filler_symbol(),
                                state.get_color_config().filler_foreground,
                                state.get_color_config().filler_background);

        for x in 0..(state.get_effective_size().width) {
            for y in contents[x].iter_mut() {
                if y.symbol.is_empty() || y.symbol == " ".to_string() {
                    y.symbol = filler.symbol.clone();
                }
            }
            while contents[x].len() < (state.get_effective_size().height) {
                contents[x].push(filler.clone());
            }
        }
        while contents.len() < state.get_effective_size().width {
            let mut new_x = Vec::new();
            for _ in 0..state.get_effective_size().height {
                new_x.push(filler.clone());
            }
            contents.push(new_x);
        }
        contents
    }

    /// Fill any empty positions with empty [Pixel]. Used to fill full size of the layout in case
    /// the user did not define a custom filler.
    pub fn add_empty_filler(&self, state_tree: &mut StateTree, mut contents: PixelMap) -> PixelMap {

        let state = state_tree.get_mut(&self.get_full_path()).unwrap()
            .as_layout_mut();
        while contents.len() < state.get_effective_size().width {
            contents.push(Vec::new());
        }
        for x in contents.iter_mut() {
            while x.len() < state.get_effective_size().height {
                x.push(Pixel::new(
                    " ".to_string(), state.get_color_config().foreground,
                    state.get_color_config().background));
            }
        }
        contents
    }
}


// Screen mode implementations
impl Layout {

    fn get_screen_mode_contents(&self, state_tree: &mut StateTree) -> PixelMap {

        let mut active_screen = state_tree.get(&self.path).unwrap()
            .as_layout().active_screen.clone();
        if active_screen.is_empty() && !self.children.is_empty() {
            active_screen = self.children.first().unwrap().as_layout().get_id();
            state_tree.get_mut(&self.path).unwrap().as_layout_mut().active_screen =
                active_screen.clone();
        }
        self.get_child(&active_screen).as_layout().get_contents(state_tree)
    }
}


// Tabbed mode implementations
impl Layout {

    fn handle_tab_left(&self, state_tree: &mut StateTree) {

        let mut next_button = false;
        for child in self.children.iter().rev() {
            if let EzObjects::Button(ref widget) = child {
                if next_button {
                    state_tree.get_mut(&self.path)
                        .unwrap().as_layout_mut().set_selected_tab_header(widget.path.clone());
                    return
                } else if state_tree
                    .get_mut(&self.path).unwrap().as_layout_mut().selected_tab_header ==
                    widget.path {
                    next_button = true;
                }
            }
        }
    }

    fn handle_tab_right(&self, state_tree: &mut StateTree) {

        let mut next_button = false;
        for child in self.children.iter() {
            if let EzObjects::Button(ref widget) = child {
                if next_button {
                    state_tree.get_mut(&self.path)
                        .unwrap().as_layout_mut().set_selected_tab_header(widget.path.clone());
                    return
                } else if state_tree
                    .get_mut(&self.path).unwrap().as_layout_mut().selected_tab_header ==
                    widget.path {
                    next_button = true;
                }
            }
        }
    }

    fn get_tabbed_mode_contents(&self, state_tree: &mut StateTree) -> PixelMap {

        if self.children.is_empty() { return PixelMap::new() }
        let state = state_tree.get_mut(&self.path).unwrap().as_layout_mut();
        let own_size = state.get_effective_size();
        let own_pos = state.get_effective_absolute_position();
        let own_colors = state.colors.clone();
        let selection = state.selected_tab_header.clone();
        if state.active_tab.is_empty() {
            state.set_active_tab(self.children[0].as_ez_object().get_full_path());
        }
        let active_tab = state.active_tab.clone();

        let mut button_content = PixelMap::new();
        let mut tab_content = PixelMap::new();
        let mut pos_x: usize = 0;
        let mut selected_pos_x: usize = 0;
        let mut selected_width: usize = 0;
        for child in self.get_children() {
            if let EzObjects::Layout(i) = child {
                if i.get_full_path() != active_tab { continue }
                let child_state = state_tree
                    .get_mut(&child.as_ez_object().get_full_path()).unwrap().as_generic_mut();
                child_state.set_effective_height(if own_size.height >=3 {own_size.height - 3} else {0});
                child_state.set_effective_width(if own_size.width >= 1 {own_size.width - 1} else {0});
                child_state.get_position_mut().x.set(0);
                child_state.get_position_mut().y.set(3);
                child_state.set_absolute_position(Coordinates::new(
                    own_pos.x, own_pos.y + 3));
                tab_content = i.get_contents(state_tree);
            } else if let EzObjects::Button(i) = child {

                let child_state = state_tree
                    .get_mut(&i.path).unwrap().as_button_mut();

                child_state.colors.foreground =
                    if selection == i.path { own_colors.selection_foreground }
                    else if active_tab.rsplit_once('/').unwrap().1 == child_state.text {
                        own_colors.active_foreground
                    } else { own_colors.tab_foreground };
                child_state.colors.background =
                    if selection == i.path { own_colors.selection_background }
                    else if active_tab.rsplit_once('/').unwrap().1 == child_state.text {
                        own_colors.active_background
                    } else { own_colors.tab_background };

                child_state.set_size_hint(SizeHint::new(None, None));
                child_state.set_auto_scale(AutoScale::new(true, true));
                child_state.set_effective_width(own_size.width);
                child_state.set_effective_height(1);
                child_state.set_x(pos_x);
                child_state.set_y(0);
                let content = i.get_contents(state_tree);
                let child_state = state_tree
                    .get_mut(&child.as_ez_object().get_full_path()).unwrap().as_button_mut();
                child_state.size = Size::new(child_state.text.len() + 2, 3);
                button_content = self.merge_horizontal_contents(
                    button_content, content,
                    Size::new(own_size.width - 1,3),own_colors.clone(),
                    child_state);
                child_state.set_absolute_position(
                    Coordinates::new(own_pos.x + pos_x, own_pos.y + 1));

                if (!selection.is_empty() && selection == i.path) || (selection.is_empty() &&
                    active_tab == i.path.strip_suffix("_tab_header").unwrap()) {
                    selected_pos_x = pos_x;
                    selected_width = child_state.size.width;
                }

                pos_x = button_content.len();

            }
        }
        let fill_pixel = Pixel::new(" ".to_string(),
                                    own_colors.foreground,
                                    own_colors.background);
        if own_size.width < button_content.len()  {
            let mut difference;
            if own_size.width <= selected_pos_x + selected_width {
                difference = (selected_pos_x + selected_width) - own_size.width;
                if button_content.len() - difference > 3 {
                    difference += 3;
                }
            } else if selected_pos_x != 0 && button_content.len() > 3 {
                difference = 3;
            } else {
                difference = 0;
            }
            button_content = button_content[difference..].to_vec();
            for child in self.children.iter() {
                if let EzObjects::Button(button) = child {
                    let state = state_tree
                        .get_mut(&button.path).unwrap().as_button_mut();
                    state.set_x(if state.get_position().x.get() >= difference
                    { state.get_position().x.get() - difference } else { 0 });
                    state.set_absolute_position(Coordinates::new(
                        if state.get_absolute_position().x >= difference
                        { state.get_absolute_position().x - difference } else { 0 },
                    state.get_absolute_position().y));
                }
            }
        }
        if button_content.len() > own_size.width {
            button_content = button_content[..own_size.width].to_vec();
        }
        while button_content.len() < own_size.width {
            let row =  vec!(fill_pixel.clone(), fill_pixel.clone(), fill_pixel.clone());
            button_content.push(row);
        }

        let state = state_tree.get_mut(&self.path).unwrap().as_layout_mut();
        self.merge_vertical_contents(
            button_content,
            tab_content,
            own_size.clone(), own_colors.clone(), state)
    }
}

// Box mode implementations
impl Layout {

    /// Used by [get_contents] when the [LayoutMode] is set to [Box] and [LayoutOrientation] is
    /// set to [Horizontal]. Merges contents of sub layouts and/or widgets horizontally, using
    /// own [height] for each.
    fn get_box_mode_horizontal_orientation_contents(&self, state_tree: &mut StateTree) -> PixelMap {

        let own_size = state_tree.get_mut(&self.get_full_path())
            .unwrap().as_layout().get_effective_size();
        let own_scaling = state_tree.get_mut(&self.get_full_path())
            .unwrap().as_layout().get_auto_scale().clone();
        let own_colors = state_tree.get_mut(&self.get_full_path())
            .unwrap().as_layout().get_color_config().clone();
        let own_scrolling = state_tree.get_mut(&self.get_full_path())
            .unwrap().as_layout().get_scrolling_config().clone();

        let mut position = Coordinates::new(0, 0);
        let mut content_list = Vec::new();
        for child in self.get_children() {

            let generic_child = child.as_ez_object();
            let state = state_tree
                .get_mut(&generic_child.get_full_path().clone()).unwrap().as_generic_mut();

            if own_size.infinite_width || own_scrolling.enable_x {
                state.get_size_mut().infinite_width = true;
            }
            if own_size.infinite_height || own_scrolling.enable_y {
                state.get_size_mut().infinite_height = true;
            }

            let width_left =
                if !own_scrolling.enable_x && !own_size.infinite_width &&
                    !state.get_size().infinite_width && own_size.width >= position.x
                    {own_size.width - position.x} else {0};
            // If autoscaling is enabled set child size to max width. It is then expected to scale
            // itself according to its' content
            if state.get_auto_scale().width {
                state.get_size_mut().width = width_left
            }
            if state.get_auto_scale().height {
                state.get_size_mut().height = own_size.height
            }
            // Scale down child to remaining size in the case that the child is too large, rather
            // panicking.
            if !own_scrolling.enable_x && !own_size.infinite_width &&
                state.get_size().width > width_left {
                state.get_size_mut().width = width_left;
            }
            if !own_scrolling.enable_y && !own_size.infinite_height &&
                state.get_size().height > own_size.height {
                state.get_size_mut().height = own_size.height;
            }

            state.set_x(position.x);
            state.set_y(position.y);
            let child_content = generic_child.get_contents(state_tree);
            if child_content.is_empty() { continue }  // handle empty widget
            let state = state_tree.get_mut(&generic_child.get_full_path())
                .unwrap().as_generic_mut(); // re-borrow
            if state.get_size().infinite_width {
                state.get_size_mut().width = child_content.len()
            }
            if state.get_size().infinite_height {
                state.get_size_mut().height = child_content[0].len()
            }

            position.x += child_content.len();
            content_list.push(child_content);
        }
        if own_scaling.width {
            state_tree.get_mut(&self.path).unwrap().as_layout_mut().set_effective_width(
                content_list.iter().map(|x| x.len()).max().unwrap());
        }
        if own_scaling.height {
            state_tree.get_mut(&self.path).unwrap().as_layout_mut().set_effective_height(
                content_list.iter().map(
                    |child| child.iter().map(|x| x.len()).max().unwrap())
                    .max().unwrap());
        }
        let own_size = state_tree.get_mut(&self.get_full_path())
            .unwrap().as_layout().get_effective_size();
        let mut merged_content = PixelMap::new();
        for (i, content) in content_list.into_iter().enumerate() {
            merged_content = self.merge_horizontal_contents(
                merged_content, content,
                own_size, own_colors.clone(),
                state_tree.get_mut(&self.children.get(i).unwrap().as_ez_object()
                    .get_full_path()).unwrap().as_generic_mut());
        }
        merged_content
    }

    /// Used by [get_contents] when the [LayoutMode] is set to [Box] and [LayoutOrientation] is
    /// set to [Vertical]. Merges contents of sub layouts and/or widgets vertically.
    fn get_box_mode_vertical_orientation_contents(&self, state_tree: &mut StateTree) -> PixelMap {

        // Some clones as we will need to borrow from state tree again later
        let own_size = state_tree.get_mut(&self.get_full_path())
            .unwrap().as_layout().get_effective_size();
        let own_scaling = state_tree.get_mut(&self.get_full_path())
            .unwrap().as_layout().get_auto_scale().clone();
        let own_colors = state_tree.get_mut(&self.get_full_path())
            .unwrap().as_layout().get_color_config().clone();
        let own_scrolling = state_tree.get_mut(&self.get_full_path())
            .unwrap().as_layout().get_scrolling_config().clone();

        let mut position = Coordinates::new(0, 0);
        let mut content_list = Vec::new();
        for child in self.get_children() {

            let generic_child = child.as_ez_object();
            let child_state =
                state_tree.get_mut(&generic_child.get_full_path()).unwrap().as_generic_mut();

            // If we're scrolling on an axis then the child can be infinite size on that axis
            if own_size.infinite_width || own_scrolling.enable_x {
                child_state.get_size_mut().infinite_width = true;
            }
            if own_size.infinite_height || own_scrolling.enable_y {
                child_state.get_size_mut().infinite_height = true;
            }

            // Determine how much height we have left to give the child. Can be 0 if we're scrolling
            // as we use [Size.infinite_height]
            let height_left =
                if !own_scrolling.enable_y && !own_size.infinite_height &&
                    own_size.height >= position.y && !child_state.get_size().infinite_height
                {own_size.height - position.y } else { 0 };
            // If autoscaling is enabled set child size to max width. It is then expected to scale
            // itself according to its' content
            if child_state.get_auto_scale().width {
                child_state.get_size_mut().width = own_size.width
            }
            if child_state.get_auto_scale().height {
                child_state.get_size_mut().height = height_left
            }
            // Scale down child to remaining size in the case that the child is too large, rather
            // panicking.
            if !own_scrolling.enable_x && !own_size.infinite_width &&
                !child_state.get_size().infinite_width &&
                child_state.get_size().width > own_size.width {
                child_state.get_size_mut().width = own_size.width;
            }
            if !own_scrolling.enable_y && !own_size.infinite_height &&
                !child_state.get_size().infinite_height &&
                child_state.get_size().height > height_left {
                child_state.get_size_mut().height = height_left;
            }

            child_state.set_x(position.x);
            child_state.set_y(position.y);
            let child_content = generic_child.get_contents(state_tree);
            let state = state_tree.get_mut(&generic_child.get_full_path())
                .unwrap().as_generic_mut(); // re-borrow
            if state.get_size().infinite_width {
                state.get_size_mut().width = child_content.len()
            }
            if state.get_size().infinite_height {
                state.get_size_mut().height = child_content[0].len()
            }
            position.y += if !child_content.is_empty() {child_content[0].len()} else {0};
            content_list.push(child_content);
        }
        if own_scaling.width {
            state_tree.get_mut(&self.path).unwrap().as_layout_mut().set_effective_width(
                content_list.iter().map(|child| child.len()).max().unwrap());
        }
        if own_scaling.height {
            state_tree.get_mut(&self.path).unwrap().as_layout_mut().set_effective_height(
                content_list.iter().map(
                    |child| child.iter().map(|x| x.len()).max().unwrap())
                    .max().unwrap());
        }
        let own_size = state_tree.get_mut(&self.get_full_path())
            .unwrap().as_layout().get_effective_size();
        let mut merged_content = PixelMap::new();
        for (i, content) in content_list.into_iter().enumerate() {
            merged_content = self.merge_vertical_contents(
                merged_content, content,
                own_size, own_colors.clone(),
                state_tree.get_mut(&self.children.get(i).unwrap().as_ez_object()
                    .get_full_path()).unwrap().as_generic_mut());
        }
        merged_content
    }

    /// Take a [PixelMap] and merge it horizontally with another [PixelMap]
    pub fn merge_horizontal_contents(&self, mut merged_content: PixelMap, mut new: PixelMap,
                                     parent_size: Size,
                                     parent_colors: ColorConfig, state: &mut dyn GenericState)
        -> PixelMap {

        if !parent_size.infinite_height && parent_size.height > new[0].len() {
            let offset;
            (new, offset) = common::widget_functions::align_content_vertically(
                new, state.get_vertical_alignment(), parent_size.height,
                parent_colors.foreground,
                parent_colors.background);
            state.set_y(state.get_position().y.get() + offset);
        }

        for x in 0..new.len() {
            merged_content.push(new[x].clone());
        }
        merged_content
    }

    /// Take a [PixelMap] and merge it vertically with another [PixelMap]
    pub fn merge_vertical_contents(&self, mut merged_content: PixelMap, mut new: PixelMap,
                                   parent_size: Size, parent_colors: ColorConfig,
                                   state: &mut dyn GenericState) -> PixelMap {

        if new.is_empty() {
            return merged_content
        }

        let offset;
        if parent_size.width > new.len() && !parent_size.infinite_width {
            (new, offset) = common::widget_functions::align_content_horizontally(
                new, state.get_horizontal_alignment(), parent_size.width,
                parent_colors.foreground,
                parent_colors.background);
            state.set_x(state.get_position().x.get() + offset);
        }

        let write_width = if !state.get_size().infinite_width { parent_size.width }
                              else { new.len() };
        for x in 0..write_width {
            for y in 0..new[0].len() {
                if x >= merged_content.len() {
                    merged_content.push(Vec::new());
                }
                if x < new.len() && y < new[x].len() {
                    merged_content[x].push(new[x][y].clone())
                } else {
                    merged_content[x].push(Pixel { symbol: " ".to_string(),
                        foreground_color: parent_colors.foreground,
                        background_color: parent_colors.background,
                        underline: false});
                }
            }
        }

        merged_content
    }
}

// Scrolling implementations
impl Layout {

    /// Handle command by user to scroll down by increasing the scroll_view of y
    fn handle_scroll_down(&self, state: &mut LayoutState) {

        if !state.get_scrolling_config().enable_y { return }
        let scroll_chunk = (state.get_effective_size().height as f32 * 0.75) as usize;
        let new_view_start;
        if state.get_scrolling_config().view_start_y + state.get_effective_size().height ==
            state.get_scrolling_config().original_height - state.get_effective_size().height {
            return
        } else if state.get_scrolling_config().view_start_y + scroll_chunk >
            state.get_scrolling_config().original_height - state.get_effective_size().height {
            new_view_start =
                state.get_scrolling_config().original_height - state.get_effective_size().height;
        } else {
            new_view_start = state.get_scrolling_config().view_start_y + scroll_chunk;
        }
        state.get_scrolling_config_mut().view_start_y = new_view_start;
    }

    /// Handle command by user to scroll down by decreasing the scroll_view of y
    fn handle_scroll_up(&self, state: &mut LayoutState) {

        if !state.get_scrolling_config().enable_y { return }
        let scroll_chunk = (state.get_effective_size().height as f32 * 0.75) as usize;
        let new_view_start;
        if state.get_scrolling_config().view_start_y == 0 {
            return
        }
        else if state.get_scrolling_config().view_start_y < scroll_chunk {
            new_view_start = 0;
        } else {
            new_view_start = state.get_scrolling_config().view_start_y - scroll_chunk;
        }
        state.get_scrolling_config_mut().view_start_y = new_view_start;
    }

    /// Handle command by user to scroll down by increasing the scroll_view of x
    fn handle_scroll_right(&self, state: &mut LayoutState) {

        if !state.get_scrolling_config().enable_x { return }
        let scroll_chunk = (state.get_effective_size().width as f32 * 0.75) as usize;
        let new_view_start;
        if state.get_scrolling_config().view_start_x + state.get_effective_size().width ==
            state.get_scrolling_config().original_width - state.get_effective_size().height {
            return
        } else if state.get_scrolling_config().view_start_x + scroll_chunk >
            state.get_scrolling_config().original_width - state.get_effective_size().width {
            new_view_start = state.get_scrolling_config().original_width - state.get_effective_size().width;
        } else {
            new_view_start = state.get_scrolling_config().view_start_x + scroll_chunk;
        }
        state.get_scrolling_config_mut().view_start_x = new_view_start;
    }

    /// Handle command by user to scroll down by decreasing the scroll_view of x
    fn handle_scroll_left(&self, state: &mut LayoutState) {

        if !state.get_scrolling_config().enable_x { return }
        let scroll_chunk = (state.get_effective_size().width as f32 * 0.75) as usize;
        let new_view_start;
        if state.get_scrolling_config().view_start_x == 0 {
            return
        }
        else if state.get_scrolling_config().view_start_x < scroll_chunk {
            new_view_start = 0;
        } else {
            new_view_start = state.get_scrolling_config().view_start_x - scroll_chunk;
        }
        state.get_scrolling_config_mut().view_start_x = new_view_start;
    }

    /// Create a horizontal scrollbox out of this layout if its contents width exceed its own width
    fn create_horizontal_scroll_box(&self, state_tree: &mut StateTree, contents: PixelMap)
                                    -> PixelMap {

        let state = state_tree.get_mut(&self.get_full_path()).unwrap()
            .as_layout_mut();
        if !state.scrolling_config.enable_x || contents.len() <= state.get_effective_size().width {
            state.scrolling_config.is_scrolling_x = false;
            return contents
        }
        state.scrolling_config.original_width = contents.len();
        state.scrolling_config.is_scrolling_x = true;
        let view_start = state.scrolling_config.view_start_x;
        let view_end =
            if contents.len() - view_start > state.get_effective_size().width {
                view_start + state.get_effective_size().width
            } else {
                contents.len()
            };
        self.propagate_absolute_positions(state_tree);
        self.create_horizontal_scrollbar(
            state_tree, contents[view_start..view_end].to_vec())
    }

    /// Create a vertical scrollbox out of this layout if its contents width exceed its own width
    fn create_vertical_scroll_box(&self, state_tree: &mut StateTree, contents: PixelMap)
        -> PixelMap {

        let state = state_tree.get_mut(&self.get_full_path()).unwrap()
            .as_layout_mut();
        if !state.scrolling_config.enable_y ||
            contents[0].len() <= state.get_effective_size().height {
            state.scrolling_config.is_scrolling_y = false;
            return contents
        }
        state.scrolling_config.original_height = contents[0].len();
        state.scrolling_config.is_scrolling_y = true;
        let view_start = state.scrolling_config.view_start_y;
        let view_end =
            if contents[0].len() - view_start > state.get_effective_size().height {
                view_start + state.get_effective_size().height
            } else {
                contents[0].len()
            };
        let scrolled_contents: PixelMap =
            contents.iter().map(|x| x[view_start..view_end].to_vec()).collect();
        self.propagate_absolute_positions(state_tree);
        self.create_vertical_scrollbar(state_tree, scrolled_contents)
    }

    /// Create a scrolling bar for a horizontal scrollbox
    fn create_horizontal_scrollbar(
        &self, state_tree: &mut StateTree, mut contents: PixelMap) -> PixelMap {

        let state = state_tree.get(&self.get_full_path()).unwrap().as_layout();
        let (fg_color, _) = state.get_context_colors();
        let bg_color = state.get_color_config().background;

        let (scrollbar_size, scrollbar_pos) = self.get_horizontal_scrollbar_parameters(
            state.get_scrolling_config().original_width,
            state.get_effective_size().width,
            state.get_scrolling_config().view_start_x);

        for (i, x) in contents.iter_mut().enumerate() {
            let symbol = if i >= scrollbar_pos
                && i <= scrollbar_pos + scrollbar_size
            { "▀".to_string()} else {" ".to_string()};
            x.push(Pixel::new(symbol, fg_color,bg_color))
        }
        contents
    }

    /// Create a scrolling bar for a vertical scrollbox
    fn create_vertical_scrollbar(
        &self, state_tree: &mut StateTree, mut contents: PixelMap) -> PixelMap {

        let mut scrollbar = Vec::new();
        let state = state_tree.get(&self.get_full_path()).unwrap().as_layout();
        let (fg_color, _) = state.get_context_colors();
        let bg_color = state.get_color_config().background;

        let (scrollbar_size, scrollbar_pos) = self.get_vertical_scrollbar_parameters(
            state.get_scrolling_config().original_height,
            state.get_effective_size().height,
            state.get_scrolling_config().view_start_y);

        for x in 0..state.get_effective_size().height {
            let symbol = if x >= scrollbar_pos
                && x <= scrollbar_pos + scrollbar_size
            { "▐".to_string()} else {" ".to_string()};
            scrollbar.push(Pixel::new(symbol, fg_color,bg_color))
        }
        contents.push(scrollbar);
        contents
    }

    fn get_horizontal_scrollbar_parameters(&self, content_width: usize, widget_width: usize,
                                           view_start: usize) -> (usize, usize) {

        let scrollbar_ratio =  content_width as f32
            / widget_width as f32;
        let scrollbar_size =
            (widget_width as f32 / scrollbar_ratio) as usize;
        let mut scrollbar_pos =
            if view_start != 0 { (view_start as f32 / scrollbar_ratio).round() as usize }
            else { 0 };
        if scrollbar_pos == 0 && view_start != 0 { scrollbar_pos = 1 }
        (scrollbar_size, scrollbar_pos)
    }

    fn get_vertical_scrollbar_parameters(&self, content_height: usize, widget_height: usize,
                                         view_start: usize) -> (usize, usize) {

        let scrollbar_ratio =  content_height as f32
            / widget_height as f32;
        let scrollbar_size =
            (widget_height as f32  / scrollbar_ratio) as usize;
        let mut scrollbar_pos =
            if view_start != 0 { (view_start as f32 / scrollbar_ratio).round() as usize }
            else { 0 };
        if scrollbar_pos == 0 && view_start != 0 { scrollbar_pos = 1 };
        (scrollbar_size, scrollbar_pos)
    }
}
