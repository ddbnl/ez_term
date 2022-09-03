//! # Input
//!
//! This module has functions that handle user input through keyboard and mouse.
use std::process::exit;

use crate::{Context, KeyMap};
use crossterm::event::{Event, KeyCode, KeyEvent, MouseButton, MouseEvent, MouseEventKind};

use crate::run::definitions::{CallbackTree, Coordinates, IsizeCoordinates, StateTree};
use crate::run::select::{get_widget_by_position, select_next, select_previous};
use crate::run::terminal::{initialize_terminal, write_to_screen};
use crate::run::tree::ViewTree;
use crate::scheduler::scheduler::SchedulerFrontend;
use crate::states::ez_state::EzState;
use crate::widgets::ez_object::{EzObject, EzObjects};
use crate::widgets::layout::layout::Layout;

use super::terminal::shutdown_terminal;

/// Try to handle an event by passing it to the active modal if any. The modal will return whether
/// it consumed the event or not.
pub fn handle_modal_event(
    event: Event,
    state_tree: &mut StateTree,
    root_widget: &Layout,
    callback_tree: &mut CallbackTree,
    scheduler: &mut SchedulerFrontend,
) -> bool {
    if !state_tree.as_layout().has_modal() {
        return false;
    }
    let modal = root_widget.state.get_modal();
    let mut consumed =
        modal
            .as_ez_object()
            .handle_event(event, state_tree, callback_tree, scheduler);
    if !consumed {
        if let EzObjects::Layout(layout) = modal {
            for child in layout.get_widgets_recursive() {
                consumed =
                    child
                        .as_ez_object()
                        .handle_event(event, state_tree, callback_tree, scheduler);
                if consumed {
                    return true;
                }
            }
        }
    }
    consumed
}

/// Try to handle an event as a global keybind. Examples are up/down keys for navigating menu,
/// left/right clicks, etc. If the event is bound globally, it will be consumed.
pub fn handle_global_event(
    event: Event,
    state_tree: &mut StateTree,
    root_widget: &Layout,
    callback_tree: &mut CallbackTree,
    scheduler: &mut SchedulerFrontend,
    selected_widget: &mut String,
    dragging: &mut Option<String>,
    last_dragging_pos: &mut IsizeCoordinates,
    global_keymap: &mut KeyMap,
) -> bool {
    match event {
        Event::Key(key) => handle_key_event(
            key,
            global_keymap,
            state_tree,
            root_widget,
            callback_tree,
            scheduler,
            selected_widget,
        ),
        Event::Mouse(event) => handle_mouse_event(
            event,
            state_tree,
            root_widget,
            callback_tree,
            scheduler,
            dragging,
            last_dragging_pos,
            selected_widget,
        ),
        _ => false,
    }
}

/// Global key handler. If a key event matches one of these keys it will be consumed and not passed
/// on any further.
fn handle_key_event(
    key: KeyEvent,
    global_keymap: &mut KeyMap,
    state_tree: &mut StateTree,
    root_widget: &Layout,
    callback_tree: &mut CallbackTree,
    scheduler: &mut SchedulerFrontend,
    selected_widget: &mut String,
) -> bool {
    let consumed = match key.code {
        KeyCode::Down => {
            select_next(
                state_tree,
                root_widget,
                callback_tree,
                scheduler,
                selected_widget,
            );
            true
        }
        KeyCode::Up => {
            select_previous(
                state_tree,
                root_widget,
                callback_tree,
                scheduler,
                selected_widget,
            );
            true
        }
        KeyCode::Enter => {
            if !selected_widget.is_empty()
                && !state_tree.get(&selected_widget).as_generic().get_disabled()
            {
                root_widget
                    .get_child_by_path(selected_widget)
                    .unwrap()
                    .as_ez_object()
                    .on_keyboard_enter(state_tree, callback_tree, scheduler);
            }
            true
        }
        KeyCode::Esc => {
            shutdown_terminal().unwrap();
            exit(0);
        }
        _ => false,
    };
    if !consumed && global_keymap.contains(key.code, key.modifiers) {
        let context = Context::new("".to_string(), state_tree, scheduler);
        let callback = global_keymap.get_mut(key.code, key.modifiers).unwrap();
        return callback(context, key.code, key.modifiers);
    }
    false
}

