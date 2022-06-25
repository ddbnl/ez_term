//! # Common:
//! A module containing common static functions used by other modules, as well as common types.
use crate::scheduler::scheduler::Scheduler;
use crate::run::definitions::{CallbackTree, Coordinates, StateTree, WidgetTree};
use crate::run::tree::ViewTree;
use crate::states::definitions::LayoutMode;
use crate::states::ez_state::{GenericState, EzState};
use crate::widgets::ez_object::{EzObject};



/// Return the widget that is currently selected. Can be none.
pub fn get_selected_widget<'a>(widget_tree: &'a WidgetTree, state_tree: &mut StateTree)
    -> Option<&'a dyn EzObject> {


    let modals = state_tree.get_by_path("/root").as_layout().get_modals();
    let path_prefix = if modals.is_empty() {
        "/root".to_string()
    } else {
        modals.first().unwrap().as_ez_object().get_full_path()
    };

    for widget in widget_tree.objects.values() {
        let generic_widget = widget.as_ez_object();
        if !generic_widget.get_full_path().starts_with(&path_prefix) { continue }
        let state = state_tree
            .get_by_path(&generic_widget.get_full_path()).as_generic();
        if state.is_selectable() && state.get_selected() {
            return Some(generic_widget)
        }
    }
    None
}


/// If any widget is currently selected, deselect it. Can always be called safely. Returns the
/// [selection_order] of the widget that was selected, or 0 if nothing was selected.
pub fn deselect_selected_widget(view_tree: &mut ViewTree, state_tree: &mut StateTree,
                                widget_tree: &WidgetTree, callback_tree: &mut CallbackTree,
                                scheduler: &mut Scheduler) -> usize {

    let selected_widget = get_selected_widget(widget_tree, state_tree);
    if let Some(i) = selected_widget {
        let state =
            state_tree.get_by_path_mut(&i.get_full_path()).as_generic_mut();
        state.set_selected(false);
        state.update(scheduler);
        let order = state.get_selection_order().value;
        i.on_deselect(view_tree, state_tree, widget_tree, callback_tree, scheduler);
        return order
    }
    0
}


pub fn select_widget(path: String, view_tree: &mut ViewTree, state_tree: &mut StateTree,
                     widget_tree: &WidgetTree,callback_tree: &mut CallbackTree,
                     scheduler: &mut Scheduler) {

    widget_tree.get_by_path(&path).as_ez_object().on_select(
        view_tree,state_tree, widget_tree, callback_tree,scheduler, None);
    let state =
        state_tree.get_by_path_mut(&path).as_generic_mut();
    state.set_selected(true);
    state.update(scheduler);
}


/// Select the next widget by selection order as defined in each selectable widget. If the last
/// widget is currently selected wrap around and select the first. This function can always be
/// called safely.
pub fn select_next(view_tree: &mut ViewTree, state_tree: &mut StateTree, widget_tree: &WidgetTree,
                   callback_tree: &mut CallbackTree, scheduler: &mut Scheduler) {

    let modals = state_tree.get_by_path("/root").as_layout().get_modals();
    let path_prefix = if modals.is_empty() {
        "/root".to_string()
    } else {
        modals.first().unwrap().as_ez_object().get_full_path()
    };

    let mut current_selection = deselect_selected_widget(view_tree, state_tree, widget_tree,
                                                               callback_tree, scheduler);
    let result = find_next_selection(current_selection, state_tree, &path_prefix);
    if let Some(i) = result {
        select_widget(i, view_tree, state_tree, widget_tree, callback_tree, scheduler);
    } else  {
        // There's no next widget. Reset to 0 to cycle back to first widget (if any)
        current_selection = 0;
        let result = find_next_selection(current_selection, state_tree, &path_prefix);
        if let Some(i) = result {
            select_widget(i, view_tree, state_tree, widget_tree, callback_tree, scheduler);
        }
    }
}


/// Given a current selection order number, find the next widget, or
/// wrap back around to the first if none. Returns the full path of the next widget to be selected.
pub fn find_next_selection(current_selection: usize, state_tree: &StateTree, path_prefix: &str)
    -> Option<String> {


    let mut next_order= 0;
    let mut next_widget: Option<String> = None;
    for (path, state) in state_tree.objects.iter()  {
        if !path.starts_with(path_prefix) { continue };
        let generic_state = state.as_generic();
        let widget_order = generic_state.get_selection_order().value;
        if generic_state.is_selectable() && !generic_state.get_disabled().value &&
            widget_order > 0 && widget_order > current_selection &&
            (next_order == 0 || widget_order < next_order) &&
            !widget_is_hidden(path.to_string(), state_tree) &&
            is_in_view(path.to_string(), state_tree) {
            next_order = widget_order;
            next_widget = Some(path.to_string());
        }
    }
    next_widget
}


