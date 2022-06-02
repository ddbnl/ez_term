//! # Dropdown Widget
//! Widget which supports and arbitrary amount of possible values of which one can be chosen at any
//! time. The active value is always displayed, and when selected drops down all other possible
//! values for the user to select.
use crossterm::event::{Event, KeyCode, MouseButton, MouseEventKind};
use crate::common;
use crate::common::{CallbackTree, StateTree, ViewTree, WidgetTree};
use crate::states::dropdown_state::{DropdownState, DroppedDownMenuState};
use crate::states::state::{self, AutoScale, EzState, GenericState, PosHint, Size, SizeHint};
use crate::widgets::widget::{self, EzObject};
use crate::ez_parser;
use crate::scheduler::Scheduler;


#[derive(Default, Clone)]
pub struct Dropdown {

    /// ID of the widget, used to construct [path]
    pub id: String,

    /// Full path to this widget, e.g. "/root_layout/layout_2/THIS_ID"
    pub path: String,

    /// Runtime state of this widget, see [DropdownState] and [State]
    pub state: DropdownState,
}


impl widget::EzObject for Dropdown {

    fn load_ez_parameter(&mut self, parameter_name: String, parameter_value: String) {

        match parameter_name.as_str() {
            "id" => self.set_id(parameter_value.trim().to_string()),
            "x" => self.state.set_x(parameter_value.trim().parse().unwrap()),
            "y" => self.state.set_y(parameter_value.trim().parse().unwrap()),
            "pos" => self.state.set_position(
                ez_parser::load_pos_parameter(parameter_value.trim())),
            "size_hint_x" => self.state.set_size_hint_x(
                ez_parser::load_size_hint_parameter(parameter_value.trim())),
            "pos_hint" => self.state.set_pos_hint(
                ez_parser::load_full_pos_hint_parameter(parameter_value.trim())),
            "pos_hint_x" => self.state.set_pos_hint_x(
                ez_parser::load_pos_hint_x_parameter(parameter_value.trim())),
            "pos_hint_y" => self.state.set_pos_hint_y(
                ez_parser::load_pos_hint_y_parameter(parameter_value.trim())),
            "auto_scale_width" =>
                self.state.set_auto_scale_width(
                    ez_parser::load_bool_parameter(parameter_value.trim())),
            "width" => self.state.get_size_mut().width = parameter_value.trim().parse().unwrap(),
            "padding" => self.state.set_padding(
                ez_parser::load_full_padding_parameter(parameter_value.trim())),
            "padding_x" => self.state.set_padding(
                ez_parser::load_padding_x_parameter(parameter_value.trim())),
            "padding_y" => self.state.set_padding(
                ez_parser::load_padding_y_parameter(parameter_value.trim())),
            "padding_top" =>
                self.state.set_padding_top(parameter_value.trim().parse().unwrap()),
            "padding_bottom" =>
                self.state.set_padding_bottom(parameter_value.trim().parse().unwrap()),
            "padding_left" =>
                self.state.set_padding_left(parameter_value.trim().parse().unwrap()),
            "padding_right" =>
                self.state.set_padding_right(parameter_value.trim().parse().unwrap()),
            "halign" =>
                self.state.halign =  ez_parser::load_halign_parameter(parameter_value.trim()),
            "valign" =>
                self.state.valign =  ez_parser::load_valign_parameter(parameter_value.trim()),
            "selection_order" => {
                self.state.selection_order = ez_parser::load_selection_order_parameter(
                    parameter_value.as_str());
            },
            "border" => self.state.border_config.enabled =
                ez_parser::load_bool_parameter(parameter_value.trim()),
            "border_horizontal_symbol" => self.state.border_config.horizontal_symbol =
                parameter_value.trim().to_string(),
            "border_vertical_symbol" => self.state.border_config.vertical_symbol =
                parameter_value.trim().to_string(),
            "border_top_right_symbol" => self.state.border_config.top_right_symbol =
                parameter_value.trim().to_string(),
            "border_top_left_symbol" => self.state.border_config.top_left_symbol =
                parameter_value.trim().to_string(),
            "border_bottom_left_symbol" => self.state.border_config.bottom_left_symbol =
                parameter_value.trim().to_string(),
            "border_bottom_right_symbol" => self.state.border_config.bottom_right_symbol =
                parameter_value.trim().to_string(),
            "border_fg_color" =>
                self.state.border_config.fg_color =
                    ez_parser::load_color_parameter(parameter_value),
            "border_bg_color" =>
                self.state.border_config.bg_color =
                    ez_parser::load_color_parameter(parameter_value),
            "fg_color" =>
                self.state.colors.foreground =
                    ez_parser::load_color_parameter(parameter_value),
            "bg_color" =>
                self.state.colors.background =
                    ez_parser::load_color_parameter(parameter_value),
            "selection_fg_color" =>
                self.state.colors.selection_foreground =
                    ez_parser::load_color_parameter(parameter_value),
            "selection_bg_color" =>
                self.state.colors.selection_background =
                    ez_parser::load_color_parameter(parameter_value),
            "allow_none" =>
                self.state.allow_none =
                    ez_parser::load_bool_parameter(parameter_value.trim()),
            "options" => {
                self.state.options = parameter_value.split(',')
                    .map(|x| x.trim().to_string()).collect();
            },
            "active" => {
                self.state.choice = parameter_value.trim().to_string();
            }
            _ => panic!("Invalid parameter name for dropdown {}", parameter_name)
        }
    }

