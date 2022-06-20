//! # Common:
//! A module containing common static functions used by other modules, as well as common types.
use crossterm::style::{PrintStyledContent};
use crossterm::{QueueableCommand, cursor, ExecutableCommand};
use std::io::{stdout, Write};
use std::collections::HashMap;
use crate::common;
use crate::common::definitions::{StateTree, ViewTree, Coordinates};
use crate::widgets::layout::Layout;
use crate::states::definitions::CallbackConfig;
use crate::widgets::widget::{Pixel, EzObject};


/// Write content to a [ViewTree]. Only writes differences. By writing to a view tree first and then
/// only writing the [ViewTree] to screen at the end of a frame cycle, we avoid unnecessary
/// expensive screen writing operations.
pub fn write_to_view_tree(base_position: Coordinates,
                          content: common::definitions::PixelMap,
                          view_tree: &mut common::definitions::ViewTree) {

    for x in 0..content.len() {
        for y in 0..content[x].len() {
            let write_pos =
                Coordinates::new(base_position.x + x, base_position.y + y);
            if write_pos.x < view_tree.len() && write_pos.y < view_tree[write_pos.x].len() {
                view_tree[write_pos.x][write_pos.y] = content[x][y].get_pixel().clone();
            }
        }
    }
}



/// Write content to screen. Only writes differences between an old [ViewTree] (previous frame) and
/// a new [ViewTree] (current frame) are written.
pub fn write_to_screen(old_view_tree: &ViewTree, view_tree: &ViewTree) {

    stdout().execute(cursor::SavePosition).unwrap();
    for x in 0..view_tree.len() {
        for y in 0..view_tree[x].len() {
            if old_view_tree[x][y] != view_tree[x][y] {
                stdout().queue(cursor::MoveTo(x as u16, y as u16)).unwrap()
                    .queue(PrintStyledContent(view_tree[x][y].clone())).unwrap();
            }
        }
    }
    stdout().flush().unwrap();
    stdout().execute(cursor::RestorePosition).unwrap();
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
    root_widget.state = state_tree.get_mut("/root").unwrap().as_layout().clone();
    if !state_tree.get("/root").unwrap().as_layout().open_modals.is_empty() &&
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
                let state = state_tree.get(&widget_path).unwrap();
                if (!state.as_generic().get_size().infinite_width &&
                    !state.as_generic().get_size().infinite_height) ||
                    widget_path == "/root" {
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


/// Create an empty view tree
pub fn initialize_view_tree(width: usize, height: usize) -> ViewTree {
    let mut view_tree = ViewTree::new();
    for x in 0..width {
        view_tree.push(Vec::new());
        for _ in 0..height {
            view_tree[x].push(Pixel::default().get_pixel())
        }
    }
    view_tree
}


/// Get the State for each child [EzWidget] and return it in a <[path], [State]> HashMap.
pub fn initialize_state_tree(root_layout: &Layout) -> StateTree {

    let mut state_tree = HashMap::new();
    for (child_path, child) in root_layout.get_widgets_recursive() {
        state_tree.insert(child_path, child.as_ez_object().get_state());
    }
    state_tree.insert(root_layout.get_full_path(), root_layout.get_state());
    state_tree
}


/// Get the State for each child [EzWidget] and return it in a <[path], [State]> HashMap.
pub fn initialize_callback_tree(root_layout: &Layout) -> common::definitions::CallbackTree {

    let mut callback_tree = HashMap::new();
    for (child_path, _child) in root_layout.get_widgets_recursive() {
        callback_tree.insert(child_path, CallbackConfig::default());
    }
    callback_tree.insert(root_layout.get_full_path(),
                         CallbackConfig::default());
    callback_tree
}


/// Clean up orphaned states and callback configs in their respective trees. E.g. for when a
/// modal closes.
pub fn clean_trees(root_widget: &mut Layout, state_tree: &mut common::definitions::StateTree,
                   callback_tree: &mut common::definitions::CallbackTree) {

    let widget_tree = root_widget.get_widget_tree();
    let state_paths: Vec<String> = state_tree.keys().into_iter().cloned().collect();
    for path in state_paths {
        if path != "/root" && !widget_tree.contains_key(&path) {
            state_tree.remove(&path);
        }
    }
    let callback_paths: Vec<String> = callback_tree.keys().into_iter().cloned().collect();
    for path in callback_paths {
        if path != "/root" && !widget_tree.contains_key(&path) {
            callback_tree.remove(&path);
        }
    }
}
