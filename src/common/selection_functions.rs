//! # Common:
//! A module containing common static functions used by other modules, as well as common types.
use crate::scheduler::scheduler::Scheduler;
use common::definitions::Coordinates;
use crate::{common};
use crate::common::definitions::{CallbackTree, StateTree, ViewTree, WidgetTree};
use crate::widgets::widget::{EzObject};



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
            !common::widget_functions::widget_is_hidden(path.to_string(), state_tree) &&
            common::widget_functions::is_in_view(path.to_string(), state_tree) {
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
            !common::widget_functions::widget_is_hidden(path.to_string(), state_tree) &&
            common::widget_functions::is_in_view(path.to_string(), state_tree) {
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
            common::widget_functions::widget_is_hidden(widget_path.clone(),  state_tree) {
            continue
        }
        if state.as_generic().collides(pos) {
            results.push(widget_tree.get_by_path(&widget_path).as_ez_object());
        } else {
        }
    }
    results
}

