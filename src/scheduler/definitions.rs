//! # Scheduler definitions
//!
//! This module implements definitions for the [Scheduler] struct.
use std::collections::HashMap;

use crossterm::event::{KeyCode, KeyModifiers};

use crate::property::ez_properties::EzProperties;
use crate::property::ez_values::EzValues;
use crate::run::definitions::{Coordinates, IsizeCoordinates, StateTree};
use crate::scheduler::scheduler::SchedulerFrontend;

/// A <Name, EzProperties> HashMap.
pub type EzPropertiesMap = HashMap<String, EzProperties>;

/// This is used for binding keyboard callbacks to widgets, meaning that any callback functions a
/// user makes should use this signature.
pub type KeyboardCallbackFunction = Box<dyn FnMut(Context, KeyCode, KeyModifiers) -> bool + Send>;

/// This is used for binding mouse event callbacks to widgets, meaning that any callback functions
/// user makes should use this signature.
pub type MouseCallbackFunction = Box<dyn FnMut(Context, Coordinates) -> bool + Send>;

/// This is used for callbacks that may or may not have been initiated by mouse. 'on_select' uses
/// this for example, because a widget may have been selected by mouse or by keyboard.
pub type OptionalMouseCallbackFunction =
    Box<dyn FnMut(Context, Option<Coordinates>) -> bool + Send>;

/// This is used for binding mouse event callbacks to widgets, meaning that any callback functions
/// user makes should use this signature.
pub type MouseDragCallbackFunction =
    Box<dyn FnMut(Context, Option<IsizeCoordinates>, IsizeCoordinates) -> bool + Send>;

/// Used for callbacks and scheduled tasks that don't require special parameter such as KeyCodes
/// or mouse positions. Used e.g. for [on_value_change] and [on_keyboard_enter].
pub type GenericFunction = Box<dyn FnMut(Context) -> bool + Send>;

/// Scheduled task implementation. Using FnMut allows users to capture variables in their scheduled
/// funcs.
pub type GenericTask = Box<dyn FnMut(Context) + Send>;

/// Scheduled task implementation. Using FnMut allows users to capture variables in their scheduled
/// funcs. Outputs a bool to indicate whether task should occur again.
pub type GenericRecurringTask = Box<dyn FnMut(Context) -> bool + Send>;

/// Closure that updates an Ez property. An Ez property can subscribe to another of the same type,
/// and it will automatically keep values in sync. When subscribed to a value, when that value
/// changes, an [EzPropertyUpdates] func will be called to do the actual sync.
pub type EzPropertyUpdater = Box<dyn FnMut(&mut StateTree, EzValues) + Send>;

/// Func that can be spawned as a background thread. Receives a dict of all [EzProperties].
/// Ez properties can be bound to widgets, so updating an EzProperty in a thread can update the UI.
pub type EzThread = Box<dyn FnOnce(ThreadedContext) + Send>;

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
pub struct Context<'a, 'b> {
    /// Path to the widget this context refers to, e.g. the widget a callback originated from
    pub widget_path: String,

    /// The current [StateTree]
    pub state_tree: &'a mut StateTree,

    /// The current [Scheduler]
    pub scheduler: &'b mut SchedulerFrontend,
}
impl<'a, 'b> Context<'a, 'b> {
    pub fn new(
        widget_path: String,
        state_tree: &'a mut StateTree,
        scheduler: &'b mut SchedulerFrontend,
    ) -> Self {
        Context {
            widget_path,
            state_tree,
            scheduler,
        }
    }
}

/// This object is provided to threaded functions callbacks. You can use it to gain access to
/// the [StateTree] and the [Scheduler].
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
pub struct ThreadedContext {
    /// Path to the widget this context refers to, e.g. the widget a callback originated from
    pub widget_path: String,

    /// The current [StateTree]
    pub state_tree: StateTree,

    /// The current [Scheduler]
    pub scheduler: SchedulerFrontend,
}
impl ThreadedContext {
    pub fn new(widget_path: String, state_tree: StateTree, scheduler: SchedulerFrontend) -> Self {
        ThreadedContext {
            widget_path,
            state_tree,
            scheduler,
        }
    }
}
