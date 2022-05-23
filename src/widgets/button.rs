//! A widget that displays text non-interactively.
use std::io::{Error, ErrorKind};
use std::time::Duration;
use crossterm::event::KeyCode;
use crossterm::style::{Color};
use crate::common::{self, Coordinates, PixelMap, MouseCallbackFunction, GenericEzFunction,
                    EzContext, StateTree, KeyMap, KeyboardCallbackFunction};
use crate::widgets::widget::{EzWidget, Pixel, EzObject};
use crate::widgets::state::{State, GenericState, SelectableState, HorizontalAlignment, VerticalAlignment};
use crate::ez_parser::{load_color_parameter, load_text_parameter, load_selection_order_parameter,
                       load_size_hint, load_halign_parameter, load_valign_parameter};

pub struct Button {

    /// ID of the widget, used to construct [path]
    pub id: String,

    /// Full path to this widget, e.g. "/root_layout/layout_2/THIS_ID"
    pub path: String,

    /// Absolute position of this layout on screen. Automatically propagated, do not set manually
    pub absolute_position: Coordinates,

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
            absolute_position: (0, 0),
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


/// [State] implementation.
#[derive(Clone)]
pub struct ButtonState {

    /// Text currently being displayed by the label
    pub text: String,

    /// Bool representing whether this widget is currently selected.
    pub selected: bool,

    /// Bool representing whether this widget is currently displaying it's flash color.
    pub flashing: bool,

    /// Horizontal position of this widget relative to its' parent [Layout]
    pub x: usize,

    /// Vertical position of this widget relative to its' parent [Layout]
    pub y: usize,

    /// Width of this widget
    pub size_hint_x: Option<f64>,

    /// Width of this widget
    pub size_hint_y: Option<f64>,

    /// Width of this widget
    pub width: usize,

    /// Width of this widget
    pub height: usize,

    /// Horizontal alignment of this widget
    pub halign: HorizontalAlignment,

    /// Vertical alignment of this widget
    pub valign: VerticalAlignment,

    /// The [Pixel.symbol] to use for the horizontal border if [border] is true
    pub border_horizontal_symbol: String,

    /// The [Pixel.symbol] to use for the vertical border if [border] is true
    pub border_vertical_symbol: String,

    /// The [Pixel.symbol] to use for the top left border if [border] is true
    pub border_top_left_symbol: String,

    /// The [Pixel.symbol] to use for the top left border if [border] is true
    pub border_top_right_symbol: String,

    /// The [Pixel.symbol] to use for the bottom left border if [border] is true
    pub border_bottom_left_symbol: String,

    /// The [Pixel.symbol] to use for the bottom right border if [border] is true
    pub border_bottom_right_symbol: String,

    /// The[Pixel.foreground_color]  to use for the border if [border] is true
    pub border_foreground_color: Color,

    /// The [Pixel.background_color] to use for the border if [border] is true
    pub border_background_color: Color,

    /// The [Pixel.foreground_color] to use for this widgets' content
    pub content_foreground_color: Color,

    /// The [Pixel.background_color] to use for this widgets' content
    pub content_background_color: Color,

    /// The [Pixel.foreground_color] to use for this widgets' content when selected
    pub selection_foreground_color: Color,

    /// The [Pixel.background_color] to use for this widgets' content when selected
    pub selection_background_color: Color,

    /// The [Pixel.foreground_color] to use for this widgets' content when flashed
    pub flash_foreground_color: Color,

    /// The [Pixel.background_color] to use for this widgets' content when flashed
    pub flash_background_color: Color,

    /// Bool representing if state has changed. Triggers widget redraw.
    pub changed: bool,

