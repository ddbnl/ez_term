//! # Terminal:
//!
//! A module containing functions that interact with the terminal or draw things on screen.
use std::io::{stdout, Write};

use crossterm::{cursor::{self, Hide, Show}, event::{DisableMouseCapture, EnableMouseCapture}, ExecutableCommand, execute,
                QueueableCommand,
                Result,
                terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode}};
use crossterm::style::PrintStyledContent;

use crate::run::definitions::StateTree;
use crate::run::select::widget_is_hidden;
use crate::run::tree::ViewTree;
use crate::widgets::ez_object::EzObject;
use crate::widgets::layout::layout::Layout;


/// Prepare state of the terminal for drawing the UI.
pub fn initialize_terminal() -> Result<()> {

    enable_raw_mode()?;
    execute!(stdout(), EnableMouseCapture)?;
    stdout().execute(Hide)?;
    stdout().execute(Clear(ClearType::All))?;
    Ok(())
}


/// Set terminal to original state.
pub fn shutdown_terminal() -> Result<()>{

    stdout().queue(DisableMouseCapture)?.queue(Show)?.flush()?;
    stdout().execute(Clear(ClearType::All))?;
    disable_raw_mode()?;
    Ok(())
}


/// Write content to screen. Only writes differences between an frame and the new frame.
pub fn write_to_screen(view_tree: &mut ViewTree) {

    stdout().execute(cursor::SavePosition).unwrap();
    for (coord, content) in view_tree.get_changed() {
        stdout().queue(cursor::MoveTo(coord.x as u16, coord.y as u16)).unwrap()
            .queue(PrintStyledContent(content.clone())).unwrap();
    }
    stdout().flush().unwrap();
    stdout().execute(cursor::RestorePosition).unwrap();
    view_tree.clear_changed();
}


/// Redraw widgets to the ViewTree (ViewTree diffs are written to screen at each frame). If
/// a forced_redraw (global screen rewrite) is queued, widgets are not redrawn.
pub fn redraw_changed_widgets(view_tree: &mut ViewTree, state_tree: &mut StateTree,
                              root_widget: &mut Layout, changed_states: &mut Vec<String>,
                              mut force_redraw: bool) -> bool {

    if !state_tree.get_by_path("/root").as_layout().get_modals().is_empty() &&
        !changed_states.is_empty(){
        force_redraw = true;
    }
    if !force_redraw {
        redraw_widgets(changed_states, view_tree, state_tree, root_widget);
    }
    force_redraw
}


/// Redraw a list of widgets to a ViewTree.
pub fn redraw_widgets(paths: &mut Vec<String>, view_tree: &mut ViewTree,
                      state_tree: &mut StateTree, root_widget: &mut Layout) {

    'outer: while !paths.is_empty() {
        let mut widget_path= paths.pop().unwrap();
        widget_path = widget_path.rsplit_once('/').unwrap().0.to_string();
        if widget_path.is_empty() || widget_path == root_widget.path {
            root_widget.redraw(view_tree, state_tree);
        } else {
            if widget_is_hidden(widget_path.clone(), state_tree) {
                continue 'outer
            }
            // If the widget has infinite height or width then somewhere upstream it is
            // scrolled; we will find the origin of the scroll and redraw that widget instead
            // to keep the view intact.
            loop {
                let state = state_tree.get_by_path(&widget_path);
                if (!state.as_generic().get_size().get_infinite_width() &&
                    !state.as_generic().get_size().get_infinite_height()) || widget_path == "/root" {
                    break
                } else {
                    widget_path = widget_path.rsplit_once('/').unwrap().0.to_string()
                }
            }
            root_widget.get_child_by_path_mut(&widget_path).unwrap().as_ez_object_mut()
                .redraw(view_tree, state_tree);
        }
    }
}
