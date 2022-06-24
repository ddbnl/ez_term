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
use crate::common::definitions::{EzPropertyUpdater};
use crate::states::definitions::{HorizontalAlignment, VerticalAlignment};
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
use crate::property::EzValues;
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
        if initialized.state.get_size().width.get() == &0  {
            initialized.state.get_size_mut().width.set(terminal_size.0 as usize);
        }
        if initialized.state.get_size().height.get() == &0 {
            initialized.state.get_size_mut().height.set(terminal_size.1 as usize);
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
            let mut template =
                templates.get_mut(&self.type_name).unwrap().clone();
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


/* Parsing funcs */
/// Convenience function use by widgets to load a color property defined in a .ez file.
/// Looks like "red".
pub fn parse_color_property(value: &str) -> Color {
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


/// Convenience function use by widgets to load a bool property defined in a .ez file.
/// Looks like "false".
pub fn parse_bool_property(value: &str) -> bool {

    if value.to_lowercase() == "true" { true }
    else if value.to_lowercase() == "false" { false }
    else {
        panic!("Ez file bool property must be true/false, not: {}", value) }
}


/// Convenience function use by widgets to load a size_hint property defined in a .ez file.
/// Looks like "0.33" or "1/3"
pub fn parse_size_hint_property(value: &str) -> Option<f64> {

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


/// Convenience function use by widgets to load a pos_hint property defined in a .ez file.
/// Looks like "pos_hint_x: right: 0.9"
pub fn parse_horizontal_pos_hint_property(value: &str) -> Option<(HorizontalAlignment, f64)> {

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
        "left" => HorizontalAlignment::Left,
        "right" => HorizontalAlignment::Right,
        "center" => HorizontalAlignment::Center,
        _ => panic!("This value is not allowed for pos_hint_x: {}. Use left/right/center",
                    value)
    };
    Some((pos, fraction))
}


/// Convenience function use by widgets to load a pos_hint_y property defined in a .ez file
/// Looks like "pos_hint_y: bottom: 0.9"
pub fn parse_vertical_pos_hint_property(value: &str)
                                         -> Option<(VerticalAlignment, f64)> {

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
        "top" => VerticalAlignment::Top,
        "bottom" => VerticalAlignment::Bottom,
        "middle" => VerticalAlignment::Middle,
        _ => panic!("This value is not allowed for pos_hint_y: {}. Use top/bottom/middle",
                    value)
    };
    Some((pos, fraction))
}


/// Convenience function use by widgets to load a horizontal alignment defined in a .ez file.
/// Looks like: "left"
pub fn parse_halign_property(value: &str) -> HorizontalAlignment {

    if value.to_lowercase() == "left" { HorizontalAlignment::Left }
    else if value.to_lowercase() == "right" { HorizontalAlignment::Right }
    else if value.to_lowercase() == "center" { HorizontalAlignment::Center }
    else { panic!("halign property must be left/right/center: {}", value) }
}


/// Convenience function use by widgets to load a vertical alignment defined in a .ez file
/// Looks like: "bottom"
pub fn parse_valign_property(value: &str) -> VerticalAlignment {

    if value.to_lowercase() == "top" { VerticalAlignment::Top }
    else if value.to_lowercase() == "bottom" { VerticalAlignment::Bottom }
    else if value.to_lowercase() == "middle" { VerticalAlignment::Middle }
    else { panic!("valign property must be left/right/center: {}", value) }
}


/* Base property loaders */
/// Bind a property to another property if the user-passed property declares it. Returns true
/// if the property was bound, else false.
pub fn bind_ez_property(value: &str, scheduler: &mut Scheduler, path: String,
                        update_func: EzPropertyUpdater) -> bool {

    if value.starts_with("parent.") {
        let new_path = resolve_parent_path(path, value);
        scheduler.subscribe_to_ez_property(new_path, update_func);
        true
    } else if value.starts_with("properties.") {
        let new_path = value.strip_prefix("properties.").unwrap();
        scheduler.subscribe_to_ez_property(new_path.to_string(), update_func);
        true
    } else {
        false
    }
}


pub fn load_ez_int_property(value: &str, scheduler: &mut Scheduler, path: String,
                            update_func: EzPropertyUpdater) -> usize {

    if bind_ez_property(value, scheduler, path, update_func) {
        0
    } else {
        value.trim().parse().unwrap()
    }
}


pub fn load_ez_bool_property(value: &str, scheduler: &mut Scheduler, path: String,
                             update_func: EzPropertyUpdater) -> bool {

    if bind_ez_property(value, scheduler, path, update_func) {
        false
    } else {
        parse_bool_property(value)
    }
}


pub fn load_ez_string_property(value: &str, scheduler: &mut Scheduler, path: String,
                               update_func: EzPropertyUpdater) -> String {

    if bind_ez_property(value, scheduler, path, update_func) {
        String::new()
    } else {
        value.trim().to_string()
    }
}


pub fn load_ez_color_property(value: &str, scheduler: &mut Scheduler, path: String,
                              update_func: EzPropertyUpdater) -> Color {

    if bind_ez_property(value, scheduler, path, update_func) {
        Color::Black
    } else {
        parse_color_property(value)
    }
}


pub fn load_ez_valign_property(value: &str, scheduler: &mut Scheduler, path: String,
                               update_func: EzPropertyUpdater) -> VerticalAlignment {

    if bind_ez_property(value, scheduler, path, update_func) {
        VerticalAlignment::Top
    } else {
        parse_valign_property(value)
    }
}


pub fn load_ez_halign_property(value: &str, scheduler: &mut Scheduler, path: String,
                               update_func: EzPropertyUpdater) -> HorizontalAlignment {

    if bind_ez_property(value, scheduler, path, update_func) {
        HorizontalAlignment::Left
    } else {
        parse_halign_property(value)
    }
}


pub fn load_ez_horizontal_pos_hint_property(value: &str, scheduler: &mut Scheduler, path: String,
                                            update_func: EzPropertyUpdater)
                                            -> Option<(HorizontalAlignment, f64)> {

    if bind_ez_property(value, scheduler, path, update_func) {
        None
    } else {
        parse_horizontal_pos_hint_property(value)
    }
}


pub fn load_ez_vertical_pos_hint_property(value: &str, scheduler: &mut Scheduler, path: String,
                                          update_func: EzPropertyUpdater)
                                          -> Option<(VerticalAlignment, f64)> {

    if bind_ez_property(value, scheduler, path, update_func) {
        None
    } else {
        parse_vertical_pos_hint_property(value)
    }
}


pub fn load_ez_size_hint_property(value: &str, scheduler: &mut Scheduler, path: String,
                                  update_func: EzPropertyUpdater) -> Option<f64> {

    if bind_ez_property(value, scheduler, path, update_func) {
        None
    } else {
        parse_size_hint_property(value)
    }
}


/// Resolve a parent containing path that was passed in an Ez file property.
/// E.g. 'parent.parent.label.size' becomes /root/layout_1/layout_2/label/size
fn resolve_parent_path(mut path: String, mut value: &str) -> String {

    loop {
        let (parent, sub_path) = value.split_once("parent.").unwrap();
        value = sub_path;
        path = path.rsplit_once('/').unwrap().0.to_string();
        if parent.is_empty() {
            path = format!("{}/{}", path, value.replace('.', "/"));
            break
        }
    }
    path.to_string()
}

/* Specific property loaders */
/// Convenience function use by widgets to load a selection order property defined in a .ez file.
/// Looks like "4".
pub fn load_selection_order_property(value: &str) -> usize {

    let value: usize = value.trim().parse().unwrap_or_else(
        |_| panic!("Could not parse this selection order number: {}", value));
    if value == 0 {
        panic!("selection_order must be higher than 0: {}", value);
    }
    value
}


pub fn load_full_pos_hint_property(state: &mut dyn GenericState, property_value: &str,
                                   scheduler: &mut Scheduler, path: String) {

    let (x_str, y_str) = property_value.split_once(',').unwrap();
    load_horizontal_pos_hint_property(state, x_str, scheduler, path.clone());
    load_vertical_pos_hint_property(state, y_str, scheduler, path);
}


pub fn load_horizontal_pos_hint_property(state: &mut dyn GenericState, property_value: &str,
                                         scheduler: &mut Scheduler, path: String) {

    state.set_pos_hint_x(load_ez_horizontal_pos_hint_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.set_pos_hint_x(*val.as_horizontal_pos_hint());
            path.clone()
        })))
}


