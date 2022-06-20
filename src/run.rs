//! # Run:
//! A module containing the functions to start and stop the main App run loop. The exposed "Run"
//! functions allows starting the app based on a root layout.
use std::io::{stdout, Write};
use std::process::exit;
use std::time::{Duration, Instant};
use crossterm::{ExecutableCommand, execute, Result, cursor::{Hide, Show},
                event::{MouseEvent, MouseEventKind, MouseButton, poll, read, DisableMouseCapture,
                        EnableMouseCapture, Event, KeyCode, KeyEvent},
                terminal::{disable_raw_mode, enable_raw_mode, self}, QueueableCommand};
use crate::common;
use crate::common::definitions::{CallbackTree, StateTree, ViewTree, WidgetTree, Coordinates};
use crate::widgets::layout::Layout;
use crate::widgets::widget::{EzObject};
use crate::scheduler::{Scheduler};
use crate::widgets::widget;


/// Set initial state of the terminal
fn initialize_terminal() -> Result<()> {

    enable_raw_mode()?;
    execute!(stdout(), EnableMouseCapture)?;
    stdout().execute(Hide)?;
    stdout().execute(terminal::Clear(terminal::ClearType::All))?;
    Ok(())
}


/// Set terminal to initial state before exit
fn shutdown_terminal() -> Result<()>{

    stdout().queue(DisableMouseCapture)?.queue(Show)?.flush()?;
    stdout().execute(terminal::Clear(terminal::ClearType::All))?;
    disable_raw_mode()?;
    Ok(())
}


pub fn stop() {
    shutdown_terminal().unwrap();
    exit(0);
}


/// # Call this to start the terminal app.
/// Make sure you load a root layout from a .ez file first and pass it to this func, like this:
/// ```
/// let mut root_widget = ez_parser::load_ez_ui("root.ez");
/// ```
/// After loading the root layout, make all the manual changes you require, such as setting
/// keybindings or binding callbacks to events. Then call this function.
pub fn run(root_widget: Layout, scheduler: Scheduler) {

    initialize_terminal().unwrap();
    let callback_tree = 
        common::screen_functions::initialize_callback_tree(&root_widget);
    run_loop(root_widget, callback_tree, scheduler).unwrap();
}


/// Called just before [run]. Creates initial view- and state trees and writes initial content
/// to the screen.
fn initialize_widgets(root_widget: &mut Layout) -> (ViewTree, StateTree) {

    // Get initial state tree, then convert all size_hints into actual sizes. After that we can
    // set absolute positions for all children as sizes are now known.
    let mut state_tree = common::screen_functions::initialize_state_tree(&root_widget);
    root_widget.set_child_sizes(&mut state_tree);
    let all_content = root_widget.get_contents(&mut state_tree);
    root_widget.propagate_absolute_positions(&mut state_tree);

    // Create an initial view tree so we can diff all future changes against it.
    let old_view_tree = common::screen_functions::initialize_view_tree(
        all_content.len(),all_content[0].len());
    let mut view_tree = old_view_tree.clone();
    common::screen_functions::write_to_view_tree(
        Coordinates::new(0, 0), all_content, &mut view_tree);
    common::screen_functions::write_to_screen(&old_view_tree, &view_tree);
    (view_tree, state_tree)

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
                } else if let MouseEventKind::Drag(i) = mouse_event.kind{
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
            else {common::selection_functions::get_selected_widget(&widget_tree, &mut state_tree)};

            // Focussed widgets get second-highest priority in consuming an event
            if !consumed {
                if let Some(i) = selected_widget {
                    consumed = i.get_focus() && i.handle_event(
                        event, &mut view_tree, &mut state_tree, &widget_tree,
                        &mut callback_tree, &mut scheduler);
                }
            }
            // Try to handle event as global
            if !consumed {
                consumed = handle_global_event(event, &mut view_tree, &mut state_tree, &widget_tree,
                                    &mut callback_tree, &mut scheduler);
            }
            // Try to let currently selected widget handle and consume the event
            if !consumed {
                if let Some(i) = selected_widget {
                    if !state_tree
                        .get(&i.get_full_path()).unwrap().as_generic().get_disabled() {
                        consumed = i.handle_event(
                            event, &mut view_tree, &mut state_tree, &widget_tree,
                            &mut callback_tree, &mut scheduler);

                    }
                };
            }
            if !consumed {
                if let Event::Resize(width, height) = event {
                    let current_size = state_tree.get(&root_widget.path).unwrap()
                        .as_generic().get_size();
                    if current_size.height != height as usize ||
                        current_size.width != width as usize {
                        view_tree = handle_resize(
                            &mut state_tree, &mut root_widget,
                            width as usize, height as usize, &mut scheduler);
                        continue
                    }
                }
            }

        }
        if last_update.elapsed() < Duration::from_millis(32) { continue }
        {
            let widget_tree = root_widget.get_widget_tree();
            scheduler.update_callback_configs(&mut callback_tree);
            scheduler.run_tasks(
                &mut view_tree, &mut state_tree, &widget_tree, &mut callback_tree);
            scheduler.update_threads(&mut view_tree, &mut state_tree, &widget_tree,
                                     &mut callback_tree);
            scheduler.update_properties(&mut state_tree);
        }
        root_widget.state = state_tree.get("/root").unwrap().as_layout().clone();
        common::screen_functions::clean_trees(
            &mut root_widget, &mut state_tree, &mut callback_tree);

        // Update the state tree for each widget, redrawing any that changed. If a global
        // forced redraw was issued by a widget we'll perform one.
        let old_view_tree = view_tree.clone();
        let forced_redraw = common::screen_functions::redraw_changed_widgets(
            &mut view_tree, &mut state_tree,  &mut root_widget,
            &mut scheduler.widgets_to_update, scheduler.force_redraw);
        if forced_redraw {
            let contents = root_widget.get_contents(&mut state_tree);
            common::screen_functions::write_to_view_tree(
                Coordinates::new(0, 0), contents,
                &mut view_tree);
        }
        common::screen_functions::write_to_screen(&old_view_tree, &view_tree);
        scheduler.force_redraw = false;

        track_mouse_pos = !root_widget.state.open_modals.is_empty();
    }
}

