//! # Tree
//!
//! This module contains implementations for the various runtime trees.
use std::collections::HashMap;
use std::io::{Error, ErrorKind};

use crossterm::style::StyledContent;

use crate::{CallbackConfig, GenericState};
use crate::run::definitions::{CallbackTree, Coordinates, Pixel, PixelMap, StateTree};
use crate::scheduler::scheduler::SchedulerFrontend;
use crate::states::button_state::ButtonState;
use crate::states::canvas_state::CanvasState;
use crate::states::checkbox_state::CheckboxState;
use crate::states::dropdown_state::{DropdownState, DroppedDownMenuState};
use crate::states::ez_state::EzState;
use crate::states::label_state::LabelState;
use crate::states::layout_state::LayoutState;
use crate::states::progress_bar_state::ProgressBarState;
use crate::states::radio_button_state::RadioButtonState;
use crate::states::slider_state::SliderState;
use crate::states::text_input_state::TextInputState;
use crate::widgets::ez_object::EzObject;
use crate::widgets::layout::layout::Layout;

/// Convenience wrapper for [StateTree], [WidgetTree] and [CallbackTree]. Allows getting objects
/// by the ID of a widget as well as full path.
#[derive(Default, Clone)]
pub struct Tree<T> {

    pub id: String,

    pub obj: T,

    /// HashMap of objects to provide caching and ID lookup for
    objects: HashMap<String, Tree<T>>,

    id_cache: HashMap<String, Vec<String>>,

}
impl<T> Tree<T> {

    pub fn new(name: String, node: T) -> Self {
        Tree { id: name, obj: node, objects: HashMap::new(), id_cache: HashMap::new(), }
    }

    /// Append another tree to this tree.
    pub fn extend(&mut self, path: String, node: Tree<T>) {

        let steps: Vec<&str> = path.split('/').collect();
        let mut steps = steps[1..].to_vec();
        if steps[0] == "root" {
            steps.remove(0);
        }
        self._extend(steps, node);
    }

    fn _extend(&mut self, mut steps: Vec<&str>, tree: Tree<T>) {

        self.id_cache.extend(tree.id_cache.clone());
        self.id_cache.insert(steps.last().unwrap().to_string(),
                             steps.iter().map(|x| x.to_string()).collect());
        if steps.len() == 1 {
            let id = steps.pop().unwrap();
            self.objects.insert(id.to_string(), tree);
        } else {
            self.objects.get_mut(steps.remove(0)).unwrap()._extend(steps, tree);
        }
    }

    /// Add a node to the state tree. There's generally no reason to use this as an
    /// end-user.
    pub fn add_node(&mut self, path: String, node: T) {

        let steps: Vec<&str> = path.split('/').collect();
        let mut steps = steps[1..].to_vec();
        if steps[0] == "root" {
            steps.remove(0);
        }
        self._add_node(steps, node);
    }

    fn _add_node(&mut self, mut steps: Vec<&str>, node: T) {

        self.id_cache.insert(steps.last().unwrap().to_string(),
                             steps.iter().map(|x| x.to_string()).collect());
        if steps.len() == 1 {
            let id = steps.pop().unwrap();
            let node = Tree::new(id.to_string(), node);
            self.objects.insert(id.to_string(), node);
        } else {
            self.objects.get_mut(steps.remove(0)).unwrap()._add_node(steps, node);
        }
    }

    /// Remove a node from the state tree. There's generally no reason to use this as
    /// an end-user; the tree will be pruned regularly.
    pub fn remove_node(&mut self, path: String) -> Self {

        let steps = if !path.contains('/') {
            let _steps = vec!(path.as_str());
            if _steps[0] == self.id {
                panic!("Cannot remove self from tree. Remove from parent \
                                           node instead")
            }
            _steps
        } else {
            let _steps: Vec<&str> = path.split('/').collect();
            let mut _steps = _steps[1..].to_vec();
            if _steps[0] == self.id {
                if _steps.len() == 1 {
                    panic!("Cannot remove self from tree. Remove from parent \
                                               node instead")
                } else {
                    _steps.remove(0);
                }
            }
            _steps
        };
        self._remove_node(steps)
    }

    fn _remove_node(&mut self, mut steps: Vec<&str>) -> Self {

        if steps.len() == 1 {
            let id = steps.remove(0);
            self.objects.remove(id).unwrap_or_else(
                || panic!("Node '{}' could not resolve '{:?}' at step '{}' when removing state",
                          self.id, steps, id))
        } else {
            let id = steps.remove(0);
            return self.objects.get_mut(id).unwrap_or_else(
                || panic!("Node '{}' could not resolve '{:?}' at step '{}' when removing state",
                          self.id, steps, id))
                ._remove_node(steps)
        }
    }