    fn set_id(&mut self, id: String) { self.id = id }

    fn get_id(&self) -> String { self.id.clone() }

    fn set_full_path(&mut self, path: String) { self.path = path }

    fn get_full_path(&self) -> String { self.path.clone() }

    fn get_state(&self) -> EzState { EzState::Dropdown(self.state.clone()) }

    /// Content of this widget depends on whether it is currently dropped down or not. If not,
    /// then display a label with a border representing the currently selected value. If dropped
    /// down show a list of all options, with the currently selected one on top.
    fn get_contents(&self, state_tree: &mut common::StateTree) -> common::PixelMap {

        let state =
            state_tree.get_mut(&self.get_full_path()).unwrap().as_dropdown_mut();
        // If dropped down get full content instead
        // Set a default value if user didn't give one
        let mut active = state.choice.clone();
        if active.is_empty() && !state.allow_none {
            active = state.options.first()
                .expect("Dropdown widget must have at least one option").to_string(); // todo move to validation
        }
        // Create a bordered label representing currently active value
        let fg_color = if state.selected {state.get_color_config().selection_foreground }
        else {state.get_color_config().foreground };
        let bg_color = if state.selected {state.get_color_config().selection_background }
        else {state.get_color_config().background };
        let mut text = active.chars().rev().collect::<String>();
        let mut contents = Vec::new();

        let write_width = if state.get_effective_size().infinite_width { text.len() }
                                else {state.get_effective_size().width };

        for _ in 0..write_width {
            let mut new_y = Vec::new();
            if !text.is_empty() {
                new_y.push(widget::Pixel::new(text.pop().unwrap().to_string(),
                    fg_color, bg_color));
            } else {
                new_y.push(widget::Pixel::new(" ".to_string(),fg_color,
                                      bg_color));
            }
            contents.push(new_y);
        }
        contents = common::add_border(contents, state.get_border_config());
        state.set_effective_height(1);
        contents
    }


    /// Called when the widgets is not dropped down and enter/left mouse click occurs on it.
    fn on_press(&self, _view_tree: &mut ViewTree, state_tree: &mut StateTree,
                _widget_tree: &WidgetTree, _callback_tree: &mut CallbackTree,
                _scheduler: &mut Scheduler) {

        let state = state_tree.get(&self.get_full_path()).unwrap()
            .as_dropdown();
        let modal_id = format!("{}_modal", self.get_id());
        let modal_path = format!("/modal/{}", modal_id);
        let new_modal_state = DroppedDownMenuState {
            size: Size::new(state.get_size().width, state.total_options() + 2),
            auto_scale: AutoScale::new(false, false),
            options: state.get_options(),
            allow_none: state.allow_none,
            size_hint: SizeHint::new(None, None),
            position: state.get_absolute_position(),
            padding: state::Padding::new(0, 0, 0, 0),
            absolute_position: state.get_absolute_position(),
            pos_hint: PosHint::new(None, None),
            dropped_down_selected_row: 1,
            border_config: state.border_config.clone(),
            colors: state.colors.clone(),
            changed: false,
            choice: state.get_choice(),
            parent_path: self.path.clone(),
            force_redraw: false
        };
        let new_modal = DroppedDownMenu {
            id: modal_id,
            path: modal_path.clone(),
            state: new_modal_state,
        };
        let root_state = state_tree.get_mut("/root").unwrap();
        let (_, extra_state_tree) = root_state.as_layout_mut()
            .open_modal(widget::EzObjects::DroppedDownMenu(new_modal));
        state_tree.extend(extra_state_tree);
    }

