//! # Ez Definition
//! This module contains a struct that represents the definition of a single widget or layout in
//! a .ez file.
//!
//! The EzWidgetDefinition is used by [parse_lang] to collect all the defined properties of a
//! widget. The struct then has methods for parsing the definition into an initialized object. It
//! functions as a bridge between parsed plain text and the initialized interface.
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::io::{Error, ErrorKind};

use crossterm::terminal::size;

use crate::parser::parse_lang;
use crate::scheduler::scheduler::SchedulerFrontend;
use crate::states::ez_state::{GenericState};
use crate::widgets::{
    ez_object::EzObjects,
    button::Button,
    canvas::Canvas,
    checkbox::Checkbox,
    dropdown::Dropdown,
    label::Label,
    layout::layout::Layout,
    progress_bar::ProgressBar,
    radio_button::RadioButton,
    slider::Slider,
    text_input::TextInput,
};


/// A hashmap to resolve template names to their [EzWidgetDefinition]'.
///
/// Used to instantiate widget templates at runtime. E.g. when spawning popups.
pub type Templates = HashMap<String, EzWidgetDefinition>;


/// Struct representing a widget definition in a .ez file.
///
/// Has methods for parsing the definition into an initialized widget. The definition of a widget
/// consists of two optional parts: the config of the widget itself (its' size, color, etc.) and
/// its' sub widgets. As the definition for a widget might contain sub widgets, it is parsed
/// recursively.
#[derive(Clone)]
pub struct EzWidgetDefinition {

    /// Name of widget class, e.g. layout, or textBox, or name of template.
    pub type_name: String,

    /// The root widget (always a layout) is treated specially, so we need to when we're parsing it
    pub is_root: bool,

    /// All raw text content belonging to this definition
    pub content: Vec<String>,

    /// File path this definition came from
    pub file: String,

    /// Offset in lines where the content of the widget definition begins in the config file.
    /// Zero-indexed. Indicates the first line AFTER the initial definition starting with '- .
    pub line_offset: usize,

    /// Indentation offset of this widget in the config
    pub indentation_offset: usize,
}
impl EzWidgetDefinition {
    /// Create a definition from a type name (Layout, Checkbox, etc.), an indentation offset and a
    /// line offset. The offsets are needed to provide useful error messages to the end-user in
    /// regards to config file parsing errors. E.g. line 10 with indentation 4 in this definition
    /// with offsets '100' and '10', is actually line '110' with indentation 14 in the actual
    /// config file.
    pub fn new(type_name: String, file: String, indentation_offset: usize, line_offset: usize)
        -> Self {
        EzWidgetDefinition {
            type_name,
            indentation_offset,
            file,
            line_offset,
            is_root: false,
            content: Vec::new(),
        }
    }

    /// Based on a template, find the base widget type. Templates can be based on templates so the
    /// base type might be a few levels deep.
    pub fn resolve_base_type(&self, templates: &Templates) -> String {

        let mut type_name = &self.type_name;
        while templates.contains_key(type_name) {
            type_name = &templates.get(type_name).unwrap().type_name;
        }
        type_name.to_string()
    }

    /// Parse a definition by separating the config lines from the sub widget definitions. Then
    /// apply the config to the initialized widget, then initialize and add sub widgets.
    pub fn parse(&mut self, scheduler: &mut SchedulerFrontend, parent_path: String, order: usize,
                 merge_config: Option<Vec<String>>) -> EzObjects {

        let (mut config, mut sub_widgets, _) =
            parse_lang::parse_level(self.content.clone(), self.indentation_offset,
                                    self.line_offset, self.file.clone()).unwrap();

        // Templates can have properties, and instances of templates can also have properties.
        // Merge the configs making sure that the instance config takes precedence.
        if let Some(config_to_merge) = merge_config {
            config = merge_configs(config, config_to_merge);
        }
        let initialized = self.initialize(config, scheduler, parent_path, order).unwrap();
        let parent_path = initialized.as_ez_object().get_path();

        if let EzObjects::Layout(mut obj) = initialized {
            for (i, sub_widget) in sub_widgets.iter_mut().enumerate() {
                let initialized_sub_widget = sub_widget.parse(
                    scheduler, parent_path.clone(),i, None);

                obj.add_child(initialized_sub_widget, scheduler);
            }
            if self.is_root {
                let terminal_size = size().unwrap();
                if obj.state.get_size().get_width() == 0 {
                    obj.state.get_size_mut().set_width(terminal_size.0 as usize);
                }
                if obj.state.get_size().get_height() == 0 {
                    obj.state.get_size_mut().set_height(terminal_size.1 as usize);
                }
            }
            return EzObjects::Layout(obj)
        }
        initialized
    }