    /// Find a node on the (state) tree and get a ref. Parameter can be an ID or a path; both will
    /// work if the node exists. When using an ID, make sure that the ID is actually
    /// unique from the point in the tree you are searching. Searching with ID from the tree root
    /// will also work fine, just make sure the ID is globally unique.
    /// Alternatively, you can hop from node to node:
    /// ```
    /// use ez_term::StateTree;
    /// let tree: StateTree;
    /// let label_state = tree.get("layout").get("sub_layout").get("widget").as_label();
    /// ```
    /// This is quite verbose however. Using path is more compact. You don't have to start your path
    /// with '/root' because the tree object is already the root (but you can if you want, it'll
    /// work):
    /// ```
    /// use ez_term::StateTree;
    /// let tree: StateTree;
    /// let label_state = tree.get("/layout/sub_layout/widget").as_label();
    /// ```
    /// The easiest and most compact way is to just make sure that your IDs are globally unique if
    /// at all possible. Then you can search by ID:
    /// ```
    /// use ez_term::StateTree;
    /// let tree: StateTree;
    /// let label_state = tree.get("widget").as_label();
    /// ```
    pub fn get(&self, node: &str) -> &Tree<T> {

        if node.starts_with('/') {
            self.get_recursive(node).unwrap()
        } else {
            if self.objects.contains_key(node) {
                self.objects.get(node).unwrap()
            } else if node == self.id {
                self
            } else {
                self.get_by_id(node).unwrap()
            }
        }
    }

    /// Find a node on the (state) tree and get a mut ref. Parameter can be an ID or a path; both will
    /// work if the node exists. When using an ID, make sure that the ID is actually
    /// unique from the point in the tree you are searching. Searching with ID from the tree root
    /// will also work fine, just make sure the ID is globally unique.
    /// Alternatively, you can hop from node to node:
    /// ```
    /// use ez_term::StateTree;
    /// let tree: StateTree;
    /// let label_state = tree.get("layout").get("sub_layout").get("widget").as_label();
    /// ```
    /// This is quite verbose however. Using path is more compact. You don't have to start your path
    /// with '/root' because the tree object is already the root (but you can if you want, it'll
    /// work):
    /// ```
    /// use ez_term::StateTree;
    /// let tree: StateTree;
    /// let label_state = tree.get("/layout/sub_layout/widget").as_label();
    /// ```
    /// The easiest and most compact way is to just make sure that your IDs are globally unique if
    /// at all possible. Then you can search by ID:
    /// ```
    /// use ez_term::StateTree;
    /// let tree: StateTree;
    /// let label_state = tree.get("widget").as_label();
    /// ```
    pub fn get_mut(&mut self, id: &str) -> &mut Tree<T> {

        if id.starts_with('/') {
            self.get_recursive_mut(id).unwrap()
        } else {
            if self.objects.contains_key(id) {
                self.objects.get_mut(id).unwrap()
            } else if id == self.id {
                self
            } else {
                self.get_by_id_mut(id).unwrap()
            }
        }
    }

    /// Check if a path or ID exists in the tree. You won't have to use this often; if a widget
    /// exists then the widget state will exist. Might be useful if you're programmatically creating
    /// and destroying widgets.
    pub fn contains(&self, id: &str) -> bool {

        if id.starts_with('/') {
            self.get_recursive(id).is_ok()
        } else {
            if self.objects.contains_key(id) {
                self.objects.get(id).is_some()
            } else if id == self.id {
                true
            } else {
                self.get_by_id(id).is_ok()
            }
        }
    }

    fn get_recursive(&self, path: &str) -> Result<&Tree<T>, Error> {
        let steps: Vec<&str> = path.split('/').collect();
        let mut steps = steps[1..].to_vec();
        if steps[0] == self.id {
            if steps.len() == 1 {
                return Ok(self)
            } else {
                steps.remove(0);
            }
        }
        Ok(self._get_recursive(&steps)?)
    }

