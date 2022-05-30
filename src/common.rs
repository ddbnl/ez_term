//! # Common:
//! A module containing common static functions used by other modules, as well as common types.
use crossterm::style::{Color, PrintStyledContent, StyledContent};
use crossterm::{QueueableCommand, cursor, ExecutableCommand};
use std::io::{stdout, Write};
use std::collections::HashMap;
use crossterm::event::KeyCode;
use crate::ez_parser::EzWidgetDefinition;
use crate::scheduler::Scheduler;
use crate::widgets::layout::Layout;
use crate::states::state::{self, EzState};
use crate::widgets::widget::{EzWidget, EzObjects, Pixel, EzObject};


/// # Convenience types
/// ## Pixel maps:
/// Used to represent the visual content of widgets. Pixels are a wrapper around
/// Crossterm StyledContent, so PixelMaps are essentially a grid of StyledContent to display.
pub type PixelMap = Vec<Vec<Pixel>>;

/// ## Key map
/// A crossterm KeyCode > Callback function lookup. Used for custom user keybinds
pub type KeyMap = HashMap<KeyCode, KeyboardCallbackFunction>;

/// ## Templates
/// A hashmap of 'Template Name > [EzWidgetDefinition]'. Used to instantiate widget templates
/// at runtime. E.g. when spawning popups.
pub type Templates = HashMap<String, EzWidgetDefinition>;

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
pub type KeyboardCallbackFunction = Box<dyn FnMut(EzContext, KeyCode)>;

/// ## Mouse callback function:
/// This is used for binding mouse event callbacks to widgets, meaning that any callback functions
/// user makes should use this signature.
pub type MouseCallbackFunction = Box<dyn FnMut(EzContext, state::Coordinates)>;

/// ## Optional mouse callback function:
/// This is used for callbacks that may or may not have been initiated by mouse. 'on_select' uses
/// this for example, because a widget may have been selected by mouse, or maybe by keyboard.
pub type OptionalMouseCallbackFunction = Box<dyn FnMut(EzContext, Option<state::Coordinates>)>;

/// ## Generic Ez function:
/// Used for callbacks and scheduled tasks that don't require special parameter such as KeyCodes
/// or mouse positions. Used e.g. for [on_value_change] and [on_keyboard_enter].
pub type GenericEzFunction = Box<dyn FnMut(EzContext)>;

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

/// Find a widget by a screen position coordinate. Used e.g. by mouse event handlers. If a modal
/// if active only the modal is searched.
pub fn get_widget_by_position<'a>(pos: state::Coordinates, widget_tree: &'a WidgetTree,
                                  state_tree: &StateTree) -> Option<&'a dyn EzWidget> {

    let modals = state_tree.get("/root").unwrap().as_layout().get_modals();
    let path_prefix = if modals.is_empty() {
        "/root".to_string()
    } else {
        modals.first().unwrap().as_ez_object().get_full_path()
    };
    for (widget_path, state) in state_tree {
        if !widget_path.starts_with(&path_prefix) { continue }
        if let EzState::Layout(_) = state { continue }
        if state.as_generic().collides(pos) {
            return Some(widget_tree.get(widget_path).unwrap().as_ez_widget())
        }
    }
    None
}


