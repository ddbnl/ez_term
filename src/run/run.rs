//! # Run:
//! A module containing the functions to start and stop the main App run loop. The exposed "Run"
//! function allows starting the app based on a root layout and scheduler.
use std::process::exit;
use std::time::{Duration, Instant};

use crossterm::{event::{Event, MouseEventKind, poll, read}, Result};
use crossterm::event::MouseButton;

use crate::CallbackConfig;
use crate::run::definitions::{CallbackTree, Coordinates, StateTree};
use crate::run::terminal::{redraw_changed_widgets, write_to_screen};
use crate::run::tree::{clean_trees, initialize_callback_tree, ViewTree};
use crate::scheduler::scheduler::SchedulerFrontend;
use crate::scheduler::scheduler_funcs::{create_new_widgets, drain_property_channels, handle_next_selection, run_tasks, update_callback_configs, update_properties, update_threads};
use crate::states::ez_state::GenericState;
use crate::widgets::ez_object::{EzObject, EzObjects};
use crate::widgets::layout::layout::Layout;

use super::input::{handle_global_event, handle_modal_event, handle_resize};
use super::terminal::{initialize_terminal, shutdown_terminal};


/// This function starts the terminal app.
///
/// Make sure you load a root layout from a .ez file first and pass it to this func, like this:
/// ```
/// let (root_widget, state_tree, scheduler) = load_ui("root.ez");
/// ```
/// After loading the root layout, make all the manual changes you require, such as setting
/// keybindings or binding callbacks to events. Then call this function.
/// ```
/// run(root_widget, state_tree, scheduler);
/// ```
pub fn run(root_widget: Layout, state_tree: StateTree, scheduler: SchedulerFrontend) {

    initialize_terminal().unwrap();
    let callback_tree = initialize_callback_tree(&root_widget);
    run_loop(root_widget, state_tree, callback_tree, scheduler).unwrap();
}


/// Gracefully stop the app, restoring the terminal to its' original state.
pub fn stop() {
    shutdown_terminal().unwrap();
    exit(0);
}


/// Called just before [run]. Creates initial view- and state trees and writes initial content
/// to the screen.
fn initialize_widgets(root_widget: &mut Layout, state_tree: &mut StateTree) -> ViewTree {

    let all_content = root_widget.get_contents(state_tree);
    root_widget.propagate_absolute_positions(state_tree);

    // Create an initial view tree so we can diff all future changes against it.
    let mut view_tree = ViewTree::default();
    view_tree.initialize(root_widget.state.get_size().get_width(),
                         root_widget.state.get_size().get_height());
    view_tree.write_content(Coordinates::new(0, 0), all_content);
    write_to_screen(&mut view_tree);
    view_tree

}


/// Support function for opening a popup. After opening the actual popup in the root layout the
/// state tree is extended with the new modal widget state, and the same is done for the callback
/// tree.
pub fn open_popup(template: String, state_tree: &mut StateTree, scheduler: &mut SchedulerFrontend)
    -> String {

    let state = state_tree.get_by_path_mut("/root").as_layout_mut();
    state.update(scheduler);
    let (path, sub_tree) = state.open_popup(template, scheduler);
    state_tree.extend(sub_tree);
    let state = state_tree.get_by_path_mut("/root").as_layout_mut();
    scheduler.overwrite_callback_config(path.as_str(),
                                        CallbackConfig::default());
    let modal = state.get_modals().first().unwrap();
    if let EzObjects::Layout(ref i) = modal {
        for sub_widget in i.get_widgets_recursive().values() {
            scheduler.overwrite_callback_config(
                sub_widget.as_ez_object().get_full_path().as_str(),
                CallbackConfig::default());
        }
    }
    path
}


