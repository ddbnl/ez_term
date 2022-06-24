//! # Common:
//! A module containing common static functions used by other modules, as well as common types.
use crossterm::style::{PrintStyledContent};
use crossterm::{QueueableCommand, cursor, ExecutableCommand};
use std::io::{stdout, Write};
use crate::common;
use crate::common::definitions::{StateTree, ViewTree, CallbackTree};
use crate::scheduler::scheduler::Scheduler;
use crate::widgets::layout::Layout;
use crate::states::definitions::CallbackConfig;
use crate::widgets::widget::{EzObject};


/// Write content to screen. Only writes differences between an old [ViewTree] (previous frame) and
/// a new [ViewTree] (current frame) are written.
pub fn write_to_screen(view_tree: &mut ViewTree) {

    stdout().execute(cursor::SavePosition).unwrap();
    for (coord, content) in view_tree.get_changed() {
        stdout().queue(cursor::MoveTo(coord.x as u16, coord.y as u16)).unwrap()
            .queue(PrintStyledContent(content.clone())).unwrap();
    }
    stdout().flush().unwrap();
    stdout().execute(cursor::RestorePosition).unwrap();
    view_tree.clear_changed();
}


/// Check each widget state tree for two things:
/// 1. If the state of the widget in the passed StateTree differs from the current widget state.
/// In this case the widget state should be updated with the new one, and the widget should be
/// redrawn.
/// 2. If the state of the widget contains a forced redraw. In this case the entire screen will
/// be redrawn, and widgets will not be redrawn individually. Their state will still be updated.
pub fn redraw_changed_widgets(view_tree: &mut ViewTree, state_tree: &mut StateTree,
                              root_widget: &mut Layout, changed_states: &mut Vec<String>,
                              mut force_redraw: bool) -> bool {

    // We update the root widgets' state only. It's a special case because it can hold new
    // modals it might need to access internally.
    root_widget.state = state_tree.get_by_path_mut("/root").as_layout().clone();
    if !state_tree.get_by_path("/root").as_layout().open_modals.is_empty() &&
        !changed_states.is_empty(){
        force_redraw = true;
    }
    if !force_redraw {
        redraw_widgets(changed_states, view_tree, state_tree, root_widget);
    }
    force_redraw
}


/// Redraw a list of widgets.
pub fn redraw_widgets(paths: &mut Vec<String>, view_tree: &mut ViewTree,
                      state_tree: &mut StateTree, root_widget: &mut Layout) {

    'outer: while !paths.is_empty() {
        let mut widget_path= paths.pop().unwrap();
        widget_path = widget_path.rsplit_once('/').unwrap().0.to_string();
        if widget_path.is_empty() || widget_path == root_widget.path {
            root_widget.redraw(view_tree, state_tree);
        } else {
            if common::widget_functions::widget_is_hidden(widget_path.clone(), state_tree) {
                continue 'outer
            }
            // If the widget has infinite height or width then somewhere upstream it is
            // scrolled; we will find the origin of the scroll and redraw that widget instead
            // to keep the view intact.
            loop {
                let state = state_tree.get_by_path(&widget_path);
                if (!state.as_generic().get_size().infinite_width &&
                    !state.as_generic().get_size().infinite_height) || widget_path == "/root" {
                    break
                } else {
                    widget_path = widget_path.rsplit_once('/').unwrap().0.to_string()
                }
            }
            root_widget.get_child_by_path_mut(&widget_path).unwrap().as_ez_object_mut()
                .redraw(view_tree, state_tree);
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