/// Global mouse event handler. Any widget that collides with the mouse_pos of the event will be
/// given a change to consume the event. This can be multiple widgets, e.g. a button as well as the
/// layout the button lives in.
fn handle_mouse_event(
    event: MouseEvent,
    state_tree: &mut StateTree,
    root_widget: &Layout,
    callback_tree: &mut CallbackTree,
    scheduler: &mut SchedulerFrontend,
    dragging: &mut Option<String>,
    last_dragging_pos: &mut IsizeCoordinates,
    selected_widget: &str,
) -> bool {
    if let MouseEventKind::Moved = event.kind {
        return handle_mouse_hover_event(
            event,
            state_tree,
            root_widget,
            callback_tree,
            scheduler,
            selected_widget,
        );
    }
    if let MouseEventKind::Drag(_) = event.kind {
        return handle_mouse_drag_event(
            event,
            state_tree,
            root_widget,
            callback_tree,
            scheduler,
            dragging,
            last_dragging_pos,
        );
    } else if let MouseEventKind::Down(button) = event.kind {
        return handle_mouse_press_event(
            event,
            button,
            state_tree,
            root_widget,
            callback_tree,
            scheduler,
        );
    } else if let MouseEventKind::ScrollUp = event.kind {
        return handle_mouse_scroll_up_event(
            event,
            state_tree,
            root_widget,
            callback_tree,
            scheduler,
        );
    } else if let MouseEventKind::ScrollDown = event.kind {
        return handle_mouse_scroll_down_event(
            event,
            state_tree,
            root_widget,
            callback_tree,
            scheduler,
        );
    }
    false
}

fn handle_mouse_press_event(
    event: MouseEvent,
    button: MouseButton,
    state_tree: &mut StateTree,
    root_widget: &Layout,
    callback_tree: &mut CallbackTree,
    scheduler: &mut SchedulerFrontend,
) -> bool {
    let consumed = false;
    let mouse_position = Coordinates::new(event.column as usize, event.row as usize);
    for widget in get_widget_by_position(mouse_position, root_widget, state_tree) {
        let abs = state_tree
            .get(&widget.get_path())
            .as_generic()
            .get_absolute_position();
        let relative_position = Coordinates::new(
            mouse_position.x - abs.usize_x(),
            mouse_position.y - abs.usize_y(),
        );
        let consumed = match button {
            MouseButton::Left => {
                widget.on_left_mouse_click(state_tree, callback_tree, scheduler, relative_position)
            }
            MouseButton::Right => {
                widget.on_right_mouse_click(state_tree, callback_tree, scheduler, relative_position)
            }
            _ => false,
        };
        if consumed {
            return true;
        }
    }
    consumed
}

fn handle_mouse_hover_event(
    event: MouseEvent,
    state_tree: &mut StateTree,
    root_widget: &Layout,
    callback_tree: &mut CallbackTree,
    scheduler: &mut SchedulerFrontend,
    selected_widget: &str,
) -> bool {
    let mouse_position = Coordinates::new(event.column as usize, event.row as usize);

    for widget in get_widget_by_position(mouse_position, root_widget, state_tree) {
        let abs = state_tree
            .get(&widget.get_path())
            .as_generic()
            .get_absolute_position();
        let relative_position = Coordinates::new(
            mouse_position.x - abs.usize_x(),
            mouse_position.y - abs.usize_y(),
        );
        if widget.on_hover(state_tree, callback_tree, scheduler, relative_position) {
            return true;
        }
    }
    if !selected_widget.is_empty() && !scheduler.backend.deselect {
        if let EzState::TextInput(_) = state_tree.get(selected_widget).obj {
        } else {
            scheduler.deselect_widget();
        }
    }
    true
}

