//! # Common:
//! A module containing common static functions used by other modules, as well as common types.
use crate::scheduler::Scheduler;
use crate::{common, states};
use crate::states::state::{EzState, GenericState};
use crate::widgets::widget::{EzObject};



/// Return the widget that is currently selected. Can be none.
pub fn get_selected_widget<'a>(widget_tree: &'a common::definitions::WidgetTree,
                               state_tree: &mut common::definitions::StateTree)
    -> Option<&'a dyn EzObject> {

    let modals = state_tree.get("/root").unwrap().as_layout().get_modals();
    let path_prefix = if modals.is_empty() {
        "/root".to_string()
    } else {
        modals.first().unwrap().as_ez_object().get_full_path()
    };

    for widget in widget_tree.values() {
        let generic_widget = widget.as_ez_object();
        if !generic_widget.get_full_path().starts_with(&path_prefix) { continue }
        let state = state_tree.get(&generic_widget.get_full_path())
            .unwrap().as_generic();
        if state.is_selectable() && state.get_selected() {
            return Some(generic_widget)
        }
    }
    None
}


/// If any widget is currently selected, deselect it. Can always be called safely.
pub fn deselect_selected_widget(view_tree: &mut common::definitions::ViewTree,
                                state_tree: &mut common::definitions::StateTree,
                                widget_tree: &common::definitions::WidgetTree,
                                callback_tree: &mut common::definitions::CallbackTree,
                                scheduler: &mut Scheduler) {


    let selected_widget = get_selected_widget(widget_tree, state_tree);
    if let Some(i) = selected_widget {
        state_tree.get_mut(&i.get_full_path()).unwrap().as_generic_mut().set_selected(false);
        i.on_deselect(view_tree, state_tree, widget_tree, callback_tree, scheduler);
    }
}


/// Select the next widget by selection order as defined in each selectable widget. If the last
/// widget is currently selected wrap around and select the first. This function can always be
/// called safely.
pub fn select_next(view_tree: &mut common::definitions::ViewTree,
                   state_tree: &mut common::definitions::StateTree,
                   widget_tree: &common::definitions::WidgetTree,
                   callback_tree: &mut common::definitions::CallbackTree,
                   scheduler: &mut Scheduler) {

    let modals = state_tree.get("/root").unwrap().as_layout().get_modals();
    let path_prefix = if modals.is_empty() {
        "/root".to_string()
    } else {
        modals.first().unwrap().as_ez_object().get_full_path()
    };

    let current_selection = get_selected_widget(widget_tree, state_tree);
    let mut current_order = if let Some(i) = current_selection {
        state_tree.get_mut(&i.get_full_path()).unwrap().as_generic_mut().set_selected(false);
        i.on_deselect(view_tree, state_tree, widget_tree, callback_tree, scheduler);
        state_tree.get_mut(&i.get_full_path()).unwrap().as_generic_mut().get_selection_order()
    } else {
        0
    };
    let result = find_next_selection(
        current_order, state_tree, &path_prefix);
    if let Some( next_widget) = result {
        state_tree.get_mut(&next_widget).unwrap().as_generic_mut().set_selected(true);
        widget_tree.get(&next_widget).unwrap().as_ez_object().on_select(
            view_tree,state_tree, widget_tree, callback_tree,scheduler, None);
    } else  {
        current_order = 0;
        let result = find_next_selection(
            current_order, state_tree, &path_prefix);
        if let Some( next_widget) = result {
            state_tree.get_mut(&next_widget).unwrap().as_generic_mut().set_selected(true);
            widget_tree.get(&next_widget).unwrap().as_ez_object().on_select(
                view_tree,state_tree, widget_tree, callback_tree,scheduler, None);
        }
    }
}


/// Given a current selection order number, find the next widget, or
/// wrap back around to the first if none. Returns the full path of the next widget to be selected.
pub fn find_next_selection(current_selection: usize, state_tree: &common::definitions::StateTree,
                           path_prefix: &str) -> Option<String> {


    let mut next_order: Option<usize> = None;
    let mut next_widget: Option<String> = None;
    for (path, state) in state_tree  {
        if !path.starts_with(path_prefix) { continue };
        let generic_state = state.as_generic();
        if generic_state.is_selectable() {
            if let Some(i) = next_order {
                if generic_state.get_selection_order() > 0 &&
                    generic_state.get_selection_order() > current_selection &&
                    generic_state.get_selection_order() < i &&
                    is_in_view(path.to_string(), state_tree) {
                    next_order = Some(generic_state.get_selection_order());
                    next_widget = Some(path.to_string());
                }
            } else if generic_state.get_selection_order() > 0 &&
                generic_state.get_selection_order() > current_selection &&
                is_in_view(path.to_string(), state_tree) {
                next_order = Some(generic_state.get_selection_order());
                next_widget = Some(path.to_string());
            }
        }
    }
    next_widget
}


