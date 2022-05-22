//! # Ez Parser
//! Module containing structs and functions for paring a .ez file into a root layout.
use std::fs::File;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use crossterm::style::Color;
use crate::widgets::layout::{Layout};
use crate::widgets::canvas_widget::CanvasWidget;
use crate::widgets::label::Label;
use crate::widgets::button::Button;
use crate::widgets::radio_button::RadioButton;
use crate::widgets::checkbox::Checkbox;
use crate::widgets::text_input::TextInput;
use crate::widgets::dropdown::Dropdown;
use crate::widgets::widget::{EzObjects, EzObject};
use std::str::FromStr;
use crossterm::terminal::size;
use crate::scheduler::Scheduler;
use crate::widgets::state::{GenericState, HorizontalAlignment, VerticalAlignment};


/// Load a file path into a root Layout. Return the root widget and a new scheduler. Both will
/// be needed to run an [App].
pub fn load_ez_ui(file_path: &str) -> (Layout, Scheduler) {
    let mut file = File::open(file_path).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");
    let root_widget = parse_ez(contents).unwrap();
    let scheduler = Scheduler::new();
    (root_widget, scheduler)
}


/// Load a string into a root widget.
fn parse_ez(file_string: String) -> Result<Layout, Error> {

    let config_lines:Vec<&str> = file_string.lines().collect();
    let (_, mut widgets) =
        parse_level(config_lines).unwrap();
    let mut root_widget = widgets.pop().unwrap();
    let mut initialized_root_widget = root_widget.parse_as_root();
    // Set full paths for all widgets now that they have all been initialized.
    initialized_root_widget.propagate_paths();

    Ok(initialized_root_widget)
}


/// Struct representing a widget definition in a .ez file. Has methods for parsing the
/// definition into an initialized widget. The definition of a widget consists of two optional
/// parts: the config of the widget itself (its' size, color, etc.) and its' sub widgets.
/// As the definition for a widget might contain sub widgets, it is parsed recursively.
struct EzWidgetDefinition<'a> {
    /// Name of widget class, e.g. layout, or textBox
    pub type_name: &'a str,
    /// Id of the widget, used to create widget paths
    pub id: &'a str,
    /// All raw text content belonging to this definition
    pub content: Vec<&'a str>,
}
impl<'a> EzWidgetDefinition<'a> {
    fn new(type_name: &'a str, id: &'a str) -> Self {
        EzWidgetDefinition {
            type_name,
            id,
            content: Vec::new(),
        }
    }

    /// Parse a definition as the root layout. The normal parsed method results in a generic
    /// EzObjects enum, whereas the root widget should be a Layout specifically.
    fn parse_as_root(&mut self) -> Layout {

        let (config, mut sub_widgets) =
            parse_level(self.content.clone()).unwrap();
        let mut initialized = Layout::default();
        for line in config {
            let (parameter_name, parameter_value) = line.split_once(':')
                .unwrap();
            initialized.load_ez_parameter(parameter_name.to_string(),
                                          parameter_value.to_string()).unwrap();
        }
        for sub_widget in sub_widgets.iter_mut() {
            let initialized_sub_widget = sub_widget.parse();
            initialized.add_child(initialized_sub_widget);
        }
        let terminal_size = size().unwrap();
        if initialized.state.width == 0  {
            initialized.state.set_width(terminal_size.0 as usize - 1);
        }
        if initialized.state.height == 0 {
            initialized.state.set_height(terminal_size.1 as usize - 5);
        }
        initialized.set_id(self.id.to_string());
        initialized.set_full_path(format!("/{}", self.id));
        initialized
    }

    /// Parse a definition by separating the config lines from the sub widget definitions. Then
    /// apply the config to the initialized widget, then initialize and add sub widgets.
    fn parse(&mut self) -> EzObjects {

        let (config, mut sub_widgets) =
            parse_level(self.content.clone()).unwrap();
        let initialized = self.initialize(config).unwrap();
        if let EzObjects::Layout(mut i) = initialized {
            for sub_widget in sub_widgets.iter_mut() {
                let initialized_sub_widget = sub_widget.parse();
                i.add_child(initialized_sub_widget);
            }
            return EzObjects::Layout(i)
        }
        initialized
    }

    /// Initialize a widget object based on the type specified by the definition.
    fn initialize(&mut self, config: Vec<&str>) -> Result<EzObjects, Error> {
        match self.type_name {
            "Layout" => Ok(EzObjects::Layout(Layout::from_config(config, self.id.to_string()))),
            "Canvas" => Ok(EzObjects::CanvasWidget(
                CanvasWidget::from_config(config, self.id.to_string()))),
            "Label" => Ok(EzObjects::Label(
                Label::from_config(config, self.id.to_string()))),
            "Button" => Ok(EzObjects::Button(
                Button::from_config(config, self.id.to_string()))),
            "CheckBox" => Ok(EzObjects::Checkbox(
                Checkbox::from_config(config, self.id.to_string()))),
            "RadioButton" => Ok(EzObjects::RadioButton(
                RadioButton::from_config(config, self.id.to_string()))),
            "TextInput" => Ok(EzObjects::TextInput(
                TextInput::from_config(config, self.id.to_string()))),
            "Dropdown" => Ok(EzObjects::Dropdown(
                Dropdown::from_config(config, self.id.to_string()))),
            _ => Err(Error::new(ErrorKind::InvalidData,
                                format!("Invalid widget type {}", self.type_name)))
        }
    }
}

