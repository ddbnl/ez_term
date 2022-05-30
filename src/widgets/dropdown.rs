//! # Dropdown Widget
//! Widget which supports and arbitrary amount of possible values of which one can be chosen at any
//! time. The active value is always displayed, and when selected drops down all other possible
//! values for the user to select.
use std::io::{Error, ErrorKind};
use crossterm::event::{Event, KeyCode, MouseButton, MouseEventKind};
use crate::common;
use crate::common::{PixelMap, StateTree, ViewTree, WidgetTree};
use crate::states::dropdown_state::{DropdownState, DroppedDownMenuState};
use crate::states::state::{self, AutoScale, EzState, GenericState, PosHint, Size, SizeHint};
use crate::widgets::widget::{self, EzObject, EzWidget};
use crate::ez_parser;


#[derive(Default)]
pub struct Dropdown {

    /// ID of the widget, used to construct [path]
    pub id: String,

    /// Full path to this widget, e.g. "/root_layout/layout_2/THIS_ID"
    pub path: String,

    /// Global order number in which this widget will be selection when user presses down/up keys
    pub selection_order: usize,

    /// Runtime state of this widget, see [DropdownState] and [State]
    pub state: DropdownState,
}


impl widget::EzObject for Dropdown {

    fn load_ez_parameter(&mut self, parameter_name: String, parameter_value: String)
                         -> Result<(), Error> {

        match parameter_name.as_str() {
            "id" => self.set_id(parameter_value.trim().to_string()),
            "x" => self.state.set_x(parameter_value.trim().parse().unwrap()),
            "y" => self.state.set_y(parameter_value.trim().parse().unwrap()),
            "pos" => self.state.set_position(
                ez_parser::load_pos_parameter(parameter_value.trim()).unwrap()),
            "size_hint_x" => self.state.set_size_hint_x(
                ez_parser::load_size_hint_parameter(parameter_value.trim()).unwrap()),
            "pos_hint" => self.state.set_pos_hint(
                ez_parser::load_full_pos_hint_parameter(parameter_value.trim()).unwrap()),
            "pos_hint_x" => self.state.set_pos_hint_x(
                ez_parser::load_pos_hint_x_parameter(parameter_value.trim()).unwrap()),
            "pos_hint_y" => self.state.set_pos_hint_y(
                ez_parser::load_pos_hint_y_parameter(parameter_value.trim()).unwrap()),
            "auto_scale_width" =>
                self.state.set_auto_scale_width(ez_parser::load_bool_parameter(parameter_value.trim())?),
            "width" => self.state.set_width(parameter_value.trim().parse().unwrap()),
            "padding" => self.state.set_padding(ez_parser::load_full_padding_parameter(
                parameter_value.trim())?),
            "padding_x" => self.state.set_padding(ez_parser::load_padding_x_parameter(
                parameter_value.trim())?),
            "padding_y" => self.state.set_padding(ez_parser::load_padding_y_parameter(
                parameter_value.trim())?),
            "padding_top" => self.state.set_padding_top(parameter_value.trim().parse().unwrap()),
            "padding_bottom" => self.state.set_padding_bottom(parameter_value.trim().parse().unwrap()),
            "padding_left" => self.state.set_padding_left(parameter_value.trim().parse().unwrap()),
            "padding_right" => self.state.set_padding_right(parameter_value.trim().parse().unwrap()),
            "halign" =>
                self.state.halign =  ez_parser::load_halign_parameter(parameter_value.trim()).unwrap(),
            "valign" =>
                self.state.valign =  ez_parser::load_valign_parameter(parameter_value.trim()).unwrap(),
            "selection_order" => {
                self.selection_order = ez_parser::load_selection_order_parameter(
                    parameter_value.as_str()).unwrap();
            },
            "border" => self.state.set_border(ez_parser::load_bool_parameter(parameter_value.trim())?),
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
                self.state.border_config.fg_color = ez_parser::load_color_parameter(parameter_value).unwrap(),
            "border_bg_color" =>
                self.state.border_config.bg_color = ez_parser::load_color_parameter(parameter_value).unwrap(),
            "fg_color" =>
                self.state.colors.foreground =
                    ez_parser::load_color_parameter(parameter_value).unwrap(),
            "bg_color" =>
                self.state.colors.background =
                    ez_parser::load_color_parameter(parameter_value).unwrap(),
            "selection_fg_color" =>
                self.state.colors.selection_foreground =
                    ez_parser::load_color_parameter(parameter_value).unwrap(),
            "selection_bg_color" =>
                self.state.colors.selection_background =
                    ez_parser::load_color_parameter(parameter_value).unwrap(),
            "allow_none" =>
                self.state.allow_none = ez_parser::load_bool_parameter(parameter_value.trim()).unwrap(),
            "options" => {
                self.state.options = parameter_value.split(',')
                    .map(|x| x.trim().to_string()).collect();
            },
            "active" => {
                self.state.choice = parameter_value.trim().to_string();
            }
            _ => return Err(Error::new(ErrorKind::InvalidData,
                                format!("Invalid parameter name for dropdown {}",
                                        parameter_name)))
        }
        Ok(())
    }

