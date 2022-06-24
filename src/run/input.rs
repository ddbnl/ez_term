use std::io::{stdout};
use std::process::exit;
use crossterm::{event::{MouseEvent, MouseEventKind, MouseButton, Event, KeyCode, KeyEvent},
                terminal, QueueableCommand};
use super::terminal::shutdown_terminal;
use crate::common;
use crate::common::definitions::{CallbackTree, StateTree, ViewTree, WidgetTree, Coordinates};
use crate::widgets::layout::Layout;
use crate::widgets::widget::{EzObject};
use crate::scheduler::scheduler::Scheduler;
use crate::states::state::EzState;
use crate::widgets::widget;


/// Try to handle an event by passing it to the active modal if any. The modal will return whether
/// it consumed the event or not.
pub fn handle_modal_event (event: Event, view_tree: &mut ViewTree, state_tree: &mut StateTree,
                       widget_tree: &WidgetTree, callback_tree: &mut CallbackTree,
                       scheduler: &mut Scheduler, root_widget: &Layout) -> bool {

    let mut consumed;
    if state_tree.get_by_path(&root_widget.path).as_layout().open_modals.is_empty() {
        return false
    }
    let modal_root = state_tree.get_by_path(&root_widget.path).as_layout()
        .open_modals.first().unwrap().as_ez_object().get_full_path();
    for (path, widget) in widget_tree.objects.iter() {
        if !path.starts_with(&modal_root) { continue }
        if let widget::EzObjects::Layout(i) = widget {
            for child in i.get_widgets_recursive().values() {
                consumed = child.as_ez_object().handle_event(
                    event, view_tree, state_tree, widget_tree, callback_tree, scheduler);
                if consumed {
                    return true
                }
            }
        } else {
            consumed = widget.as_ez_object().handle_event(
                event, view_tree, state_tree, widget_tree, callback_tree, scheduler);
            if consumed {
                return true
            }
        }
    }
    false
}

/// Try to handle an event as a global keybind. Examples are up/down keys for navigating menu
pub fn handle_global_event(event: Event, view_tree: &mut ViewTree, state_tree: &mut StateTree,
                       widget_tree: &WidgetTree, callback_tree: &mut CallbackTree, 
                       scheduler: &mut Scheduler) -> bool {

    match event {
        Event::Key(key) => {
            handle_key_event(key, view_tree, state_tree, widget_tree, callback_tree, scheduler)
        }
        Event::Mouse(event) => {
            handle_mouse_event(event, view_tree, state_tree, widget_tree, callback_tree, scheduler)
        }
        _ => false,
    }
}


/// Global key handler. If a key event matches one of these keys it will be consumed and not passed
/// on any further.
fn handle_key_event(key: KeyEvent, view_tree: &mut ViewTree, state_tree: &mut StateTree,
                    widget_tree: &WidgetTree, callback_tree: &mut CallbackTree,
                    scheduler: &mut Scheduler) -> bool {

    match key.code {
        KeyCode::Down => {
            common::selection_functions::select_next(
                view_tree, state_tree, widget_tree, callback_tree, scheduler);
            true
        },
        KeyCode::Up => {
            common::selection_functions::select_previous(
                view_tree, state_tree, widget_tree, callback_tree, scheduler);
            true
        },
        KeyCode::Enter => {
            let selected_widget =
                common::selection_functions::get_selected_widget(widget_tree, state_tree);
            if let Some(widget) = selected_widget {
                if !state_tree
                    .get_by_path(&widget.get_full_path()).as_generic().get_disabled().value {
                    widget.on_keyboard_enter(view_tree, state_tree, widget_tree,
                                             callback_tree, scheduler);
                }
            }
            true
        },
        KeyCode::Esc => {
            shutdown_terminal().unwrap();
            exit(0);
        }
        _ => false
    }
}


/// Global mouse event handler. If the click pos collides a widget it will be consumed and not
/// passed on any further.
fn handle_mouse_event(event: MouseEvent, view_tree: &mut ViewTree, state_tree: &mut StateTree,
                      widget_tree: &WidgetTree, callback_tree: &mut CallbackTree, 
                      scheduler: &mut Scheduler) -> bool {

    if let MouseEventKind::Down(button) = event.kind {
        let mouse_position = Coordinates::new(
            event.column as usize,event.row as usize);
        for widget in common::selection_functions::get_widget_by_position(
            mouse_position, widget_tree, state_tree) {

            let abs = state_tree.get_by_path(&widget.get_full_path()).as_generic()
                .get_absolute_position();
            let relative_position = Coordinates::new(
                mouse_position.x - abs.x, mouse_position.y - abs.y);
            let consumed = match button {
                MouseButton::Left => {
                    common::selection_functions::deselect_selected_widget(
                        view_tree, state_tree, widget_tree, callback_tree, scheduler);

                    let consumed = widget.on_left_mouse_click(view_tree, state_tree, widget_tree,
                                               callback_tree, scheduler,relative_position);
                    if consumed && state_tree.get_by_path(&widget.get_full_path()).as_generic()
                            .is_selectable() {
                        widget.on_select(view_tree, state_tree, widget_tree, callback_tree,
                                         scheduler,Some(relative_position));
                    }
                    consumed
                },
                MouseButton::Right => {
                    widget.on_right_mouse_click(view_tree, state_tree, widget_tree,
                                                callback_tree, scheduler,
                                                relative_position)
                }
                _ => { false }
            };
            if consumed { return true }
        }
    } else if let MouseEventKind::ScrollUp = event.kind {
        let mouse_position = Coordinates::new(event.column as usize,event.row as usize);
        for widget in common::selection_functions::get_widget_by_position(
            mouse_position, widget_tree, state_tree) {
            let consumed = widget.on_scroll_up(view_tree, state_tree, widget_tree, callback_tree,
                                       scheduler);
            if consumed { return consumed }
        }
    } else if let MouseEventKind::ScrollDown = event.kind {
        let mouse_position = Coordinates::new(
        event.column as usize,event.row as usize);
        for widget in common::selection_functions::get_widget_by_position(
            mouse_position, widget_tree, state_tree) {
            let consumed = widget.on_scroll_down(view_tree, state_tree, widget_tree, callback_tree,
                                       scheduler);
            if consumed { return consumed }
        }
    }
    false
}


/// Handle a resize event by setting the size of the root widget to the new window size, updating
/// the sizes/positions of all children and generating a new view tree of the right size.
pub fn handle_resize(view_tree: &mut ViewTree, state_tree: &mut StateTree, root_widget: &mut Layout,
                 new_width: usize, new_height: usize){

    for state in state_tree.objects.values_mut() {
        if let EzState::Layout(_) = state {
            state.as_layout_mut().scrolling_config.view_start_x = 0;
            state.as_layout_mut().scrolling_config.view_start_y = 0;
        }
    }
    let state = state_tree.get_by_path_mut(&root_widget.path).as_generic_mut();
    state.get_size_mut().width.set(new_width as usize);
    state.get_size_mut().height.set(new_height as usize);
    root_widget.set_child_sizes(state_tree);
    let contents = root_widget.get_contents(state_tree);
    root_widget.propagate_absolute_positions(state_tree);
    // Cleartype purge is tempting but causes issues on at least Windows
    stdout().queue(terminal::Clear(terminal::ClearType::All)).unwrap();
    view_tree.initialize(new_width, new_height);
    view_tree.write_content(Coordinates::new(0, 0), contents);
    common::screen_functions::write_to_screen(view_tree);
}