/// Parse a single indentation level of a config file.
fn parse_level<'a>(config_lines: Vec<&'a str>)
                       -> Result<(Vec<&str>, Vec<EzWidgetDefinition<'a>>), Error> {

    // All lines before the first widget definition are considered config lines for the widget
    // on this indentation level
    let mut config = Vec::new();
    let mut parsing_config = true;
    // All top level widgets on this indentation level
    let mut level = Vec::new();

    for (i, line) in config_lines.into_iter().enumerate() {
        // find all widget definitions, they start with -
        if line.starts_with('-') {
            // We encountered a widget, so config section of this level is over.
            parsing_config = false;
            // A new widget definition. Get it's type and ID
            let (type_name, id) = line.split_once(':').unwrap();
            // Add to level, all next lines that are not widget definitions append to this widget
            level.push(EzWidgetDefinition::new(type_name.strip_prefix("- ").unwrap().trim(), id.trim()));

        }
        else if parsing_config {
            config.push(line);
        } else {
            // Line was not a new widget definition, so it must be config/content of the current one
            if  !line.starts_with("    ") {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("Error at Line {}: '{}'. Invalid indentation. \
                    4 whitespaces per level required.", i, line)))
            }
            let new_line = line.strip_prefix("    ").unwrap();
            level.last_mut().unwrap().content.push(new_line);
        }
    }
    Ok((config, level))
}


/// Convenience function use by widgets to load a color parameter defined in a .ez file
pub fn load_color_parameter(value: String) -> Result<Color, Error> {
    if value.contains(',') {
        let rgb: Vec<&str> = value.split(',').collect();
        if rgb.len() != 3 {
            return Err(Error::new(ErrorKind::InvalidData,
                                  format!("Invalid rgb data: {:?}", rgb)))
        }
        Ok(Color::from((rgb[0].trim().parse().unwrap(),
                        rgb[1].trim().parse().unwrap(),
                        rgb[2].trim().parse().unwrap())))
    } else {
        Ok(Color::from_str(value.trim()).unwrap())
    }
}

/// Convenience function use by widgets to load a bool parameter defined in a .ez file
pub fn load_bool_parameter(value: &str) -> Result<bool, Error> {

    if value.to_lowercase() == "true" { Ok(true) }
    else if value.to_lowercase() == "false" { Ok(false) }
    else { Err(Error::new(ErrorKind::InvalidData, "bool parameter must be true/false")) }
}


/// Convenience function use by widgets to load a selection order parameter defined in a .ez file
pub fn load_selection_order_parameter(value: &str) -> Result<usize, Error> {

    let value: usize = value.trim().parse().unwrap();
    if value == 0 {
        return Err(Error::new(ErrorKind::InvalidData,
                              "selection_order must be higher than 0."))
    }
    Ok(value)
}

/// Convenience function use by widgets to load a selection order parameter defined in a .ez file
pub fn load_text_parameter(mut value: &str) -> Result<String, Error> {

    if value.starts_with(' ') {
        value = value.strip_prefix(' ').unwrap();
    }
    Ok(value.to_string())
}

/// Convenience function use by widgets to load a size_hint parameter defined in a .ez file
pub fn load_size_hint(value: &str) -> Result<Option<f64>, Error> {

    let to_parse = value.trim();
    // Size hint can be None
    if to_parse.to_lowercase() == "none" {
        Ok(None)
    }
    // Size hint can be a fraction
    else if to_parse.contains("/") {
        let (left_str, right_str) = to_parse.split_once('/').unwrap();
        let left: f64 = left_str.trim().parse().unwrap();
        let right: f64 = right_str.trim().parse().unwrap();
        let result = left / right;
        Ok(Some(result))
    }
    // Size hint can be a straight number
    else {
        Ok(Some(value.parse().unwrap()))
    }
}


/// Convenience function use by widgets to load a horizontal alignment defined in a .ez file
pub fn load_halign_parameter(value: &str) -> Result<HorizontalAlignment, Error> {

    if value.to_lowercase() == "left" { Ok(HorizontalAlignment::Left) }
    else if value.to_lowercase() == "right" { Ok(HorizontalAlignment::Right) }
    else if value.to_lowercase() == "center" { Ok(HorizontalAlignment::Center) }
    else { Err(Error::new(ErrorKind::InvalidData,
                          "halign parameter must be left/right/center")) }
}

/// Convenience function use by widgets to load a horizontal alignment defined in a .ez file
pub fn load_valign_parameter(value: &str) -> Result<VerticalAlignment, Error> {

    if value.to_lowercase() == "top" { Ok(VerticalAlignment::Top) }
    else if value.to_lowercase() == "bottom" { Ok(VerticalAlignment::Bottom) }
    else if value.to_lowercase() == "middle" { Ok(VerticalAlignment::Middle) }
    else { Err(Error::new(ErrorKind::InvalidData,
                          "halign parameter must be left/right/center")) }
}