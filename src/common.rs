//! # Common:
//! A module containing common static functions used by other modules, as well as common types.
use crossterm::style::{PrintStyledContent, StyledContent};
use crossterm::{QueueableCommand, cursor};
use std::io::{stdout, Write};
use std::collections::HashMap;
use crossterm::event::KeyCode;
use crate::widgets::widget_state::{WidgetState};
use crate::widgets::widget::{EzWidget, EzObjects, Pixel};
use crate::widgets::layout::Layout;


/// Convenience types
/// # Pixel maps:
/// Used to represent the visual content of widgets. Pixels are a wrapper around
/// Crossterm StyledContent, so PixelMaps are essentially a grid of StyledContent to display.
pub type PixelMap = Vec<Vec<Pixel>>;
/// # Coordinates:
/// Convenience wrapper around an XY tuple.
pub type Coordinates = (usize, usize);
/// # View tree:
/// Grid of StyledContent representing the entire screen currently being displayed. After each frame
/// an updated ViewTree is diffed to the old one, and only changed parts of the screen are updated.
pub type ViewTree = Vec<Vec<StyledContent<String>>>;
/// # State tree:
/// A <WidgetPath, WidgetState> HashMap. The WidgetState contains all run-time information for a
/// widget, such as the text of a label, or whether a checkbox is currently checked. Callbacks
/// receive a mutable reference to the widget state and can change what they need. Then after each
/// frame the updated StateTree is diffed with the old one, and only changed widgets are redrawn.
pub type StateTree = HashMap<String, WidgetState>;

/// # Widget tree:
/// A read-only list of all widgets, passed to callbacks. Can be used to access static information
/// of a widget that is not in its' WidgetState. Widgets are represented by the EzWidget enum, but
/// can be cast to the generic UxObject or IsWidget trait. If you are sure of the type of widget
/// you are dealing with it can also be cast to specific widget types.
pub type WidgetTree<'a> = HashMap<String, &'a EzObjects>;

/// # Keyboard callback function:
/// This is used for binding keyboard callbacks to widgets, meaning that any callback functions a
/// user makes should use this signature.
pub type KeyboardCallbackFunction = fn(widget: String, key: KeyCode, view_tree: &mut ViewTree,
                                       state_tree: &mut StateTree, widget_tree: &WidgetTree);

/// # Mouse callback function:
/// This is used for binding mouse event callbacks to widgets, meaning that any callback functions
/// user makes should use this signature.
pub type MouseCallbackFunction = fn(widget: String, mouse_pos: Coordinates, view_tree: &mut ViewTree,
                                    state_tree: &mut StateTree, widget_tree: &WidgetTree);

/// # Value change callback function:
/// This is used for binding mouse event callbacks to widgets, meaning that any callback functions
/// user makes should use this signature.
pub type ValueChangeCallbackFunction = fn(widget: String, view_tree: &mut ViewTree,
                                          state_tree: &mut StateTree, widget_tree: &WidgetTree);


/// Find a widget by a screen position coordinate. Used e.g. by mouse event handlers.
pub fn get_widget_by_position<'a>(pos: Coordinates, widget_tree: &'a WidgetTree)
    -> Option<&'a dyn EzWidget> {
    for widget in widget_tree.values() {
        let generic_widget = widget.as_ez_widget();
        if generic_widget.collides(pos) {
            return Some(generic_widget)
        }
    }
    None
}


/// Write content to screen. Only writes differences between the passed view tree (current content)
/// and passed content (new content). The view tree is updated when changes are made.
pub fn write_to_screen(base_position: Coordinates, content: PixelMap,
                       view_tree: &mut ViewTree) {
    stdout().queue(cursor::SavePosition).unwrap().flush().unwrap();
    for x in 0..content.len() {
        for y in 0..content[0].len() {
            let write_pos = (base_position.0 + x, base_position.1 + y);
            let write_content = content[x][y].get_pixel();
            if view_tree[x][y] != write_content {
                view_tree[write_pos.0][write_pos.1] = write_content.clone();
                stdout().queue(cursor::MoveTo(
                    write_pos.0 as u16, write_pos.1 as u16)).unwrap()
                    .queue(PrintStyledContent(write_content)).unwrap();
            }
        }
    }
    stdout().queue(cursor::RestorePosition).unwrap().flush().unwrap();
}


/// Check each widget in a state tree for two things:
/// 1. If the state of the widget in the passed StateTree differs from the current widget state.
/// In this case the widget state should be updated with the new one, and the widget should be
/// redrawn.
/// 2. If its' state contains a forced redraw. In this case the entire screen will be rewritten,
/// and as such widgets will not be redrawn individually. Their state will still be updated.
pub fn update_state_tree(view_tree: &mut ViewTree, state_tree: &mut StateTree,
                         root_widget: &mut Layout) -> bool {
    let mut force_redraw = false;
    for widget_path in state_tree.keys() {
        let state = state_tree.get(widget_path).unwrap();
        let widget = root_widget.get_child_by_path_mut(widget_path).unwrap();
        let redraw_state = state.as_redraw_state();
        if redraw_state.get_force_redraw() {
            force_redraw = true;
        }
        if widget.state_changed(state) {
            widget.update_state(state);
            if !force_redraw { widget.redraw(view_tree); }
        }
    }
    force_redraw
}


