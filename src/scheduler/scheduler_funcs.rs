//! # Scheduler funcs
//! 
//! This module contains supporting functions for the [Scheduler] struct.
use std::thread::{JoinHandle, spawn};
use std::time::Instant;

use crate::{CallbackConfig, EzContext};
use crate::property::ez_values::EzValues;
use crate::run::definitions::{CallbackTree, StateTree};
use crate::run::select::{deselect_widget, select_widget};
use crate::scheduler::scheduler::SchedulerFrontend;
use crate::widgets::ez_object::EzObjects;
use crate::widgets::layout::layout::Layout;


/// Check if any new thread are ready to be spawned, or if any spawned threads are ready to be 
/// joined.
pub fn update_threads(scheduler: &mut SchedulerFrontend, state_tree: &mut StateTree) {

    start_new_threads(scheduler);
    check_finished_threads(scheduler, state_tree)
}


/// Any threads that are scheduled to be started will be spawned. Threads can be scheduled by the
/// user.
pub fn start_new_threads(scheduler: &mut SchedulerFrontend) {

    while !scheduler.backend.threads_to_start.is_empty() {
        let (thread_func, on_finish) =
            scheduler.backend.threads_to_start.pop().unwrap();

        let properties = scheduler.backend.properties.clone();
        let handle: JoinHandle<()> = spawn(move || thread_func(properties));
        scheduler.backend.thread_handles.push((handle, on_finish))
    }
}


/// Check if any threads have finished running. Joined them if so and remove all references.
pub fn check_finished_threads(scheduler: &mut SchedulerFrontend, state_tree: &mut StateTree) {

    let mut finished = Vec::new();
    for (i, (handle, _)) in scheduler.backend.thread_handles.iter_mut().enumerate() {
        if handle.is_finished() {
            finished.push(i);
        }
    }
    for i in finished {
        let (handle, on_finish) = scheduler.backend.thread_handles.remove(i);
        if let Some(mut func) = on_finish {
            let context = EzContext::new(String::new(), state_tree, scheduler);
            func(context);
        }
        handle.join().unwrap();
    }
}


/// Check if any scheduled tasks are ready to be run, or if any RunOnce tasks were scheduled by the
/// user. 
pub fn run_tasks(scheduler: &mut SchedulerFrontend, state_tree: &mut StateTree) {

    let mut remaining_tasks = Vec::new();
    while !scheduler.backend.tasks.is_empty() {
        let mut task = scheduler.backend.tasks.pop().unwrap();
        let context = EzContext::new(task.widget.clone(), state_tree, scheduler);

        if let Some(time) = task.last_execution {
            let elapsed = time.elapsed();
            if elapsed >= task.interval && !task.canceled {
                let result = (task.func)(context);
                task.last_execution = Some(Instant::now());
                if task.recurring && result {
                    remaining_tasks.push(task);
                }
            } else if !task.canceled {
                remaining_tasks.push(task);
            }
        } else if !task.canceled {
            let result = (task.func)(context);
            task.last_execution = Some(Instant::now());
            if task.recurring && result {
                remaining_tasks.push(task);
            }
        }
    }
    scheduler.backend.tasks = remaining_tasks;
}


/// Check if there are any new widgets to create.
pub fn create_new_widgets(scheduler: &mut SchedulerFrontend, root_widget: &mut Layout,
                          state_tree: &mut StateTree, callback_tree: &mut CallbackTree) {

    let widgets_to_create = scheduler.backend.widgets_to_create.clone();
    scheduler.backend.widgets_to_create.clear();
    for (path, base_type, state) in widgets_to_create {
        let (parent_path, id) = path.rsplit_once('/').unwrap();
        let parent = root_widget.get_child_by_path_mut(parent_path).unwrap_or_else(
            || panic!("Could not create new widget, parent path does not exist: {}", parent_path)
        ).as_layout_mut();
        let widget =
            EzObjects::from_string(&base_type, path.to_string(),
                                   id.to_string(), scheduler, state.clone());
        parent.add_child(widget, scheduler);
        state_tree.insert(path.clone(), state);
        callback_tree.insert(path, CallbackConfig::default());
        scheduler.force_redraw();
    }
}


