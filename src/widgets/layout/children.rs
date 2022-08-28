use crate::{CallbackConfig, Context, EzObject};
use crate::run::definitions::{IsizeCoordinates, PixelMap, StateTree};
use crate::scheduler::scheduler::SchedulerFrontend;
use crate::states::ez_state::GenericState;
use crate::states::definitions::{LayoutMode, LayoutOrientation};
use crate::widgets::button::Button;
use crate::widgets::ez_object::EzObjects;
use crate::widgets::helper_functions::{offset_scrolled_absolute_position, resize_with_size_hint};
use crate::widgets::layout::layout::Layout;

impl Layout {


    /// Set the sizes of children that use size_hint(s) using own proportions.
    pub fn set_child_sizes(&self, state_tree: &mut StateTree) {

        let own_state = state_tree.get_mut(&self.get_path())
            .as_layout();
        let own_width = own_state.get_effective_size().width;
        let own_height = own_state.get_effective_size().height;
        let own_mode = own_state.mode.value.clone();
        let own_rows = own_state.get_table_config().rows.value;
        let own_cols = own_state.get_table_config().cols.value;

        // Check if there are multiple children who ALL have size_hint=1, and in
        // that case give them '1 / number_of_children'. That way the user can add
        // multiple children in a Box layout and have them distributed equally automatically. Any
        // kind of asymmetry breaks this behavior.
        if self.children.len() > 1 &&
            [LayoutMode::Box, LayoutMode::Table].contains(own_state.get_mode()) {
            let (all_default_size_hint_x, all_default_size_hint_y) =
                self.check_default_size_hints(state_tree);
            if all_default_size_hint_x {
                if own_mode == LayoutMode::Box {
                    for child in self.get_children() {
                        let generic_child = child.as_ez_object();
                        let state = state_tree.get_mut(&generic_child.get_path())
                            .as_generic_mut();
                        state.set_size_hint_x(Some(1.0 / (self.children.len() as f64)));
                    }
                } else {
                    let divide_by =
                        if own_cols > 0 {
                            own_cols
                        } else {
                            self.children.len() / own_rows
                                + if self.children.len() % own_rows
                                    > 0 {1} else {0}
                        };
                    for child in self.get_children() {
                        let generic_child = child.as_ez_object();
                        let state = state_tree.get_mut(&generic_child.get_path())
                            .as_generic_mut();
                        state.set_size_hint_x(Some(1.0 / divide_by as f64));
                    }
                }
            }
            if all_default_size_hint_y {
                if own_mode == LayoutMode::Box {
                    for child in self.get_children() {
                        let generic_child = child.as_ez_object();
                        let state = state_tree.get_mut(&generic_child.get_path())
                            .as_generic_mut();
                        state.set_size_hint_y(Some(1.0 / (self.children.len() as f64)));
                    }
                } else {
                    let divide_by =
                        if own_rows > 0 {
                            own_rows
                        } else {
                            self.children.len() / own_cols
                                + if self.children.len() % own_cols > 0 {1} else {0}
                        };
                    for child in self.get_children() {
                        let generic_child = child.as_ez_object();
                        let state = state_tree.get_mut(&generic_child.get_path())
                            .as_generic_mut();
                        state.set_size_hint_y(Some(1.0 / divide_by as f64));
                    }
                }
            }
        }
        // Now calculate actual sizes.
        for child in self.get_children() {
            let generic_child = child.as_ez_object();
            let state = &mut state_tree.get_mut(&generic_child.get_path()).obj;
            resize_with_size_hint(state, own_width, own_height);
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
        let own_orientation = state_tree.get(&self.path)
            .as_layout().get_orientation().clone();
        for child in self.get_children() {
            if !all_default_size_hint_x && !all_default_size_hint_y {
                break
            }
            let generic_child = child.as_ez_object();
            let state = state_tree
                .get(&generic_child.get_path()).as_generic();
            if own_orientation != LayoutOrientation::Vertical {
                if let Some(size_hint_x) = state.get_size_hint().get_size_hint_x()
                {
                    if size_hint_x != 1.0 || state.get_auto_scale().get_auto_scale_width() ||
                        state.get_size().fixed_width {
                        all_default_size_hint_x = false;
                    }
                } else {
                    all_default_size_hint_x = false;
                }
            } else {
                all_default_size_hint_x = false;
            }
            if own_orientation != LayoutOrientation::Horizontal {
                if let Some(size_hint_y) = state.get_size_hint().get_size_hint_y() {
                    if size_hint_y != 1.0 || state.get_auto_scale().get_auto_scale_height() ||
                        state.get_size().fixed_height {
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
        let absolute_position = state_tree.get(&self.path).as_generic()
            .get_effective_absolute_position();
        let effective_size = state_tree.get(&self.path).as_layout()
            .get_effective_size().clone();
        let scrolling = state_tree.get(&self.path).as_layout()
            .get_scrolling_config().clone();
        for child in self.get_children() {
            if let EzObjects::Layout(i) = child {
                let child_state =
                    state_tree.get_mut(&i.get_path()).as_generic_mut();
                let pos = child_state.get_position();
                let mut new_absolute_position = IsizeCoordinates::new(
                    absolute_position.x + pos.get_x() as isize,
                    absolute_position.y + pos.get_y() as isize);
                new_absolute_position = offset_scrolled_absolute_position(
                    new_absolute_position, &scrolling, &effective_size);
                child_state.set_absolute_position(new_absolute_position);
                i.propagate_absolute_positions(state_tree);
            } else {
                let child_state = state_tree.get_mut(
                    &child.as_ez_object().get_path()).as_generic_mut();
                let pos = child_state.get_position();
                let mut new_absolute_position = IsizeCoordinates::new(
                    absolute_position.x + pos.get_x() as isize,
                    absolute_position.y + pos.get_y() as isize);
                new_absolute_position = offset_scrolled_absolute_position(
                    new_absolute_position, &scrolling, &effective_size);
                child_state.set_absolute_position(new_absolute_position);
            }
        }
    }

    /// Takes full [path] of this layout and adds the [id] of children to create and set
    /// their [path]. Then calls this method on children, thus recursively setting
    /// [path] for all children. Use on root layout to propagate all absolute positions.
    pub fn propagate_paths(&mut self) {

        let path = self.get_path();
        for child in self.get_children_mut() {
            if let EzObjects::Layout(i) = child {
                i.set_path(format!("{}/{}", path, i.get_id()).as_str());
                i.propagate_paths();
            } else {
                let generic_child = child.as_ez_object_mut();
                generic_child.set_path(format!("{}/{}", path, generic_child.get_id()).as_str());
            }
        }
    }

    /// Add a child ([layout] or [EzWidget]) to this layout.
    pub fn add_child(&mut self, mut child: EzObjects, scheduler: &mut SchedulerFrontend) {
        
        let generic_child = child.as_ez_object_mut();
        let id = generic_child.get_id();
        let parent_path = self.path.clone();
        if self.child_lookup.contains_key(&id) {
            panic!("A layout may not contain two children with identical IDs: \"{}\"",
                   generic_child.get_id());
        }

        self.child_lookup.insert(generic_child.get_id(), self.children.len());
        self.children.push(child.clone());

        if self.state.get_mode() == &LayoutMode::Tab {
            if let EzObjects::Layout(_) = child.clone() {
                let tab_name = child.as_layout().state.get_tab_name();
                let tab_path = child.as_layout().get_path();
                let new_id = format!("{}_tab_header", tab_name);
                let new_path = format!("{}/{}", parent_path, new_id);
                let mut tab_header = Button::new(new_id, new_path.clone(), scheduler);
                tab_header.state.set_size_hint_x(None);
                tab_header.state.set_size_hint_y(None);
                tab_header.state.set_text(tab_name.clone());
                tab_header.state.colors.fg_color.set(self.state.colors.tab_header_fg_color.value);
                tab_header.state.colors.bg_color.set(self.state.colors.tab_header_bg_color.value);
                tab_header.state.colors.disabled_fg_color.set(self.state.colors.tab_header_active_fg_color.value);
                tab_header.state.colors.disabled_bg_color.set(self.state.colors.tab_header_active_bg_color.value);
                tab_header.state.colors.border_fg_color.set(self.state.colors.tab_header_border_fg_color.value);
                tab_header.state.colors.border_bg_color.set(self.state.colors.tab_header_border_bg_color.value);
                tab_header.state.colors.selection_fg_color.set(self.state.colors.selection_fg_color.value);
                tab_header.state.colors.selection_bg_color.set(self.state.colors.selection_bg_color.value);

                let tab_on_click = move |context: Context| {
                    let state = context.state_tree
                        .get_mut(&parent_path).as_layout_mut();
                    state.set_active_tab(&tab_path.clone());
                    state.update(context.scheduler);
                    true
                };
                let callback_config = CallbackConfig::from_on_press(
                    Box::new(tab_on_click));
                scheduler.overwrite_callback_config(tab_header.path.as_str(), callback_config);
                self.add_child(EzObjects::Button(tab_header), scheduler);
            }
        }
    }

    /// Remove a widget. Never remove a child directly but call this instead. It keeps the child
    /// lookup table cache up to date.
    pub fn remove_child(&mut self, id: &str) {

        let widget_index = self.child_lookup.get(id)
            .unwrap_or_else(|| panic!("Could not remove widget: {}. It could not be found.",
                                      id)).clone();
        self.children.remove(widget_index);
        self.child_lookup.clear();
        for (i, child) in self.children.iter().enumerate() {
            self.child_lookup.insert(child.as_ez_object().get_id(), i);
        }


    }

    /// Get a list of children non-recursively. Can be [layout] or [EzWidget]
    pub fn get_children(&self) -> &Vec<EzObjects> { &self.children }

    /// Get a mutable list of children non-recursively. Can be [layout] or [EzWidget]
    pub fn get_children_mut(&mut self) -> &mut Vec<EzObjects> { &mut self.children }

    /// Get a specific child ref by its' [id]
    pub fn get_child(&self, id: &str) -> Option<&EzObjects> {

        if let Some(index) = self.child_lookup.get(id) {
            Some(self.children.get(*index).unwrap())
        } else {
            None
        }
    }

    /// Get a specific child mutable ref by its'[id]
    pub fn get_child_mut(&mut self, id: &str) -> Option<&mut EzObjects> {

        if let Some(index) = self.child_lookup.get(id) {
            Some(self.children.get_mut(*index).unwrap())
        } else {
            None
        }
    }

    /// Get a specific child ref by its' [path]. Call on root layout to find any EzObject that
    /// exists
    pub fn get_child_by_path(&self, path: &str) -> Option<&EzObjects> {

        if path == "/root/modal" && self.path == "/root" {
            return if self.state.has_modal() {
                Some(self.state.get_modal())
            } else {
                None
            }
        }

        let mut paths: Vec<&str> = path.split('/').filter(|x| !x.is_empty()).collect();
        // If user passed a path starting with this layout, take it off first.
        if paths.first().unwrap() == &self.get_id() {
            paths.remove(0);
        }
        paths.reverse();

        let first = paths.pop().unwrap();
        let mut root = if first == "modal" {
            if self.state.has_modal() { self.state.get_modal() }
            else { return None }
        } else {
            if let Some(i) = self.get_child(first) { i }
            else { return None }
        };
        while !paths.is_empty() {
            if let EzObjects::Layout(layout) = root {
                if let Some(i) = layout.get_child(paths.pop().unwrap()) {
                    root = i;
                } else {
                    return None
                }
            } else {
                return None
            }
        }
        Some(root)
    }

    /// Get a specific child mutable ref by its' [path]. Call on root layout to find any
    /// [EzObject] that exists
    pub fn get_child_by_path_mut(&mut self, path: &str) -> Option<&mut EzObjects> {

        if path == "/root/modal" && self.path == "/root" {
            return if self.state.has_modal() {
                Some(self.state.get_modal_mut())
            } else {
                None
            }
        }

        let mut paths: Vec<&str> = path.split('/').filter(|x| !x.is_empty()).collect();
        if paths.first().unwrap() == &self.get_id() {
            paths.remove(0);
        }
        paths.reverse();

        let first = paths.pop().unwrap();
        let mut root = if first == "modal" {
            if self.state.has_modal() { self.state.get_modal_mut() }
            else { return None }
        } else {
            if let Some(i) = self.get_child_mut(first) { i }
            else { return None } };
        while !paths.is_empty() {
            if let Some(i) = root.as_layout_mut().get_child_mut(paths.pop().unwrap()) {
                root = i
            } else {
                return None
            }
        }
        Some(root)
    }

    /// Get a list of all children refs recursively. Call on root [layout] for all [EzWidgets] that
    /// exist.
    pub fn get_widgets_recursive(&self) -> Vec<&EzObjects> {
        let mut results = Vec::new();
        self._get_widgets_recursive(&mut results);
        results
    }
    fn _get_widgets_recursive<'a>(&'a self, results: &mut Vec<&'a EzObjects>) {
        for child in self.get_children() {
            results.push(child);
            if let EzObjects::Layout(i) = child {
                i._get_widgets_recursive(results);
            }
        }
    }

    pub fn scale_to_largest_child(&self, content_list: &[PixelMap], state_tree: &mut StateTree){

        let state = state_tree.get_mut(&self.path).as_layout_mut();
        if state.get_auto_scale().get_auto_scale_width() {
            state.set_effective_width(
                if state.get_orientation() == &LayoutOrientation::Vertical {
                    content_list.iter().map(|x| x.len()).max().unwrap_or(0)
                } else {
                    content_list.iter().map(|x| x.len()).sum()
                });
        }
        if state.get_auto_scale().get_auto_scale_height() {
            state.set_effective_height(
                if state.get_orientation() == &LayoutOrientation::Horizontal {
                    content_list.iter().map(
                        |child| child.iter().map(|x| x.len()).max().unwrap_or(0))
                        .max().unwrap_or(0)
                } else {
                    content_list.iter().map(
                        |child| child.iter().map(|x| x.len()).max().unwrap_or(0))
                        .sum()
                });
        }
    }
}
