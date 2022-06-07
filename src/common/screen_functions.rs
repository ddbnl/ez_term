//! # Common:
//! A module containing common static functions used by other modules, as well as common types.
use crossterm::style::{PrintStyledContent};
use crossterm::{QueueableCommand, cursor, ExecutableCommand};
use std::io::{stdout, Write};
use std::collections::HashMap;
use crate::common;
use crate::widgets::layout::Layout;
use crate::states;
use crate::widgets::widget::{EzObjects, Pixel, EzObject};


/// Write content to a [ViewTree]. Only writes differences. By writing to a view tree first and then
/// only writing the [ViewTree] to screen at the end of a frame cycle, we avoid unnecessary
/// expensive screen writing operations.
pub fn write_to_view_tree(base_position: states::definitions::Coordinates,
                          content: common::definitions::PixelMap,
                          view_tree: &mut common::definitions::ViewTree) {

    for x in 0..content.len() {
        for y in 0..content[x].len() {
            let write_pos =
                states::definitions::Coordinates::new(base_position.x + x, base_position.y + y);
            if write_pos.x < view_tree.len() && write_pos.y < view_tree[write_pos.x].len() {
                view_tree[write_pos.x][write_pos.y] = content[x][y].get_pixel().clone();
            }
        }
    }
}



/// Write content to screen. Only writes differences between an old [ViewTree] (previous frame) and
/// a new [ViewTree] (current frame) are written.
pub fn write_to_screen(old_view_tree: &common::definitions::ViewTree,
                       view_tree: &common::definitions::ViewTree) {

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
pub fn redraw_changed_widgets(view_tree: &mut common::definitions::ViewTree, state_tree: &mut common::definitions::StateTree,
                              root_widget: &mut Layout) -> bool {

    // We update the root widgets' state only. It's a special case because it can hold new
    // modals it might need to access internally.
    root_widget.state = state_tree.get_mut("/root").unwrap().as_layout().clone();
    let (force_redraw, widgets_to_redraw, modals_to_redraw)
        = get_widgets_to_redraw(state_tree);
    if !force_redraw {
        redraw_widgets(widgets_to_redraw, view_tree, state_tree, root_widget);
        redraw_modals(modals_to_redraw, view_tree, state_tree, root_widget);
    }
    force_redraw
}


/// Redraw a list of widgets.
pub fn redraw_widgets(paths: Vec<String>, view_tree: &mut common::definitions::ViewTree, state_tree: &mut common::definitions::StateTree,
                      root_widget: &mut Layout) {

    for mut widget_path in paths.into_iter() {
        if widget_path.is_empty() || widget_path == root_widget.path {
            root_widget.redraw(view_tree, state_tree);
        } else {
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


/// Redraw a list of modals
pub fn redraw_modals(paths: Vec<String>, view_tree: &mut common::definitions::ViewTree, state_tree: &mut common::definitions::StateTree,
                     root_widget: &mut Layout) {

    for modal_path in paths.iter() {
        let modal = root_widget.state.open_modals.first_mut().unwrap();
        if let EzObjects::Layout(ref mut i) = modal {
            if modal_path == &i.get_full_path() {
                i.redraw(view_tree, state_tree);
            } else {
                for child in i.get_widgets_recursive().values() {
                    if modal_path == &child.as_ez_object().get_full_path() {
                        child.as_ez_object().redraw(view_tree, state_tree);
                    }
                }
            }
        } else if modal_path == &root_widget.state.open_modals.first_mut().unwrap()
            .as_ez_object().get_full_path() {
            root_widget.state.open_modals.first_mut().unwrap().as_ez_object()
                .redraw(view_tree, state_tree);
        }
    }
}


/// Based on a state tree, determine which widgets and/or modals need redrawing, and whether
/// a global redraw is necessary. We separate widgets and modals to redraw because modals need to
/// be drawn last so they stay on top.
/// Returns: (Global redraw, widgets_to_redraw, modals_to_redraw)
pub fn get_widgets_to_redraw(state_tree: &mut common::definitions::StateTree) -> (bool, Vec<String>, Vec<String>) {

    let mut force_redraw = false;
    let mut widgets_to_redraw = Vec::new();
    let mut modals_to_redraw = Vec::new();

    // We get the box of the open modal if there's any. If any widgets to redraw overlap this box,
    // we redraw the modal as well, so it stays on top.
    let mut modal_box = None;
    if !state_tree.get("/root").unwrap().as_layout().open_modals.is_empty() {
        let modal_name = state_tree.get("/root").unwrap().as_layout()
            .open_modals.first().unwrap().as_ez_object().get_full_path().clone();
        modal_box = Some((modal_name.clone(),
                          state_tree.get(&modal_name).unwrap().as_generic().get_box()));
    }

    for (widget_path, state) in state_tree.iter_mut() {
        let generic_state = state.as_generic_mut();
        if generic_state.get_force_redraw() {
            force_redraw = true;
            generic_state.set_force_redraw(false);
        }
        if generic_state.get_changed() {
            generic_state.set_changed(false);
            if !widget_path.starts_with("/modal") {
                let parent_path = widget_path.rsplit_once('/').unwrap().0;
                widgets_to_redraw.push(parent_path.to_string());
                if let Some((ref modal_name,
                                modal_box_coords)) = modal_box {
                    if generic_state.overlaps(modal_box_coords) {
                        modals_to_redraw.push(modal_name.clone());
                        modal_box = None;
                    }
                }
            } else{
                modals_to_redraw.push(widget_path.clone());
            }
        }
    }
    (force_redraw, widgets_to_redraw, modals_to_redraw)
}


/// Create an empty view tree
pub fn initialize_view_tree(width: usize, height: usize) -> common::definitions::ViewTree {
    let mut view_tree = common::definitions::ViewTree::new();
    for x in 0..width {
        view_tree.push(Vec::new());
        for _ in 0..height {
            view_tree[x].push(Pixel::default().get_pixel())
        }
    }
    view_tree
}


/// Get the State for each child [EzWidget] and return it in a <[path], [State]> HashMap.
pub fn initialize_state_tree(root_layout: &Layout) -> common::definitions::StateTree {

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
        callback_tree.insert(child_path, states::definitions::CallbackConfig::default());
    }
    callback_tree.insert(root_layout.get_full_path(),
                         states::definitions::CallbackConfig::default());
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
