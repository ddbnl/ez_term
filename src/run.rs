//! # Run:
//! A module containing the functions to start and stop the main App run loop. The exposed "Run"
//! functions allows starting the app based on a root layout.
use std::io::{stdout};
use std::process::exit;
use std::time::{Duration};
use crossterm::{ExecutableCommand, execute, Result, cursor::{Hide},
                event::{MouseEvent, MouseEventKind, MouseButton, poll, read, DisableMouseCapture,
                        EnableMouseCapture, Event, KeyCode, KeyEvent},
                terminal::{disable_raw_mode, enable_raw_mode}};
use crate::common::{self, StateTree, ViewTree, WidgetTree};
use crate::widgets::layout::Layout;
use crate::widgets::widget::{EzObject, Pixel};


/// Set initial state of the terminal
fn initialize_terminal() -> Result<()> {

    enable_raw_mode()?;
    execute!(stdout(), EnableMouseCapture)?;
    stdout().execute(Hide)?;
    Ok(())
}


/// Set terminal to initial state before exit
fn shutdown_terminal() -> Result<()>{

    execute!(stdout(), DisableMouseCapture)?;
    //stdout().execute(terminal::Clear(terminal::ClearType::All))?;
    disable_raw_mode()?;
    Ok(())
}


/// # Call this to start the terminal app.
/// Make sure you load a root layout from a .ez file first and pass it to this func, like this:
/// ```
/// let mut root_widget = ez_parser::load_ez_ui("root.ez");
/// ```
/// After loading the root layout, make all the manual changes you require, such as setting
/// keybindings or binding callbacks to events. Then call this function.
pub fn run(root_widget: Layout) {
    initialize_terminal().unwrap();
    run_loop(root_widget).unwrap();
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
/// and a WidgetState object as the value. The WidgetState contains all important run time
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
fn run_loop(mut root_widget: Layout) -> Result<()>{

    // Create initial state and widget tree so we can pre-select the first widget before running
    let mut state_tree = root_widget.get_state_tree();
    let widget_tree = root_widget.get_widget_tree();
    common::select_next(&widget_tree, &mut state_tree);
    // Create the initial view tree
    let all_content = root_widget.get_contents();
    root_widget.propagate_absolute_positions();
    let mut view_tree = ViewTree::new();
    for x in 0..all_content.len() {
        view_tree.push(Vec::new());
        for _ in 0..all_content[0].len() {
            view_tree[x].push(Pixel::from_symbol("".to_string()).get_pixel())
        }
    }
    common::write_to_screen((0, 0), all_content, &mut view_tree);
    // Process the widget selection we made
    common::update_state_tree(&mut view_tree, &mut state_tree, &mut root_widget);
    // Start app
    loop {
        if poll(Duration::from_millis(1_000))? {
            let mut state_tree = root_widget.get_state_tree();
            let widget_tree = root_widget.get_widget_tree();

            // Get the event; it can only be consumed once, then the loop continues to next iter
            let event = read().unwrap();
            let mut consumed = false;

            let selected_widget =
                common::get_selected_widget(&widget_tree);
            // Focussed widgets get priority consuming an event
            if let Some(i) = selected_widget {
                consumed = i.get_focus() &&
                    i.handle_event(event, &mut view_tree, &mut state_tree, &widget_tree)
            }
            // Try to handle event as global bound event next
            if !consumed {
                match event {
                    Event::Key(key) => {
                        consumed = handle_key_event(key, &mut view_tree, &mut state_tree,
                                                    &widget_tree);
                    }
                    Event::Mouse(event) => {
                        consumed = handle_mouse_event(event, &mut view_tree, &mut state_tree,
                                                      &widget_tree);
                    }
                    _ => ()
                }
            }
            // Try to let currently selected widget handle and consume the event
            if !consumed {
                if let Some(i) = selected_widget {
                    let _ = i.handle_event(event, &mut view_tree, &mut state_tree, &widget_tree);
                };
            }
            // Update the state tree for each widget, redrawing any that changed. If a global
            // forced redraw was issued by a widget we'll perform one.
            let forced_redraw = common::update_state_tree(&mut view_tree, &mut state_tree,
                                                          &mut root_widget);
            if forced_redraw {
                common::write_to_screen((0, 0), root_widget.get_contents(),
                                        &mut view_tree);
            }
        } else { // Timeout expired, no event for 1s
        }
    }
}


/// Global key handler. If a key event matches one of these keys it will be consumed and not passed
/// on any further. The order for events is:
/// 1. Focussed widget
/// 2. Global key binds (this function)
/// 3. Selected widget
fn handle_key_event(key: KeyEvent, view_tree: &mut ViewTree,
                        state_tree: &mut StateTree, widget_tree: &WidgetTree) -> bool {

    match key.code {
        KeyCode::Down => {
            common::deselect_selected_widget(widget_tree, state_tree);
            common::select_next(widget_tree, state_tree);
            true
        },
        KeyCode::Up => {
            common::deselect_selected_widget(widget_tree, state_tree);
            common::select_previous(widget_tree, state_tree);
            true
        },
        KeyCode::Enter => {
            let selected_widget =
                common::get_selected_widget(widget_tree);
            if let Some(widget) = selected_widget {
                widget.on_keyboard_enter(widget.get_full_path(), view_tree, state_tree,
                                         widget_tree);
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
fn handle_mouse_event(event: MouseEvent, view_tree: &mut ViewTree,
                          state_tree: &mut StateTree, widget_tree: &WidgetTree) -> bool {

    if let MouseEventKind::Down(button) = event.kind {
        let mouse_position = (event.column as usize, event.row as usize);
        return match common::get_widget_by_position(mouse_position, widget_tree) {
            Some(widget) => {
                let abs = widget.get_absolute_position();
                let relative_position = (mouse_position.0 - abs.0,
                                         mouse_position.1 - abs.1);
                match button {
                    MouseButton::Left => {
                        common::deselect_selected_widget(widget_tree, state_tree);
                        widget.on_left_click(relative_position,
                                             view_tree, state_tree,
                                             widget_tree);
                        if widget.is_selectable() {
                            state_tree.get_mut(&widget.get_full_path()).unwrap()
                                .as_selectable_mut().set_selected(true);
                        }
                    },
                    MouseButton::Right => {
                        widget.on_right_click(relative_position, view_tree,
                                              state_tree, widget_tree);
                    }
                    _ => return false
                }
                true
            },
            None => false, // click outside of root widget bounds
        }
    }
    false
}