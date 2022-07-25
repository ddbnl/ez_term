//! # Scheduler funcs
//! 
//! This module contains supporting functions for the [Scheduler] struct.
use std::thread::{JoinHandle, spawn};
use std::time::Instant;

use crate::{CallbackConfig, EzContext};
use crate::run::definitions::{CallbackTree, StateTree};
use crate::run::select::{deselect_widget, select_widget};
use crate::widgets::ez_object::EzObjects;
use crate::widgets::layout::layout::Layout;

use super::scheduler::Scheduler;


/// Check if any new thread are ready to be spawned, or if any spawned threads are ready to be 
/// joined.
pub fn update_threads(scheduler: &mut Scheduler, state_tree: &mut StateTree) {

    start_new_threads(scheduler);
    check_finished_threads(scheduler, state_tree)
}


/// Any threads that are scheduled to be started will be spawned. Threads can be scheduled by the
/// user.
pub fn start_new_threads(scheduler: &mut Scheduler) {

    while !scheduler.threads_to_start.is_empty() {
        let (thread_func, on_finish) =
            scheduler.threads_to_start.pop().unwrap();

        let properties = scheduler.properties.clone();
        let handle: JoinHandle<()> = spawn(move || thread_func(properties));
        scheduler.thread_handles.push((handle, on_finish))
    }
}


/// Check if any threads have finished running. Joined them if so and remove all references.
pub fn check_finished_threads(scheduler: &mut Scheduler, state_tree: &mut StateTree) {

    let mut finished = Vec::new();
    for (i, (handle, _)) in scheduler.thread_handles.iter_mut().enumerate() {
        if handle.is_finished() {
            finished.push(i);
        }
    }
    for i in finished {
        let (handle, on_finish) = scheduler.thread_handles.remove(i);
        if let Some(mut func) = on_finish {
            let context = EzContext::new(String::new(), state_tree, scheduler);
            func(context);
        }
        handle.join().unwrap();
    }
}


/// Check if any scheduled tasks are ready to be run, or if any RunOnce tasks were scheduled by the
/// user. 
pub fn run_tasks(scheduler: &mut Scheduler, state_tree: &mut StateTree) {

    let mut remaining_tasks = Vec::new();
    while !scheduler.tasks.is_empty() {
        let mut task = scheduler.tasks.pop().unwrap();
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
    scheduler.tasks = remaining_tasks;
}


/// Check if there are any new widgets to create.
pub fn create_new_widgets(scheduler: &mut Scheduler, root_widget: &mut Layout,
                          state_tree: &mut StateTree, callback_tree: &mut CallbackTree) {

    let mut widgets_to_create = scheduler.widgets_to_create.clone();
    scheduler.widgets_to_create.clear();
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
pub fn update_callback_configs(scheduler: &mut Scheduler, callback_tree: &mut CallbackTree) {

    while !scheduler.new_callback_configs.is_empty() {
        let (path, callback_config) =
            scheduler.new_callback_configs.pop().unwrap();
        callback_tree.insert(path, callback_config);
    }
    while !scheduler.updated_callback_configs.is_empty() {
        let (path_or_id, callback_config) =
            scheduler.updated_callback_configs.pop().unwrap();
        if path_or_id.contains('/') {
            callback_tree.get_by_path_mut(&path_or_id).update_from(callback_config);
        } else {
            callback_tree.get_by_id_mut(&path_or_id).update_from(callback_config);
        }
    }
}

/// Check all EzProperty that have at least one subscriber and check if they've send a new
/// value. If so, call the update func of all subsribers and any registered user callbacks.
pub fn update_properties(scheduler: &mut Scheduler, state_tree: &mut StateTree,
                         callback_tree: &mut CallbackTree) {

    let mut to_update = Vec::new();
    let mut to_callback = Vec::new();

    for (name, update_funcs) in
            scheduler.property_subscribers.iter_mut() {
        let receiver = scheduler.property_receivers.get(name).unwrap();
        let mut new_val = None;
        // Drain all new values if any, we only care about the latest.
        while let Ok(new) = receiver.try_recv() {
            new_val = Some(new);
        }
        if let Some(val) = new_val {
            to_callback.push(name.clone());
            for update_func in update_funcs {
                to_update.push(update_func(state_tree, val.clone()));
            }
        }
    }
    for name in to_callback {
        if callback_tree.objects.contains_key(&name) {
            let context =
                EzContext::new(name.clone(), state_tree, scheduler);
            if let Some(ref mut callback) =
                    callback_tree.get_by_path_mut(&name).on_value_change {
                callback(context);
            }
        }
    }
    scheduler.widgets_to_update.extend(to_update);
}

/// Drain channel values. Called occasionally to drain channels which have no subscribers.
pub fn drain_property_channels(scheduler: &mut Scheduler) {
    for receiver in scheduler.property_receivers.values() {
        while receiver.try_recv().is_ok() {}
    }
}

/// Clean up a property completely. Called automatically when widget states are cleaned up.
/// E.g. when a modal is removed from a layout.
pub fn clean_up_property(scheduler: &mut Scheduler, name: &str) {

    scheduler.properties.remove(name);
    scheduler.property_callbacks.remove(name);
    scheduler.property_receivers.remove(name);
    scheduler.property_subscribers.remove(name);
}


pub fn handle_next_selection(scheduler: &mut Scheduler, state_tree: &mut StateTree,
                             root_widget: &Layout, callback_tree: &mut CallbackTree,
                             mut current_selection: String) -> String {

    if scheduler.deselect && !current_selection.is_empty() {
        deselect_widget(&current_selection, state_tree, root_widget, callback_tree,
                        scheduler);
        current_selection.clear();
    }
    scheduler.deselect = false;

    if let Some((selection, mouse_pos)) = scheduler.next_selection.clone() {
        if selection != current_selection {
            if !current_selection.is_empty() {
                deselect_widget(&current_selection, state_tree, root_widget, callback_tree,
                                scheduler);
            }
            select_widget(&selection, state_tree, root_widget, callback_tree,
                          scheduler, mouse_pos);
        }
        scheduler.next_selection = None;
        selection
    } else {
        current_selection
    }
}