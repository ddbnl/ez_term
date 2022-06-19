//! # Ez Parser
//! Module containing structs and functions for paring a .ez file into a root layout.
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use crossterm::style::{Color};
use std::str::FromStr;
use crossterm::terminal::size;
use unicode_segmentation::UnicodeSegmentation;
use crate::common::definitions::Coordinates;
use crate::states::definitions::{HorizontalAlignment, VerticalAlignment, AutoScale, Size,
                                 SizeHint, Padding, ScrollingConfig, PosHint,
                                 HorizontalPositionHint, VerticalPositionHint};
use crate::widgets::layout::{Layout};
use crate::widgets::canvas::Canvas;
use crate::widgets::label::Label;
use crate::widgets::button::Button;
use crate::widgets::radio_button::RadioButton;
use crate::widgets::checkbox::Checkbox;
use crate::widgets::text_input::TextInput;
use crate::widgets::dropdown::Dropdown;
use crate::widgets::widget::{EzObjects, EzObject};
use crate::common::definitions::{StateTree, Templates};
use crate::scheduler::Scheduler;
use crate::widgets::progress_bar::ProgressBar;
use crate::widgets::slider::Slider;
use crate::states::state::GenericState;


/// Load a file path into a root Layout. Return the root widget and a new scheduler. Both will
/// be needed to run an [App].
pub fn load_ez_ui(file_path: &str) -> (Layout, Scheduler) {
    let mut file = File::open(file_path).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");
    let (root_widget, scheduler) = parse_ez(contents).unwrap();
    (root_widget, scheduler)
}


/// Load a string from an Ez file into a root widget. Parse the first level and interpret the
/// widget definition found there as the root widget (must be a layout or panic). Then parse the
/// root widget definition into the actual widget, which will parse sub-widgets, who will parse
/// their sub-widgets, etc. Thus recursively loading the UI.
fn parse_ez(file_string: String) -> Result<(Layout, Scheduler), Error> {

    let config_lines:Vec<String> = file_string.lines().map(|x| x.to_string()).collect();
    let (_, mut widgets, templates) =
        parse_level(config_lines, 0, 0).unwrap();
    if widgets.len() > 1 {
        panic!("There can be only one root widget but {} were found ({:?}). If you meant to use \
        multiple screens, create one root layout with \"mode: screen\" and add the screen \
        layouts to this root.", widgets.len(), widgets);
    }
    let mut root_widget = widgets.pop().unwrap();
    if root_widget.type_name != "Layout" {
        panic!("Root widget of an Ez file must be a Layout")
    }

    let mut scheduler = Scheduler::default();
    let initialized_root_widget = root_widget.parse_as_root(templates, &mut scheduler);

    Ok((initialized_root_widget, scheduler))
}


/// Struct representing a widget definition in a .ez file. Has methods for parsing the
/// definition into an initialized widget. The definition of a widget consists of two optional
/// parts: the config of the widget itself (its' size, color, etc.) and its' sub widgets.
/// As the definition for a widget might contain sub widgets, it is parsed recursively.
#[derive(Clone)]
pub struct EzWidgetDefinition {

    /// Name of widget class, e.g. layout, or textBox
    pub type_name: String,

    /// All raw text content belonging to this definition
    pub content: Vec<String>,

    /// Offset in lines where the content of the widget definition begins in the config file.
    /// Zero-indexed. Indicates the first line AFTER the initial definition starting with '- .
    pub line_offset: usize,

    /// Indentation offset of this widget in the config
    pub indentation_offset: usize,
}
impl EzWidgetDefinition {
    fn new(type_name: String, indentation_offset: usize, line_offset: usize) -> Self {
        EzWidgetDefinition {
            type_name,
            content: Vec::new(),
            indentation_offset,
            line_offset,
        }
    }