/// Select the previous widget by selection order as defined in each selectable widget. If the first
/// widget is currently selected wrap around and select the last. This function can always be
/// called safely.
pub fn select_previous(view_tree: &mut common::definitions::ViewTree, state_tree: &mut common::definitions::StateTree,
                       widget_tree: &common::definitions::WidgetTree, callback_tree: &mut common::definitions::CallbackTree,
                       scheduler: &mut Scheduler) {

    let modals = state_tree.get("/root").unwrap().as_layout().get_modals();
    let path_prefix = if modals.is_empty() {
        "/root".to_string()
    } else {
        modals.first().unwrap().as_ez_object().get_full_path()
    };

    let current_selection = get_selected_widget(widget_tree, state_tree);
    let mut current_order = if let Some(i) = current_selection {
        state_tree.get_mut(&i.get_full_path()).unwrap().as_generic_mut().set_selected(false);
        i.on_deselect(view_tree, state_tree, widget_tree, callback_tree, scheduler);
        state_tree.get_mut(&i.get_full_path()).unwrap().as_generic().get_selection_order()
    } else {
        0
    };
    let result = find_previous_selection(
        current_order, state_tree, &path_prefix);
    if let Some( previous_widget) = result {
        state_tree.get_mut(&previous_widget).unwrap().as_generic_mut().set_selected(true);
        widget_tree.get(&previous_widget).unwrap().as_ez_object().on_select(
            view_tree,state_tree, widget_tree, callback_tree,scheduler, None);
    } else {
        current_order = 99999999;
        let result = find_previous_selection(
            current_order, state_tree, &path_prefix);
        if let Some( previous_widget) = result {
            state_tree.get_mut(&previous_widget).unwrap().as_generic_mut().set_selected(true);
            widget_tree.get(&previous_widget).unwrap().as_ez_object().on_select(
                view_tree,state_tree, widget_tree, callback_tree,scheduler, None);
        }
    }
}


/// Given a current selection order number, find the previous widget, or
/// wrap back around to the last if none. Returns the full path of the previous widget.
pub fn find_previous_selection(current_selection: usize,
                               state_tree: &common::definitions::StateTree, path_prefix: &str)
    -> Option<String> {

    let mut previous_order: Option<usize> = None;
    let mut previous_widget: Option<String> = None;
    for (path, state) in state_tree  {
        if !path.starts_with(path_prefix) { continue }
        let generic_state = state.as_generic();
        if generic_state.is_selectable() {
            if let Some(i) = previous_order {
                if generic_state.get_selection_order() > 0 &&
                    generic_state.get_selection_order() < current_selection &&
                    generic_state.get_selection_order() > i &&
                    is_in_view(path.to_string(), state_tree) {
                    previous_order = Some(generic_state.get_selection_order());
                    previous_widget = Some(path.to_string());
                }
            } else if generic_state.get_selection_order() > 0 &&
                generic_state.get_selection_order() < current_selection &&
                is_in_view(path.to_string(), state_tree) {
                previous_order = Some(generic_state.get_selection_order());
                previous_widget = Some(path.to_string());
            }
        }
    }
    previous_widget
}


