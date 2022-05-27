//! # Common:
//! A module containing common static functions used by other modules, as well as common types.
use crossterm::style::{Color, PrintStyledContent, StyledContent};
use crossterm::{QueueableCommand, cursor, ExecutableCommand};
use std::io::{stdout, Write};
use std::collections::HashMap;
use crossterm::event::KeyCode;
use crate::scheduler::Scheduler;
use crate::widgets::layout::Layout;
use crate::states::state::{self, Padding};
use crate::widgets::widget::{EzWidget, EzObjects, Pixel, EzObject};


/// # Convenience types
/// ## Pixel maps:
/// Used to represent the visual content of widgets. Pixels are a wrapper around
/// Crossterm StyledContent, so PixelMaps are essentially a grid of StyledContent to display.
pub type PixelMap = Vec<Vec<Pixel>>;

/// ## Key map
/// A crossterm KeyCode > Callback function lookup. Used for custom user keybinds
pub type KeyMap = HashMap<KeyCode, KeyboardCallbackFunction>;

/// ## View tree:
/// Grid of StyledContent representing the entire screen currently being displayed. After each frame
/// an updated ViewTree is diffed to the old one, and only changed parts of the screen are updated.
pub type ViewTree = Vec<Vec<StyledContent<String>>>;

/// ## State tree:
/// A <WidgetPath, State> HashMap. The State contains all run-time information for a
/// widget, such as the text of a label, or whether a checkbox is currently checked. Callbacks
/// receive a mutable reference to the widget state and can change what they need. Then after each
/// frame the updated StateTree is diffed with the old one, and only changed widgets are redrawn.
pub type StateTree = HashMap<String, state::EzState>;

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
pub type MouseCallbackFunction = fn(EzContext, mouse_pos: state::Coordinates);

/// ## Generic Ez function:
/// Used for callbacks and scheduled tasks that don't require special parameter such as KeyCodes
/// or mouse positions. Used e.g. for [on_value_change] and [on_keyboard_enter].
pub type GenericEzFunction = fn(EzContext);

/// ## Generic Ez task:
/// Scheduled task implementation. Using FnMut allows users to capture variables in their scheduled
/// funcs.
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
pub fn get_widget_by_position<'a>(pos: state::Coordinates, widget_tree: &'a WidgetTree,
                                  state_tree: &StateTree) -> Option<&'a dyn EzWidget> {

    for (widget_path, state) in state_tree {
        if let state::EzState::Layout(_) = state { continue }
        if state.as_generic().collides(pos) {
            return Some(widget_tree.get(widget_path).unwrap().as_ez_widget())
        }
    }
    None
}