    /// Parse a definition as the root layout. The normal parsed method results in a generic
    /// EzObjects enum, whereas the root widget should be a Layout specifically.
    fn parse_as_root(&mut self, mut templates: Templates, scheduler: &mut Scheduler) -> Layout {

        let (config, mut sub_widgets, _) =
            parse_level(self.content.clone(), self.indentation_offset, self.line_offset)
                .unwrap();
        let mut initialized = Layout::new("root".to_string(), "/root".to_string(),
                                                  scheduler);
        for line in config {
            let (parameter_name, parameter_value) = line.split_once(':')
                .unwrap();
            initialized.load_ez_parameter(parameter_name.to_string(),
                                          parameter_value.to_string(),
                                          scheduler);
        }
        for (i, sub_widget) in sub_widgets.iter_mut().enumerate() {
            let initialized_sub_widget = sub_widget.parse(
                &mut templates, scheduler, initialized.get_full_path().clone(),
            i, None);
            initialized.add_child(initialized_sub_widget, scheduler);
        }
        let terminal_size = size().unwrap();
        if initialized.state.get_size().width == 0  {
            initialized.state.get_size_mut().width = terminal_size.0 as usize;
        }
        if initialized.state.get_size().height == 0 {
            initialized.state.get_size_mut().height = terminal_size.1 as usize;
        }
        initialized.state.templates = templates;
        initialized
    }

    /// Parse a definition by separating the config lines from the sub widget definitions. Then
    /// apply the config to the initialized widget, then initialize and add sub widgets.
    pub fn parse(&mut self, templates: &mut Templates, scheduler: &mut Scheduler,
                 parent_path: String, order: usize, merge_config: Option<Vec<String>>)
                 -> EzObjects {

        let (mut config, mut sub_widgets, _) =
            parse_level(self.content.clone(), self.indentation_offset, self.line_offset)
                .unwrap();

        // Templates can have options, and instances of templates can also have options. Merge the
        // configs making sure that the instance config takes precedence.
        let mut merged_config: Vec<String> = Vec::new();
        if let Some(config_to_merge) = merge_config {
            let mut existing_options: Vec<String> = Vec::new();
            for line in config_to_merge {
                merged_config.push(line.clone());
                existing_options.push(line.split_once(':').unwrap().0.to_string());
            }
            for line in config {
                if !merged_config.contains(&line.split_once(':').unwrap().0.to_string()) {
                    merged_config.push(line);
                }
            }
            config = merged_config;
        }
        let initialized = self.initialize(config, templates, scheduler,
                                          parent_path.clone(), order).unwrap();
        let parent_path = initialized.as_ez_object().get_full_path();

        if let EzObjects::Layout(mut obj) = initialized {
            for (i, sub_widget) in sub_widgets.iter_mut().enumerate() {
                let initialized_sub_widget = sub_widget.parse(
                    templates, scheduler, parent_path.clone(),i, None);

                obj.add_child(initialized_sub_widget, scheduler);
            }
            return EzObjects::Layout(obj)
        }
        initialized
    }