fn handle_mouse_drag_event(
    event: MouseEvent,
    state_tree: &mut StateTree,
    root_widget: &Layout,
    callback_tree: &mut CallbackTree,
    scheduler: &mut SchedulerFrontend,
    dragging: &mut Option<String>,
    last_dragging_pos: &mut IsizeCoordinates,
) -> bool {
    let mouse_position = Coordinates::new(event.column as usize, event.row as usize);

    if let Some(ref path) = dragging {
        let widget = root_widget.get_child_by_path(&path).unwrap().as_ez_object();
        let abs = state_tree.get(path).as_generic().get_absolute_position();
        let relative_position = IsizeCoordinates::new(
            mouse_position.x as isize - abs.usize_x() as isize,
            mouse_position.y as isize - abs.usize_y() as isize,
        );
        let consumed = widget.on_drag(
            state_tree,
            callback_tree,
            scheduler,
            Some(*last_dragging_pos),
            relative_position,
        );
        if consumed {
            dragging.replace(widget.get_path());
            last_dragging_pos.x = relative_position.x;
            last_dragging_pos.y = relative_position.y;
            return true;
        }
    }
    for widget in get_widget_by_position(mouse_position, root_widget, state_tree) {
        let abs = state_tree
            .get(&widget.get_path())
            .as_generic()
            .get_absolute_position();
        let relative_position = IsizeCoordinates::new(
            mouse_position.x as isize - abs.usize_x() as isize,
            mouse_position.y as isize - abs.usize_y() as isize,
        );
        let consumed = widget.on_drag(
            state_tree,
            callback_tree,
            scheduler,
            None,
            relative_position,
        );
        if consumed {
            dragging.replace(widget.get_path());
            last_dragging_pos.x = relative_position.x;
            last_dragging_pos.y = relative_position.y;
            return true;
        }
    }
    true
}

fn handle_mouse_scroll_up_event(
    event: MouseEvent,
    state_tree: &mut StateTree,
    root_widget: &Layout,
    callback_tree: &mut CallbackTree,
    scheduler: &mut SchedulerFrontend,
) -> bool {
    let consumed = false;
    let mouse_position = Coordinates::new(event.column as usize, event.row as usize);
    for widget in get_widget_by_position(mouse_position, root_widget, state_tree) {
        let consumed = widget.on_scroll_up(state_tree, callback_tree, scheduler);
        if consumed {
            return consumed;
        }
    }
    consumed
}

fn handle_mouse_scroll_down_event(
    event: MouseEvent,
    state_tree: &mut StateTree,
    root_widget: &Layout,
    callback_tree: &mut CallbackTree,
    scheduler: &mut SchedulerFrontend,
) -> bool {
    let consumed = false;
    let mouse_position = Coordinates::new(event.column as usize, event.row as usize);
    for widget in get_widget_by_position(mouse_position, root_widget, state_tree) {
        let consumed = widget.on_scroll_down(state_tree, callback_tree, scheduler);
        if consumed {
            return consumed;
        }
    }
    consumed
}

/// Handle a resize event by setting the size of the root widget to the new window size, updating
/// the sizes/positions of all children and generating a new view tree of the right size.
pub fn handle_resize(
    view_tree: &mut ViewTree,
    state_tree: &mut StateTree,
    root_widget: &mut Layout,
    new_width: usize,
    new_height: usize,
) {
    for state in state_tree.get_all_mut() {
        if let EzState::Layout(_) = state {
            state
                .as_layout_mut()
                .get_scrolling_config_mut()
                .set_scroll_start_x(0.0);
            state
                .as_layout_mut()
                .get_scrolling_config_mut()
                .set_scroll_start_y(0.0);
        }
    }
    let state = state_tree.get_mut(&root_widget.path).as_generic_mut();
    state.get_size_mut().set_width(new_width as usize);
    state.get_size_mut().set_height(new_height as usize);
    let contents = root_widget.get_contents(state_tree);
    root_widget.propagate_absolute_positions(state_tree);
    // We need to re-initialize the terminal, because on Windows the hidden cursor will come back
    // on resize.
    initialize_terminal().unwrap();
    view_tree.initialize(new_width, new_height);
    view_tree.write_content(Coordinates::new(0, 0), contents);
    write_to_screen(view_tree);
}