    fn _get_recursive(&self, steps: &Vec<&str>) -> Result<&Tree<T>, Error> {

        let mut node = self;
        for step in steps {
            let next = node.objects.get(*step);
            if let Some(i) = next {
                node = i;
            } else {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("Could not resolve '{:?}'", steps)));
            }
        }
        Ok(node)
    }

    fn get_recursive_mut(&mut self, path: &str) -> Result<&mut Tree<T>, Error> {
        let steps: Vec<&str> = path.split('/').collect();
        let mut steps = steps[1..].to_vec();
        if steps[0] == self.id {
            if steps.len() == 1 {
                return Ok(self)
            } else {
                steps.remove(0);
            }
        }
        Ok(self._get_recursive_mut(&steps)?)
    }

    fn _get_recursive_mut(&mut self, steps: &Vec<&str>) -> Result<&mut Tree<T>, Error> {
        let mut node = self;
        for step in steps {
            let next = node.objects.get_mut(*step);
            if let Some(i) = next {
                node = i;
            } else {
                 return Err(Error::new(
                     ErrorKind::InvalidData,
                     format!("Could not resolve '{:?}'", steps)));
            }

        }
        Ok(node)
    }

    /// Get object refs in the tree recursively, including self.
    pub fn get_all(&self) -> Vec<&T> {

        let mut results = Vec::new();
        for node in self.objects.values() {
            results.extend(node.get_all());
        }
        results.push(&self.obj);
        results
    }

    /// Get object mut refs in the tree recursively including self.
    pub fn get_all_mut(&mut self) -> Vec<&mut T> {

        let mut results = Vec::new();
        for node in self.objects.values_mut() {
            results.extend(node.get_all_mut());
        }
        results.push(&mut self.obj);
        results
    }

    /// Get a ref by full widget ID. If the widget ID is not globally unique it will panic in
    /// order to prevent unexpected behavior. If you want to find widgets by ID make sure the ID
    /// is unique.
    fn get_by_id(&self, id: &str) -> Result<&Tree<T>, Error> {

        let steps = self.id_cache.get(id).cloned();
        if let Some(path) = steps {
            Ok(self._get_recursive(&path.iter().map(|x| x.as_str()).collect())?)
        } else {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("ID '{}' could not be found in '{}'. It either does not \
                       exist or is not unique in this tree", id, self.id)))
        }
    }

    /// Get a mut ref by full widget ID. If the widget ID is not globally unique it will panic in
    /// order to prevent unexpected behavior. If you want to find widgets by ID make sure the ID
    /// is unique.
    fn get_by_id_mut(&mut self, id: &str) -> Result<&mut Tree<T>, Error> {

        let steps = self.id_cache.get(id).cloned();
        if let Some(path) = steps {
            Ok(self._get_recursive_mut(&path.iter().map(|x| x.as_str()).collect())?)
        } else {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("ID '{}' could not be found in '{}'. It either does not \
                       exist or is not unique in this tree", id, self.id)))
        }
    }
}
impl Tree<EzState> {

    /// Cast this state as a layout state ref, you must be sure you have one.
    pub fn as_generic(&self) -> &dyn GenericState {
        self.obj.as_generic()
    }

    /// Cast this state as a mutable layout state ref, you must be sure you have one.
    pub fn as_generic_mut(&mut self) -> &mut dyn GenericState {
        self.obj.as_generic_mut()
    }

    /// Cast this state as a layout state ref, you must be sure you have one.
    pub fn as_layout(&self) -> &LayoutState {
        if let EzState::Layout(ref i) = self.obj { i }
        else { panic!("LayoutState is the wrong type for {}.", self.id) }
    }

    /// Cast this state as a mutable layout state ref, you must be sure you have one.
    pub fn as_layout_mut(&mut self) -> &mut LayoutState {
        if let EzState::Layout(ref mut i) = self.obj { i }
        else { panic!("LayoutState is the wrong type for {}.", self.id) }
    }

    /// Cast this state as a Canvas widget state ref, you must be sure you have one.
    pub fn as_canvas(&self) -> &CanvasState {
        if let EzState::Canvas(ref i) = self.obj { i }
        else { panic!("CanvasState is the wrong type for {}.", self.id) }
    }

    /// Cast this state as a mutable Canvas widget state ref, you must be sure you have one.
    pub fn as_canvas_mut(&mut self) -> &mut CanvasState {
        if let EzState::Canvas(ref mut i) = self.obj { i }
        else { panic!("CanvasState is the wrong type for {}.", self.id) }
    }

    /// Cast this state as a Label widget state ref, you must be sure you have one.
    pub fn as_label(&self) -> &LabelState {
        if let EzState::Label(ref i) = self.obj { i }
        else { panic!("LabelState is the wrong type for {}.", self.id) }
    }

    /// Cast this state as a mutable Label widget state ref, you must be sure you have one.
    pub fn as_label_mut(&mut self) -> &mut LabelState {
        if let EzState::Label(ref mut i) = self.obj { i }
        else { panic!("LabelState is the wrong type for {}.", self.id) }
    }