    /// Initialize a widget object based on the type specified by the definition.
    fn initialize(&mut self, config: Vec<String>, templates: &mut Templates,
                  scheduler: &mut Scheduler, parent_path: String, order: usize)
        -> Result<EzObjects, Error> {

        // return new root
        if templates.contains_key(&self.type_name) {
            let mut template = templates.get_mut(&self.type_name).unwrap().clone();
            let object = template.parse(templates, scheduler, parent_path,
                                                    order, Some(config));
            Ok(object)
        } else {
            let mut id = String::new();
            for line in config.iter() {
                if line.trim().to_lowercase().starts_with("id:") {
                    id = line.trim().split_once(':').unwrap().1.to_string();
                }
            }
            if id.is_empty() { id = order.to_string() };
            let path = format!("{}/{}", parent_path, id.trim());
            match self.type_name.as_str() {
                "Layout" => Ok(EzObjects::Layout(Layout::from_config(config, id, path,  scheduler))),
                "Canvas" => Ok(EzObjects::CanvasWidget(Canvas::from_config(config, id, path,  scheduler))),
                "Label" => Ok(EzObjects::Label(Label::from_config(config, id, path,  scheduler))),
                "Button" => Ok(EzObjects::Button(Button::from_config(config, id, path, scheduler))),
                "CheckBox" => Ok(EzObjects::Checkbox(Checkbox::from_config(config, id, path,  scheduler))),
                "RadioButton" => Ok(EzObjects::RadioButton(RadioButton::from_config(config, id, path,  scheduler))),
                "TextInput" => Ok(EzObjects::TextInput(TextInput::from_config(config, id, path,  scheduler))),
                "Dropdown" => Ok(EzObjects::Dropdown(Dropdown::from_config(config, id, path,  scheduler))),
                "Slider" => Ok(EzObjects::Slider(Slider::from_config(config, id, path,  scheduler))),
                "ProgressBar" => Ok(EzObjects::ProgressBar(ProgressBar::from_config(config, id, path,  scheduler))),
                _ => Err(Error::new(ErrorKind::InvalidData,
                                    format!("Invalid widget type {}", self.type_name)))
            }
        }
    }
}
impl Debug for EzWidgetDefinition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.type_name)
    }
}

/// Parse a single indentation level of a config file. Returns a Vec of config lines, a Vec
/// of [EzWidgetDefinition] of widgets found on that level, and a Vec of [EzWidgetDefinition] of
/// templates found on that level
fn parse_level(config_lines: Vec<String>, indentation_offset: usize, line_offset: usize)
         -> Result<(Vec<String>, Vec<EzWidgetDefinition>, Templates), Error> {

    // All lines before the first widget definition are considered config lines for the widget
    // on this indentation level
    let mut config = Vec::new();
    let mut parsing_config = true;
    let mut parsing_template: Option<String> = None;
    // All top level widgets on this indentation level
    let mut level = Vec::new();
    let mut templates = HashMap::new();

    for (i, line) in config_lines.clone().into_iter().enumerate() {
        // Skip empty lines and comments
        // find all widget definitions, they start with -
        if line.trim().starts_with("//") || line.trim().is_empty() {
            continue
        } else {
            for (j, char) in line.graphemes(true).enumerate() {
                if char != " " {
                    if parsing_config && j != 0 {
                        panic!("Error at Line {0}: \"{1}\". Invalid indentation between lines \
                        {2} and {0}. Indentation level of line {0} should be {3} but it is {4}.",
                               i + line_offset + 1, line, i + line_offset, indentation_offset,
                               indentation_offset + j);
                    }
                    if j % 4 != 0 {
                        panic!("Error at Line {}: \"{}\". Invalid indentation. indentation must be \
                            in multiples of four.", i + 1 + line_offset, line);
                    }
                    if !parsing_config && !line.starts_with('-') && j < 4 {
                        panic!("Error at Line {0}: \"{1}\". This line must be indented. Try this:\
                        \n{2}{3}\n{4}{1}",
                               i + 1 + line_offset, line, " ".repeat(indentation_offset),
                               config_lines[i-1], " ".repeat(indentation_offset + 4));

                    }
                    break
                }

            }
        }
        // Find widget definitions which starts with -
        if line.starts_with('-') {
            // We encountered a widget, so config section of this level is over.
            parsing_config = false;
            // A new widget definition. Get it's type and ID
            let type_name = line.strip_prefix('-').unwrap().trim()
                .strip_suffix(':')
                .unwrap_or_else(|| panic!("Error at line {}: {}. Widget definition should be \
                followed by a \":\"", i + line_offset + 1, line)).to_string();

            if type_name.starts_with('<') {  // This is a template
                let (type_name, proto_type) = type_name.strip_prefix('<').unwrap()
                    .strip_suffix('>').unwrap().split_once('@').unwrap();
                let def = EzWidgetDefinition::new(
                    proto_type.to_string(),indentation_offset + 4,
                    i + 1 + line_offset);
                templates.insert(type_name.to_string(), def);
                parsing_template = Some(type_name.to_string());
            } else {  // This is a regular widget definition
                // Add to level, all next lines that are not widget definitions append to this widget
                level.push(EzWidgetDefinition::new(
                    type_name.to_string(),indentation_offset + 4,
                    i + 1 + line_offset));
                parsing_template = None;
            }
        }
        else if parsing_config {
            config.push(line);
        } else {
            // Line was not a new widget definition, so it must be config/content of the current one
            let new_line = line.strip_prefix("    ").unwrap_or_else(
                || panic!("Error at line {}: {}. Could not strip indentation.",
                           i + line_offset + 1, line));
            if let Some(name) = &parsing_template {
                templates.get_mut(name).unwrap().content.push(new_line.to_string());
            } else {
                level.last_mut().unwrap().content.push(new_line.to_string());
            }
        }
    }
    Ok((config, level, templates))
}


