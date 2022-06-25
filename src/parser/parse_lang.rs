//! # Ez Parser
//! Module containing structs and functions for paring a .ez file into a root layout.
use std::collections::HashMap;
use std::fs;
use std::fs::{DirEntry, File, read_dir};
use std::io::prelude::*;
use std::io::{Error};
use std::path::Path;
use unicode_segmentation::UnicodeSegmentation;
use crate::widgets::layout::layout::Layout;
use crate::scheduler::scheduler::Scheduler;
use crate::parser::ez_definition::{EzWidgetDefinition, Templates};


/// Load a file path into a root layout. Return the root widget and a new scheduler. Both will
/// be needed to run an [App].
pub fn load_ez_file(file_path: &str) -> (Layout, Scheduler) {

    let mut file = File::open(file_path).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");
    let (root_widget, scheduler) = load_ez_text(contents).unwrap();
    (root_widget, scheduler)
}


/// Load multiple file paths into a root layout. Return the root widget and a new scheduler.
/// Both will be needed to run an [App].
pub fn load_ez_files(file_paths: Vec<&str>) -> (Layout, Scheduler) {

    let mut contents = String::new();
    for path in file_paths {
        let mut file = File::open(path)
            .expect(format!("Unable to open file {}", path).as_str());
        file.read_to_string(&mut contents)
            .expect(format!("Unable to read file {}", path).as_str());
        contents = format!("{}\n", contents);
    }
    let (root_widget, scheduler) = load_ez_text(contents).unwrap();
    (root_widget, scheduler)
}


/// Load all '.ez' files from a folder recursively. There can only be one root widget, so when
/// loading multiple files make sure all definitions are templates, except for the one root Layout.
pub fn load_ez_folder(folder_path: &str) -> (Layout, Scheduler) {

    let path = Path::new(folder_path);
    let mut ez_files = Vec::new();
    collect_ez_files(path, &mut ez_files);
    load_ez_files(ez_files.iter().map(|x| x.as_str()).collect())
}


/// Find all files that end with '.ez' in a folder recursively.
fn collect_ez_files(dir: &Path, ez_files: &mut Vec<String>) {

    if dir.is_dir() {
        for entry in read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                collect_ez_files(&path, ez_files);
            } else {
                if let Some(extension) = path.extension() {
                    if extension == "ez" {
                        ez_files.push(path.to_str().unwrap().to_string());
                    }
                }
            }
        }
    }
}


/// Load a string from an Ez file into a root widget. Parse the first level and interpret the
/// widget definition found there as the root widget (must be a layout or panic). Then parse the
/// root widget definition into the actual widget, which will parse sub-widgets, who will parse
/// their sub-widgets, etc. Thus recursively loading the UI.
pub fn load_ez_text(text: String) -> Result<(Layout, Scheduler), Error> {

    let config_lines:Vec<String> = text.lines().map(|x| x.to_string()).collect();
    let (_, mut widgets, templates) =
        parse_level(config_lines, 0, 0).unwrap();
    if widgets.len() > 1 {
        panic!("There can be only one root widget but {} were found ({:?}). If you meant to use \
        multiple screens, create one root layout with \"mode: screen\" and add the screen \
        layouts to this root.", widgets.len(), widgets);
    }
    let mut root_widget = widgets.pop().unwrap();
    println!("JIJ {}", root_widget.type_name);
    if root_widget.type_name.to_lowercase() != "layout" {
        panic!("Root widget of an Ez file must be a layout")
    }

    let mut scheduler = Scheduler::default();
    let initialized_root_widget = root_widget.parse_as_root(templates, &mut scheduler);

    Ok((initialized_root_widget, scheduler))
}

/// Parse a single indentation level of a config file. Returns a Vec of config lines, a Vec
/// of [EzWidgetDefinition] of widgets found on that level, and a Vec of [EzWidgetDefinition] of
/// templates found on that level
pub fn parse_level(config_lines: Vec<String>, indentation_offset: usize, line_offset: usize)
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