    /// Cast this state as a Button widget state ref, you must be sure you have one.
    pub fn as_button(&self) -> &ButtonState {
        if let EzState::Button(ref i) = self.obj { i }
        else { panic!("ButtonState is the wrong type for {}.", self.id) }
    }

    /// Cast this state as a mutable Button widget state ref, you must be sure you have one.
    pub fn as_button_mut(&mut self) -> &mut ButtonState {
        if let EzState::Button(ref mut i) = self.obj { i }
        else { panic!("ButtonState is the wrong type for {}.", self.id) }
    }

    /// Cast this state as a Slider widget state ref, you must be sure you have one.
    pub fn as_slider(&self) -> &SliderState {
        if let EzState::Slider(ref i) = self.obj { i }
        else { panic!("SliderState is the wrong type for {}.", self.id) }
    }

    /// Cast this state as a mutable Slider widget state ref, you must be sure you have one.
    pub fn as_slider_mut(&mut self) -> &mut SliderState {
        if let EzState::Slider(ref mut i) = self.obj { i }
        else { panic!("SliderState is the wrong type for {}.", self.id) }
    }

    /// Cast this state as a Progress bar widget state ref, you must be sure you have one.
    pub fn as_progress_bar(&self) -> &ProgressBarState {
        if let EzState::ProgressBar(ref i) = self.obj { i }
        else { panic!("ProgressBarState is the wrong type for {}.", self.id) }
    }

    /// Cast this state as a mutable Progress bar widget state ref, you must be sure you have one.
    pub fn as_progress_bar_mut(&mut self) -> &mut ProgressBarState {
        if let EzState::ProgressBar(ref mut i) = self.obj { i }
        else { panic!("ProgressBarState is the wrong type for {}.", self.id) }
    }

    /// Cast this state as a Checkbox widget state ref, you must be sure you have one.
    pub fn as_checkbox(&self) -> &CheckboxState {
        if let EzState::Checkbox(ref i) = self.obj { i }
        else { panic!("CheckboxState is the wrong type for {}.", self.id) }
    }

    /// Cast this state as a mutable Checkbox widget state ref, you must be sure you have one.
    pub fn as_checkbox_mut(&mut self) -> &mut CheckboxState {
        if let EzState::Checkbox(ref mut i) = self.obj { i }
        else { panic!("CheckboxState is the wrong type for {}.", self.id) }
    }

    /// Cast this state as a Dropdown widget state ref, you must be sure you have one.
    pub fn as_dropdown(&self) -> &DropdownState {
        if let EzState::Dropdown(ref i) = self.obj { i }
        else { panic!("DropdownState is the wrong type for {}.", self.id) }
    }

    /// Cast this state as a mutable Dropdown widget state ref, you must be sure you have one.
    pub fn as_dropdown_mut(&mut self) -> &mut DropdownState {
        if let EzState::Dropdown(ref mut i) = self.obj { i }
        else { panic!("DropdownState is the wrong type for {}.", self.id) }
    }

    /// Cast this state as a dropped down menu modal state ref, you must be sure you have one.
    pub fn as_dropped_down_menu(&self) -> &DroppedDownMenuState {
        if let EzState::DroppedDownMenu(ref i) = self.obj { i }
        else { panic!("DroppedDownMenuState is the wrong type for {}.", self.id) }
    }

    /// Cast this state as a mutable dropped down menu modal state ref, you must be sure you have one.
    pub fn as_dropped_down_menu_mut(&mut self) -> &mut DroppedDownMenuState {
        if let EzState::DroppedDownMenu(ref mut i) = self.obj { i }
        else { panic!("DroppedDownMenuState is the wrong type for {}.", self.id) }
    }

    /// Cast this state as a RadioButton widget state ref, you must be sure you have one.
    pub fn as_radio_button(&self) -> &RadioButtonState {
        if let EzState::RadioButton(ref i) = self.obj { i }
        else { panic!("RadioButtonState is the wrong type for {}.", self.id) }
    }

    /// Cast this state as a mutable RadioButton widget state ref, you must be sure you have one.
    pub fn as_radio_button_mut(&mut self) -> &mut RadioButtonState {
        if let EzState::RadioButton(ref mut i) = self.obj { i }
        else { panic!("RadioButtonState is the wrong type for {}.", self.id) }
    }