pub fn load_vertical_pos_hint_property(state: &mut dyn GenericState, property_value: &str,
                                       scheduler: &mut Scheduler, path: String) {

    state.set_pos_hint_y(load_ez_vertical_pos_hint_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.set_pos_hint_y(*val.as_vertical_pos_hint());
            path.clone()
        })))
}


/// Convenience function use by widgets to load a size_hint property defined in a .ez file.
/// Looks like "0.33, 0.33" or "1/3, 1/3"
pub fn load_full_size_hint_property(state: &mut dyn GenericState, property_value: &str,
                                    scheduler: &mut Scheduler, path: String) {

    let (x_str, y_str) = property_value.split_once(',').unwrap();
    load_size_hint_x_property(state, x_str, scheduler,path.clone());
    load_size_hint_y_property(state, y_str, scheduler, path);
}


pub fn load_size_hint_x_property(state: &mut dyn GenericState, property_value: &str,
                                          scheduler: &mut Scheduler, path: String) {

    state.set_size_hint_x(load_ez_size_hint_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.set_size_hint_x(*val.as_size_hint());
            path.clone()
        })))
}


pub fn load_size_hint_y_property(state: &mut dyn GenericState, property_value: &str,
                                  scheduler: &mut Scheduler, path: String) {

    state.set_size_hint_y(load_ez_size_hint_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.set_size_hint_y(*val.as_size_hint());
            path.clone()
        })))
}


