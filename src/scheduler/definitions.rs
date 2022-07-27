//! # Scheduler definitions
//!
//! This module implements definitions for the [Scheduler] struct.
use std::collections::HashMap;

use crossterm::event::KeyCode;

use crate::property::ez_properties::EzProperties;
use crate::property::ez_values::EzValues;
use crate::run::definitions::{Coordinates, StateTree};
use crate::scheduler::scheduler::{SchedulerFrontend};


/// A <Name, EzProperties> HashMap.
pub type EzPropertiesMap = HashMap<String, EzProperties>;


/// This is used for binding keyboard callbacks to widgets, meaning that any callback functions a
/// user makes should use this signature.
pub type KeyboardCallbackFunction = Box<dyn FnMut(EzContext, KeyCode) -> bool >;


/// This is used for binding mouse event callbacks to widgets, meaning that any callback functions
/// user makes should use this signature.
pub type MouseCallbackFunction = Box<dyn FnMut(EzContext, Coordinates) -> bool>;


/// This is used for callbacks that may or may not have been initiated by mouse. 'on_select' uses
/// this for example, because a widget may have been selected by mouse or by keyboard.
pub type OptionalMouseCallbackFunction = Box<dyn FnMut(EzContext, Option<Coordinates>) -> bool>;


/// This is used for binding mouse event callbacks to widgets, meaning that any callback functions
/// user makes should use this signature.
pub type MouseDragCallbackFunction = Box<dyn FnMut(EzContext, Option<Coordinates>, Coordinates)
    -> bool>;


/// Used for callbacks and scheduled tasks that don't require special parameter such as KeyCodes
/// or mouse positions. Used e.g. for [on_value_change] and [on_keyboard_enter].
pub type GenericEzFunction = Box<dyn FnMut(EzContext) -> bool>;


/// Scheduled task implementation. Using FnMut allows users to capture variables in their scheduled
/// funcs.
pub type GenericEzTask = Box<dyn FnMut(EzContext) -> bool>;


/// Closure that updates an Ez property. An Ez property can subscribe to another of the same type,
/// and it will automatically keep values in sync. When subscribed to a value, when that value
/// changes, an [EzPropertyUpdates] func will be called to do the actual sync.
pub type EzPropertyUpdater = Box<dyn FnMut(&mut StateTree, EzValues) -> String>;


/// Func that can be spawned as a background thread. Receives a dict of all [EzProperties].
/// Ez properties can be bound to widgets, so updating an EzProperty in a thread can update the UI.
pub type EzThread = Box<dyn FnOnce(HashMap<String, EzProperties>) + Send>;


/// This object is provided to callbacks. You can use it to gain access to the [StateTree] and the
/// [Scheduler]. 
/// # Change widget states
/// To change properties of widget, use the state tree to get the state of the widget
/// you want to change, and change the property from there, for example:
/// ```
/// let state = context.state_tree.get_by_id_mut("my_label").as_label_mut();
/// state.set_text("new text".to_string());
/// state.update(context.scheduler);
/// ```
/// The [Scheduler] has many uses, see the documentation for [Scheduler] for info. Access it like
/// this:
/// ```
/// context.scheduler
/// ```
pub struct EzContext<'a, 'b> {

    /// Path to the widget this context refers to, e.g. the widget a callback originated from
    pub widget_path: String,

    /// The current [StateTree]
    pub state_tree: &'a mut StateTree,

    /// The current [Scheduler]
    pub scheduler: &'b mut SchedulerFrontend,
}
impl<'a, 'b> EzContext<'a, 'b> {

    pub fn new(widget_path: String, state_tree: &'a mut StateTree,
               scheduler: &'b mut SchedulerFrontend)
        -> Self { EzContext { widget_path, state_tree, scheduler } }
}