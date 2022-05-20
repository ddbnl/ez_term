//! # Common:
//! A module containing common static functions used by other modules, as well as common types.
use crossterm::style::{Color, PrintStyledContent, StyledContent};
use crossterm::{QueueableCommand, cursor, ExecutableCommand};
use std::io::{stdout, Write};
use std::collections::HashMap;
use crossterm::event::KeyCode;
use crate::scheduler::Scheduler;
use crate::widgets::layout::Layout;
use crate::widgets::state::{State};
use crate::widgets::widget::{EzWidget, EzObjects, Pixel};


/// # Convenience types
/// ## Pixel maps:
/// Used to represent the visual content of widgets. Pixels are a wrapper around
/// Crossterm StyledContent, so PixelMaps are essentially a grid of StyledContent to display.
pub type PixelMap = Vec<Vec<Pixel>>;

/// ## Key map
/// A crossterm KeyCode > Callback function lookup. Used for custom user keybinds
pub type KeyMap = HashMap<KeyCode, KeyboardCallbackFunction>;

/// ## Coordinates:
/// Convenience wrapper around an XY tuple.
pub type Coordinates = (usize, usize);

/// ## View tree:
/// Grid of StyledContent representing the entire screen currently being displayed. After each frame
/// an updated ViewTree is diffed to the old one, and only changed parts of the screen are updated.
pub type ViewTree = Vec<Vec<StyledContent<String>>>;

/// ## State tree:
/// A <WidgetPath, State> HashMap. The State contains all run-time information for a
/// widget, such as the text of a label, or whether a checkbox is currently checked. Callbacks
/// receive a mutable reference to the widget state and can change what they need. Then after each
/// frame the updated StateTree is diffed with the old one, and only changed widgets are redrawn.
pub type StateTree = HashMap<String, State>;

/// ## Widget tree:
/// A read-only list of all widgets, passed to callbacks. Can be used to access static information
/// of a widget that is not in its' State. Widgets are represented by the EzWidget enum, but
/// can be cast to the generic UxObject or IsWidget trait. If you are sure of the type of widget
/// you are dealing with it can also be cast to specific widget types.
pub type WidgetTree<'a> = HashMap<String, &'a EzObjects>;

/// ## Keyboard callback function:
/// This is used for binding keyboard callbacks to widgets, meaning that any callback functions a
/// user makes should use this signature.
pub type KeyboardCallbackFunction = fn(EzContext, key: KeyCode);

/// ## Mouse callback function:
/// This is used for binding mouse event callbacks to widgets, meaning that any callback functions
/// user makes should use this signature.
pub type MouseCallbackFunction = fn(EzContext, mouse_pos: Coordinates);

/// ## Generic Ez function:
/// Used for callbacks and scheduled tasks that don't require special parameter such as KeyCodes
/// or mouse positions. Used e.g. for [on_value_change] and [on_keyboard_enter].
pub type GenericEzFunction = fn(EzContext);
pub type GenericEzTask = Box<dyn FnMut(EzContext) -> bool>;

/// ## Ez Context:
/// Used for providing context to callbacks and scheduled tasks.
pub struct EzContext<'a, 'b, 'c, 'd> {

    /// Path to the widget this context refers to, e.g. the widget a callback originatec from
    pub widget_path: String,

    /// The current [ViewTree]
    pub view_tree: &'a mut ViewTree,

    /// The current [StateTree]
    pub state_tree: &'b mut StateTree,

    /// The current [WidgetTree]
    pub widget_tree: &'c WidgetTree<'c>,

    /// The current [Scheduler]
    pub scheduler: &'d mut Scheduler,
}
impl<'a, 'b , 'c, 'd> EzContext<'a, 'b , 'c, 'd> {
    pub fn new(widget_path: String, view_tree: &'a mut ViewTree, state_tree: &'b mut StateTree,
           widget_tree: &'c WidgetTree, scheduler: &'d mut Scheduler) -> Self {
        EzContext { widget_path, view_tree, state_tree, widget_tree, scheduler }
    }
}