    /// If true this forces a global screen redraw on the next frame. Screen redraws are diffed
    /// so this can be called when needed without degrading performance. If only screen positions
    /// that fall within this widget must be redrawn, call [EzObject.redraw] instead.
    pub force_redraw: bool,
}
impl Default for ButtonState {
    fn default() -> Self {

       ButtonState {
           x: 0,
           y: 0,
           size_hint_x: Some(1.0),
           size_hint_y: Some(1.0),
           width: 0,
           height: 0,
           halign: HorizontalAlignment::Left,
           valign: VerticalAlignment::Top,
           text: String::new(),
           selected: false,
           flashing: false,
           border_horizontal_symbol: "━".to_string(),
           border_vertical_symbol: "│".to_string(),
           border_top_left_symbol: "┌".to_string(),
           border_top_right_symbol: "┐".to_string(),
           border_bottom_left_symbol: "└".to_string(),
           border_bottom_right_symbol: "┘".to_string(),
           border_foreground_color: Color::White,
           border_background_color: Color::Black,
           content_foreground_color: Color::White,
           content_background_color: Color::Black,
           selection_foreground_color: Color::Yellow,
           selection_background_color: Color::Blue,
           flash_foreground_color: Color::Yellow,
           flash_background_color: Color::White,
           changed: false,
           force_redraw: false,
       }
    }
}
impl GenericState for ButtonState {

    fn set_changed(&mut self, changed: bool) { self.changed = changed }

    fn get_changed(&self) -> bool { self.changed }

    fn set_size_hint_x(&mut self, size_hint: Option<f64>) {
        self.size_hint_x = size_hint;
        self.changed = true; 
    }

    fn get_size_hint_x(&self) -> Option<f64> { self.size_hint_x }

    fn set_size_hint_y(&mut self, size_hint: Option<f64>) {
        self.size_hint_y = size_hint;
        self.changed = true;
    }

    fn get_size_hint_y(&self) -> Option<f64> { self.size_hint_y }

    fn set_width(&mut self, width: usize) { self.width = width; self.changed = true; }

    fn get_width(&self) -> usize { self.width }

    fn set_effective_width(&mut self, width: usize) { self.set_width(width + 2) }

    fn get_effective_width(&self) -> usize { self.get_width() - 2 }

    fn set_height(&mut self, height: usize) { self.height = height }

    fn get_height(&self) -> usize { self.height }

    fn set_effective_height(&mut self, height: usize) { self.set_height(height + 2) }

    fn get_effective_height(&self) -> usize { self.get_height() - 2 }

    fn set_position(&mut self, position: Coordinates) {
        self.x = position.0;
        self.y = position.1;
        self.changed = true;
    }

    fn get_position(&self) -> Coordinates { (self.x, self.y) }

    fn get_effective_position(&self) -> Coordinates {
        (self.x +if self.has_border() {1} else {0},
         self.y +if self.has_border() {1} else {0})
    }

    fn set_horizontal_alignment(&mut self, alignment: HorizontalAlignment) {
        self.halign = alignment;
        self.changed = true;
    }

    fn get_horizontal_alignment(&self) -> HorizontalAlignment { self.halign }

    fn set_vertical_alignment(&mut self, alignment: VerticalAlignment) {
        self.valign = alignment;
        self.changed = true;
    }

    fn get_vertical_alignment(&self) -> VerticalAlignment { self.valign }

    fn set_force_redraw(&mut self, redraw: bool) {
        self.force_redraw = redraw;
        self.changed = true;
    }

    fn get_force_redraw(&self) -> bool { self.force_redraw }
}
impl SelectableState for ButtonState {
    fn set_selected(&mut self, state: bool) {
        self.selected = state;
        self.changed = true;
    }
    fn get_selected(&self) -> bool { self.selected }
}
impl ButtonState {

    pub fn set_text(&mut self, text: String) {
        self.text = text;
        self.changed = true;
    }

    pub fn get_text(&self) -> String {
        self.text.clone()
    }

    pub fn set_flashing(&mut self, flashing: bool) {
        self.flashing = flashing;
        self.changed = true;
    }

    pub fn get_flashing(&self) -> bool {
        self.flashing
    }

    pub fn set_border_horizontal_symbol(&mut self, symbol: String) {
        self.border_horizontal_symbol = symbol
    }

    pub fn get_border_horizontal_symbol(&self) -> String { self.border_horizontal_symbol.clone() }

