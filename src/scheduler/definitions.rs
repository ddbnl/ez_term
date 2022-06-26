use std::collections::HashMap;
use crossterm::event::KeyCode;
use crate::scheduler::scheduler::Scheduler;
use crate::property::ez_properties::EzProperties;
use crate::property::ez_values::EzValues;
use crate::run::definitions::{Coordinates, StateTree, WidgetTree};
use crate::run::tree::ViewTree;


/// A <Name, EzProperties> HashMap.
pub type EzPropertiesMap = HashMap<String, EzProperties>;


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