/// Select the previous widget by selection order as defined in each selectable widget. If the first
/// widget is currently selected wrap around and select the last. This function can always be
/// called safely.
pub fn select_previous(view_tree: &mut ViewTree, state_tree: &mut StateTree,
                       widget_tree: &WidgetTree, callback_tree: &mut CallbackTree,
                       scheduler: &mut Scheduler) {

    let modals = state_tree.get_by_path("/root").as_layout().get_modals();
    let path_prefix = if modals.is_empty() {
        "/root".to_string()
    } else {
        modals.first().unwrap().as_ez_object().get_full_path()
    };

    let mut current_selection = deselect_selected_widget(view_tree, state_tree, widget_tree,
                                                               callback_tree, scheduler);
    let result = find_previous_selection(current_selection, state_tree, &path_prefix);
    if let Some( previous_widget) = result {
        select_widget(previous_widget, view_tree, state_tree, widget_tree, callback_tree,
                      scheduler);
    } else {
        // There's no previous widget. Try again with 0 to cycle back to last widget
        current_selection = 0;
        let result = find_previous_selection(
            current_selection, state_tree, &path_prefix);
        if let Some( previous_widget) = result {
            select_widget(previous_widget, view_tree, state_tree, widget_tree,
                          callback_tree, scheduler);
        }
    }
}


/// Given a current selection order number, find the previous widget, or
/// wrap back around to the last if none. Returns the full path of the previous widget.
pub fn find_previous_selection(current_selection: usize, state_tree: &StateTree, path_prefix: &str)
    -> Option<String> {

    let mut previous_order = 0;
    let mut previous_widget: Option<String> = None;
    for (path, state) in state_tree.objects.iter() {
        if !path.starts_with(path_prefix) { continue }
        let generic_state = state.as_generic();
        let widget_order = generic_state.get_selection_order().value;
        if generic_state.is_selectable() && !generic_state.get_disabled().value &&
            widget_order > 0 && (current_selection == 0 || widget_order < current_selection) &&
            (previous_order == 0 || widget_order > previous_order) &&
            !widget_is_hidden(path.to_string(), state_tree) &&
            is_in_view(path.to_string(), state_tree) {
                if generic_state.get_selection_order().value - previous_order == 1
                    { return Some(path.to_string()) }
                previous_order = generic_state.get_selection_order().value;
                previous_widget = Some(path.to_string());
        }
    }
    previous_widget
}



/// Find a widget by a screen position coordinate. Used e.g. by mouse event handlers. If a modal
/// if active only the modal is searched.
pub fn get_widget_by_position<'a>(pos: Coordinates, widget_tree: &'a WidgetTree,
                                  state_tree: &StateTree) -> Vec<&'a dyn EzObject> {

    let modals = state_tree.get_by_path("/root").as_layout().get_modals();
    let path_prefix = if modals.is_empty() {
        "/root".to_string()
    } else {
        modals.first().unwrap().as_ez_object().get_full_path()
    };
    let mut results = Vec::new();
    for (widget_path, state) in state_tree.objects.iter() {
        if !widget_path.starts_with(&path_prefix) || widget_path == "/root" ||
            state.as_generic().get_disabled().value ||
            widget_is_hidden(widget_path.clone(),  state_tree) {
            continue
        }
        if state.as_generic().collides(pos) {
            results.push(widget_tree.get_by_path(&widget_path).as_ez_object());
        } else {
        }
    }
    results
}



/// Determine whether a widget (by path) is in view. We start with the root widget and make our
/// way down to the widget in question. We check whether the absolute pos of each widget is within
/// the bounds of the window. If we encounter a scrollview along the way, we will check if each
/// subsequent object is in bounds of the scrollview instead.
pub fn is_in_view(path: String, state_tree: &StateTree) -> bool {

    // If the widget belongs to a tab or screen that is not active, it is not in view
    let window_size = state_tree.get_by_path("/root").as_generic().get_size().clone();

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
        let state = state_tree.get_by_path(&working_path);
        // Determine if this part of the tree is in view. It's not in view if a visible area
        // was determined and this is not in it (visible area means we're scrolling somewhere),
        // or if absolute positions falls outside of window size.

        // If there's a visible width we're scrolling horizontally. Check if obj is in scrollview
        if let Some((visible_w_start, visible_w_end)) = visible_width {
            // If object lies completely left- or completely right of visible area it's out of view
            if state.as_generic().get_effective_position().x > visible_w_end ||
                state.as_generic().get_effective_position().x +
                    state.as_generic().get_effective_size().width < visible_w_start {
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
            if state.as_generic().get_effective_position().y > visible_h_end ||
                state.as_generic().get_effective_position().y +
                    state.as_generic().get_effective_size().height < visible_h_start {
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
            state.as_generic().get_effective_absolute_position().x > window_size.width.value) ||
            (visible_height == None &&
                state.as_generic().get_effective_absolute_position().y > window_size.height.value) {
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


pub fn widget_is_hidden(widget_path: String, state_tree: &StateTree) -> bool {

    if widget_path.starts_with("/modal") { return false }
    let mut check_parent =
        widget_path.rsplit_once('/').unwrap().0.to_string();
    let mut check_child = widget_path.clone();
    loop {
        let parent_state = state_tree.get_by_path(&check_parent).as_layout();
        if parent_state.mode == LayoutMode::Screen &&
            parent_state.active_screen.value != check_child.rsplit_once('/').unwrap().1 {
            return true
        }
        if parent_state.mode == LayoutMode::Tabbed {
            if let EzState::Layout(_) = state_tree.get_by_path(&check_child) {
                if parent_state.active_tab != check_child {
                    return true
                }
            }
        }
        if check_parent == "/root" { break }
        check_child = check_parent.clone();
        check_parent = check_parent.rsplit_once('/').unwrap().0.to_string();
    }
    false
}