    pub fn set_border_vertical_symbol(&mut self, symbol: String) {
        self.border_vertical_symbol = symbol
    }

    pub fn get_border_vertical_symbol(&self) -> String { self.border_vertical_symbol.clone() }

    pub fn set_border_bottom_left_symbol(&mut self, symbol: String) {
        self.border_bottom_left_symbol = symbol
    }

    pub fn get_border_bottom_left_symbol(&self) -> String { self.border_bottom_left_symbol.clone() }

    pub fn set_border_bottom_right_symbol(&mut self, symbol: String) {
        self.border_bottom_right_symbol = symbol
    }

    pub fn get_border_bottom_right_symbol(&self) -> String { self.border_bottom_right_symbol.clone() }

    pub fn set_border_top_left_symbol(&mut self, symbol: String) {
        self.border_top_left_symbol = symbol
    }

    pub fn get_border_top_left_symbol(&self) -> String { self.border_top_left_symbol.clone() }

    pub fn set_border_top_right_symbol(&mut self, symbol: String) {
        self.border_top_right_symbol = symbol
    }

    pub fn get_border_top_right_symbol(&self) -> String { self.border_top_right_symbol.clone() }

    pub fn has_border(&self) -> bool { true }

    pub fn set_border_foreground_color(&mut self, color: Color) {
        self.border_foreground_color = color;
        self.changed = true;
    }

    pub fn get_border_foreground_color(&self) -> Color {
        self.border_foreground_color
    }

    pub fn set_border_background_color(&mut self, color: Color) {
        self.border_background_color = color;
        self.changed = true;
    }

    pub fn get_border_background_color(&self) -> Color {
        self.border_background_color
    }

    pub fn set_content_foreground_color(&mut self, color: Color) {
        self.content_foreground_color = color;
        self.changed = true;
    }

    pub fn get_content_foreground_color(&self) -> Color {
        self.content_foreground_color
    }

    pub fn set_content_background_color(&mut self, color: Color) {
        self.content_background_color = color;
        self.changed = true;
    }

    pub fn get_content_background_color(&self) -> Color {
        self.content_background_color
    }

    pub fn set_selection_foreground_color(&mut self, color: Color) {
        self.selection_foreground_color = color;
        self.changed = true;
    }

    pub fn get_selection_foreground_color(&self) -> Color {
        self.selection_foreground_color
    }

    pub fn set_selection_background_color(&mut self, color: Color) {
        self.selection_background_color = color;
        self.changed = true;
    }

    pub fn get_selection_background_color(&self) -> Color {
        self.selection_background_color
    }

    pub fn set_flash_foreground_color(&mut self, color: Color) {
        self.flash_foreground_color = color;
        self.changed = true;
    }

    pub fn get_flash_foreground_color(&self) -> Color {
        self.flash_foreground_color
    }

    pub fn set_flash_background_color(&mut self, color: Color) {
        self.flash_background_color = color;
        self.changed = true;
    }

    pub fn get_flash_background_color(&self) -> Color {
        self.flash_background_color
    }
}


impl EzObject for Button {

