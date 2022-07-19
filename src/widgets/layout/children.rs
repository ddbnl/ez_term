use std::collections::HashMap;
use crate::{CallbackConfig, EzContext, EzObject};
use crate::run::definitions::{Coordinates, PixelMap, StateTree};
use crate::scheduler::scheduler::Scheduler;
use crate::states::ez_state::GenericState;
use crate::states::definitions::{LayoutMode, LayoutOrientation};
use crate::widgets::button::Button;
use crate::widgets::ez_object::EzObjects;
use crate::widgets::helper_functions::{offset_scrolled_absolute_position, resize_with_size_hint};
use crate::widgets::layout::layout::Layout;

impl Layout {


    /// Set the sizes of children that use size_hint(s) using own proportions.
    pub fn set_child_sizes(&self, state_tree: &mut StateTree) {

        let own_state = state_tree.get_by_path_mut(&self.get_full_path())
            .as_layout();
        let own_width = own_state.get_effective_size().width;
        let own_height = own_state.get_effective_size().height;

        // Check if there are multiple children who ALL have size_hint=1, and in
        // that case give them '1 / number_of_children'. That way the user can add
        // multiple children in a Box layout and have them distributed equally automatically. Any
        // kind of asymmetry breaks this behavior.
        if self.children.len() > 1 &&
            own_state.mode != LayoutMode::Tabbed && own_state.mode != LayoutMode::Screen {
            let (all_default_size_hint_x, all_default_size_hint_y) =
                self.check_default_size_hints(state_tree);
            if all_default_size_hint_x {
                for child in self.get_children() {
                    let generic_child = child.as_ez_object();
                    let state = state_tree.get_by_path_mut(&generic_child.get_full_path())
                        .as_generic_mut();
                    state.set_size_hint_x(Some(1.0 / (self.children.len() as f64)));
                }
            }
            if all_default_size_hint_y {
                for child in self.get_children() {
                    let generic_child = child.as_ez_object();
                    let state = state_tree.get_by_path_mut(&generic_child.get_full_path())
                        .as_generic_mut();
                    state.set_size_hint_y(Some(1.0 / (self.children.len() as f64)));
                }
            }
        }
        // Now calculate actual sizes.
        for child in self.get_children() {
            let generic_child = child.as_ez_object();
            let state = state_tree.get_by_path_mut(&generic_child.get_full_path());
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
        let own_orientation = state_tree.get_by_path(&self.path)
            .as_layout().orientation.clone();
        for child in self.get_children() {
            if !all_default_size_hint_x && !all_default_size_hint_y {
                break
            }
            let generic_child = child.as_ez_object();
            let state = state_tree
                .get_by_path(&generic_child.get_full_path()).as_generic();
            if let LayoutOrientation::Horizontal = own_orientation {
                if let Some(size_hint_x) = state.get_size_hint().x.value
                {
                    if size_hint_x != 1.0 || state.get_auto_scale().width.value ||
                        state.get_auto_scale().height.value || state.get_size().width > 0 {
                        all_default_size_hint_x = false;
                    }
                } else {
                    all_default_size_hint_x = false;
                }
            } else {
                all_default_size_hint_x = false;
            }
            if let LayoutOrientation::Vertical = own_orientation {
                if let Some(size_hint_y) = state.get_size_hint().y.value {
                    if size_hint_y != 1.0 || state.get_auto_scale().height.value ||
                        state.get_auto_scale().width.value || state.get_size().height > 0 {
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
        let absolute_position = state_tree.get_by_path(&self.path).as_generic()
            .get_effective_absolute_position();
        let size = state_tree.get_by_path(&self.path).as_layout()
            .get_size().clone();
        let scrolling = state_tree.get_by_path(&self.path).as_layout()
            .get_scrolling_config().clone();
        for child in self.get_children() {
            if let EzObjects::Layout(i) = child {
                let child_state =
                    state_tree.get_by_path_mut(&i.get_full_path()).as_generic_mut();
                let pos = child_state.get_position();
                let mut new_absolute_position = Coordinates::new(
                    absolute_position.x + pos.x.get(), absolute_position.y + pos.y.get());
                new_absolute_position = offset_scrolled_absolute_position(
                    new_absolute_position, &scrolling, &size);
                child_state.set_absolute_position(new_absolute_position);
                i.propagate_absolute_positions(state_tree);
            } else {
                let child_state = state_tree.get_by_path_mut(
                    &child.as_ez_object().get_full_path()).as_generic_mut();
                let pos = child_state.get_position();
                let mut new_absolute_position = Coordinates::new(
                    absolute_position.x + pos.x.get(), absolute_position.y + pos.y.get());
                new_absolute_position = offset_scrolled_absolute_position(
                    new_absolute_position, &scrolling, &size);
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

    /// Add a child ([layout] or [EzWidget]) to this layout.
    pub fn add_child(&mut self, mut child: EzObjects, scheduler: &mut Scheduler) {
        
        let generic_child = child.as_ez_object_mut();
        let id = generic_child.get_id();
        let path = generic_child.get_full_path();
        let parent_path = self.path.clone();
        if self.child_lookup.contains_key(&id) {
            panic!("A layout may not contain two children with identical IDs: \"{}\"",
                   generic_child.get_id());
        }

        self.child_lookup.insert(generic_child.get_id(), self.children.len());
        self.children.push(child.clone());

        if self.state.mode == LayoutMode::Tabbed {
            if let EzObjects::Layout(_) = child.clone() {
                let new_id = format!("{}_tab_header", id);
                let new_path = format!("{}/{}", parent_path, new_id);
                let mut tab_header = Button::new(new_id, new_path, scheduler);
                tab_header.state.set_size_hint_x(None);
                tab_header.state.set_size_hint_y(None);
                tab_header.state.text.set(id);

                let tab_on_click = move |context: EzContext| {
                    let state = context.state_tree
                        .get_by_path_mut(&parent_path).as_layout_mut();
                    state.set_active_tab(path.clone());
                    state.update(context.scheduler);
                    true
                };
                let callback_config = CallbackConfig::from_on_press(
                    Box::new(tab_on_click));
                scheduler.set_callback_config(tab_header.path.as_str(), callback_config);
                self.add_child(EzObjects::Button(tab_header), scheduler);
            }
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

        let mut paths: Vec<&str> = path.split('/').filter(|x| !x.is_empty()).collect();
        // If user passed a path starting with this layout, take it off first.
        if paths.first().unwrap() == &self.get_id() {
            paths.remove(0);
        }
        paths.reverse();

        let first = paths.pop().unwrap();
        let mut root = if first == "modal" {
            if let Some(i) = self.state.open_modals.first() { paths.pop();  i }
            else { return None }
        } else {
            if let Some(i) = self.get_child(first) { i }
            else { return None }
        };
        while !paths.is_empty() {
            if let Some(i) = root.as_layout().get_child(paths.pop().unwrap()) {
                root = i;
            } else {
                return None
            }
        }
        Some(root)
    }

    /// Get a specific child mutable ref by its' [path]. Call on root layout to find any
    /// [EzObject] that exists
    pub fn get_child_by_path_mut(&mut self, path: &str) -> Option<&mut EzObjects> {

        let mut paths: Vec<&str> = path.split('/').filter(|x| !x.is_empty()).collect();
        if paths.first().unwrap() == &self.get_id() {
            paths.remove(0);
        }
        paths.reverse();

        let first = paths.pop().unwrap();
        let mut root = if first == "modal" {
            if let Some(i) = self.state.open_modals.first_mut() { i }
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

    pub fn scale_to_largest_child(&self, content_list: &[PixelMap], state_tree: &mut StateTree){

        let state = state_tree.get_by_path_mut(&self.path).as_layout_mut();
        if state.auto_scale.width.value {
            state.set_effective_width(
                content_list.iter().map(|x| x.len()).max().unwrap());
        }
        if state.auto_scale.height.value {
            state.set_effective_height(
                content_list.iter().map(
                    |child| child.iter().map(|x| x.len()).max().unwrap_or(0))
                    .max().unwrap());
        }
    }
}