pub fn load_full_auto_scale_property(state: &mut dyn GenericState, property_value: &str,
                                      scheduler: &mut Scheduler, path: String) {

    let (width_str, height_str) = property_value.split_once(',').unwrap();
    load_auto_scale_width_property(state, width_str, scheduler,
                                   path.clone());
    load_auto_scale_height_property(state, height_str, scheduler, path);
}


pub fn load_auto_scale_width_property(state: &mut dyn GenericState, property_value: &str,
                                       scheduler: &mut Scheduler, path: String) {

    state.set_auto_scale_width(load_ez_bool_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.set_auto_scale_width(val.as_bool().clone());
            path.clone()
        })))
}


pub fn load_auto_scale_height_property(state: &mut dyn GenericState, property_value: &str,
                                        scheduler: &mut Scheduler, path: String) {

    state.set_auto_scale_height(load_ez_bool_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.set_auto_scale_height(val.as_bool().clone());
            path.clone()
        })))
}


pub fn load_border_enable_property(state: &mut dyn GenericState, property_value: &str,
                                    scheduler: &mut Scheduler, path: String) {

    state.get_border_config_mut().enabled.set(load_ez_bool_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.get_border_config_mut().enabled.set(val.as_bool().clone());
            path.clone()
        })))
}


pub fn load_x_property(state: &mut dyn GenericState, property_value: &str,
                        scheduler: &mut Scheduler, path: String) {

    state.set_x(load_ez_int_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.set_x(val.as_usize().clone());
            path.clone()
        })))
}


pub fn load_y_property(state: &mut dyn GenericState, property_value: &str,
                        scheduler: &mut Scheduler, path: String) {

    state.set_y(load_ez_int_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.set_y(val.as_usize().clone());
            path.clone()
        })))
}


pub fn load_width_property(state: &mut dyn GenericState, property_value: &str,
                            scheduler: &mut Scheduler, path: String) {

    state.set_width(load_ez_int_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.set_width(val.as_usize().clone());
            path.clone()
        })))
}


pub fn load_height_property(state: &mut dyn GenericState, property_value: &str,
                            scheduler: &mut Scheduler, path: String) {

    state.set_height(load_ez_int_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.set_height(val.as_usize().clone());
            path.clone()
        })))
}


pub fn load_padding_top_property(state: &mut dyn GenericState, property_value: &str,
                             scheduler: &mut Scheduler, path: String) {

    state.set_padding_top(load_ez_int_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.set_padding_top(val.as_usize().clone());
            path.clone()
        })))
}


pub fn load_padding_bottom_property(state: &mut dyn GenericState, property_value: &str,
                                  scheduler: &mut Scheduler, path: String) {

    state.set_padding_bottom(load_ez_int_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.set_padding_bottom(val.as_usize().clone());
            path.clone()
        })))
}


pub fn load_padding_left_property(state: &mut dyn GenericState, property_value: &str,
                                  scheduler: &mut Scheduler, path: String) {

    state.set_padding_left(load_ez_int_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.set_padding_left(val.as_usize().clone());
            path.clone()
        })))
}


pub fn load_padding_right_property(state: &mut dyn GenericState, property_value: &str,
                                  scheduler: &mut Scheduler, path: String) {

    state.set_padding_right(load_ez_int_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.set_padding_right(val.as_usize().clone());
            path.clone()
        })))
}


