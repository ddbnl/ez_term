//! # Scheduler funcs
//! 
//! This module contains supporting functions for the [Scheduler] struct.
use std::collections::HashMap;
use std::thread::{JoinHandle, spawn};
use std::time::Instant;

use crate::{CallbackConfig, EzContext, EzObject, EzProperties, KeyMap, LayoutMode};
use crate::run::definitions::{CallbackTree, StateTree};
use crate::run::select::{deselect_widget, select_widget};
use crate::scheduler::scheduler::SchedulerFrontend;
use crate::widgets::ez_object::EzObjects;
use crate::widgets::layout::layout::Layout;


/// Check if any new thread are ready to be spawned, or if any spawned threads are ready to be 
/// joined.
pub fn update_threads(scheduler: &mut SchedulerFrontend, state_tree: &mut StateTree) {

    start_new_threads(scheduler, state_tree);
    check_finished_threads(scheduler, state_tree)
}


/// Any threads that are scheduled to be started will be spawned. Threads can be scheduled by the
/// user.
pub fn start_new_threads(scheduler: &mut SchedulerFrontend, state_tree: &mut StateTree) {

    while !scheduler.backend.threads_to_start.is_empty() {
        let (thread_func, on_finish) =
            scheduler.backend.threads_to_start.pop().unwrap();
        let mut custom_properties: HashMap<String, EzProperties> = HashMap::new();

        for (name, custom_property) in scheduler.backend.properties.iter()
            .filter(|(name, (property, _))| !name.contains('/'))
            .map(|(name, (property, _)) | (name, property)) {
            custom_properties.insert(name.clone(), custom_property.clone());
        }

        let state_tree = state_tree.clone();
        let handle: JoinHandle<()> = spawn(move || thread_func(custom_properties, state_tree));
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
        if task.canceled { continue }
        let context = EzContext::new(String::new(), state_tree, scheduler);

        let elapsed = task.created.elapsed();
        if elapsed >= task.delay {
            (task.func)(context);
        } else {
            remaining_tasks.push(task);
        }
    }
    scheduler.backend.tasks = remaining_tasks;

    let mut remaining_tasks = Vec::new();
    while !scheduler.backend.recurring_tasks.is_empty() {
        let mut task = scheduler.backend.recurring_tasks.pop().unwrap();
        let context = EzContext::new(String::new(), state_tree, scheduler);
        if task.canceled { continue }

        if let Some(time) = task.last_execution {
            let elapsed = time.elapsed();
            // Interval elapsed, execute task and reschedule if it returned true
            if elapsed >= task.interval {
                let result = (task.func)(context);
                task.last_execution = Some(Instant::now());
                if result {
                    remaining_tasks.push(task);
                }
            // Interval not elapsed, schedule again to check next frame
            } else {
                remaining_tasks.push(task)
            }
        // Task has not been executed before, do so immediately
        } else {
            let result = (task.func)(context);
            task.last_execution = Some(Instant::now());
            if result {
                remaining_tasks.push(task);
            }
        }
    }
    scheduler.backend.recurring_tasks = remaining_tasks;
}


/// Check if there are any new widgets to create.
pub fn create_new_widgets(scheduler: &mut SchedulerFrontend, root_widget: &mut Layout,
                          callback_tree: &mut CallbackTree) {

    let widgets_to_create = scheduler.backend.widgets_to_create.clone();
    scheduler.backend.widgets_to_create.clear();
    for new_widget in widgets_to_create {
        let widget_path = new_widget.as_ez_object().get_path();
        let (parent_path, _) = widget_path.rsplit_once('/').unwrap();

        let parent = root_widget.get_child_by_path_mut(parent_path).unwrap_or_else(
            || panic!("Could not create new widget, parent path does not exist: {}", parent_path)
        ).as_layout_mut();
        callback_tree.add_node(widget_path, CallbackConfig::default());
        if let EzObjects::Layout(ref i) = new_widget {
            for child in i.get_widgets_recursive() {
                callback_tree.add_node(child.as_ez_object().get_path(),
                                       CallbackConfig::default());
            }
        }
        parent.add_child(new_widget, scheduler);
        scheduler.force_redraw();
    }
}