    fn load_ez_parameter(&mut self, parameter_name: String, parameter_value: String)
                         -> Result<(), Error> {
        match parameter_name.as_str() {
            "x" => self.state.x = parameter_value.trim().parse().unwrap(),
            "y" => self.state.y = parameter_value.trim().parse().unwrap(),
            "size_hint_x" => self.state.size_hint_x = 
                load_size_hint(parameter_value.trim()).unwrap(),
            "size_hint_y" => self.state.size_hint_y =
                load_size_hint(parameter_value.trim()).unwrap(),
            "width" => self.state.width = parameter_value.trim().parse().unwrap(),
            "height" => self.state.height = parameter_value.trim().parse().unwrap(),
            "halign" =>
                self.state.halign =  load_halign_parameter(parameter_value.trim()).unwrap(),
            "valign" =>
                self.state.valign =  load_valign_parameter(parameter_value.trim()).unwrap(),
            "fg_color" => self.state.content_foreground_color =
                load_color_parameter(parameter_value).unwrap(),
            "bg_color" => self.state.content_background_color =
                load_color_parameter(parameter_value).unwrap(),
            "selection_fg_color" => self.state.selection_foreground_color =
                load_color_parameter(parameter_value).unwrap(),
            "selection_bg_color" => self.state.selection_background_color =
                load_color_parameter(parameter_value).unwrap(),
            "flash_fg_color" => self.state.flash_foreground_color =
                load_color_parameter(parameter_value).unwrap(),
            "flash_bg_color" => self.state.flash_background_color =
                    load_color_parameter(parameter_value).unwrap(),
            "selection_order" => { self.selection_order = load_selection_order_parameter(
                parameter_value.as_str()).unwrap(); },
            "text" => { self.state.text = 
                load_text_parameter(parameter_value.as_str()).unwrap(); },
            "border_horizontal_symbol" => self.state.border_horizontal_symbol =
                parameter_value.trim().to_string(),
            "border_vertical_symbol" => self.state.border_vertical_symbol =
                parameter_value.trim().to_string(),
            "border_top_right_symbol" => self.state.border_top_right_symbol =
                parameter_value.trim().to_string(),
            "border_top_left_symbol" => self.state.border_top_left_symbol =
                parameter_value.trim().to_string(),
            "border_bottom_left_symbol" => self.state.border_bottom_left_symbol =
                parameter_value.trim().to_string(),
            "border_bottom_right_symbol" => self.state.border_bottom_right_symbol =
                parameter_value.trim().to_string(),
            "border_fg_color" =>
                self.state.border_foreground_color = load_color_parameter(parameter_value).unwrap(),
            "border_bg_color" =>
                self.state.border_background_color = load_color_parameter(parameter_value).unwrap(),
            _ => return Err(Error::new(ErrorKind::InvalidData,
                                       format!("Invalid parameter name for button {}",
                                               parameter_name)))
        }
        Ok(())
    }


    fn set_id(&mut self, id: String) { self.id = id }

    fn get_id(&self) -> String { self.id.clone() }

    fn set_full_path(&mut self, path: String) {
        self.path = path
    }

    fn get_full_path(&self) -> String {
        self.path.clone()
    }

    fn update_state(&mut self, new_state: &State) {
        let state = new_state.as_button();
        self.state = state.clone();
        self.state.changed = false;
        self.state.force_redraw = false;
    }

    fn get_state(&self) -> State { State::Button(self.state.clone()) }

    fn get_contents(&self, state_tree: &mut StateTree) -> PixelMap {

        let state = state_tree.get(&self.get_full_path()).unwrap().as_button();
        let mut text = state.text.clone().chars().rev().collect::<String>();
        let mut contents = Vec::new();
        let fg_color = if state.flashing {state.flash_foreground_color}
            else if state.selected {state.selection_foreground_color}
            else {state.content_foreground_color};
        let bg_color = if state.flashing {state.flash_background_color}
            else if state.selected {state.selection_background_color}
            else {state.content_background_color};
        for _ in 0..state.get_effective_width() {
            let mut new_y = Vec::new();
            for _ in 0..state.get_effective_height() {
                if !text.is_empty() {
                    new_y.push(Pixel { symbol: text.pop().unwrap().to_string(),
                        foreground_color: fg_color, background_color: bg_color, underline: false })
                } else {
                    new_y.push(Pixel { symbol: " ".to_string(), foreground_color: fg_color,
                        background_color: bg_color, underline: false })
                }
            }
            contents.push(new_y);
        }
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

    fn set_absolute_position(&mut self, pos: Coordinates) { self.absolute_position = pos }

    fn get_absolute_position(&self) -> Coordinates { self.absolute_position }

    fn get_effective_absolute_position(&self) -> Coordinates {
        let (x, y) = self.get_absolute_position();
        (x +if self.state.has_border() {1} else {0}, y +if self.state.has_border() {1} else {0})
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