pub fn load_border_horizontal_property(state: &mut dyn GenericState, property_value: &str,
                                        scheduler: &mut Scheduler, path: String) {

    state.get_border_config_mut().horizontal_symbol.set(load_ez_string_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path)
                .as_generic_mut();
            state.get_border_config_mut().horizontal_symbol.set(val.as_string().clone());
            path.to_string()
        })))
}


pub fn load_border_vertical_property(state: &mut dyn GenericState, property_value: &str,
                                        scheduler: &mut Scheduler, path: String) {

    state.get_border_config_mut().vertical_symbol.set(load_ez_string_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.get_border_config_mut().vertical_symbol.set(val.as_string().clone());
            path.clone()
        })))
}


pub fn load_border_top_left_property(state: &mut dyn GenericState, property_value: &str,
                                     scheduler: &mut Scheduler, path: String) {

    state.get_border_config_mut().top_left_symbol.set(load_ez_string_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.get_border_config_mut().top_left_symbol.set(val.as_string().clone());
            path.clone()
        })))
}


pub fn load_border_top_right_property(state: &mut dyn GenericState, property_value: &str,
                                      scheduler: &mut Scheduler, path: String) {

    state.get_border_config_mut().top_right_symbol.set(load_ez_string_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.get_border_config_mut().top_right_symbol.set(val.as_string().clone());
            path.clone()
        })))
}


pub fn load_border_bottom_left_property(state: &mut dyn GenericState, property_value: &str,
                                        scheduler: &mut Scheduler, path: String) {
    state.get_border_config_mut().bottom_left_symbol.set(load_ez_string_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.get_border_config_mut().bottom_left_symbol.set(val.as_string().clone());
            path.clone()
        })))
}


pub fn load_border_bottom_right_property(state: &mut dyn GenericState, property_value: &str,
                                         scheduler: &mut Scheduler, path: String) {

    state.get_border_config_mut().bottom_right_symbol.set(load_ez_string_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.get_border_config_mut().bottom_right_symbol.set(val.as_string().clone());
            path.clone()
        })))
}


pub fn load_border_foreground_color_property(state: &mut dyn GenericState, property_value: &str,
                                      scheduler: &mut Scheduler, path: String) {

    state.get_border_config_mut().fg_color.set(load_ez_color_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.get_border_config_mut().fg_color.set(*val.as_color());
            path.clone()
        })))
}


pub fn load_border_background_color_property(state: &mut dyn GenericState, property_value: &str,
                                      scheduler: &mut Scheduler, path: String) {

    state.get_border_config_mut().bg_color.set(load_ez_color_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.get_border_config_mut().bg_color.set(*val.as_color());
            path.clone()
        })))
}


pub fn load_foreground_color_property(state: &mut dyn GenericState, property_value: &str,
                                             scheduler: &mut Scheduler, path: String) {

    state.get_colors_config_mut().foreground.set(load_ez_color_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.get_colors_config_mut().foreground.set(*val.as_color());
            path.clone()
        })))
}


pub fn load_background_color_property(state: &mut dyn GenericState, property_value: &str,
                                             scheduler: &mut Scheduler, path: String) {

    state.get_colors_config_mut().background.set(load_ez_color_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.get_colors_config_mut().background.set(*val.as_color());
            path.clone()
        })))
}


pub fn load_selection_foreground_color_property(state: &mut dyn GenericState, property_value: &str,
                                             scheduler: &mut Scheduler, path: String) {

    state.get_colors_config_mut().selection_foreground.set(load_ez_color_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.get_colors_config_mut().selection_foreground.set(*val.as_color());
            path.clone()
        })))
}


pub fn load_selection_background_color_property(state: &mut dyn GenericState, property_value: &str,
                                             scheduler: &mut Scheduler, path: String) {

    state.get_colors_config_mut().selection_background.set(load_ez_color_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.get_colors_config_mut().selection_background.set(*val.as_color());
            path.clone()
        })))
}


pub fn load_disabled_foreground_color_property(state: &mut dyn GenericState, property_value: &str,
                                             scheduler: &mut Scheduler, path: String) {

    state.get_colors_config_mut().disabled_foreground.set(load_ez_color_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.get_colors_config_mut().disabled_foreground.set(*val.as_color());
            path.clone()
        })))
}


pub fn load_disabled_background_color_property(state: &mut dyn GenericState, property_value: &str,
                                             scheduler: &mut Scheduler, path: String) {

    state.get_colors_config_mut().disabled_background.set(load_ez_color_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.get_colors_config_mut().disabled_background.set(*val.as_color());
            path.clone()
        })))
}