pub fn remove_widgets(scheduler: &mut SchedulerFrontend, root_widget: &mut Layout,
                      state_tree: &mut StateTree, callback_tree: &mut CallbackTree) {

    while !scheduler.backend.widgets_to_remove.is_empty() {
        let name = scheduler.backend.widgets_to_remove.pop().unwrap();
        let full_path = state_tree.get(&name).as_generic().get_path().clone();

        let (parent, id) = full_path.rsplit_once('/').unwrap();
        let parent_widget = root_widget.get_child_by_path_mut(parent)
            .unwrap_or_else(|| panic!("Could not remove widget: {}. It could not be found.",
                                      full_path)).as_layout_mut();

        parent_widget.remove_child(id);
        let parent_state = state_tree.get_mut(parent).as_layout();
        if parent_state.mode.value == LayoutMode::Tab {
            let own_state = state_tree.get(&full_path).as_layout();
            let header_id = format!("{}_tab_header", own_state.get_tab_name());
            let header_path =
                parent_widget.get_child(&header_id).unwrap().as_ez_object().get_path().clone();
            parent_widget.remove_child(&header_id);
            let removed = state_tree.remove_node(header_path);
            removed.obj.as_generic().clean_up_properties(scheduler);
        }
        scheduler.update_widget(parent_widget.get_path());

        let removed_state = state_tree.remove_node(full_path.clone());
        for child in removed_state.get_all() {
            child.as_generic().clean_up_properties(scheduler);
        }
        callback_tree.remove_node(full_path.clone());
    }
}


/// Check if any callback configs were scheduled to be updated or replaced.
pub fn update_callback_configs(scheduler: &mut SchedulerFrontend, callback_tree: &mut CallbackTree,
                               global_keymap: &mut KeyMap) {

    while !scheduler.backend.new_callback_configs.is_empty() {
        let (path, callback_config) =
            scheduler.backend.new_callback_configs.remove(0);
        callback_tree.add_node(path, callback_config);
    }
    while !scheduler.backend.updated_callback_configs.is_empty() {
        let (path_or_id, callback_config) =
            scheduler.backend.updated_callback_configs.remove(0);
        callback_tree.get_mut(&path_or_id).obj.update_from(callback_config);
    }
    for (key, callback) in
            scheduler.backend.update_global_keymap.drain() {
        global_keymap.keymap.insert(key, callback);
    }
    scheduler.backend.update_global_keymap.clear();
    
    for key in scheduler.backend.remove_global_keymap.iter() {
        global_keymap.keymap.remove_entry(&key);
    }
    scheduler.backend.remove_global_keymap.clear();
    
    if scheduler.backend.clear_global_keymap {
        global_keymap.keymap.clear()
    }

}

/// Check all EzProperty that have at least one subscriber and check if they've send a new
/// value. If so, call the update func of all subscribers and any registered user callbacks.
pub fn update_properties(scheduler: &mut SchedulerFrontend,
                         state_tree: &mut StateTree, callback_tree: &mut CallbackTree) {

    let mut to_update = Vec::new();
    let mut to_callback: Vec<String> = Vec::new();

    for (name, (property, receiver))
            in scheduler.backend.properties.iter() {
        let mut new_val = None;
        // Drain all new values if any, we only care about the latest.
        while let Ok(new) = receiver.try_recv() {
            new_val = Some(new);
        }
        if let Some(val) = new_val {
            if scheduler.backend.property_subscribers.contains_key(name) {
                for update_func in scheduler.backend
                        .property_subscribers.get_mut(name).unwrap() {
                    to_update.push(update_func(state_tree, val.clone()));
                }
            }
            if scheduler.backend.property_callbacks.contains(name) {
                to_callback.push(name.clone());
            }
            if name.starts_with("/root") { // custom properties do not start with /root
                let (widget_path, property_name) =
                    name.rsplit_once('/').unwrap();
                let state =
                    state_tree.get_mut(widget_path).as_generic_mut();
                if state.update_property(property_name, val) {
                    to_update.push(state.get_path().clone());
                }
            }
        }
    }
    for name in to_callback {
        for callback in callback_tree.get_mut(&name).obj
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
        callback_tree.get_mut(&name).obj.property_callbacks.push(callback);
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