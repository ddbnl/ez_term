//! # Ez Parser
//! Module containing structs and functions for paring a .ez file into a root layout.
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::io::{Error, ErrorKind};
use crossterm::terminal::size;
use crate::widgets::layout::layout::Layout;
use crate::widgets::canvas::Canvas;
use crate::widgets::label::Label;
use crate::widgets::button::Button;
use crate::widgets::radio_button::RadioButton;
use crate::widgets::checkbox::Checkbox;
use crate::widgets::text_input::TextInput;
use crate::widgets::dropdown::Dropdown;
use crate::widgets::ez_object::{EzObjects, EzObject};
use crate::scheduler::scheduler::Scheduler;
use crate::widgets::progress_bar::ProgressBar;
use crate::widgets::slider::Slider;
use crate::states::ez_state::GenericState;
use crate::parser::parse_lang;


/// ## Templates
/// A hashmap of 'Template Name > [EzWidgetDefinition]'. Used to instantiate widget templates
/// at runtime. E.g. when spawning popups.
pub type Templates = HashMap<String, EzWidgetDefinition>;


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
    pub fn new(type_name: String, indentation_offset: usize, line_offset: usize) -> Self {
        EzWidgetDefinition {
            type_name,
            content: Vec::new(),
            indentation_offset,
            line_offset,
        }
    }

    /// Parse a definition as the root layout. The normal parsed method results in a generic
    /// EzObjects enum, whereas the root widget should be a layout specifically.
    pub fn parse_as_root(&mut self, mut templates: Templates, scheduler: &mut Scheduler) -> Layout {

        let (config, mut sub_widgets, _) =
            parse_lang::parse_level(self.content.clone(), self.indentation_offset, self.line_offset)
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
            parse_lang::parse_level(self.content.clone(), self.indentation_offset, self.line_offset)
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "{}", self.type_name) }
}