/// Determine whether a widget (by path) is in view. We start with the root widget and make our
/// way down to the widget in question. We check whether the absolute pos of each widget is within
/// the bounds of the window. If we encounter a scrollview along the way, we will check if each
/// subsequent object is in bounds of the scrollview instead.
pub fn is_in_view(path: String, state_tree: &common::definitions::StateTree) -> bool {

    let window_size = state_tree.get("/root").unwrap().as_generic().get_size().clone();

    // Prepare to iterate from root widget to subwidget to sub-sub-widget etc.
    let mut paths: Vec<&str> = path.split('/').collect();
    paths = paths[1..].to_vec();
    paths.reverse();
    let mut working_path = format!("/{}", paths.pop().unwrap());

    // If we encounter a scrollview we will start using visible_width and visible_height to check
    // if further subwidgets are in view
    let mut visible_width: Option<(usize, usize)> = None;
    let mut visible_height: Option<(usize, usize)> = None;

    loop { // Loop from root widget to subwidget until we complete the full path or something is not in view

        if working_path == "/modal" {
            working_path = format!("{}/{}", working_path, paths.pop().unwrap());
            continue
        }
        let state = state_tree.get(&working_path).unwrap();
        // Determine if this part of the tree is in view. It's not in view if a visible area
        // was determined and this is not in it (visible area means we're scrolling somewhere),
        // or if absolute positions falls outside of window size.

        // If there's a visible width we're scrolling horizontally. Check if obj is in scrollview
        if let Some((visible_w_start, visible_w_end)) = visible_width {
            // If object lies completely left- or completely right of visible area it's out of view
            if state.as_generic().get_effective_position().x >= visible_w_end ||
                state.as_generic().get_effective_position().x +
                    state.as_generic().get_effective_size().width <= visible_w_start {
                return false
                // If object lies partly left of view, we take the part that's still in view as the new
                // visible area
            } else if state.as_generic().get_effective_position().x <= visible_w_start {
                visible_width = Some((visible_w_start -
                                          state.as_generic().get_effective_position().x,
                                      state.as_generic().get_effective_position().x +
                                          state.as_generic().get_effective_size().width));
                // If object lies partly right of view, we take the part that's still in view as the new
                // visible area
            } else if state.as_generic().get_effective_position().x +
                state.as_generic().get_effective_size().width >= visible_w_end {
                visible_width = Some((visible_w_start,
                                      visible_w_end -
                                          state.as_generic().get_effective_position().x));
                // If object lies entirely in view, we take its full width as the new visible area
            } else {
                visible_width = Some((0, state.as_generic().get_effective_size().width));
            }
        }

        // If there's a visible height we're scrolling vertically. Check if obj is in scrollview
        if let Some((visible_h_start, visible_h_end)) = visible_height {
            // If object lies completely above or completely below visible area it's out of view
            if state.as_generic().get_effective_position().y >= visible_h_end ||
                state.as_generic().get_effective_position().y +
                    state.as_generic().get_effective_size().height <= visible_h_start {
                return false
                // If object lies partly above of view, we take the part that's still in view as the new
                // visible area
            } else if state.as_generic().get_effective_position().y <= visible_h_start {
                visible_height = Some((visible_h_start -
                                           state.as_generic().get_effective_position().y,
                                       state.as_generic().get_effective_position().y +
                                           state.as_generic().get_effective_size().height));
                // If object lies partly below view, we take the part that's still in view as the new
                // visible area
            } else if state.as_generic().get_effective_position().y +
                state.as_generic().get_effective_size().height >= visible_h_end {
                visible_height = Some((visible_h_start,
                                       visible_h_end -
                                           state.as_generic().get_effective_position().y));
                // If object lies entirely in view, we take its full height as the new visible area
            } else {
                visible_height = Some((0, state.as_generic().get_effective_size().height));
            }
        }

        // If there's no visible height and width then we're not scrolling. Simply check if obj is
        // inside of the root window.
        if (visible_width == None &&
            state.as_generic().get_effective_absolute_position().x > window_size.width) ||
            (visible_height == None &&
                state.as_generic().get_effective_absolute_position().y > window_size.height) {
            return false
        }

        if !paths.is_empty() {
            // This is not the end of the path so this obj must be a layout. This means we have to
            // check if it is scrolling. If it is, we must check if each subsequent subwidget is in
            // this scrollview.
            if state.as_layout().get_scrolling_config().is_scrolling_x {
                visible_width =
                    Some((state.as_layout().get_scrolling_config().view_start_x,
                          state.as_layout().get_scrolling_config().view_start_x +
                              state.as_layout().get_effective_size().width));
            }
            if state.as_layout().get_scrolling_config().is_scrolling_y {
                visible_height =
                    Some((state.as_layout().get_scrolling_config().view_start_y,
                          state.as_layout().get_scrolling_config().view_start_y +
                              state.as_layout().get_effective_size().height));
            }
            working_path = format!("{}/{}", working_path, paths.pop().unwrap());
        } else {
            // End of the path and we did not encounter any out-of-view conditions. Obj is in view.
            return true
        }

    }
}


/// Find a widget by a screen position coordinate. Used e.g. by mouse event handlers. If a modal
/// if active only the modal is searched.
pub fn get_widget_by_position<'a>(pos: states::definitions::Coordinates,
                                  widget_tree: &'a common::definitions::WidgetTree,
                                  state_tree: &common::definitions::StateTree)
    -> Option<&'a dyn EzObject> {

    let modals = state_tree.get("/root").unwrap().as_layout().get_modals();
    let path_prefix = if modals.is_empty() {
        "/root".to_string()
    } else {
        modals.first().unwrap().as_ez_object().get_full_path()
    };
    for (widget_path, state) in state_tree {
        if !widget_path.starts_with(&path_prefix) { continue }
        if let EzState::Layout(_) = state { continue }
        if state.as_generic().collides(pos) {
            return Some(widget_tree.get(widget_path).unwrap().as_ez_object())
        }
    }
    None
}