/// Convenience function use by widgets to load a color parameter defined in a .ez file.
/// Looks like "red".
pub fn load_color_parameter(value: String) -> Color {
    if value.contains(',') {
        let rgb: Vec<&str> = value.split(',').collect();
        if rgb.len() != 3 {
            panic!("Invalid rgb data in Ez file: {:?}. Must be in format: '255, 0, 0'", rgb)
        }
        Color::from(
            (rgb[0].trim().parse().unwrap_or_else(
                |_| panic!("Could not parse the first number in this RGB value: {}", value)),
            rgb[1].trim().parse().unwrap_or_else(
                |_| panic!("Could not parse the second number in this RGB value: {}", value)),
            rgb[2].trim().parse().unwrap_or_else(
                |_| panic!("Could not parse the third number in this RGB value: {}", value)),
            ))
    } else {
        Color::from_str(value.trim()).unwrap()
    }
}


/// Convenience function use by widgets to load a bool parameter defined in a .ez file.
/// Looks like "false".
pub fn load_bool_parameter(value: &str) -> bool {

    if value.to_lowercase() == "true" { true }
    else if value.to_lowercase() == "false" { false }
    else {
        panic!("Ez file bool parameter must be true/false, not: {}", value) }
}


/// Convenience function use by widgets to load a selection order parameter defined in a .ez file.
/// Looks like "4".
pub fn load_selection_order_parameter(value: &str) -> usize {

    let value: usize = value.trim().parse().unwrap_or_else(
        |_| panic!("Could not parse this selection order number: {}", value));
    if value == 0 {
        panic!("selection_order must be higher than 0: {}", value);
    }
    value
}


/// Convenience function use by widgets to load a selection order parameter defined in a .ez file.
/// Looks like "this is text".
pub fn load_text_parameter(mut value: &str) -> String {

    if value.starts_with(' ') {
        value = value.strip_prefix(' ').unwrap();
    }
    value.to_string()
}


/// Convenience function used by widgets to load a size parameter defined in an .ez file
/// Looks like size: 20, 10
pub fn load_size_parameter(value: &str) -> Size {

    let (width_str, height_str) = value.split_once(',').unwrap();
    let width = width_str.trim().parse().unwrap_or_else(
        |_| panic!("Could not parse width of this position: {}", width_str));
    let height = height_str.trim().parse().unwrap_or_else(
        |_| panic!("Could not parse height of this position: {}", height_str));
    Size::new(width, height)
}


/// Convenience function used by widgets to load a full auto_scale parameter defined in an .ez file
/// Looks like "auto_scale: true, false"
pub fn load_full_enable_scrolling_parameter(
    value: &str, scrolling_config: &mut ScrollingConfig) {

    let (x_str, y_str) = value.split_once(',').unwrap();
    let x = load_bool_parameter(x_str.trim());
    let y = load_bool_parameter(y_str.trim());
    scrolling_config.enable_x = x;
    scrolling_config.enable_y = y;
}


