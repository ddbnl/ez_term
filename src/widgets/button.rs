//! A widget that displays text non-interactively.
use std::io::{Error, ErrorKind};
use std::time::Duration;
use crossterm::event::KeyCode;
use crate::states::state::{self, GenericState, SelectableState};
use crate::states::button_state::ButtonState;
use crate::common;
use crate::widgets::widget::{EzWidget, Pixel, EzObject};
use crate::ez_parser;

pub struct Button {

    /// ID of the widget, used to construct [path]
    pub id: String,

    /// Full path to this widget, e.g. "/root_layout/layout_2/THIS_ID"
    pub path: String,

    /// Global order number in which this widget will be selection when user presses down/up keys
    pub selection_order: usize,

    /// Optional function to call when this widget is selected via keyboard up/down or mouse hover,
    /// see [set_bind_on_select] for examples.
    pub bound_on_select: Option<fn(context: common::EzContext, mouse_position: Option<state::Coordinates>)>,

    /// Optional function to call when this widget is right clicked, see
    /// [MouseCallbackFunction] for the callback fn type, or [set_right_left_click] for
    /// examples.
    pub bound_on_deselect: Option<common::GenericEzFunction>,

    /// Optional function to call when this widget is keyboard entered or left clicked,
    /// [GenericCallbackFunction] for the callback fn type, or [set_bind_on_press] for
    /// examples.
    pub bound_on_press: Option<common::GenericEzFunction>,

    /// Optional function to call when this widget is right clicked, see
    /// [MouseCallbackFunction] for the callback fn type, or [set_right_left_click] for
    /// examples.
    pub bound_right_mouse_click: Option<common::MouseCallbackFunction>,

    /// A Key to callback function lookup used to store keybinds for this widget. See
    /// [KeyboardCallbackFunction] type for callback function signature.
    pub keymap: common::KeyMap,

    /// Runtime state of this widget, see [LabelState] and [State]
    pub state: ButtonState,
}

impl Default for Button {
    fn default() -> Self {
        Button {
            id: "".to_string(),
            path: String::new(),
            selection_order: 0,
            bound_on_select: None,
            bound_on_deselect: None,
            bound_on_press: None,
            bound_right_mouse_click: None,
            keymap: common::KeyMap::new(),
            state: ButtonState::default(),
        }
    }
}


impl EzObject for Button {

    fn load_ez_parameter(&mut self, parameter_name: String, parameter_value: String)
                         -> Result<(), Error> {
        match parameter_name.as_str() {
            "id" => self.set_id(parameter_value.trim().to_string()),
            "x" => self.state.set_x(parameter_value.trim().parse().unwrap()),
            "y" => self.state.set_y(parameter_value.trim().parse().unwrap()),
            "pos" => self.state.set_position(
                ez_parser::load_pos_parameter(parameter_value.trim()).unwrap()),
            "size_hint" => self.state.set_size_hint(
                ez_parser::load_full_size_hint_parameter(parameter_value.trim()).unwrap()),
            "size_hint_x" => self.state.set_size_hint_x(
                ez_parser::load_size_hint_parameter(parameter_value.trim()).unwrap()),
            "size_hint_y" => self.state.set_size_hint_y(
                ez_parser::load_size_hint_parameter(parameter_value.trim()).unwrap()),
            "size" => self.state.set_size(
                ez_parser::load_size_parameter(parameter_value.trim()).unwrap()),
            "width" => self.state.set_width(parameter_value.trim().parse().unwrap()),
            "height" => self.state.set_height(parameter_value.trim().parse().unwrap()),
            "pos_hint" => self.state.set_pos_hint(
                ez_parser::load_full_pos_hint_parameter(parameter_value.trim()).unwrap()),
            "pos_hint_x" => self.state.set_pos_hint_x(
                ez_parser::load_pos_hint_x_parameter(parameter_value.trim()).unwrap()),
            "pos_hint_y" => self.state.set_pos_hint_y(
                ez_parser::load_pos_hint_y_parameter(parameter_value.trim()).unwrap()),
            "auto_scale" => self.state.set_auto_scale(ez_parser::load_full_auto_scale_parameter(
                parameter_value.trim())?),
            "auto_scale_width" =>
                self.state.set_auto_scale_width(ez_parser::load_bool_parameter(parameter_value.trim())?),
            "auto_scale_height" =>
                self.state.set_auto_scale_height(ez_parser::load_bool_parameter(parameter_value.trim())?),
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
                self.state.set_horizontal_alignment(
                    ez_parser::load_halign_parameter(parameter_value.trim()).unwrap()),
            "valign" =>
                self.state.set_vertical_alignment(
                    ez_parser::load_valign_parameter(parameter_value.trim()).unwrap()),
            "fg_color" => self.state.colors.foreground =
                ez_parser::load_color_parameter(parameter_value).unwrap(),
            "bg_color" => self.state.colors.background =
                ez_parser::load_color_parameter(parameter_value).unwrap(),
            "selection_fg_color" => self.state.colors.selection_foreground =
                ez_parser::load_color_parameter(parameter_value).unwrap(),
            "selection_bg_color" => self.state.colors.selection_background =
                ez_parser::load_color_parameter(parameter_value).unwrap(),
            "flash_fg_color" => self.state.colors.flash_foreground =
                ez_parser::load_color_parameter(parameter_value).unwrap(),
            "flash_bg_color" => self.state.colors.flash_background =
                    ez_parser::load_color_parameter(parameter_value).unwrap(),
            "selection_order" => { self.selection_order = ez_parser::load_selection_order_parameter(
                parameter_value.as_str()).unwrap(); },
            "text" => self.state.set_text(
                ez_parser::load_text_parameter(parameter_value.as_str()).unwrap()),
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
            _ => return Err(Error::new(ErrorKind::InvalidData,
                                       format!("Invalid parameter name for button {}",
                                               parameter_name)))
        }
        Ok(())
    }