/// Check if any callback configs were scheduled to be updated or replaced.
pub fn update_callback_configs(scheduler: &mut SchedulerFrontend, callback_tree: &mut CallbackTree) {

    while !scheduler.backend.new_callback_configs.is_empty() {
        let (path, callback_config) =
            scheduler.backend.new_callback_configs.pop().unwrap();
        callback_tree.insert(path, callback_config);
    }
    while !scheduler.backend.updated_callback_configs.is_empty() {
        let (path_or_id, callback_config) =
            scheduler.backend.updated_callback_configs.pop().unwrap();
        if path_or_id.contains('/') {
            callback_tree.get_by_path_mut(&path_or_id).update_from(callback_config);
        } else {
            callback_tree.get_by_id_mut(&path_or_id).update_from(callback_config);
        }
    }
}

/// Check all EzProperty that have at least one subscriber and check if they've send a new
/// value. If so, call the update func of all subscribers and any registered user callbacks.
pub fn update_properties(scheduler: &mut SchedulerFrontend, state_tree: &mut StateTree,
                         callback_tree: &mut CallbackTree) {

    let mut to_update = Vec::new();
    let mut to_callback: Vec<String> = Vec::new();

    // Collect all property names that are either subscribed to or that have attached callbacks
    let mut sb_names: Vec<String> = scheduler.backend.property_subscribers
        .keys().map(|x| x.clone()).collect();
    sb_names.extend(scheduler.backend.property_callbacks.clone());

    for name in sb_names {
        let mut new_val = None;
        let receiver =
            scheduler.backend.property_receivers.get(&name).unwrap();
        // Drain all new values if any, we only care about the latest.
        while let Ok(new) = receiver.try_recv() {
            new_val = Some(new);
        }
        if let Some(val) = new_val {
            if scheduler.backend.property_subscribers.contains_key(&name) {
                for update_func in scheduler.backend
                        .property_subscribers.get_mut(&name).unwrap() {
                    to_update.push(update_func(state_tree, val.clone()));
                }
            }
            if scheduler.backend.property_callbacks.contains(&name) {
                to_callback.push(name.clone());
            }
        }
    }
    for name in to_callback {
        for callback in callback_tree.get_by_path_mut(&name)
                .property_callbacks.iter_mut() {
            let context = EzContext::new(name.to_string(), state_tree, scheduler);
            callback(context);
        }

    }

    scheduler.backend.widgets_to_update.extend(to_update);
}

/// Take new property callbacks and add them to the existing callback config in the callback tree
pub fn add_property_callbacks(scheduler: &mut SchedulerFrontend, callback_tree: &mut CallbackTree) {
    for (name, callback) in scheduler.backend.new_property_callbacks.pop() {
        callback_tree.get_by_path_mut(&name).property_callbacks.push(callback);
    }

}

/// Drain channel values. Called occasionally to drain channels which have no subscribers.
pub fn drain_property_channels(scheduler: &mut SchedulerFrontend) {
    for receiver in scheduler.backend.property_receivers.values() {
        while receiver.try_recv().is_ok() {}
    }
}

/// Clean up a property completely. Called automatically when widget states are cleaned up.
/// E.g. when a modal is removed from a layout.
pub fn clean_up_property(scheduler: &mut SchedulerFrontend, name: &str) {

    scheduler.backend.properties.remove(name);

    let mut index = None;
    for (i, callbacks) in scheduler.backend.property_callbacks.iter().enumerate() {
        if callbacks == name {
            index = Some(i);
        }
    }
    if let Some(i) = index {
        scheduler.backend.property_callbacks.remove(i);
    }

    scheduler.backend.property_receivers.remove(name);
    scheduler.backend.property_subscribers.remove(name);
}


pub fn handle_next_selection(scheduler: &mut SchedulerFrontend, state_tree: &mut StateTree,
                             root_widget: &Layout, callback_tree: &mut CallbackTree,
                             mut current_selection: String) -> String {

    if scheduler.backend.deselect && !current_selection.is_empty() {
        deselect_widget(&current_selection, state_tree, root_widget, callback_tree,
                        scheduler);
        current_selection.clear();
    }
    scheduler.backend.deselect = false;

    if let Some((selection, mouse_pos)) = scheduler.backend.next_selection.clone() {
        if selection != current_selection {
            if !current_selection.is_empty() {
                deselect_widget(&current_selection, state_tree, root_widget, callback_tree,
                                scheduler);
            }
            select_widget(&selection, state_tree, root_widget, callback_tree,
                          scheduler, mouse_pos);
        }
        scheduler.backend.next_selection = None;
        selection
    } else {
        current_selection
    }
}