    /// Initialize a widget object based on the type specified by the definition. The type can be
    /// a template defined by the user.
    fn initialize(&mut self, mut config: Vec<String>, scheduler: &mut SchedulerFrontend,
                  parent_path: String, order: usize) -> Result<EzObjects, Error> {

        if self.is_root {
            let id = peek_id_from_config(&config);
            if !id.is_empty() {
                if id != "root" {
                    return Err(Error::new(ErrorKind::InvalidData,
                                          "Root widget cannot have an ID parameter; \
                                          it is \"root\" by default"))
                }
            }  else {
                config.push("id: root".to_string());
            }
        }
        // If this is a template, clone the template definition and parse that instead; we want to
        // keep the original template definition intact as it might be used multiple times.
        if scheduler.backend.templates.contains_key(&self.type_name) {
            let mut template =
                scheduler.backend.templates.get_mut(&self.type_name).unwrap().clone();
            template.is_root = self.is_root;
            let object = template.parse(scheduler, parent_path, order,
                                        Some(config));
            let object = object.as_ez_object().get_clone(scheduler);
            Ok(object)
        // If this is a base widget definition initialize a widget of that type from the config of
        // this widget definition.
        } else {
            let mut id = peek_id_from_config(&config);
            if id.is_empty() { id = order.to_string() };
            let path = format!("{}/{}", parent_path, id.trim());
            match self.type_name.as_str() {
                "Layout" => Ok(EzObjects::Layout(
                    Layout::from_config(config, id, path, scheduler, self.file.clone(),
                                        self.line_offset))),
                "Canvas" => Ok(EzObjects::Canvas(
                    Canvas::from_config(config, id, path, scheduler, self.file.clone(),
                                        self.line_offset))),
                "Label" => Ok(EzObjects::Label(
                    Label::from_config(config, id, path, scheduler, self.file.clone(),
                                       self.line_offset))),
                "Button" => Ok(EzObjects::Button(
                    Button::from_config(config, id, path, scheduler, self.file.clone(),
                                        self.line_offset))),
                "CheckBox" => Ok(EzObjects::Checkbox(
                    Checkbox::from_config(config, id, path, scheduler, self.file.clone(),
                                          self.line_offset))),
                "RadioButton" => Ok(EzObjects::RadioButton(
                    RadioButton::from_config(config, id, path, scheduler, self.file.clone(),
                                             self.line_offset))),
                "TextInput" => Ok(EzObjects::TextInput(
                    TextInput::from_config(config, id, path, scheduler, self.file.clone(),
                                           self.line_offset))),
                "Dropdown" => Ok(EzObjects::Dropdown(
                    Dropdown::from_config(config, id, path, scheduler, self.file.clone(),
                                          self.line_offset))),
                "Slider" => Ok(EzObjects::Slider(
                    Slider::from_config(config, id, path, scheduler, self.file.clone(),
                                        self.line_offset))),
                "ProgressBar" => Ok(EzObjects::ProgressBar(
                    ProgressBar::from_config(config, id, path, scheduler, self.file.clone(),
                                             self.line_offset))),
                _ => Err(Error::new(ErrorKind::InvalidData,
                                    format!("Invalid widget type {}", self.type_name)))
            }
        }
    }
}
impl Debug for EzWidgetDefinition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.type_name) }
}


/// Check if a widget definition config contains an ID. If so, return an ID and path from it.
fn peek_id_from_config(config: &[String]) -> String {

    let mut id = String::new();
    for line_str in config.iter() {
        if line_str.trim().to_lowercase().starts_with("id:") {
            id = line_str.trim().split_once(':').unwrap().1.trim().to_string();
        }
    }
    id
}


/// Merge two configs, where config_1 takes precedence, overwriting any properties it has in common
/// with config_2. This is used for templates which can have properties, where the instance of a
/// template may have the same property defined. In that case the instance of the template takes
/// precedence.
fn merge_configs(config_1: Vec<String>, config_2: Vec<String>) -> Vec<String>{

    let mut merged_config: Vec<String> = Vec::new();
    let mut existing_options: Vec<String> = Vec::new();
    for line in config_1 {
        if !line.contains(":") {
            panic!("Line should contain a \":\" to separate property and value: {}", line)
        }
        merged_config.push(line.clone());
        existing_options.push(line.split_once(':').unwrap().0.to_string());
    }
    for line in config_2 {
        if !merged_config.contains(&line.split_once(':').unwrap().0.to_string()) {
            merged_config.push(line);
        }
    }
    merged_config
}