/// Write content to screen. Only writes differences between the passed view tree (current content)
/// and passed content (new content). The view tree is updated when changes are made.
pub fn write_to_screen(base_position: state::Coordinates, content: PixelMap,
                       view_tree: &mut ViewTree, state_tree: &StateTree, protect_modal: bool) {
    stdout().execute(cursor::SavePosition).unwrap();

    let root_state = state_tree.get("/root").unwrap().as_layout();
    // Get list of coordinates for the open modal if any so we can exclude those for redraw
    let modal_coords = if protect_modal && !root_state.open_modals.is_empty() {
        println!("AAL {:?}", root_state.open_modals.first().unwrap()
            .as_ez_object().get_state().as_generic().get_box_coords());
        root_state.open_modals.first().unwrap()
            .as_ez_object().get_state().as_generic().get_box_coords()
    } else {
        Vec::new()
    };
    for x in 0..content.len() {
        for y in 0..content[x].len() {
            let write_pos = state::Coordinates::new(base_position.x + x, base_position.y + y);
            let write_content = content[x][y].get_pixel().clone();
            if modal_coords.contains(&write_pos) { continue}
            if write_pos.x < view_tree.len() && write_pos.y < view_tree[write_pos.x].len() &&
                view_tree[write_pos.x][write_pos.y] != write_content {
                view_tree[write_pos.x][write_pos.y] = write_content.clone();
                stdout().queue(cursor::MoveTo(
                    write_pos.x as u16, write_pos.y as u16)).unwrap()
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





/// Check each widget state tree for two things:
/// 1. If the state of the widget in the passed StateTree differs from the current widget state.
/// In this case the widget state should be updated with the new one, and the widget should be
/// redrawn.
/// 2. If the state of the widget contains a forced redraw. In this case the entire screen will
/// be redrawn, and widgets will not be redrawn individually. Their state will still be updated.
pub fn redraw_changed_widgets(view_tree: &mut ViewTree, state_tree: &mut StateTree,
                              root_widget: &mut Layout) -> bool {

    let mut force_redraw = false;
    // We separate widgets and modals to redraw because modals need to be drawn last
    let mut widgets_to_redraw = Vec::new();
    let mut modals_to_redraw = Vec::new();

    for (widget_path, state) in state_tree.iter_mut() {
        let generic_state = state.as_generic_mut();
        if generic_state.get_force_redraw() {
            force_redraw = true;
            generic_state.set_force_redraw(false);
        }
        if generic_state.get_changed() {
            // Check overlap between widget and modal. Redraw modal if there's any.
            generic_state.set_changed(false);
            if !widget_path.starts_with("/modal") {
                let parent_path = widget_path.rsplit_once('/').unwrap().0;
                widgets_to_redraw.push(parent_path.to_string());
                force_redraw = true;
            } else{
                modals_to_redraw.push(widget_path.to_string());
            }
        }
    }
    let widget_tree = root_widget.get_widget_tree();
    if !force_redraw {
        for widget_path in widgets_to_redraw.iter() {
            if widget_path.is_empty() || widget_path == &root_widget.path {
                root_widget.redraw(view_tree, state_tree, &widget_tree, false);
            } else {
                root_widget.get_child_by_path_mut(widget_path).unwrap().as_ez_object_mut()
                    .redraw(view_tree, state_tree, &widget_tree, true);
            }
        }
        for widget_path in modals_to_redraw.iter() {
            let root_state = state_tree.get_mut("/root").unwrap().as_layout_mut();
            for modal in root_state.open_modals.iter_mut() {
                if &modal.as_ez_object().get_full_path() == widget_path {
                    modal.as_ez_object().redraw(view_tree, state_tree, &widget_tree, false);
                }
            }
        }
    }
    force_redraw
}


/// Get the State for each child [EzWidget] and return it in a <[path], [State]> HashMap.
pub fn get_state_tree(root_layout: &Layout) -> StateTree {
    let mut state_tree = HashMap::new();
    for (child_path, child) in root_layout.get_widgets_recursive() {
        state_tree.insert(child_path, child.as_ez_object().get_state());
    }
    state_tree.insert(root_layout.get_full_path(), root_layout.get_state());
    state_tree
}


/// Return the widget that is currently selected. Can be none.
pub fn get_selected_widget<'a>(widget_tree: &'a WidgetTree, state_tree: &mut StateTree)
    -> Option<&'a dyn EzWidget> {
    for widget in widget_tree.values() {
        if let EzObjects::Layout(_) = widget { continue }  // Layouts cannot be selected
        let generic_widget = widget.as_ez_widget();
        if generic_widget.is_selectable() && state_tree.get(&generic_widget.get_full_path())
            .unwrap().as_selectable().get_selected() {
            return Some(generic_widget)
        }
    }
    None
}


/// If any widget is currently selected, deselect it. Can always be called safely.
pub fn deselect_selected_widget(view_tree: &mut ViewTree, state_tree: &mut StateTree,
        widget_tree: &WidgetTree, scheduler: &mut Scheduler) {
    let selected_widget = get_selected_widget(widget_tree, state_tree);
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
    let current_selection = get_selected_widget(widget_tree, state_tree);

    let mut current_order = if let Some(i) = current_selection {
        state_tree.get_mut(&i.get_full_path()).unwrap().as_selectable_mut().set_selected(false);
        i.get_selection_order()
    } else {
        0
    };
    let result = find_next_selection(current_order, widget_tree);
    if let Some( next_widget) = result {
        let context = EzContext::new(next_widget.clone(), view_tree,
                                     state_tree, widget_tree, scheduler);
        widget_tree.get(&next_widget).unwrap().as_ez_widget().on_select(context,None);
    } else  {
        current_order = 0;
        let result = find_next_selection(current_order, widget_tree);
        if let Some( next_widget) = result {
            let context = EzContext::new(next_widget.clone(), view_tree,
                                         state_tree, widget_tree, scheduler);
            widget_tree.get(&next_widget).unwrap().as_ez_widget()
                .on_select(context,None);
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

    let current_selection = get_selected_widget(widget_tree, state_tree);
    let mut current_order = if let Some(i) = current_selection {
        state_tree.get_mut(&i.get_full_path()).unwrap().as_selectable_mut().set_selected(false);
        i.get_selection_order()
    } else {
        0
    };
    let result = find_previous_selection(current_order, widget_tree);
    if let Some( previous_widget) = result {

        let context = EzContext::new(previous_widget.clone(), view_tree,
                                     state_tree, widget_tree, scheduler);
        widget_tree.get(&previous_widget).unwrap()
            .as_ez_widget().on_select(context,None);
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

pub fn resize_with_size_hint(state: &mut EzState, parent_width: usize, parent_height: usize) {

    let mut_state = state.as_generic_mut();
    if let Some(size_hint_x) = mut_state.get_size_hint().x {
        let raw_child_size = parent_width as f64 * size_hint_x;
        let child_size = raw_child_size.round() as usize;
        mut_state.set_width(child_size);
    }

    if let Some(size_hint_y) = mut_state.get_size_hint().y {
        let raw_child_size = parent_height as f64 * size_hint_y;
        let child_size = raw_child_size.round() as usize;
        mut_state.set_height(child_size);
    }
}


/// Set the positions of children that use pos_hint(s) using own proportions and position.
pub fn reposition_with_pos_hint(parent_width: usize, parent_height: usize,
                                child_state: &mut dyn state::GenericState) {

    // Set x by pos_hint if any
    if let Some((keyword, fraction)) = child_state.get_pos_hint().x {
        let initial_pos = match keyword {
            state::HorizontalPositionHint::Left => 0,
            state::HorizontalPositionHint::Right => parent_width - child_state.get_size().width,
            state::HorizontalPositionHint::Center =>
                (parent_width as f64 / 2.0).round() as usize -
                    (child_state.get_size().width as f64 / 2.0).round() as usize,
        };
        let x = (initial_pos as f64 * fraction).round() as usize;
        child_state.set_x(x);
    }
    // Set y by pos hint if any
    if let Some((keyword, fraction)) = child_state.get_pos_hint().y {
        let initial_pos = match keyword {
            state::VerticalPositionHint::Top => 0,
            state::VerticalPositionHint::Bottom => parent_height - child_state.get_size().height,
            state::VerticalPositionHint::Middle =>
                (parent_height as f64 / 2.0).round() as usize -
                    (child_state.get_size().height as f64 / 2.0).round() as usize,
        };
        let y = (initial_pos as f64 * fraction).round() as usize;
        child_state.set_y(y);
    }
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
pub fn add_padding(mut content: PixelMap, padding: &state::Padding, bg_color: Color, fg_color: Color)
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


pub fn open_popup(template: String, state_tree: &mut StateTree) -> String {
    let (path, sub_tree) = state_tree.get_mut("/root").unwrap().as_layout_mut()
        .open_popup(template);
    state_tree.extend(sub_tree);
    path
}