/// Convenience function used by widgets to load a full auto_scale parameter defined in an .ez file
/// Looks like "auto_scale: true, false"
pub fn load_full_auto_scale_parameter(value: &str) -> AutoScale {

    let (width_str, height_str) = value.split_once(',').unwrap();
    let width = load_bool_parameter(width_str.trim());
    let height = load_bool_parameter(height_str.trim());
    AutoScale::new(width, height)
}


/// Convenience function used by widgets to load a full padding parameter defined in an .ez file
/// Looks like "padding: 2, 2, 2, 2"
pub fn load_full_padding_parameter(value: &str) -> Padding {

    let strings: Vec<&str> = value.split(',').collect();
    if strings.len() != 4 {
        if strings.len() == 1 {
            panic!("Padding parameter must have four values, e.g.: \"2, 2, 2, 2\". You used one \
            value; perhaps you meant to use \"padding_top\", \"padding_bottom\",\
            \"padding_left\" or \"padding_right\"?.")

        } else if strings.len() == 2 {
            panic!("Padding parameter must have four values, e.g.: \"2, 2, 2, 2\". You used two \
            values; perhaps you meant to use \"padding_x\" or \"padding_y\"?.")
        }
        else {
            panic!("Padding parameter must have four values, e.g.: \"2, 2, 2, 2\".")
        }
    }
    let top = strings[0].trim().parse().unwrap();
    let bottom = strings[1].trim().parse().unwrap();
    let left = strings[2].trim().parse().unwrap();
    let right = strings[3].trim().parse().unwrap();
    Padding::new(top, bottom, left, right)
}


/// Convenience function used by widgets to load an x padding parameter defined in an .ez file
/// Looks like "padding_x: 2, 2"
pub fn load_padding_x_parameter(value: &str) -> Padding {

    let (left_str, right_str) = value.split_once(',').unwrap();
    let left = left_str.trim().parse().unwrap();
    let right = right_str.trim().parse().unwrap();
    Padding::new(0, 0, left, right)
}


/// Convenience function used by widgets to load a y padding parameter defined in an .ez file
/// Looks like "padding_y: 2, 2"
pub fn load_padding_y_parameter(value: &str) -> Padding {

    let (top_str, bottom_str) = value.split_once(',').unwrap();
    let top = top_str.trim().parse().unwrap();
    let bottom = bottom_str.trim().parse().unwrap();
    Padding::new(top, bottom, 0, 0)
}


/// Convenience function use by widgets to load a size_hint parameter defined in a .ez file.
/// Looks like "0.33, 0.33" or "1/3, 1/3"
pub fn load_full_size_hint_parameter(value: &str) -> SizeHint {

    let (x_str, y_str) = value.split_once(',').unwrap();
    let x = load_size_hint_parameter(x_str.trim());
    let y = load_size_hint_parameter(y_str.trim());
    SizeHint::new(x, y)
}


/// Convenience function use by widgets to load a size_hint parameter defined in a .ez file.
/// Looks like "0.33" or "1/3"
pub fn load_size_hint_parameter(value: &str) -> Option<f64> {

    let to_parse = value.trim();
    // Size hint can be None
    if to_parse.to_lowercase() == "none" {
        None
    }
    // Size hint can be a fraction
    else if to_parse.contains('/') {
        let (left_str, right_str) = to_parse.split_once('/').unwrap_or_else(
            || panic!("Size hint contains an invalid fraction: {}. Must be in format '1/3'",
                       value));
        let left: f64 = left_str.trim().parse().unwrap_or_else(
            |_| panic!("Could not parse left side of size hint fraction: {}", value));
        let right: f64 = right_str.trim().parse().unwrap_or_else(
            |_| panic!("Could not parse right side of size hint fraction: {}", value));
        let result = left / right;
        Some(result)
    }
    // Size hint can be a straight number
    else {
        let size_hint = value.parse().unwrap_or_else(
            |_| panic!("Could not parse this size hint number: {}", value));
        Some(size_hint)
    }
}