    /// Cast this state as a TextInput widget state ref, you must be sure you have one.
    pub fn as_text_input(&self) -> &TextInputState {
        if let EzState::TextInput(ref i) = self.obj { i }
        else { panic!("TextInputState is the wrong type for {}.", self.id) }
    }

    /// Cast this state as a mutable TextInput widget state ref, you must be sure you have one.
    pub fn as_text_input_mut(&mut self) -> &mut TextInputState {
        if let EzState::TextInput(ref mut i) = self.obj { i }
        else { panic!("TextInputState is the wrong type for {}.", self.id) }
    }
}


/// Wrapper around a grid of StyledContent representing the entire screen currently being displayed.
/// After each frame an updated ViewTree is diffed to the old one, and only changed parts of the
/// screen are updated.
#[derive(Clone, Default, Debug)]
pub struct ViewTree {
    screen: Vec<Vec<StyledContent<String>>>,
    changed: Vec<Coordinates>
}
impl ViewTree {

    /// Get a Coordinate and the corresponding content for each screen position that has changred
    /// since the last frame.
    pub fn get_changed(&self) -> Vec<(&Coordinates, &StyledContent<String>)>{
        let mut results = Vec::new();
        for coord in self.changed.iter() {
            results.push((coord, &self.screen[coord.x][coord.y]));
        }
        results
    }

    /// Clear the cache of changed positions.
    pub fn clear_changed(&mut self) {
        self.changed.clear();
    }

    /// Write content to a [ViewTree]. Only writes differences. By writing to a view tree first
    /// and then only writing the [ViewTree] to screen at the end of a frame cycle, we avoid 
    /// unnecessary expensive screen writing operations.
    pub fn write_content(&mut self, base_position: Coordinates, content: PixelMap) {
        for x in 0..content.len() {
            for y in 0..content[x].len() {
                let write_pos =
                    Coordinates::new(base_position.x + x, base_position.y + y);
                if write_pos.x < self.width() && write_pos.y < self.height(write_pos.x) {
                    self.write_pixel(write_pos,content[x][y].get_pixel());
                }
            }
        }
    }

    /// Write a pixel to the ViewTree. To write [PixelMap]s use [write_content].
    pub fn write_pixel(&mut self, position: Coordinates, content: StyledContent<String>) {
        if self.screen[position.x][position.y] != content {
            self.screen[position.x][position.y] = content;
            self.changed.push(position);
        }
    }

    /// Get the current width of the view tree.
    pub fn width(&self) -> usize { return self.screen.len() }

    /// Get the height of the highest row in the view treee.
    pub fn height(&self, width: usize) -> usize { return self.screen[width].len() }

    /// Initialize the view tree with empty pixel based on a passed with and height.
    pub fn initialize(&mut self, width: usize, height: usize) {

        self.screen.clear();
        for x in 0..width {
            self.screen.push(Vec::new());
            for _ in 0..height {
                self.screen[x].push(Pixel::default().get_pixel())
            }
        }
    }
}


/// Get the State for each child [EzWidget] and return it in a <[path], [State]> HashMap.
pub fn initialize_state_tree(root_layout: &Layout) -> StateTree {

    let mut state_tree = StateTree::new("root".to_string(),
                                        root_layout.get_state());
    for child in root_layout.get_widgets_recursive() {
        state_tree.add_node(child.as_ez_object().get_path(),
                            child.as_ez_object().get_state());
    }
    state_tree
}


/// Get the State for each child [EzWidget] and return it in a <[path], [State]> HashMap.
pub fn initialize_callback_tree(root_layout: &Layout) -> CallbackTree {

    let mut callback_tree = CallbackTree::new("root".to_string(),
                                                               CallbackConfig::default());
    for child in root_layout.get_widgets_recursive() {
        callback_tree.add_node(child.as_ez_object().get_path(),
                               CallbackConfig::default());
    }
    callback_tree
}


/// Clean up orphaned states and callback configs in their respective trees. E.g. for when a
/// modal closes.
pub fn clean_trees(root_widget: &mut Layout, state_tree: &mut StateTree,
                   callback_tree: &mut CallbackTree, scheduler: &mut SchedulerFrontend) {

    let state_paths: Vec<String> = state_tree.get_all().iter()
        .map(|x| x.as_generic().get_path().clone()).collect();
    for path in state_paths {
        if path != "/root" && root_widget.get_child_by_path(&path).is_none() {
            state_tree.get(&path).as_generic().clean_up_properties(scheduler);
            state_tree.remove_node(path.clone());
            if !scheduler.backend.properties.contains_key(&path) {
                callback_tree.remove_node(path);
            }
        }
    }
}