/// Write content to screen. Only writes differences between the passed view tree (current content)
/// and passed content (new content). The view tree is updated when changes are made.
pub fn write_to_screen(base_position: state::Coordinates, content: PixelMap, view_tree: &mut ViewTree) {
    stdout().execute(cursor::SavePosition).unwrap();
    for x in 0..content.len() {
        for y in 0..content[x].len() {
            let write_pos = (base_position.x + x, base_position.y + y);
            let write_content = content[x][y].get_pixel().clone();
            if write_pos.0 < view_tree.len() && write_pos.1 < view_tree[write_pos.0].len() &&
                view_tree[write_pos.0][write_pos.1] != write_content {
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

/// Create an empty view tree
pub fn initialize_view_tree(width: usize, height: usize) -> ViewTree {
    let mut view_tree = ViewTree::new();
    for x in 0..width {
        view_tree.push(Vec::new());
        for _ in 0..height {
            view_tree[x].push(Pixel::default().get_pixel())
        }
    }
    view_tree
}


/// Check each widget in a state tree for two things:
/// 1. If the state of the widget in the passed StateTree differs from the current widget state.
/// In this case the widget state should be updated with the new one, and the widget parent
/// should be redrawn.
/// 2. If its' state contains a forced redraw. In this case the entire screen will be rewritten,
/// and as such widgets will not be redrawn individually. Their state will still be updated.
pub fn update_state_tree(view_tree: &mut ViewTree, state_tree: &mut StateTree,
                         root_widget: &mut Layout) -> bool {
    let mut force_redraw = false;
    let mut widgets_to_redraw = Vec::new();
    for widget_path in state_tree.keys() {
        let state = state_tree.get(widget_path).unwrap();
        if state.as_generic().get_force_redraw() {
            force_redraw = true;
        }
        if state.as_generic().get_changed() {
            if widget_path == &root_widget.get_full_path() {
                root_widget.update_state(state);
                force_redraw = true;
            } else {
                let widget =
                    root_widget.get_child_by_path_mut(widget_path).unwrap().as_ez_object_mut();
                widget.update_state(state);
                if let state::EzState::Dropdown(i) = state {
                    // Dropdowns are a special case, we don't redraw their parent when they are
                    // dropped down because they overlap the parent. We don't redraw the parent
                    // afterwards either because a dropdown forces a global redraw when it retracts.
                    if state.as_dropdown().dropped_down {
                        widgets_to_redraw.push(widget.get_full_path());
                    } else {
                        widgets_to_redraw.push(widget_path.rsplit_once('/')
                            .unwrap().0.to_string()); // Redraw parent of changed widget
                    }
                } else {
                    widgets_to_redraw.push(widget_path.rsplit_once('/')
                        .unwrap().0.to_string()); // Redraw parent of changed widget
                }
            };
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
pub fn add_border(mut content: PixelMap, config: &state::BorderConfig) -> PixelMap {
    if content.is_empty() { return content }
    // Create border elements
    let horizontal_border = Pixel::new(config.horizontal_symbol.clone(),
                                      config.fg_color, config.bg_color);
    let vertical_border = Pixel::new(config.vertical_symbol.clone(),
                                    config.fg_color, config.bg_color);
    let top_left_border = Pixel::new(config.top_left_symbol.clone(),
                                    config.fg_color, config.bg_color);
    let top_right_border = Pixel::new(config.top_right_symbol.clone(),
                                     config.fg_color, config.bg_color);
    let bottom_left_border = Pixel::new(config.bottom_left_symbol.clone(),
                                       config.fg_color, config.bg_color);
    let bottom_right_border = Pixel::new(config.bottom_right_symbol.clone(),
                                        config.fg_color, config.bg_color);
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


/// Add padding around a PixelMap.
pub fn add_padding(mut content: PixelMap, padding: &Padding, bg_color: Color, fg_color: Color)
    -> PixelMap {

    if content.is_empty() {
        return content
    }
    let padding_pixel = Pixel::new(" ".to_string(), fg_color,
                                   bg_color);

    // Create vertical padding
    let mut vertical_padding = Vec::new();
    for _ in 0..content[0].len() {
        vertical_padding.push(padding_pixel.clone());
    }
    for _ in 0..padding.left {
        content.insert(0, vertical_padding.clone());
    }
    for _ in 0..padding.right {
        content.push(vertical_padding.clone());
    }
    if padding.top != 0 {
        for x in content.iter_mut() {
            for _ in 0..padding.top {
                x.insert(0, padding_pixel.clone());
            }
        }
    }
    if padding.bottom != 0 {
        for x in content.iter_mut() {
            for _ in 0..padding.bottom {
                x.push(padding_pixel.clone());
            }
        }
    }
    content
}


/// Align the passed content horizontally within a desired total width. Return the aligned
/// [PixelMap] and an offset for how much the content moved horizontally. E.g. content:
///
/// XXX...
///
/// With halign [state::HorizontalAlignment::Middle] and total width 5 would return:
///
/// .XXX.
///
/// With offset 1.
pub fn align_content_horizontally(mut content: PixelMap, halign: state::HorizontalAlignment,
                                  total_width: usize, fg_color: Color, bg_color: Color)
                                  -> (PixelMap, usize) {

    let empty_pixel = Pixel { symbol: " ".to_string(), foreground_color: fg_color,
        background_color: bg_color, underline: false};
    let mut offset = 0;
    for i in 0..total_width - content.len() {
        match halign {
            // Widgets are aligned left by default
            state::HorizontalAlignment::Left => {

            },
            // We align right by filling out empty space from the left
            state::HorizontalAlignment::Right => {
                content.insert(0, Vec::new());
                offset += 1;
                for _ in 0..content.last().unwrap().len() {
                    content.first_mut().unwrap().push(empty_pixel.clone());
                }
            },
            // We align in the center by filling out empty space alternating left and right
            state::HorizontalAlignment::Center => {
                if i % 2 == 0 {
                    content.push(Vec::new());
                    for _ in 0..content[0].len() {
                        content.last_mut().unwrap().push(empty_pixel.clone());
                    }
                } else {
                    content.insert(0, Vec::new());
                    offset += 1;
                    for _ in 0..content.last().unwrap().len() {
                        content.first_mut().unwrap().push(empty_pixel.clone());
                    }
                }
            }
        }
    }
    (content, offset)
}


    /// Align the passed content vertically within a desired total height. Return the aligned
    /// [PixelMap] and an offset for how much the content moved vertically. E.g. content:
    /// ```
    /// XXX
    /// ```
    /// With valign [state::VerticalAlignment::Middle] and total height 3 would return:
    /// ```
    ///
    /// XXX
    ///
    /// ````
    /// With offset 1.
pub fn align_content_vertically(mut content: PixelMap, valign: state::VerticalAlignment,
                                total_height: usize, fg_color: Color, bg_color: Color)
                                -> (PixelMap, usize){

    let empty_pixel = Pixel { symbol: " ".to_string(), foreground_color: fg_color,
        background_color: bg_color, underline: false};

    let mut offset = 0;
    for (i, x) in content.iter_mut().enumerate() {
        for j in 0..total_height - x.len() {
            match valign {
                // Widgets are aligned to top by default.
                state::VerticalAlignment::Top => {
                },
                // We align bottom by filling out empty space to the top
                state::VerticalAlignment::Bottom => {
                    x.insert(0, empty_pixel.clone());
                    if i == 0 {
                        offset += 1;
                    }
                },
                // We align in the middle by filling out empty space alternating top and bottom
                state::VerticalAlignment::Middle => {
                    if j % 2 == 0 {
                        x.push(empty_pixel.clone());
                    } else {
                        x.insert(0, empty_pixel.clone());
                        if i == 0 {
                            offset += 1;
                        }
                    }
                }
            }
        }
    }
    (content, offset)
}

// Make list of lines, splitting into lines at line breaks in the text or when the widget width
// has been exceeded. If the latter occurs, we will try to split on a word boundary if there is
// any in that chunk of text, to keep things readable.
pub fn wrap_text (mut text: String, width: usize) -> Vec<String> {

    let mut content_lines = Vec::new();
    loop {
        if width == 0 { break } // edge case: widget size 0
        if text.len() >= width {
            let peek = text[0..width].to_string();
            let lines: Vec<&str> = peek.lines().collect();
            // There's a line break in the sentence.
            if lines.len() > 1 {
                // Push all lines except the last one. If there's line breaks within a text
                // chunk that's smaller than widget width, we know for sure that all lines
                // within it fit as well. We don't push the last line because it might be part
                // of a larger sentence, and we're no longer filling full widget width.
                for line in lines[0..lines.len() - 1].iter() {
                    if line.is_empty() {
                        content_lines.push(' '.to_string());
                    } else {
                        content_lines.push(line.to_string());
                    }
                }
                if !lines.last().unwrap().is_empty() {
                    text = text[peek.rfind(lines.last().unwrap()).unwrap()..].to_string();
                } else {
                    text = text[width..].to_string();
                }
            }
            // Chunk naturally ends on word boundary, so just push the chunk.
            else if peek.ends_with(' ') {
                content_lines.push(peek);
                text = text[width..].to_string();
                // We can find a word boundary somewhere to split the string on. Push up until the
                // boundary.
            } else if let Some(index) = peek.rfind(' ') {
                content_lines.push(peek[..index].to_string());
                text = text[index+1..].to_string();
                // No boundaries at all, just push the entire chunk.
            } else {
                content_lines.push(peek);
                text = text[width..].to_string();
            }
            // Not enough content left to fill widget width. Just push entire text
        } else {
            content_lines.push(text);
            break
        }
    }
    content_lines
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