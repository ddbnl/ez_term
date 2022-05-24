//! A widget that displays text non-interactively.
use std::io::{Error, ErrorKind};
use std::time::Duration;
use crossterm::event::KeyCode;
use crate::states::state::{EzState, HorizontalAlignment, VerticalAlignment, GenericState};
use crate::states::button_state::ButtonState;
use crate::common::{self, Coordinates, PixelMap, MouseCallbackFunction, GenericEzFunction,
                    EzContext, StateTree, KeyMap, KeyboardCallbackFunction};
use crate::widgets::widget::{EzWidget, Pixel, EzObject};
use crate::ez_parser::{load_color_parameter, load_text_parameter, load_selection_order_parameter, load_size_hint_parameter, load_halign_parameter, load_valign_parameter, load_bool_parameter, load_pos_hint_x_parameter, load_pos_hint_y_parameter};

pub struct Button {

    /// ID of the widget, used to construct [path]
    pub id: String,

    /// Full path to this widget, e.g. "/root_layout/layout_2/THIS_ID"
    pub path: String,

    /// Global order number in which this widget will be selection when user presses down/up keys
    pub selection_order: usize,

    /// Optional function to call when this widget is selected via keyboard up/down or mouse hover,
    /// see [set_bind_on_select] for examples.
    pub bound_on_select: Option<fn(context: EzContext, mouse_position: Option<Coordinates>)>,

    /// Optional function to call when this widget is right clicked, see
    /// [MouseCallbackFunction] for the callback fn type, or [set_right_left_click] for
    /// examples.
    pub bound_on_deselect: Option<GenericEzFunction>,

    /// Optional function to call when this widget is keyboard entered or left clicked,
    /// [GenericCallbackFunction] for the callback fn type, or [set_bind_on_press] for
    /// examples.
    pub bound_on_press: Option<GenericEzFunction>,

    /// Optional function to call when this widget is right clicked, see
    /// [MouseCallbackFunction] for the callback fn type, or [set_right_left_click] for
    /// examples.
    pub bound_right_mouse_click: Option<MouseCallbackFunction>,

    /// A Key to callback function lookup used to store keybinds for this widget. See
    /// [KeyboardCallbackFunction] type for callback function signature.
    pub keymap: KeyMap,

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
            keymap: KeyMap::new(),
            state: ButtonState::default(),
        }
    }
}


impl EzObject for Button {

