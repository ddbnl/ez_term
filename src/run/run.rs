//! # Run:
//! A module containing the functions to start and stop the main App run loop. The exposed "Run"
//! functions allows starting the app based on a root layout.
use std::process::exit;
use std::time::{Duration, Instant};
use crossterm::{Result, event::{MouseEventKind, poll, read, Event}};
use crate::CallbackConfig;
use crate::run::definitions::{CallbackTree, Coordinates, StateTree};
use crate::run::select::get_selected_widget;
use crate::run::terminal::{redraw_changed_widgets, write_to_screen};
use crate::run::tree::{clean_trees, initialize_callback_tree, initialize_state_tree, ViewTree};
use super::terminal::{initialize_terminal, shutdown_terminal};
use super::input::{handle_resize, handle_modal_event, handle_global_event};
use crate::states::ez_state::GenericState;
use crate::widgets::layout::layout::Layout;
use crate::widgets::ez_object::{EzObject, EzObjects};
use crate::scheduler::scheduler::Scheduler;
use crate::scheduler::scheduler_funcs;
use crate::scheduler::scheduler_funcs::{run_tasks, update_callback_configs, update_properties,
                                        update_threads};


/// # Call this to start the terminal app.
/// Make sure you load a root layout from a .ez file first and pass it to this func, like this:
/// ```
/// let mut root_widget = ez_parser::load_ez_ui("root.ez");
/// ```
/// After loading the root layout, make all the manual changes you require, such as setting
/// keybindings or binding callbacks to events. Then call this function.
pub fn run(root_widget: Layout, scheduler: Scheduler) {

    initialize_terminal().unwrap();
    let callback_tree = initialize_callback_tree(&root_widget);
    run_loop(root_widget, callback_tree, scheduler).unwrap();
}


pub fn stop() {
    shutdown_terminal().unwrap();
    exit(0);
}


/// Called just before [run]. Creates initial view- and state trees and writes initial content
/// to the screen.
fn initialize_widgets(root_widget: &mut Layout) -> (ViewTree, StateTree) {

    // Get initial state tree, then convert all size_hints into actual sizes. After that we can
    // set absolute positions for all children as sizes are now known.
    let mut state_tree = initialize_state_tree(&root_widget);
    root_widget.set_child_sizes(&mut state_tree);
    let all_content = root_widget.get_contents(&mut state_tree);
    root_widget.propagate_absolute_positions(&mut state_tree);

    // Create an initial view tree so we can diff all future changes against it.
    let mut view_tree = ViewTree::default();
    view_tree.initialize(root_widget.state.size.width.value,
                         root_widget.state.size.height.value);
    view_tree.write_content(Coordinates::new(0, 0), all_content);
    write_to_screen(&mut view_tree);
    (view_tree, state_tree)

}