/// Convenience function used by widgets to load a pos parameter defined in an .ez file
/// Looks like pos: 20, 10
pub fn load_pos_parameter(value: &str) -> Coordinates {

    let (x_str, y_str) = value.split_once(',').unwrap();
    let x = x_str.to_string().parse().unwrap_or_else(
        |_| panic!("Could not parse x coordinate of this position: {}", value));
    let y = y_str.to_string().parse().unwrap_or_else(
        |_| panic!("Could not parse y coordinate of this position: {}", value));
    Coordinates::new(x, y)
}


/// Convenience function use by widgets to load a full pos_hint tuple parameter defined in a .ez
/// file. Looks like: "pos_hint: center_x, bottom: 0.9"
pub fn load_full_pos_hint_parameter(value: &str) -> PosHint {

    let (x_str, y_str) = value.split_once(',').unwrap();
    let x = load_pos_hint_x_parameter(x_str);
    let y = load_pos_hint_y_parameter(y_str);
    PosHint::new(x, y)
}


/// Convenience function use by widgets to load a pos_hint parameter defined in a .ez file.
/// Looks like "pos_hint_x: right: 0.9"
pub fn load_pos_hint_x_parameter(value: &str) -> Option<(HorizontalPositionHint, f64)> {

    let to_parse = value.trim();
    // Pos hint can be None
    if to_parse.to_lowercase() == "none" {
        return None
    }
    // pos hint can one or two values. E.g. "top" or "top:0.8"
    let (keyword, fraction);
    if to_parse.contains(':') {
        let (keyword_str, fraction_str) = to_parse.split_once(':').unwrap();
        fraction = fraction_str.trim().parse().unwrap_or_else(
            |_| panic!("Could not parse pos hint: {}", value));
        keyword = keyword_str.trim();
    } else {
        keyword = value.trim();
        fraction = 1.0;  // Default fraction
    }
    let pos = match keyword {
        "left" => HorizontalPositionHint::Left,
        "right" => HorizontalPositionHint::Right,
        "center" => HorizontalPositionHint::Center,
        _ => panic!("This value is not allowed for pos_hint_x: {}. Use left/right/center",
                      value)
    };
    Some((pos, fraction))
}


/// Convenience function use by widgets to load a pos_hint_y parameter defined in a .ez file
/// Looks like "pos_hint_y: bottom: 0.9"
pub fn load_pos_hint_y_parameter(value: &str)
    -> Option<(VerticalPositionHint, f64)> {

    let to_parse = value.trim();
    // Pos hint can be None
    if to_parse.to_lowercase() == "none" {
        return None
    }
    // pos hint can one or two values. E.g. "top" or "top:0.8"
    let (keyword, fraction);
    if to_parse.contains(':') {
        let (keyword_str, fraction_str) = to_parse.split_once(':').unwrap();
        fraction = fraction_str.trim().parse().unwrap_or_else(
            |_| panic!("Could not parse pos hint: {}", value));
        keyword = keyword_str.trim();
    } else {
        keyword = value.trim();
        fraction= 1.0;  // Default fraction
    }
    let pos = match keyword {
        "top" => VerticalPositionHint::Top,
        "bottom" => VerticalPositionHint::Bottom,
        "middle" => VerticalPositionHint::Middle,
        _ => panic!("This value is not allowed for pos_hint_y: {}. Use top/bottom/middle",
                      value)
    };
    Some((pos, fraction))
}


