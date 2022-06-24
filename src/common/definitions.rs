use std::collections::HashMap;
use crossterm::event::KeyCode;
use crossterm::style::StyledContent;
use crate::widgets::widget::{Pixel, EzObjects};
use crate::scheduler::scheduler::Scheduler;
use crate::states::definitions::CallbackConfig;
use crate::states::state::{EzState};
use crate::property::properties::EzProperties;
use crate::property::values::EzValues;
use crate::parser::widget_definition::EzWidgetDefinition;


/// # Convenience types
/// ## Pixel maps:
/// Used to represent the visual content of widgets. Pixels are a wrapper around
/// Crossterm StyledContent, so PixelMaps are essentially a grid of StyledContent to display.
pub type PixelMap = Vec<Vec<Pixel>>;


/// ## Key map
/// A crossterm KeyCode > Callback function lookup. Used for custom user keybinds
pub type KeyMap = HashMap<KeyCode, KeyboardCallbackFunction>;


/// ## Templates
/// A hashmap of 'Template Name > [EzWidgetDefinition]'. Used to instantiate widget templates
/// at runtime. E.g. when spawning popups.
pub type Templates = HashMap<String, EzWidgetDefinition>;


/// ## Keyboard callback function:
/// This is used for binding keyboard callbacks to widgets, meaning that any callback functions a
/// user makes should use this signature.
pub type KeyboardCallbackFunction = Box<dyn FnMut(EzContext, KeyCode) -> bool >;


/// ## Mouse callback function:
/// This is used for binding mouse event callbacks to widgets, meaning that any callback functions
/// user makes should use this signature.
pub type MouseCallbackFunction = Box<dyn FnMut(EzContext, Coordinates) -> bool>;


/// ## Optional mouse callback function:
/// This is used for callbacks that may or may not have been initiated by mouse. 'on_select' uses
/// this for example, because a widget may have been selected by mouse, or maybe by keyboard.
pub type OptionalMouseCallbackFunction = Box<dyn FnMut(EzContext, Option<Coordinates>) -> bool>;


/// ## Generic Ez function:
/// Used for callbacks and scheduled tasks that don't require special parameter such as KeyCodes
/// or mouse positions. Used e.g. for [on_value_change] and [on_keyboard_enter].
pub type GenericEzFunction = Box<dyn FnMut(EzContext) -> bool>;


/// ## Generic Ez task:
/// Scheduled task implementation. Using FnMut allows users to capture variables in their scheduled
/// funcs.
pub type GenericEzTask = Box<dyn FnMut(EzContext) -> bool>;


/// # Ez Property Updates
/// Close that updates an Ez property. An Ez property can subscribe to another of the same type,
/// and it will automatically keep values in sync. When subscribed to a value, when that value
/// changes, an [EzPropertyUpdates] func will be called to do the actual sync.
pub type EzPropertyUpdater = Box<dyn FnMut(&mut StateTree, EzValues) -> String>;


/// # Ez Thread
/// Func that can be spawned as a background thread. Receives a dict of all [EzProperties].
/// Ez properties can be bound to widgets, so updating an EzProperty in a thread can update the UI.
pub type EzThread = Box<dyn FnOnce(HashMap<String, EzProperties>) + Send>;


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


/// ## State tree:
/// A <WidgetPath, State> HashMap. The State contains all run-time information for a
/// widget, such as the text of a label, or whether a checkbox is currently checked. Callbacks
/// receive a mutable reference to the widget state and can change what they need. Then after each
/// frame the updated StateTree is diffed with the old one, and only changed widgets are redrawn.
pub type StateTree = Tree<EzState>;


/// ## Widget tree:
/// A read-only list of all widgets, passed to callbacks. Can be used to access static information
/// of a widget that is not in its' State. Widgets are represented by the EzWidget enum, but
/// can be cast to the generic UxObject or IsWidget trait. If you are sure of the type of widget
/// you are dealing with it can also be cast to specific widget types.
pub type CallbackTree = Tree<CallbackConfig>;


/// ## Widget tree:
/// A read-only list of all widgets, passed to callbacks. Can be used to access static information
/// of a widget that is not in its' State. Widgets are represented by the EzWidget enum, but
/// can be cast to the generic UxObject or IsWidget trait. If you are sure of the type of widget
/// you are dealing with it can also be cast to specific widget types.
pub type WidgetTree<'a> = Tree<&'a EzObjects>;


/// ## Ez Context:
/// Used for providing context to callbacks and scheduled tasks.
pub struct EzContext<'a, 'b, 'c, 'd> {

    /// Path to the widget this context refers to, e.g. the widget a callback originatec from
    pub widget_path: String,

    /// The current [ViewTree]
    pub view_tree: &'a mut ViewTree,

    /// The current [StateTree]
    pub state_tree: &'b mut StateTree,

    /// The current [WidgetTree]
    pub widget_tree: &'c WidgetTree<'c>,

    /// The current [Scheduler]
    pub scheduler: &'d mut Scheduler,
}
impl<'a, 'b , 'c, 'd> EzContext<'a, 'b , 'c, 'd> {

    pub fn new(widget_path: String, view_tree: &'a mut ViewTree, state_tree: &'b mut StateTree,
               widget_tree: &'c WidgetTree, scheduler: &'d mut Scheduler) -> Self {
        EzContext { widget_path, view_tree, state_tree, widget_tree, scheduler }
    }
}


/// Convenience wrapper around an XY tuple.
#[derive(PartialEq, Clone, Copy, Debug, Default)]
pub struct Coordinates {
    pub x: usize,
    pub y: usize,
}
impl Coordinates {
    pub fn new(x: usize, y: usize) -> Self { Coordinates{x, y} }
}


#[derive(PartialEq, Clone, Copy, Debug, Default)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}
impl Size {
    pub fn new(width: usize, height: usize) -> Self { Size{width, height} }
}