pub fn load_tab_foreground_color_property(state: &mut dyn GenericState, property_value: &str,
                                             scheduler: &mut Scheduler, path: String) {

    state.get_colors_config_mut().tab_foreground.set(load_ez_color_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.get_colors_config_mut().tab_foreground.set(*val.as_color());
            path.clone()
        })))
}


pub fn load_tab_background_color_property(state: &mut dyn GenericState, property_value: &str,
                                             scheduler: &mut Scheduler, path: String) {

    state.get_colors_config_mut().tab_background.set(load_ez_color_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.get_colors_config_mut().tab_background.set(*val.as_color());
            path.clone()
        })))
}


pub fn load_flash_foreground_color_property(state: &mut dyn GenericState, property_value: &str,
                                             scheduler: &mut Scheduler, path: String) {

    state.get_colors_config_mut().filler_foreground.set(load_ez_color_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.get_colors_config_mut().filler_foreground.set(*val.as_color());
            path.clone()
        })))
}


pub fn load_flash_background_color_property(state: &mut dyn GenericState, property_value: &str,
                                             scheduler: &mut Scheduler, path: String) {

    state.get_colors_config_mut().flash_background.set(load_ez_color_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.get_colors_config_mut().flash_background.set(*val.as_color());
            path.clone()
        })))
}


pub fn load_fill_foreground_color_property(state: &mut dyn GenericState, property_value: &str,
                                             scheduler: &mut Scheduler, path: String) {

    state.get_colors_config_mut().filler_foreground.set(load_ez_color_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.get_colors_config_mut().filler_foreground.set(*val.as_color());
            path.clone()
        })))
}


pub fn load_fill_background_color_property(state: &mut dyn GenericState, property_value: &str,
                                             scheduler: &mut Scheduler, path: String) {

    state.get_colors_config_mut().filler_background.set(load_ez_color_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.get_colors_config_mut().filler_background.set(*val.as_color());
            path.clone()
        })))
}


pub fn load_cursor_background_color_property(state: &mut dyn GenericState, property_value: &str,
                                           scheduler: &mut Scheduler, path: String) {

    state.get_colors_config_mut().cursor.set(load_ez_color_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.get_colors_config_mut().cursor.set(*val.as_color());
            path.clone()
        })))
}


pub fn load_valign_property(state: &mut dyn GenericState, property_value: &str,
                            scheduler: &mut Scheduler, path: String) {

    state.set_vertical_alignment(load_ez_valign_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.set_vertical_alignment(*val.as_vertical_alignment());
            path.clone()
        })))
}


pub fn load_halign_property(state: &mut dyn GenericState, property_value: &str,
                            scheduler: &mut Scheduler, path: String) {

    state.set_horizontal_alignment(load_ez_halign_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.set_horizontal_alignment(*val.as_horizontal_alignment());
            path.clone()
        })))
}