/// Convenience function use by widgets to load a horizontal alignment defined in a .ez file.
/// Looks like: "left"
pub fn load_halign_parameter(value: &str) -> HorizontalAlignment {

    if value.to_lowercase() == "left" { HorizontalAlignment::Left }
    else if value.to_lowercase() == "right" { HorizontalAlignment::Right }
    else if value.to_lowercase() == "center" { HorizontalAlignment::Center }
    else { panic!("halign parameter must be left/right/center: {}", value) }
}


/// Convenience function use by widgets to load a vertical alignment defined in a .ez file
/// Looks like: "bottom"
pub fn load_valign_parameter(value: &str) -> VerticalAlignment {

    if value.to_lowercase() == "top" { VerticalAlignment::Top }
    else if value.to_lowercase() == "bottom" { VerticalAlignment::Bottom }
    else if value.to_lowercase() == "middle" { VerticalAlignment::Middle }
    else { panic!("valign parameter must be left/right/center: {}", value) }
}


pub fn load_int_parameter(mut value: &str, scheduler: &mut Scheduler, path: String,
                          update_func: Box<dyn FnMut(&mut StateTree, usize)>) -> usize {

    if value.starts_with("parent.") {
        let mut new_path = path;
        loop {
            let (parent, path) = value.split_once("parent.").unwrap();
            value = path;
            new_path = new_path.rsplit_once('/').unwrap().0.to_string();
            if parent.is_empty() {
                new_path = format!("{}/{}", new_path, value.replace('.', "/"));
                break
            }
        }
        scheduler.subscribe_to_usize_callback(new_path, update_func);
        0
    } else if value.starts_with("properties.") {
        let new_path = value.strip_prefix("properties.").unwrap();
        scheduler.subscribe_to_usize_callback(new_path.to_string(), update_func);
        0
    } else {
        value.trim().parse().unwrap()
    }
}