    fn load_ez_parameter(&mut self, parameter_name: String, parameter_value: String)
                         -> Result<(), Error> {
        match parameter_name.as_str() {
            "x" => self.state.set_position((parameter_value.trim().parse().unwrap(),
                                            self.state.get_position().1)),
            "y" => self.state.set_position((self.state.get_position().0,
                                            parameter_value.trim().parse().unwrap())),
            "size_hint_x" => self.state.set_size_hint_x(
                load_size_hint_parameter(parameter_value.trim()).unwrap()),
            "size_hint_y" => self.state.set_size_hint_y(
                load_size_hint_parameter(parameter_value.trim()).unwrap()),
            "width" => self.state.set_width(parameter_value.trim().parse().unwrap()),
            "height" => self.state.set_height(parameter_value.trim().parse().unwrap()),
            "pos_hint_x" => self.state.set_pos_hint_x(
                load_pos_hint_x_parameter(parameter_value.trim()).unwrap()),
            "pos_hint_y" => self.state.set_pos_hint_y(
                load_pos_hint_y_parameter(parameter_value.trim()).unwrap()),
            "auto_scale_width" =>
                self.state.set_auto_scale_width(load_bool_parameter(parameter_value.trim())?),
            "auto_scale_height" =>
                self.state.set_auto_scale_height(load_bool_parameter(parameter_value.trim())?),
            "halign" =>
                self.state.set_horizontal_alignment(
                    load_halign_parameter(parameter_value.trim()).unwrap()),
            "valign" =>
                self.state.set_vertical_alignment(
                    load_valign_parameter(parameter_value.trim()).unwrap()),
            "fg_color" => self.state.set_content_foreground_color(
                load_color_parameter(parameter_value).unwrap()),
            "bg_color" => self.state.set_content_background_color(
                load_color_parameter(parameter_value).unwrap()),
            "selection_fg_color" => self.state.set_selection_foreground_color(
                load_color_parameter(parameter_value).unwrap()),
            "selection_bg_color" => self.state.set_selection_background_color(
                load_color_parameter(parameter_value).unwrap()),
            "flash_fg_color" => self.state.set_flash_foreground_color(
                load_color_parameter(parameter_value).unwrap()),
            "flash_bg_color" => self.state.set_flash_background_color(
                    load_color_parameter(parameter_value).unwrap()),
            "selection_order" => { self.selection_order = load_selection_order_parameter(
                parameter_value.as_str()).unwrap(); },
            "text" => self.state.set_text(
                load_text_parameter(parameter_value.as_str()).unwrap()),
            "border_horizontal_symbol" => self.state.set_border_horizontal_symbol(
                parameter_value.trim().to_string()),
            "border_vertical_symbol" => self.state.set_border_vertical_symbol(
                parameter_value.trim().to_string()),
            "border_top_right_symbol" => self.state.set_border_top_right_symbol(
                parameter_value.trim().to_string()),
            "border_top_left_symbol" => self.state.set_border_top_left_symbol(
                parameter_value.trim().to_string()),
            "border_bottom_left_symbol" => self.state.set_border_bottom_left_symbol(
                parameter_value.trim().to_string()),
            "border_bottom_right_symbol" => self.state.set_border_bottom_right_symbol(
                parameter_value.trim().to_string()),
            "border_fg_color" => self.state.set_border_foreground_color(
                load_color_parameter(parameter_value).unwrap()),
            "border_bg_color" => self.state.set_border_background_color(
                load_color_parameter(parameter_value).unwrap()),
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

    fn update_state(&mut self, new_state: &EzState) {
        let state = new_state.as_button();
        self.state = state.clone();
        self.state.changed = false;
        self.state.force_redraw = false;
    }

    fn get_state(&self) -> EzState { EzState::Button(self.state.clone()) }

    fn get_contents(&self, state_tree: &mut StateTree) -> PixelMap {

        let state = state_tree.get(&self.get_full_path()).unwrap().as_button();
        let text = state.text.clone();
        let content_lines = common::wrap_text(text, state.get_effective_width());

        let fg_color = if state.flashing {state.flash_foreground_color}
            else if state.selected {state.selection_foreground_color}
            else {state.content_foreground_color};
        let bg_color = if state.flashing {state.flash_background_color}
            else if state.selected {state.selection_background_color}
            else {state.content_background_color};

        let longest_line = content_lines.iter().map(|x| x.len()).max();
        let longest_line = if let Some(i) = longest_line { i } else { 0 };
        let mut contents = Vec::new();
        for x in 0..longest_line {
            let mut new_y = Vec::new();
            for y in 0..state.get_effective_height() {
                if y < content_lines.len() && x < content_lines[y].len() {
                    new_y.push(Pixel { symbol: content_lines[y][x..x+1].to_string(),
                        foreground_color: fg_color, background_color: bg_color, underline: false })
                }
            }
            contents.push(new_y);
        }
        (contents, _) = common::align_content_horizontally(
            contents,HorizontalAlignment::Center, state.get_effective_width(),
                    fg_color, bg_color);
        (contents, _) = common::align_content_vertically(
            contents,VerticalAlignment::Middle, state.get_effective_height(),
            fg_color, bg_color);
        contents = common::add_border(contents,
                                      state.border_horizontal_symbol.clone(),
                                      state.border_vertical_symbol.clone(),
                                      state.border_top_left_symbol.clone(),
                                      state.border_top_right_symbol.clone(),
                                      state.border_bottom_left_symbol.clone(),
                                      state.border_bottom_right_symbol.clone(),
                                      state.border_background_color,
                                      state.border_foreground_color);
        contents
    }

}


impl EzWidget for Button {

    fn is_selectable(&self) -> bool { true }

    fn is_selected(&self) -> bool { self.state.selected }


    fn get_selection_order(&self) -> usize { self.selection_order }

    fn get_key_map(&self) -> &KeyMap {
        &self.keymap
    }

    fn bind_key(&mut self, key: KeyCode, func: KeyboardCallbackFunction) {
        self.keymap.insert(key, func);
    }

    fn set_bind_on_press(&mut self, func: GenericEzFunction) { self.bound_on_press = Some(func) }

    fn get_bind_on_press(&self) -> Option<GenericEzFunction> { self.bound_on_press }

    fn on_left_click(&self, context: EzContext, _position: Coordinates) { self._on_press(context) }

    fn on_keyboard_enter(&self, context: EzContext) { self._on_press(context) }

    fn set_bind_on_select(&mut self, func: fn(EzContext, Option<Coordinates>)) {
       self.bound_on_select = Some(func);
    }

    fn get_bind_on_select(&self) -> Option<fn(EzContext, Option<Coordinates>)> {
       self.bound_on_select
    }
}

impl Button {

    /// Initialize an instance of this object using the passed config coming from [ez_parser]
    pub fn from_config(config: Vec<&str>, id: String) -> Self {
        let mut obj = Button::default();
        obj.set_id(id);
        obj.load_ez_config(config).unwrap();
        obj
    }

    fn _on_press(&self, context: EzContext) {

        context.state_tree.get_mut(&context.widget_path.clone()).unwrap().as_button_mut()
            .set_flashing(true);
        let scheduled_func =
            | context: EzContext | {
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