    fn set_id(&mut self, id: String) { self.id = id }

    fn get_id(&self) -> String { self.id.clone() }

    fn set_full_path(&mut self, path: String) { self.path = path }

    fn get_full_path(&self) -> String { self.path.clone() }

    fn get_state(&self) -> EzState { EzState::Dropdown(DropdownState::default()) }

    /// Content of this widget depends on whether it is currently dropped down or not. If not,
    /// then display a label with a border representing the currently selected value. If dropped
    /// down show a list of all options, with the currently selected one on top.
    fn get_contents(&self, state_tree: &mut common::StateTree, widget_tree: &common::WidgetTree) -> common::PixelMap {

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
        let fg_color = if state.selected {state.get_colors().selection_foreground }
        else {state.get_colors().foreground };
        let bg_color = if state.selected {state.get_colors().selection_background }
        else {state.get_colors().background };
        let mut text = active.chars().rev().collect::<String>();
        let mut contents = Vec::new();
        for _ in 0..state.get_effective_size().width {
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
}

impl widget::EzWidget for Dropdown {
    fn set_focus(&mut self, enabled: bool) { self.state.focussed = enabled }

    fn get_focus(&self) -> bool { self.state.focussed }

    fn is_selectable(&self) -> bool { true }

    fn get_selection_order(&self) -> usize { self.selection_order }

    fn on_left_click(&self, context: common::EzContext, _position: state::Coordinates) {
        self.on_press(context);
    }

    fn on_keyboard_enter(&self, context: common::EzContext) { self.on_press(context); }
}

impl Dropdown {

    /// Initialize an instance of this object using the passed config coming from [ez_parser]
    pub fn from_config(config: Vec<String>) -> Self {
        let mut obj = Dropdown::default();
        obj.load_ez_config(config).unwrap();
        obj
    }

    /// Called when the widgets is not dropped down and enter/left mouse click occurs on it.
    fn on_press(&self, context: common::EzContext) {

        let root_state = context.state_tree.get_mut("/root").unwrap();
        let state = context.state_tree.get(&self.get_full_path()).unwrap()
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
            absolute_position: state.get_absolute_position(),
            pos_hint: PosHint::new(None, None),
            dropped_down_selected_row: 1,
            border_config: state.border_config.clone(),
            colors: state.colors.clone(),
            changed: false,
            choice: state.get_choice(),
            force_redraw: false
        };
        let new_modal = DroppedDownMenu {
            id: modal_id,
            path: modal_path.clone(),
            parent_path: self.path.clone(),
            state: new_modal_state,
        };
        let (_, extra_state_tree) = root_state.as_layout_mut()
            .open_modal(widget::EzObjects::DroppedDownMenu(new_modal));
        context.state_tree.extend(extra_state_tree);
    }
}



#[derive(Default)]
/// This is the menu displayed when the dropdown is actually dropped down. It is implemented as a
/// separate modal.
pub struct DroppedDownMenu {

    /// ID of the widget, used to construct [path]
    pub id: String,

    /// Full path to this widget, e.g. "/root_layout/layout_2/THIS_ID"
    pub path: String,

    /// Runtime state of this widget, see [DropdownState] and [State]
    pub state: DroppedDownMenuState,
}

impl EzObject for DroppedDownMenu {

    fn load_ez_parameter(&mut self, _parameter_name: String, _parameter_value: String)
        -> Result<(), Error> { Ok(()) }

    fn set_id(&mut self, id: String) { self.id = id }

    fn get_id(&self) -> String { self.id.clone() }

    fn set_full_path(&mut self, path: String) { self.path = path }

    fn get_full_path(&self) -> String { self.path.clone() }

    fn get_state(&self) -> EzState { EzState::Dropdown(DropdownState::default()) }

    fn get_contents(&self, state_tree: &mut common::StateTree, widget_tree: &common::WidgetTree) -> common::PixelMap {

        let state = state_tree
            .get_mut(&self.get_full_path()).unwrap().as_dropped_down_menu_mut();
        let mut options:Vec<String> = self.get_dropped_down_options(state).iter()
            .map(|x| x.chars().rev().collect::<String>()).collect();

        let mut contents = Vec::new();
        for _ in 0..state.get_size().width - 2{
            let mut new_y = Vec::new();
            for y in 0..options.len() {
                let fg = if y == state.dropped_down_selected_row
                {state.get_colors().selection_foreground }
                else {state.get_colors().foreground };
                let bg = if y == state.dropped_down_selected_row
                {state.get_colors().selection_background }
                else {state.get_colors().background };
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
            .get(&self.get_full_path()).unwrap().as_dropdown();
        contents = common::add_padding(
            contents, state.get_padding(), state.colors.background,
            state.colors.foreground);
        contents = common::add_border(contents, state.get_border_config());
        contents
    }
}