/// Load a parameter common to all [EzObjects]. Returns a bool representing whether the parameter
/// was consumed. If not consumed it should be a parameter specific to a widget.
pub fn load_common_parameters(parameter_name: &str, parameter_value: String,
                              mut obj: Box<&mut dyn EzObject>, scheduler: &mut Scheduler) -> bool {

    let path = obj.get_full_path().clone();
    let mut state = obj.get_state_mut();
    match parameter_name {
        "id" => obj.set_id(parameter_value.trim().to_string()),
        "x" => {
            state.set_x(load_int_parameter(parameter_value.trim(), scheduler, path.clone(),
                                           Box::new(move |state_tree: &mut StateTree, val: usize| {
                                               state_tree.get_mut(&path.clone())
                                                   .unwrap().as_generic_mut().set_x(val);
                                           })))
        },
        "y" => state.set_y(parameter_value.trim().parse().unwrap()),
        "pos" => {
            let coordinates = load_pos_parameter(parameter_value.trim());
            state.set_x(coordinates.x);
            state.set_y(coordinates.y);
        },
        "size_hint" => state.set_size_hint(
            load_full_size_hint_parameter(parameter_value.trim())),
        "size_hint_x" => state.set_size_hint_x(
            load_size_hint_parameter(parameter_value.trim())),
        "size_hint_y" => state.set_size_hint_y(
            load_size_hint_parameter(parameter_value.trim())),
        "size" => state.set_size(load_size_parameter(parameter_value.trim())),
        "width" => state.get_size_mut().width = parameter_value.trim().parse().unwrap(),
        "height" => state.get_size_mut().height = parameter_value.trim().parse().unwrap(),
        "pos_hint" => state.set_pos_hint(load_full_pos_hint_parameter(parameter_value.trim())),
        "pos_hint_x" => state.set_pos_hint_x(
            load_pos_hint_x_parameter(parameter_value.trim())),
        "pos_hint_y" => state.set_pos_hint_y(
            load_pos_hint_y_parameter(parameter_value.trim())),
        "auto_scale" => state.set_auto_scale(load_full_auto_scale_parameter(
            parameter_value.trim())),
        "auto_scale_width" =>
            state.set_auto_scale_width(load_bool_parameter(parameter_value.trim())),
        "auto_scale_height" =>
            state.set_auto_scale_height(load_bool_parameter(parameter_value.trim())),
        "padding" => state.set_padding(load_full_padding_parameter(parameter_value.trim())),
        "disabled" => state.set_disabled(load_bool_parameter(parameter_value.trim())),
        "selection_order" => { state.set_selection_order(
            load_selection_order_parameter(parameter_value.as_str())) },
        "padding_x" => state.set_padding(load_padding_x_parameter(parameter_value.trim())),
        "padding_y" => state.set_padding(load_padding_y_parameter(parameter_value.trim())),
        "padding_top" => state.set_padding_top(parameter_value.trim().parse().unwrap()),
        "padding_bottom" => state.set_padding_bottom(parameter_value.trim().parse().unwrap()),
        "padding_left" => state.set_padding_left(parameter_value.trim().parse().unwrap()),
        "padding_right" => state.set_padding_right(parameter_value.trim().parse().unwrap()),
        "halign" => state.set_horizontal_alignment(
            load_halign_parameter(parameter_value.trim())),
        "valign" => state.set_vertical_alignment(
            load_valign_parameter(parameter_value.trim())),
        "fg_color" => state.get_colors_config_mut().foreground =
            load_color_parameter(parameter_value),
        "bg_color" => state.get_colors_config_mut().background =
            load_color_parameter(parameter_value),
        "disabled_fg_color" => state.get_colors_config_mut().disabled_foreground =
            load_color_parameter(parameter_value),
        "disabled_bg_color" => state.get_colors_config_mut().disabled_background =
            load_color_parameter(parameter_value),
        "selection_fg_color" => state.get_colors_config_mut().selection_foreground =
            load_color_parameter(parameter_value),
        "selection_bg_color" => state.get_colors_config_mut().selection_background =
            load_color_parameter(parameter_value),
        "flash_fg_color" => state.get_colors_config_mut().flash_foreground =
            load_color_parameter(parameter_value),
        "flash_bg_color" => state.get_colors_config_mut().flash_background =
            load_color_parameter(parameter_value),
        "tab_fg_color" => state.get_colors_config_mut().tab_foreground =
            load_color_parameter(parameter_value),
        "tab_bg_color" => state.get_colors_config_mut().tab_background =
            load_color_parameter(parameter_value),
        "fill_fg_color" => state.get_colors_config_mut().filler_foreground =
            load_color_parameter(parameter_value),
        "fill_bg_color" => state.get_colors_config_mut().filler_background =
            load_color_parameter(parameter_value),
        "cursor_color" =>
            state.get_colors_config_mut().cursor = load_color_parameter(parameter_value),
        "border" => state.get_border_config_mut().enabled =
            load_bool_parameter(parameter_value.trim()),
        "border_horizontal_symbol" => state.get_border_config_mut().horizontal_symbol =
            parameter_value.trim().to_string(),
        "border_vertical_symbol" => state.get_border_config_mut().vertical_symbol =
            parameter_value.trim().to_string(),
        "border_top_right_symbol" => state.get_border_config_mut().top_right_symbol =
            parameter_value.trim().to_string(),
        "border_top_left_symbol" => state.get_border_config_mut().top_left_symbol =
            parameter_value.trim().to_string(),
        "border_bottom_left_symbol" => state.get_border_config_mut().bottom_left_symbol =
            parameter_value.trim().to_string(),
        "border_bottom_right_symbol" => state.get_border_config_mut().bottom_right_symbol =
            parameter_value.trim().to_string(),
        "border_fg_color" => state.get_border_config_mut().fg_color =
            load_color_parameter(parameter_value),
        "border_bg_color" => state.get_border_config_mut().bg_color =
            load_color_parameter(parameter_value),
        _ => return false,
    }
    true
}