/// Find a widget by a screen position coordinate. Used e.g. by mouse event handlers.
pub fn get_widget_by_position<'a>(pos: Coordinates, widget_tree: &'a WidgetTree)
    -> Option<&'a dyn EzWidget> {
    for widget in widget_tree.values() {
        if let EzObjects::Layout(_) = widget { continue }
        let generic_widget = widget.as_ez_widget();
        if generic_widget.collides(pos) {
            return Some(generic_widget)
        }
    }
    None
}


/// Write content to screen. Only writes differences between the passed view tree (current content)
/// and passed content (new content). The view tree is updated when changes are made.
pub fn write_to_screen(base_position: Coordinates, content: PixelMap, view_tree: &mut ViewTree) {
    stdout().execute(cursor::SavePosition).unwrap();
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
    stdout().flush().unwrap();
    stdout().execute(cursor::RestorePosition).unwrap();
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
    let mut widgets_to_redraw = Vec::new();
    for widget_path in state_tree.keys() {
        let state = state_tree.get(widget_path).unwrap();
        if state.as_generic_state().get_force_redraw() {
            force_redraw = true;
        }
        if state.as_generic_state().get_changed() {
            let widget = root_widget.get_child_by_path_mut(widget_path).unwrap()
                .as_ez_object_mut();
            widget.update_state(state);
            widgets_to_redraw.push(widget_path.clone());
        }
    }
    if !force_redraw {
        for widget_path in widgets_to_redraw {
            root_widget.get_child_by_path_mut(&widget_path).unwrap().as_ez_object_mut()
                .redraw(view_tree, state_tree);
        }
    }
    force_redraw
}


/// Return the widget that is currently selected. Can be none.
pub fn get_selected_widget<'a>(widget_tree: &'a WidgetTree) -> Option<&'a dyn EzWidget> {
    for widget in widget_tree.values() {
        if let EzObjects::Layout(_) = widget { continue }  // Layouts cannot be selected
        let generic_widget = widget.as_ez_widget();
        if generic_widget.is_selectable() && generic_widget.is_selected() {
            return Some(generic_widget)
        }
    }
    None
}


/// If any widget is currently selected, deselect it. Can always be called safely.
pub fn deselect_selected_widget(view_tree: &mut ViewTree, state_tree: &mut StateTree,
        widget_tree: &WidgetTree, scheduler: &mut Scheduler) {
    let selected_widget = get_selected_widget(widget_tree);
    if let Some(i) = selected_widget {
        let context = EzContext::new(i.get_full_path(), view_tree,
                                      state_tree, widget_tree, scheduler);
        i.on_deselect(context)
    }

}


/// Select the next widget by selection order as defined in each selectable widget. If the last
/// widget is currently selected wrap around and select the first. This function can always be
/// called safely.
pub fn select_next(view_tree: &mut ViewTree, state_tree: &mut StateTree,
                   widget_tree: &WidgetTree, scheduler: &mut Scheduler) {
    let current_selection = get_selected_widget(widget_tree);
    let mut current_order = if let Some(i) = current_selection {
        i.get_selection_order() } else { 0 };
    let result = find_next_selection(current_order,
                                     widget_tree);
    if let Some( next_widget) = result {
        let context = EzContext::new(next_widget.clone(), view_tree,
                                     state_tree, widget_tree, scheduler);
        widget_tree.get(&next_widget).unwrap().as_ez_widget().on_select(context,
                                                                        None);
    } else  {
        current_order = 0;
        let result = find_next_selection(current_order, widget_tree);
        if let Some( next_widget) = result {
            let context = EzContext::new(next_widget.clone(), view_tree,
                                         state_tree, widget_tree, scheduler);
            widget_tree.get(&next_widget).unwrap()
                .as_ez_widget().on_select(context,None);
        }
    }
}


