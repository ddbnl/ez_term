use std::collections::HashMap;
use crossterm::style::StyledContent;
use crate::CallbackConfig;
use crate::run::definitions::{CallbackTree, Coordinates, Pixel, PixelMap, StateTree};
use crate::scheduler::scheduler::Scheduler;
use crate::widgets::ez_object::EzObject;
use crate::widgets::layout::layout::Layout;

/// Convenience wrapper for [StateTree], [WidgetTree] and [CallbackTree]. Allows getting objects
/// by ID of a widget instead of full path.
#[derive(Default)]
pub struct Tree<T> {

    /// Name of the tree, used in panic messages to make errors more clear
    pub name: String,

    /// HashMap of objects to provide caching and ID lookup for
    pub objects: HashMap<String, T>,

    /// Cache that translates widget IDs to paths
    cache: HashMap<String, String>,
}
impl<T> Tree<T> {

    pub fn new(name: String) -> Self {
        Tree { name, objects: HashMap::new(), cache: HashMap::new() }
    }

    pub fn insert(&mut self, k: String, v: T) {
        if k.contains('/') {
            self.cache.insert(k.rsplit_once('/').unwrap().1.to_string(), k.clone());
        } else {
            self.cache.insert(k.clone(), k.clone());
        }
        self.objects.insert(k, v);
    }

    pub fn extend(&mut self, other: Tree<T>) {
        for (k, v) in other.objects.into_iter() {
            self.insert(k, v);
        }
    }

    pub fn remove(&mut self, k: &str) {
        self.cache.remove(k.rsplit_once('/').unwrap().1);
        self.objects.remove(k);
    }

    pub fn get_by_path(&self, path: &str) -> &T {
        self.objects.get(path).unwrap_or_else(|| panic!("Object {} not in {}", path, self.name))
    }

    pub fn get_by_path_mut(&mut self, path: &str) -> &mut T {
        self.objects.get_mut(path)
            .unwrap_or_else(|| panic!("Object {} not in {}", path, self.name))
    }

    pub fn get_by_id(&self, id: &str) -> &T {

        if let Some(path) = self.cache.get(id) {
            self.get_by_path(path)
        } else {
            panic!("Tried a lookup by ID in {}, but the ID \"{}\" is not unique. \
            Make the ID unique or use \"get_by_path\" instead", self.name, id)
        }
    }

    pub fn get_by_id_mut(&mut self, id: &str) -> &mut T {

        let full_path;
        if let Some(path) = self.cache.get(id) {
            full_path = path.clone();
        } else {
            panic!("Tried a lookup by ID in {}, but the ID \"{}\" is not unique. \
            Make the ID unique or use \"get_by_path\" instead", self.name, id)
        }
        self.get_by_path_mut(&full_path)
    }
}



/// ## View tree:
/// Grid of StyledContent representing the entire screen currently being displayed. After each frame
/// an updated ViewTree is diffed to the old one, and only changed parts of the screen are updated.
#[derive(Clone, Default, Debug)]
pub struct ViewTree {
    screen: Vec<Vec<StyledContent<String>>>,
    changed: Vec<Coordinates>
}
impl ViewTree {

    pub fn get_changed(&self) -> Vec<(&Coordinates, &StyledContent<String>)>{
        let mut results = Vec::new();
        for coord in self.changed.iter() {
            results.push((coord, &self.screen[coord.x][coord.y]));
        }
        results
    }

    pub fn clear_changed(&mut self) {
        self.changed.clear();
    }

    /// Write content to a [ViewTree]. Only writes differences. By writing to a view tree first and then
    /// only writing the [ViewTree] to screen at the end of a frame cycle, we avoid unnecessary
    /// expensive screen writing operations.
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

    pub fn write_pixel(&mut self, position: Coordinates, content: StyledContent<String>) {
        if self.screen[position.x][position.y] != content {
            self.screen[position.x][position.y] = content;
            self.changed.push(position);
        }
    }

    pub fn width(&self) -> usize {
        return self.screen.len()
    }

    pub fn height(&self, width: usize) -> usize {
        return self.screen[width].len()
    }

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

    let mut state_tree = StateTree::new("state_tree".to_string());
    for (child_path, child) in root_layout.get_widgets_recursive() {
        state_tree.insert(child_path, child.as_ez_object().get_state());
    }
    state_tree.insert(root_layout.get_full_path(), root_layout.get_state());
    state_tree
}


/// Get the State for each child [EzWidget] and return it in a <[path], [State]> HashMap.
pub fn initialize_callback_tree(root_layout: &Layout) -> CallbackTree {

    let mut callback_tree = CallbackTree::new("callback_tree".to_string());
    for (child_path, _child) in root_layout.get_widgets_recursive() {
        callback_tree.insert(child_path, CallbackConfig::default());
    }
    callback_tree.insert(root_layout.get_full_path(),
                         CallbackConfig::default());
    callback_tree
}


/// Clean up orphaned states and callback configs in their respective trees. E.g. for when a
/// modal closes.
pub fn clean_trees(root_widget: &mut Layout, state_tree: &mut StateTree,
                   callback_tree: &mut CallbackTree, scheduler: &mut Scheduler) {

    let widget_tree = root_widget.get_widget_tree();
    let state_paths: Vec<String> = state_tree.objects.keys().into_iter().cloned().collect();
    for path in state_paths {
        if path != "/root" && !widget_tree.objects.contains_key(&path) {
            state_tree.get_by_path(&path).as_generic().clean_up_properties(scheduler);
            state_tree.remove(&path);
        }
    }
    let callback_paths: Vec<String> = callback_tree.objects.keys().into_iter().cloned().collect();
    for path in callback_paths {
        if path != "/root" && !widget_tree.objects.contains_key(&path)
            && !scheduler.properties.contains_key(&path) {
            callback_tree.remove(&path);
        }
    }
}