/// Main loop of the app. Consumes Crossterm events to handle key/mouse input. The app works with
/// three trees in order play nice with Rusts' "only one mutable state" requirement. Instead of
/// passing around a mutable root widget to all callbacks and event handlers, a widget tree,
/// view tree and state tree are created an passed around instead of the whole root widget.
/// # View tree:
/// The view tree is a Vec<Vec<StyledContent>>, essentially a XY grid with a Crossterm StyledContent
/// in each cell. Every time something on screen changes the view tree is updated. This way, we can
/// update the screen only where needed by diffing the old view state to the current view state.
/// # State tree:
/// The state tree is a HashMap<String, EzWidget>, a dictionary with full widget path as the key,
/// and a State object as the value. The State contains all important run time
/// information of a widget, e.g. the text of a label, or whether a checkbox is checked. Callbacks
/// are passed a mutable ref to the state tree and can alter it as they like. After a single run
/// loop the updated state tree is diffed against the state of each widget, triggering a redraw if
/// anything was changed.
/// # Widget tree:
/// The widget tree is a Vec<EzWidget> vector, basically a list of every widget. These are used to
/// gain read-only access to a widget to any information from it that is not stored in its' state
/// (i.e. static callbacks). EzWidget enums can be downcast to EzObject trait objects to
/// access common functions, or downcast to their specific widget type if you know for sure what it
/// is.
fn run_loop(mut root_widget: Layout, mut state_tree: StateTree, mut callback_tree: CallbackTree,
            mut scheduler: SchedulerFrontend) -> Result<()>{

    let mut view_tree = initialize_widgets(&mut root_widget, &mut state_tree);
    let last_update = Instant::now(); // Time of last screen update,
    let tick_rate = (1/60) as u64; // Screen update interval
    let mut last_mouse_pos: (u16, u16) = (0, 0); // To ignore move events if pos is not different
    let mut cleanup_timer = 0; // Interval for cleaning up orphaned states and callbacks
    let mut selected_widget = String::new(); // Currently selected widget
    let mut dragging: Option<String> = None; // Widget currently being dragged if any
    let mut last_dragging_pos = Coordinates::new(0, 0);
    loop {

        // We check for and deal with a possible event
        if poll(Duration::from_millis(tick_rate))? {

            // Get the event; it can only be consumed once
            let mut consumed = false;
            let mut event = read().unwrap();

            // Prevent mouse moved spam. if a mouse move event is detected, drain as many of those
            // events as possible before the next frame, then check if it moved position.
            if let Event::Mouse(mouse_event) = event {
                if let MouseEventKind::Moved = mouse_event.kind {
                    dragging = None;
                    let pos = (mouse_event.column, mouse_event.row);
                    while let Ok(true) = poll(Duration::from_millis(1)) {
                        let spam_event = read();
                        if let Ok(Event::Mouse(spam_mouse_event)) = spam_event {
                            if let MouseEventKind::Moved = spam_mouse_event.kind {
                                event = spam_event.unwrap();
                                if last_mouse_pos != pos {
                                    last_mouse_pos = pos;
                                    break
                                }
                            }
                        } else {
                            event = spam_event.unwrap();
                            break
                        }
                    }
                }
            }
            if let Event::Mouse(mouse_event) = event {
                if let MouseEventKind::Drag(_) = mouse_event.kind {
                    let pos = (mouse_event.column, mouse_event.row);
                    while let Ok(true) = poll(Duration::from_millis(1)) {
                        let spam_event = read();
                        if let Ok(Event::Mouse(spam_mouse_event)) = spam_event {
                            if let MouseEventKind::Drag(button) = spam_mouse_event.kind {
                                event = spam_event.unwrap();
                                if last_mouse_pos != pos {
                                    last_mouse_pos = pos;
                                    if button != MouseButton::Left { consumed = true }
                                    break
                                }
                            }
                        } else {
                            event = spam_event.unwrap();
                            break
                        }
                    }
                }
            } else {
                dragging = None;
            }

            // Modals get top priority in consuming events
            if !consumed {
                consumed = handle_modal_event(
                    event, &mut state_tree, &root_widget, &mut callback_tree, &mut scheduler);
            }

            // Try to handle event as a global event
            if !consumed {
                consumed = handle_global_event(
                    event, &mut state_tree, &root_widget, &mut callback_tree, &mut scheduler,
                    &mut selected_widget, &mut dragging, &mut last_dragging_pos);
            }
            // Try to let currently selected widget handle and consume the event
            if !consumed && !selected_widget.is_empty() &&
                !state_tree.get_by_path(&selected_widget).as_generic().get_disabled() {
                if let Some(widget) =
                root_widget.get_child_by_path(&selected_widget) {
                    consumed = widget.as_ez_object().handle_event(
                        event, &mut state_tree, &mut callback_tree, &mut scheduler);
                }
            }
            if !consumed {
                if let Event::Resize(width, height) = event {
                    let current_size = state_tree.get_by_path(&root_widget.path)
                        .as_generic().get_size();
                    if current_size.get_height() != height as usize ||
                        current_size.get_width() != width as usize {
                        handle_resize(&mut view_tree, &mut state_tree, &mut root_widget,
                                      width as usize, height as usize);
                        continue
                    }
                }
            }

        }

        // We only update the screen if the tick timer has elapsed
        if last_update.elapsed() < Duration::from_millis(tick_rate) { continue }

        // Update scheduler
        create_new_widgets(&mut scheduler, &mut root_widget, &mut state_tree, &mut callback_tree);
        selected_widget = handle_next_selection(&mut scheduler, &mut state_tree, &root_widget,
                                                &mut callback_tree, selected_widget);
        update_callback_configs(&mut scheduler, &mut callback_tree);
        run_tasks(&mut scheduler, &mut state_tree);
        update_threads(&mut scheduler, &mut state_tree);
        update_properties(&mut scheduler, &mut state_tree, &mut callback_tree);
        // Update root widget state as it might contain new modals it need to access internally
        root_widget.state = state_tree.get_by_path("/root").as_layout().clone();

        // Redraw individual widgets or the entire screen in case of forced_redraw. If the entire
        // Screen is redrawn individual widgets are not redrawn.
        let forced_redraw = if !scheduler.backend.force_redraw {
            redraw_changed_widgets(
                &mut view_tree, &mut state_tree,  &mut root_widget,
                &mut scheduler.backend.widgets_to_update,
                scheduler.backend.force_redraw)
        } else {
            true
        };
        if forced_redraw {
            let contents = root_widget.get_contents(&mut state_tree);
            view_tree.write_content(Coordinates::new(0, 0), contents);
        }
        write_to_screen(&mut view_tree);
        scheduler.backend.force_redraw = false;

        // Every now and then we perform cleanup of orphaned states (e.g. modals that were closed)
        // and their properties.
        if cleanup_timer == 100 {
            clean_trees(&mut root_widget, &mut state_tree, &mut callback_tree, &mut scheduler);
            drain_property_channels(&mut scheduler);
            cleanup_timer = 0;
        } else {
            cleanup_timer += 1;
        }
    }
}