/// Given a current selection order number, find the next widget, or
/// wrap back around to the first if none. Returns the full path of the next widget to be selected.
pub fn find_next_selection(current_selection: usize, widget_tree: &WidgetTree) -> Option<String> {
    let mut next_order: Option<usize> = None;
    let mut next_widget: Option<String> = None;
    for widget in widget_tree.values()  {
        if let EzObjects::Layout(_) = widget { continue }  // Layouts cannot be selected
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
pub fn select_previous(view_tree: &mut ViewTree, state_tree: &mut StateTree,
                       widget_tree: &WidgetTree, scheduler: &mut Scheduler) {

    let current_selection = get_selected_widget(widget_tree);
    let mut current_order = if let Some(i) = current_selection {
        i.get_selection_order() } else { 0 };
    let result = find_previous_selection(current_order, widget_tree);
    if let Some( previous_widget) = result {

        let context = EzContext::new(previous_widget.clone(), view_tree,
                                     state_tree, widget_tree, scheduler);
        widget_tree.get(&previous_widget).unwrap()
            .as_ez_widget().on_select(context,None);
        state_tree.get_mut(&previous_widget).unwrap().as_selectable_mut().set_selected(true);
    } else {
        current_order = 99999999;
        let result = find_previous_selection(current_order, widget_tree);
        if let Some( previous_widget) = result {
            let context = EzContext::new(previous_widget.clone(), view_tree,
                                         state_tree, widget_tree, scheduler);
            widget_tree.get(&previous_widget).unwrap()
                .as_ez_widget().on_select(context,None);
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
        if let EzObjects::Layout(_) = widget { continue }  // Layouts cannot be selected
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


/// Add a border around a PixelMap.
pub fn add_border(mut content: PixelMap, horizontal_symbol: String, vertical_symbol: String,
              top_left_symbol: String, top_right_symbol: String, bottom_left_symbol: String,
              bottom_right_symbol: String, bg_color: Color, fg_color: Color) -> PixelMap {
    // Create border elements
    let horizontal_border = Pixel{ symbol: horizontal_symbol, background_color: bg_color,
        foreground_color: fg_color, underline: false};
    let vertical_border = Pixel{ symbol: vertical_symbol, background_color: bg_color,
        foreground_color: fg_color, underline: false};
    let top_left_border = Pixel{ symbol:top_left_symbol, background_color: bg_color,
        foreground_color: fg_color, underline: false};
    let top_right_border = Pixel{ symbol: top_right_symbol, background_color: bg_color,
        foreground_color: fg_color, underline: false};
    let bottom_left_border = Pixel{ symbol: bottom_left_symbol, background_color: bg_color,
        foreground_color: fg_color, underline: false};
    let bottom_right_border = Pixel{ symbol: bottom_right_symbol, background_color: bg_color,
        foreground_color: fg_color, underline: false};
    // Create horizontal borders
    for x in 0..content.len() {
        let mut new_x = vec!(horizontal_border.clone());
        for y in &content[x] {
            new_x.push(y.clone());
        }
        new_x.push(horizontal_border.clone());
        content[x] = new_x
    }
    // Create left border
    let mut left_border = vec!(top_left_border);
    for _ in 0..content[0].len() -  2{
        left_border.push(vertical_border.clone());
    }
    left_border.push(bottom_left_border);
    // Create right border
    let mut right_border = vec!(top_right_border);
    for _ in 0..content[0].len() - 2 {
        right_border.push(vertical_border.clone())
    }
    right_border.push(bottom_right_border);
    // Merge all borders around the content
    let mut new_content = vec!(left_border);
    for x in content {
        new_content.push(x);
    }
    new_content.push(right_border);
    new_content

}

/// Take a PixelMap and rotate it. Normally a Vec<Vec<Pixel>> is essentially a list
/// of rows. Therefore "for X in PixelMap, for Y in X" let's you iterate in the Y direction.
/// By rotating the PixelMap it is turned into a list of columns, so that it becomes
/// "for Y in PixelMap, for X in Y", so you can iterate in the X direction.
pub fn rotate_pixel_map(content: PixelMap) -> PixelMap {

    let mut rotated_content = PixelMap::new();
    for x in 0..content.len() {
        for y in 0..content[0].len() {
            if x == 0 { rotated_content.push(Vec::new()) };
            rotated_content[y].push(content[x][y].clone())
        }
    }
    rotated_content
}