/// Try to handle an event by passing it to the active modal if any. The modal will return whether
/// it consumed the event or not.
fn handle_modal_event (event: Event, view_tree: &mut ViewTree, state_tree: &mut StateTree,
                       widget_tree: &WidgetTree, callback_tree: &mut CallbackTree,
                       scheduler: &mut Scheduler, root_widget: &Layout) -> bool {

    let mut consumed;
    if state_tree.get(&root_widget.path.clone()).unwrap().as_layout().open_modals.is_empty() {
        return false
    }
    let modal_root = state_tree.get(&root_widget.path.clone()).unwrap().as_layout()
        .open_modals.first().unwrap().as_ez_object().get_full_path();
    for (path, widget) in widget_tree {
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
fn handle_global_event(event: Event, view_tree: &mut ViewTree, state_tree: &mut StateTree,
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
/// on any further. The order for events is:
/// 1. Focussed widget
/// 2. Global key binds (this function)
/// 3. Selected widget
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
                    .get(&widget.get_full_path()).unwrap().as_generic().get_disabled() {
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
/// passed on any further. The order for events is:
/// 1. Focussed widget
/// 2. Global key binds (this function)
/// 3. Selected widget
fn handle_mouse_event(event: MouseEvent, view_tree: &mut ViewTree, state_tree: &mut StateTree,
                      widget_tree: &WidgetTree, callback_tree: &mut CallbackTree, 
                      scheduler: &mut Scheduler) -> bool {

    if let MouseEventKind::Down(button) = event.kind {
        let mouse_position = Coordinates::new(
            event.column as usize,event.row as usize);
        for widget in common::selection_functions::get_widget_by_position(
            mouse_position, widget_tree, state_tree) {

            let abs = state_tree.get(&widget.get_full_path()).unwrap()
                .as_generic()
                .get_absolute_position();
            let relative_position = Coordinates::new(
                mouse_position.x - abs.x, mouse_position.y - abs.y);
            let consumed = match button {
                MouseButton::Left => {
                    common::selection_functions::deselect_selected_widget(
                        view_tree, state_tree, widget_tree, callback_tree, scheduler);

                    let consumed = widget.on_left_mouse_click(view_tree, state_tree, widget_tree,
                                               callback_tree, scheduler,relative_position);
                    if consumed && state_tree.get(&widget.get_full_path()).unwrap().as_generic()
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
    }
    false
}


/// Handle a resize event by setting the size of the root widget to the new window size, updating
/// the sizes/positions of all children and generating a new view tree of the right size.
fn handle_resize(state_tree: &mut StateTree, root_widget: &mut Layout,
                 new_width: usize, new_height: usize, scheduler: &mut Scheduler) -> ViewTree{

    let state = state_tree.get_mut(&root_widget.path).unwrap()
        .as_generic_mut();
    state.get_size_mut().width = new_width as usize;
    state.get_size_mut().height = new_height as usize;
    state.update(scheduler);
    let old_view_tree = common::screen_functions::initialize_view_tree(
        new_width, new_height);
    root_widget.set_child_sizes(state_tree);
    let contents = root_widget.get_contents(state_tree);
    root_widget.propagate_absolute_positions(state_tree);
    // Cleartype purge is tempting but causes issues on at least Windows
    stdout().queue(terminal::Clear(terminal::ClearType::All)).unwrap();
    let mut view_tree = old_view_tree.clone();
    common::screen_functions::write_to_view_tree(
        Coordinates::new(0, 0), contents,
        &mut view_tree);
    common::screen_functions::write_to_screen(&old_view_tree, &view_tree);
    old_view_tree
}