/// Return the widget that is currently selected. Can be none.
pub fn get_selected_widget<'a>(widget_tree: &'a WidgetTree) -> Option<&'a dyn EzWidget> {
    for widget in widget_tree.values() {
        let generic_widget = widget.as_ez_widget();
        if generic_widget.is_selectable() && generic_widget.is_selected() {
            return Some(generic_widget)
        }
    }
    None
}


/// If any widget is currently selected, deselect it. Can always be called safely.
pub fn deselect_selected_widget(widget_tree: &WidgetTree, state_tree: &mut StateTree) {
    let selected_widget = get_selected_widget(widget_tree);
    if let Some(i) = selected_widget {
        state_tree.get_mut(&i.get_full_path()).unwrap().as_selectable_mut().set_selected(false);
    }

}


/// Select the next widget by selection order as defined in each selectable widget. If the last
/// widget is currently selected wrap around and select the first. This function can always be
/// called safely.
pub fn select_next(widget_tree: &WidgetTree, state_tree: &mut StateTree) {
    let current_selection = get_selected_widget(widget_tree);
    let mut current_order = if let Some(i) = current_selection {
        i.get_selection_order() } else { 0 };
    let result = find_next_selection(current_order,
                                     widget_tree);
    if let Some( next_widget) = result {
        state_tree.get_mut(&next_widget).unwrap().as_selectable_mut().set_selected(true);
    } else  {
        current_order = 0;
        let result = find_next_selection(current_order, widget_tree);
        if let Some( next_widget) = result {
            state_tree.get_mut(&next_widget).unwrap().as_selectable_mut().set_selected(true);
        }
    }
}


/// Given a current selection order number, find the next widget, or
/// wrap back around to the first if none. Returns the full path of the next widget to be selected.
pub fn find_next_selection(current_selection: usize, widget_tree: &WidgetTree) -> Option<String> {
    let mut next_order: Option<usize> = None;
    let mut next_widget: Option<String> = None;
    for widget in widget_tree.values()  {
        let generic_widget = widget.as_ez_widget();
        if generic_widget.is_selectable() {
            if let Some(i) = next_order {
                if generic_widget.get_selection_order() > current_selection &&
                    generic_widget.get_selection_order() < i {
                    next_order = Some(generic_widget.get_selection_order());
                    next_widget = Some(generic_widget.get_full_path());
                }
            } else if generic_widget.get_selection_order() > current_selection {
                next_order = Some(generic_widget.get_selection_order());
                next_widget = Some(generic_widget.get_full_path());
            }
        }
    }
    next_widget
}


/// Select the previous widget by selection order as defined in each selectable widget. If the first
/// widget is currently selected wrap around and select the last. This function can always be
/// called safely.
pub fn select_previous(widget_tree: &WidgetTree, state_tree: &mut StateTree) {

    let current_selection = get_selected_widget(widget_tree);
    let mut current_order = if let Some(i) = current_selection {
        i.get_selection_order() } else { 0 };
    let result = find_previous_selection(current_order,
                                         widget_tree);
    if let Some( previous_widget) = result {
        state_tree.get_mut(&previous_widget).unwrap().as_selectable_mut().set_selected(true);
    } else {
        current_order = 99999999;
        let result = find_previous_selection(current_order, widget_tree);
        if let Some( previous_widget) = result {
            state_tree.get_mut(&previous_widget).unwrap().as_selectable_mut().set_selected(true);
        }
    }
}


/// Given a current selection order number, find the previous widget, or
/// wrap back around to the last if none. Returns the full path of the previous widget.
pub fn find_previous_selection(current_selection: usize, widget_tree: &WidgetTree)
                                   -> Option<String> {

    let mut previous_order: Option<usize> = None;
    let mut previous_widget: Option<String> = None;
    for widget in widget_tree.values()  {
        let generic_widget = widget.as_ez_widget();
        if generic_widget.is_selectable() {
            if let Some(i) = previous_order {
                if generic_widget.get_selection_order() < current_selection &&
                    generic_widget.get_selection_order() > i {
                    previous_order = Some(generic_widget.get_selection_order());
                    previous_widget = Some(generic_widget.get_full_path());
                }
            } else if generic_widget.get_selection_order() < current_selection {
                previous_order = Some(generic_widget.get_selection_order());
                previous_widget = Some(generic_widget.get_full_path());
            }
        }
    }
    previous_widget
}