pub fn open_popup(template: String, state_tree: &mut StateTree,
                  scheduler: &mut Scheduler) -> String {

    let state = state_tree.get_by_path_mut("/root").as_layout_mut();
    state.update(scheduler);
    let (path, sub_tree) = state.open_popup(template, scheduler);
    state_tree.extend(sub_tree);
    let state = state_tree.get_by_path_mut("/root").as_layout_mut();
    scheduler.set_callback_config(path.as_str(),
                                  CallbackConfig::default());
    let modal = state.open_modals.first().unwrap();
    if let EzObjects::Layout(ref i) = modal {
        for sub_widget in i.get_widgets_recursive().values() {
            scheduler.set_callback_config(
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
/// (i.e. static things). EzWidget enums can be downcast to UxObject or EzWidget trait objects to
/// access common functions, or downcast to their specific widget type if you know for sure what it
/// is.
fn run_loop(mut root_widget: Layout, mut callback_tree: CallbackTree, mut scheduler: Scheduler) 
    -> Result<()>{

    let (mut view_tree, mut state_tree) = initialize_widgets(&mut root_widget);
    let last_update = Instant::now();
    let mut last_mouse_pos: (u16, u16) = (0, 0);
    let mut track_mouse_pos = false;
    let mut cleanup_timer = 0;
    loop {

        // Now we check for and deal with a possible event
        if poll(Duration::from_millis(32))? {

            // Get the event; it can only be consumed once
            let mut consumed = false;
            let mut event = read().unwrap();

            // Prevent mouse moved spam. if a mouse move event is detected, drain as many of those
            // events as possible before the next frame, then check if it moved position.
            if let Event::Mouse(mouse_event) = event {
                if let MouseEventKind::Moved = mouse_event.kind {
                    if !track_mouse_pos { continue }
                    let mut pos = (mouse_event.column, mouse_event.row);
                    while let Ok(true) = poll(Duration::from_millis(1)) {
                        let spam_event = read();
                        if let Ok(Event::Mouse(spam_mouse_event)) = spam_event {
                            if let MouseEventKind::Moved = spam_mouse_event.kind {
                                pos = (spam_mouse_event.column, spam_mouse_event.row);
                                event = spam_event.unwrap();
                            }
                        } else {
                            event = spam_event.unwrap();
                            break
                        }
                    }
                    if last_mouse_pos != pos {
                        last_mouse_pos = pos;
                    } else {
                        consumed = true;
                    }
                } else if let MouseEventKind::Drag(_) = mouse_event.kind{
                    continue
                }
            }

            let widget_tree = root_widget.get_widget_tree();
            // Modals get top priority in consuming events
            if !consumed {
                consumed = handle_modal_event(
                    event, &mut view_tree, &mut state_tree, &widget_tree, &mut callback_tree,
                    &mut scheduler, &root_widget);
            }

            let selected_widget = if consumed {None}
            else {get_selected_widget(&widget_tree, &mut state_tree)};

            // Try to handle event as global
            if !consumed {
                consumed = handle_global_event(event, &mut view_tree, &mut state_tree, &widget_tree,
                                    &mut callback_tree, &mut scheduler);
            }
            // Try to let currently selected widget handle and consume the event
            if !consumed {
                if let Some(i) = selected_widget {
                    if !state_tree.get_by_path(&i.get_full_path())
                        .as_generic().get_disabled().value {
                        consumed = i.handle_event(
                            event, &mut view_tree, &mut state_tree, &widget_tree,
                            &mut callback_tree, &mut scheduler);

                    }
                };
            }
            if !consumed {
                if let Event::Resize(width, height) = event {
                    let current_size = state_tree.get_by_path(&root_widget.path)
                        .as_generic().get_size();
                    if current_size.height != height as usize ||
                        current_size.width != width as usize {
                        handle_resize(&mut view_tree, &mut state_tree, &mut root_widget,
                                      width as usize, height as usize);
                        continue
                    }
                }
            }

        }
        if last_update.elapsed() < Duration::from_millis(32) { continue }
        {
            let widget_tree = root_widget.get_widget_tree();
            update_callback_configs(&mut scheduler, &mut callback_tree);
            run_tasks(&mut scheduler, &mut view_tree, &mut state_tree, &widget_tree);
            update_threads(&mut scheduler, &mut view_tree, &mut state_tree, &widget_tree);
            update_properties(&mut scheduler, &mut view_tree, &mut state_tree, &widget_tree,
                              &mut callback_tree);
        }
        root_widget.state = state_tree.get_by_path("/root").as_layout().clone();

        // Update the state tree for each widget, redrawing any that changed. If a global
        // forced redraw was issued by a widget we'll perform one.
        let forced_redraw = redraw_changed_widgets(
            &mut view_tree, &mut state_tree,  &mut root_widget,
            &mut scheduler.widgets_to_update, scheduler.force_redraw);
        if forced_redraw {
            let contents = root_widget.get_contents(&mut state_tree);
            view_tree.write_content(Coordinates::new(0, 0), contents);
        }
        write_to_screen(&mut view_tree);
        scheduler.force_redraw = false;


        track_mouse_pos = !root_widget.state.open_modals.is_empty();
        if cleanup_timer == 100 {
            clean_trees(&mut root_widget, &mut state_tree, &mut callback_tree, &mut scheduler);
            scheduler_funcs::drain_property_channels(&mut scheduler);
            cleanup_timer = 0;
        } else {
            cleanup_timer += 1;
        }
    }
}