/// Load a property common to all [EzObjects]. Returns a bool representing whether the property
/// was consumed. If not consumed it should be a property specific to a widget.
pub fn load_common_property(property_name: &str, property_value: String,
                              obj: &mut dyn EzObject, scheduler: &mut Scheduler) -> bool {

    let path = obj.get_full_path().clone();
    let state = obj.get_state_mut();
    match property_name {
        "id" => obj.set_id(property_value.trim().to_string()),
        "x" => load_x_property(state, property_value.trim(), scheduler, path),
        "y" => load_y_property(state, property_value.trim(), scheduler, path),
        "pos" => {
            let (x,y) = property_value.trim().split_once(',').unwrap();
            load_x_property(state, x.trim(), scheduler, path.clone());
            load_y_property(state, y.trim(), scheduler, path);
        },
        "size_hint" => load_full_size_hint_property(
            state,property_value.trim(), scheduler, path),
        "size_hint_x" => load_size_hint_x_property(
            state, property_value.trim(), scheduler, path),
        "size_hint_y" => load_size_hint_y_property(
            state, property_value.trim(), scheduler, path),
        "size" => {
            let (width, height) = property_value.trim().split_once(',').unwrap();
            load_width_property(state, width.trim(), scheduler,
                                 path.clone());
            load_height_property(state, height.trim(), scheduler, path);
        },
        "width" => load_width_property(
            state, property_value.trim(), scheduler, path),
        "height" => load_height_property(
            state, property_value.trim(), scheduler, path),
        "pos_hint" => load_full_pos_hint_property(
            state, property_value.trim(), scheduler, path),
        "pos_hint_x" => load_horizontal_pos_hint_property(
            state, property_value.trim(), scheduler, path),
        "pos_hint_y" => load_vertical_pos_hint_property(
            state, property_value.trim(), scheduler, path),
        "auto_scale" => load_full_auto_scale_property(
            state, property_value.trim(), scheduler, path),
        "auto_scale_width" => load_auto_scale_width_property(
            state, property_value.trim(), scheduler, path),
        "auto_scale_height" => load_auto_scale_height_property(
            state, property_value.trim(), scheduler, path),
        "padding" => {
            let padding_params: Vec<&str> = property_value.trim().split(',').collect();
            load_padding_top_property(state, padding_params[0].trim(),
                                       scheduler,path.clone());
            load_padding_bottom_property(state, padding_params[1].trim(),
                                       scheduler,path.clone());
            load_padding_left_property(state, padding_params[2].trim(),
                                       scheduler,path.clone());
            load_padding_right_property(state, padding_params[3].trim(),
                                       scheduler,path);
        },
        "disabled" => state.set_disabled(parse_bool_property(property_value.trim())),
        "selection_order" => { state.set_selection_order(
            load_selection_order_property(property_value.as_str())) },
        "padding_x" => {
            let (left, right) = property_value.split_once(',').unwrap();
            load_padding_left_property(state, left.trim(), scheduler,path.clone());
            load_padding_right_property(state, right.trim(), scheduler,path);
        },
        "padding_y" => {
            let (top, bottom) = property_value.split_once(',').unwrap();
            load_padding_left_property(state, top.trim(), scheduler,path.clone());
            load_padding_right_property(state, bottom.trim(), scheduler,path);
        },
        "padding_top" => state.set_padding_top(property_value.trim().parse().unwrap()),
        "padding_bottom" => state.set_padding_bottom(property_value.trim().parse().unwrap()),
        "padding_left" => state.set_padding_left(property_value.trim().parse().unwrap()),
        "padding_right" => state.set_padding_right(property_value.trim().parse().unwrap()),
        "halign" => load_halign_property(state, property_value.trim(), scheduler, path),
        "valign" => load_valign_property(state, property_value.trim(), scheduler, path),
        "fg_color" => load_foreground_color_property(
            state, property_value.trim(), scheduler, path),
        "bg_color" => load_background_color_property(
            state, property_value.trim(), scheduler, path),
        "disabled_fg_color" => load_disabled_foreground_color_property(
            state, property_value.trim(), scheduler, path),
        "disabled_bg_color" => load_disabled_background_color_property(
            state, property_value.trim(), scheduler, path),
        "selection_fg_color" => load_selection_foreground_color_property(
            state, property_value.trim(), scheduler, path),
        "selection_bg_color" => load_selection_background_color_property(
            state, property_value.trim(), scheduler, path),
        "flash_fg_color" => load_flash_foreground_color_property(
            state, property_value.trim(), scheduler, path),
        "flash_bg_color" => load_flash_background_color_property(
            state, property_value.trim(), scheduler, path),
        "tab_fg_color" => load_tab_foreground_color_property(
            state, property_value.trim(), scheduler, path),
        "tab_bg_color" => load_tab_background_color_property(
            state, property_value.trim(), scheduler, path),
        "fill_fg_color" => load_fill_foreground_color_property(
            state, property_value.trim(), scheduler, path),
        "fill_bg_color" => load_fill_background_color_property(
            state, property_value.trim(), scheduler, path),
        "cursor_color" => load_cursor_background_color_property(
            state, property_value.trim(), scheduler, path),
        "border" => load_border_enable_property(
            state, property_value.trim(), scheduler, path),
        "border_horizontal_symbol" => load_border_horizontal_property(
            state, property_value.trim(), scheduler, path),
        "border_vertical_symbol" => load_border_vertical_property(
            state, property_value.trim(), scheduler, path),
        "border_top_right_symbol" => load_border_top_left_property(
            state, property_value.trim(), scheduler, path),
        "border_top_left_symbol" => load_border_top_right_property(
            state, property_value.trim(), scheduler, path),
        "border_bottom_left_symbol" => load_border_bottom_left_property(
            state, property_value.trim(), scheduler, path),
        "border_bottom_right_symbol" => load_border_bottom_right_property(
            state, property_value.trim(), scheduler, path),
        "border_fg_color" => load_border_foreground_color_property(
            state, property_value.trim(), scheduler, path),
        "border_bg_color" => load_border_background_color_property(
            state, property_value.trim(), scheduler, path),
        _ => return false,
    }
    true
}