    fn set_focus(&mut self, enabled: bool) { self.state.focussed = enabled }

    fn get_focus(&self) -> bool { self.state.focussed }
}

impl Dropdown {

    /// Initialize an instance of this object using the passed config coming from [ez_parser]
    pub fn from_config(config: Vec<String>) -> Self {
        let mut obj = Dropdown::default();
        obj.load_ez_config(config).unwrap();
        obj
    }
}



#[derive(Default)]
/// This is the menu displayed when the dropdown is actually dropped down. It is implemented as a
/// separate modal.
#[derive(Clone)]
pub struct DroppedDownMenu {

    /// ID of the widget, used to construct [path]
    pub id: String,

    /// Full path to this widget, e.g. "/root_layout/layout_2/THIS_ID"
    pub path: String,

    /// Runtime state of this widget, see [DropdownState] and [State]
    pub state: DroppedDownMenuState,
}

impl EzObject for DroppedDownMenu {

    fn load_ez_parameter(&mut self, _parameter_name: String, _parameter_value: String) { }

    fn set_id(&mut self, id: String) { self.id = id }

    fn get_id(&self) -> String { self.id.clone() }

    fn set_full_path(&mut self, path: String) { self.path = path }

    fn get_full_path(&self) -> String { self.path.clone() }

    fn get_state(&self) -> EzState { EzState::DroppedDownMenu(self.state.clone()) }

    fn get_contents(&self, state_tree: &mut common::StateTree) -> common::PixelMap {

        let state = state_tree
            .get_mut(&self.get_full_path()).unwrap().as_dropped_down_menu_mut();
        let mut options:Vec<String> = state.get_dropped_down_options().iter()
            .map(|x| x.chars().rev().collect::<String>()).collect();

        let mut contents = Vec::new();
        for _ in 0..state.get_effective_size().width {
            let mut new_y = Vec::new();
            for y in 0..options.len() {
                let fg = if y == state.dropped_down_selected_row
                {state.get_color_config().selection_foreground }
                else {state.get_color_config().foreground };
                let bg = if y == state.dropped_down_selected_row
                {state.get_color_config().selection_background }
                else {state.get_color_config().background };
                if !options[y].is_empty(){
                    new_y.push(widget::Pixel{symbol: options[y].pop().unwrap().to_string(),
                        foreground_color: fg, background_color: bg, underline: false})
                } else {
                    new_y.push(widget::Pixel::new(" ".to_string(), fg,
                                          bg))
                }
            }
            contents.push(new_y.clone());

        }
        let state = state_tree
            .get(&self.get_full_path()).unwrap().as_dropped_down_menu();
        contents = common::add_padding(
            contents, state.get_padding(), state.colors.background,
            state.colors.foreground);
        contents = common::add_border(contents, state.get_border_config());
        contents
    }

    fn handle_event(&self, event: Event, view_tree: &mut ViewTree,
                    state_tree: &mut StateTree, widget_tree: &WidgetTree,
                    callback_tree: &mut CallbackTree, scheduler: &mut Scheduler) -> bool {

        let state = state_tree.get_mut(&self.get_full_path().clone())
            .unwrap().as_dropped_down_menu_mut();
        match event {
            Event::Key(key) => {
                match key.code {
                    KeyCode::Enter => {
                        self.handle_enter(view_tree, state_tree, widget_tree, callback_tree,
                                          scheduler);
                        return true
                    }
                    KeyCode::Down => {
                        self.handle_down(state);
                        return true
                    },
                    KeyCode::Up => {
                        self.handle_up(state);
                        return true
                    },
                    _ => ()
                }
            }
            Event::Mouse(event) => {
                let mouse_position = state::Coordinates::new(event.column as usize,
                                                             event.row as usize);
                if let MouseEventKind::Down(button) = event.kind {
                    if button == MouseButton::Left {
                        self.handle_left_click(view_tree, state_tree, widget_tree, callback_tree,
                                               scheduler,mouse_position);
                        return true
                    }
                } else if event.kind == MouseEventKind::Moved &&
                    state.collides(mouse_position) {
                    return self.handle_hover(state, mouse_position)
                }
            },
            _ => ()
        }
        false
    }
}

