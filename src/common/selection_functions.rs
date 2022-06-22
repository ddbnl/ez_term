//! # Common:
//! A module containing common static functions used by other modules, as well as common types.
use crate::scheduler::Scheduler;
use common::definitions::Coordinates;
use crate::{common};
use crate::widgets::widget::{EzObject};



/// Return the widget that is currently selected. Can be none.
pub fn get_selected_widget<'a>(widget_tree: &'a common::definitions::WidgetTree,
                               state_tree: &mut common::definitions::StateTree)
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
        let state = state_tree.get_by_path(&generic_widget.get_full_path())
            .as_generic();
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
        let state = state_tree.get_by_path_mut(&i.get_full_path()).as_generic_mut();
        state.set_selected(false);
        state.update(scheduler);
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

    let modals = state_tree.get_by_path("/root").as_layout().get_modals();
    let path_prefix = if modals.is_empty() {
        "/root".to_string()
    } else {
        modals.first().unwrap().as_ez_object().get_full_path()
    };

    let current_selection = get_selected_widget(widget_tree, state_tree);
    let mut current_order = if let Some(i) = current_selection {
        let state = state_tree.get_by_path_mut(&i.get_full_path()).as_generic_mut();
        state.set_selected(false);
        state.update(scheduler);
        let order = state.get_selection_order();
        i.on_deselect(view_tree, state_tree, widget_tree, callback_tree, scheduler);
        order
    } else {
        0
    };
    let result = find_next_selection(
        current_order, state_tree, &path_prefix);
    if let Some( next_widget) = result {
        widget_tree.get_by_path(&next_widget).as_ez_object().on_select(
            view_tree,state_tree, widget_tree, callback_tree,scheduler, None);
        let state = state_tree.get_by_path_mut(&next_widget).as_generic_mut();
        state.set_selected(true);
        state.update(scheduler);
    } else  {
        current_order = 0;
        let result = find_next_selection(
            current_order, state_tree, &path_prefix);
        if let Some( next_widget) = result {
            widget_tree.get_by_path(&next_widget).as_ez_object().on_select(
                view_tree,state_tree, widget_tree, callback_tree,scheduler, None);
            let state =
                state_tree.get_by_path_mut(&next_widget).as_generic_mut();
            state.set_selected(true);
            state.update(scheduler);
        }
    }
}


/// Given a current selection order number, find the next widget, or
/// wrap back around to the first if none. Returns the full path of the next widget to be selected.
pub fn find_next_selection(current_selection: usize, state_tree: &common::definitions::StateTree,
                           path_prefix: &str) -> Option<String> {


    let mut next_order: Option<usize> = None;
    let mut next_widget: Option<String> = None;
    for (path, state) in state_tree.objects.iter()  {
        if !path.starts_with(path_prefix) { continue };
        let generic_state = state.as_generic();
        if generic_state.is_selectable() && !generic_state.get_disabled() {
            if let Some(i) = next_order {
                if generic_state.get_selection_order() > 0 &&
                    generic_state.get_selection_order() > current_selection &&
                    generic_state.get_selection_order() < i &&
                    !common::widget_functions::widget_is_hidden(
                        path.to_string(), state_tree) &&
                    common::widget_functions::is_in_view(path.to_string(), state_tree) {
                    next_order = Some(generic_state.get_selection_order());
                    next_widget = Some(path.to_string());
                }
            } else if generic_state.get_selection_order() > 0 &&
                generic_state.get_selection_order() > current_selection &&
                !common::widget_functions::widget_is_hidden(
                    path.to_string(), state_tree) &&
                common::widget_functions::is_in_view(path.to_string(), state_tree) {
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

    let modals = state_tree.get_by_path("/root").as_layout().get_modals();
    let path_prefix = if modals.is_empty() {
        "/root".to_string()
    } else {
        modals.first().unwrap().as_ez_object().get_full_path()
    };

    let current_selection = get_selected_widget(widget_tree, state_tree);
    let mut current_order = if let Some(i) = current_selection {
        let state = state_tree.get_by_path_mut(&i.get_full_path()).as_generic_mut();
        state.set_selected(false);
        state.update(scheduler);
        let order = state.get_selection_order();
        i.on_deselect(view_tree, state_tree, widget_tree, callback_tree, scheduler);
        order
    } else {
        0
    };
    let result = find_previous_selection(
        current_order, state_tree, &path_prefix);
    if let Some( previous_widget) = result {
        let state = state_tree.get_by_path_mut(&previous_widget)
            .as_generic_mut();
        state.set_selected(true);
        state.update(scheduler);
        widget_tree.get_by_path(&previous_widget).as_ez_object().on_select(
            view_tree,state_tree, widget_tree, callback_tree,scheduler, None);
    } else {
        current_order = 99999999;
        let result = find_previous_selection(
            current_order, state_tree, &path_prefix);
        if let Some( previous_widget) = result {
            let state = state_tree.get_by_path_mut(&previous_widget).as_generic_mut();
            state.set_selected(true);
            state.update(scheduler);
            widget_tree.get_by_path(&previous_widget).as_ez_object().on_select(
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
    for (path, state) in state_tree.objects.iter()  {
        if !path.starts_with(path_prefix) { continue }
        let generic_state = state.as_generic();
        if generic_state.is_selectable() && !generic_state.get_disabled() {
            if let Some(i) = previous_order {
                if generic_state.get_selection_order() > 0 &&
                    generic_state.get_selection_order() < current_selection &&
                    generic_state.get_selection_order() > i &&
                    !common::widget_functions::widget_is_hidden(
                        path.to_string(), state_tree) &&
                    common::widget_functions::is_in_view(path.to_string(), state_tree) {
                    previous_order = Some(generic_state.get_selection_order());
                    previous_widget = Some(path.to_string());
                }
            } else if generic_state.get_selection_order() > 0 &&
                generic_state.get_selection_order() < current_selection &&
                !common::widget_functions::widget_is_hidden(
                    path.to_string(), state_tree) &&
                common::widget_functions::is_in_view(path.to_string(), state_tree) {
                previous_order = Some(generic_state.get_selection_order());
                previous_widget = Some(path.to_string());
            }
        }
    }
    previous_widget
}



/// Find a widget by a screen position coordinate. Used e.g. by mouse event handlers. If a modal
/// if active only the modal is searched.
pub fn get_widget_by_position<'a>(pos: Coordinates,
                                  widget_tree: &'a common::definitions::WidgetTree,
                                  state_tree: &common::definitions::StateTree)
    -> Vec<&'a dyn EzObject> {

    let modals = state_tree.get_by_path("/root").as_layout().get_modals();
    let path_prefix = if modals.is_empty() {
        "/root".to_string()
    } else {
        modals.first().unwrap().as_ez_object().get_full_path()
    };
    let mut results = Vec::new();
    for (widget_path, state) in state_tree.objects.iter() {
        if !widget_path.starts_with(&path_prefix) || widget_path == "/root" ||
            state.as_generic().get_disabled() ||
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