    fn set_id(&mut self, id: String) { self.id = id }

    fn get_id(&self) -> String { self.id.clone() }

    fn set_full_path(&mut self, path: String) { self.path = path }

    fn get_full_path(&self) -> String { self.path.clone() }

    fn update_state(&mut self, new_state: &state::EzState) {
        let state = new_state.as_button();
        self.state = state.clone();
        self.state.changed = false;
        self.state.force_redraw = false;
    }

    fn get_state(&self) -> state::EzState { state::EzState::Button(self.state.clone()) }

    fn get_contents(&self, state_tree: &mut common::StateTree) -> common::PixelMap {

        let state = state_tree.get_mut(&self.get_full_path()).unwrap()
            .as_button_mut();
        let text = state.text.clone();
        let content_lines = common::wrap_text(text, state.get_effective_size().width);

        let fg_color = if state.flashing {state.get_colors().flash_foreground }
            else if state.selected {state.get_colors().selection_foreground }
            else {state.get_colors().foreground };
        let bg_color = if state.flashing {state.get_colors().flash_background }
            else if state.selected {state.get_colors().selection_background }
            else {state.get_colors().background };

        let longest_line = content_lines.iter().map(|x| x.len()).max();
        let longest_line = if let Some(i) = longest_line { i } else { 0 };
        let mut contents = Vec::new();
        for x in 0..longest_line {
            let mut new_y = Vec::new();
            for y in 0..state.get_effective_size().height {
                if y < content_lines.len() && x < content_lines[y].len() {
                    new_y.push(Pixel { symbol: content_lines[y][x..x+1].to_string(),
                        foreground_color: fg_color, background_color: bg_color, underline: false })
                }
            }
            contents.push(new_y);
        }
        if state.get_auto_scale().width {
            state.set_effective_width(contents.len());
        }
        if state.get_auto_scale().height {
            state.set_effective_height(contents[0].len());
        }
        (contents, _) = common::align_content_horizontally(
            contents,state::HorizontalAlignment::Center, state.get_effective_size().width,
                    fg_color, bg_color);
        (contents, _) = common::align_content_vertically(
            contents,state::VerticalAlignment::Middle, state.get_effective_size().height,
            fg_color, bg_color);
        contents = common::add_border(contents, state.get_border_config());
        let state = state_tree.get(&self.get_full_path()).unwrap().as_button();
        let parent_colors = state_tree.get(self.get_full_path()
            .rsplit_once('/').unwrap().0).unwrap().as_generic().get_colors();
        contents = common::add_padding(
            contents, state.get_padding(), parent_colors.background,
            parent_colors.foreground);
        contents
    }
}


impl EzWidget for Button {

    fn is_selectable(&self) -> bool { true }

    fn is_selected(&self) -> bool { self.state.selected }


    fn get_selection_order(&self) -> usize { self.selection_order }

    fn get_key_map(&self) -> &common::KeyMap {
        &self.keymap
    }

    fn bind_key(&mut self, key: KeyCode, func: common::KeyboardCallbackFunction) {
        self.keymap.insert(key, func);
    }

    fn set_bind_on_press(&mut self, func: common::GenericEzFunction) {
        self.bound_on_press = Some(func)
    }

    fn get_bind_on_press(&self) -> Option<common::GenericEzFunction> { self.bound_on_press }

    fn on_left_click(&self, context: common::EzContext, _position: state::Coordinates) {
        context.state_tree.get_mut(&self.get_full_path()).unwrap().as_button_mut()
            .set_selected(false);
        self._on_press(context);
    }

    fn on_keyboard_enter(&self, context: common::EzContext) { self._on_press(context) }

    fn set_bind_on_select(&mut self, func: fn(common::EzContext, Option<state::Coordinates>)) {
       self.bound_on_select = Some(func);
    }

    fn get_bind_on_select(&self) -> Option<fn(common::EzContext, Option<state::Coordinates>)> {
       self.bound_on_select
    }
}

impl Button {

    /// Initialize an instance of this object using the passed config coming from [ez_parser]
    pub fn from_config(config: Vec<&str>) -> Self {
        let mut obj = Button::default();
        obj.load_ez_config(config).unwrap();
        obj
    }

    fn _on_press(&self, context: common::EzContext) {

        context.state_tree.get_mut(&context.widget_path.clone()).unwrap().as_button_mut()
            .set_flashing(true);
        let scheduled_func =
            | context: common::EzContext | {
                context.state_tree.get_mut(&context.widget_path).unwrap().as_button_mut()
                    .set_flashing(false);
                true
            };
        context.scheduler.schedule_once(self.get_full_path(),
                                        Box::new(scheduled_func),
                                        Duration::from_millis(50));
        self.on_press(context);
    }
}