impl DroppedDownMenu {

    /// Called when this widget is already dropped down and enter is pressed
    pub fn handle_enter(&self, view_tree: &mut ViewTree, state_tree: &mut StateTree,
                        widget_tree: &WidgetTree, callback_tree: &mut CallbackTree,
                        scheduler: &mut Scheduler) {

        let selected = state_tree.get(&self.get_full_path()).unwrap()
            .as_dropped_down_menu().dropped_down_selected_row;
        let choice = state_tree
            .get(&self.get_full_path()).unwrap().as_dropped_down_menu()
            .get_dropped_down_options()[selected].clone();
        let parent = state_tree.get(&self.get_full_path()).unwrap()
            .as_dropped_down_menu().parent_path.clone();
        state_tree.get_mut(&parent).unwrap()
            .as_dropdown_mut().set_choice(choice);
        state_tree
            .get_mut("/root").unwrap().as_layout_mut().dismiss_modal();
        if let Some(ref mut i) = callback_tree
            .get_mut(&parent).unwrap().on_value_change {
            let context = common::EzContext::new(parent,
                                                 view_tree, state_tree, widget_tree, scheduler);
            i(context);
        }
    }

    /// Called when this widget is already dropped down and up is pressed
    pub fn handle_up(&self, state: &mut DroppedDownMenuState) {

        if state.dropped_down_selected_row == 0 {
            state.set_dropped_down_selected_row(state.total_options() - 1);
        } else {
            state.set_dropped_down_selected_row(state.dropped_down_selected_row - 1);
        }
    }

    /// Called when this widget is already dropped down and down is pressed
    pub fn handle_down(&self, state: &mut DroppedDownMenuState) {
        if state.dropped_down_selected_row == state.total_options() - 1 {
            state.set_dropped_down_selected_row(0);
        } else {
            state.set_dropped_down_selected_row(state.dropped_down_selected_row + 1);
        }
    }

    /// Called when this widget is already dropped down and widget is left clicked
    pub fn handle_left_click(&self, view_tree: &mut ViewTree, state_tree: &mut StateTree,
                             widget_tree: &WidgetTree, callback_tree: &mut CallbackTree,
                             scheduler: &mut Scheduler, pos: state::Coordinates) {

        let state = state_tree.get(&self.get_full_path()).unwrap()
            .as_dropped_down_menu();
        let parent = state.parent_path.clone();
        if state.collides(pos) {
            let clicked_row = pos.y - state.absolute_position.y;
            // Check if not click on border
            if clicked_row != 0 && clicked_row <= state.get_effective_size().height {
                let choice = state.get_dropped_down_options()[clicked_row - 1]
                    .clone();
                state_tree.get_mut(&parent).unwrap()
                    .as_dropdown_mut().set_choice(choice);
                state_tree.get_mut("/root").unwrap().as_layout_mut().dismiss_modal();
                if let Some(ref mut i) = callback_tree
                    .get_mut(&parent).unwrap().on_value_change {
                    let context = common::EzContext::new(parent,
                                                         view_tree, state_tree, widget_tree, scheduler);
                    i(context);
                }
            }
        } else {
            state_tree.get_mut("/root").unwrap().as_layout_mut().dismiss_modal();
        }
    }

    /// Called when this widget is already dropped down and widget is hovered
    pub fn handle_hover(&self, state: &mut DroppedDownMenuState, pos: state::Coordinates) -> bool {
        let hovered_row = pos.y - state.absolute_position.y;
        // Check if not hover on border
        if hovered_row - 1 != state.dropped_down_selected_row &&
            hovered_row != 0 && hovered_row <= state.get_dropped_down_options().len() {
            state.set_dropped_down_selected_row(hovered_row - 1);
            return true
        }
        false
    }
}
