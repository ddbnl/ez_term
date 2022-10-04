//! # Parse Ez Language
//! Module containing functions to parse .ez files and generate [EzWidgetDefinition] objects that
//! can be used to initialize actual widgets.
use std::collections::HashMap;
use std::io::Error;

use unicode_segmentation::UnicodeSegmentation;

use crate::parser::ez_definition::{EzWidgetDefinition, Templates};
use crate::run::definitions::StateTree;
use crate::run::tree::initialize_state_tree;
use crate::scheduler::definitions::CustomDataMap;
use crate::scheduler::scheduler::{Scheduler, SchedulerFrontend};
use crate::widgets::layout::layout::Layout;

include!(concat!(env!("OUT_DIR"), "/ez_file_gen.rs"));

/// Load a file path into a root layout. Return the root widget, state tree and a new scheduler.
/// These will be needed to run the ui.
pub fn load_ui<'a>() -> (Layout, StateTree, SchedulerFrontend, CustomDataMap) {
    let contents = ez_config(); // ez_config is generated from build.rs
    let (root_widget, scheduler) = load_ez_text(contents).unwrap();
    let state_tree = initialize_state_tree(&root_widget);
    (root_widget, state_tree, scheduler, CustomDataMap::new())
}

/// Load a string from an Ez file into a root widget. Parse the first level and interpret the
/// widget definition found there as the root widget (must be a layout or panic). Then parse the
/// root widget definition into the actual widget, which will parse sub-widgets, who will parse
/// their sub-widgets, etc. Thus recursively loading the UI.
pub fn load_ez_text(files: HashMap<String, String>) -> Result<(Layout, SchedulerFrontend), Error> {
    let mut widgets: Vec<EzWidgetDefinition> = Vec::new();
    let mut templates = Templates::new();
    for (path, config) in files {
        let (_, loaded_widgets, loaded_templates) = parse_level(
            config.lines().into_iter().map(|x| x.to_string()).collect(),
            0,
            0,
            path,
        )
        .unwrap();
        widgets.extend(loaded_widgets);
        templates.extend(loaded_templates);
    }
    if widgets.len() > 1 {
        panic!(
            "There can be only one root widget but {} were found ({:?}). If you meant to use \
        multiple screens, create one root layout with \"mode: screen\" and add the screen \
        layouts to this root.",
            widgets.len(),
            widgets
        );
    }
    let mut root_widget = widgets.pop().unwrap();
    root_widget.is_root = true;

    // Ensure root widget is a [Layout], or a template inherited from [Layout]
    if root_widget.type_name.to_lowercase() != "layout" {
        let mut type_name = &root_widget.type_name;
        loop {
            if templates.contains_key(type_name) {
                type_name = &templates.get(type_name).unwrap().type_name;
            } else {
                if type_name.to_lowercase() != "layout" {
                    panic!("Root widget of an Ez file must be a layout");
                }
                break;
            }
        }
    }

    let mut scheduler = Scheduler::new();
    scheduler.templates = templates.clone();
    let mut scheduler_frontend = SchedulerFrontend::default();
    scheduler_frontend.backend = scheduler;
    let initialized_root_widget =
        root_widget.parse(&mut scheduler_frontend, String::new(), 0, None);
    let mut root = initialized_root_widget.as_layout().to_owned();
    root.state.set_templates(templates);

    Ok((root, scheduler_frontend))
}

/// Parse a single indentation level of a config file. Returns a Vec of config lines, a Vec
/// of [EzWidgetDefinition] of widgets found on that level, and a Vec of [EzWidgetDefinition] of
/// templates found on that level
pub fn parse_level(
    config_lines: Vec<String>,
    indentation_offset: usize,
    line_offset: usize,
    file: String,
) -> Result<(Vec<String>, Vec<EzWidgetDefinition>, Templates), Error> {
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
            continue;
        } else {
            for (j, char) in line.graphemes(true).enumerate() {
                if char != " " {
                    if parsing_config && j != 0 {
                        panic!(
                            "Error at line {0} in file {1}:\n \"{2}\".\n Invalid indentation \
                        between lines \
                        {3} and {0}. Indentation level of line {0} should be {4} but it is {5}.",
                            i + line_offset + 1,
                            file,
                            line,
                            i + line_offset,
                            indentation_offset,
                            indentation_offset + j
                        );
                    }
                    if j % 4 != 0 {
                        panic!(
                            "Error at line {} in file {}:\n\
                        \"{}\"\n. Invalid indentation. indentation must be \
                            in multiples of four.",
                            i + 1 + line_offset,
                            file,
                            line
                        );
                    }
                    if !parsing_config && !line.starts_with('-') && j < 4 {
                        panic!(
                            "Error at Line {0} in file {1}:\n \"{2}\".\n This line must be \
                         indented. Try this:\n{3}{4}\n{5}{2}",
                            i + 1 + line_offset,
                            file,
                            line,
                            " ".repeat(indentation_offset),
                            config_lines[i - 1],
                            " ".repeat(indentation_offset + 4)
                        );
                    }
                    break;
                }
            }
        }
        // Find widget definitions which starts with -
        if line.starts_with('-') {
            // We encountered a widget, so config section of this level is over.
            parsing_config = false;
            // A new widget definition. Get it's type and ID
            let type_name = line
                .strip_prefix('-')
                .unwrap()
                .trim()
                .strip_suffix(':')
                .unwrap_or_else(|| {
                    panic!(
                        "Error at line {}: \"{}\". Widget definition should be \
                followed by a \":\"",
                        i + line_offset + 1,
                        line
                    )
                })
                .to_string();

            if type_name.starts_with('<') {
                // This is a template
                let (type_name, proto_type) = type_name.strip_prefix('<').unwrap()
                    .strip_suffix('>').unwrap_or_else(
                        || panic!("Error at line {} in file {}: \"{}\". Expected '>' to close layout template.",
                            i + line_offset + 1, file, line)).split_once('@').unwrap_or_else(
                    || panic!("Error at line {} in file {}: \"{}\". Expected '@' to separate template name from \
                    type name.", i + line_offset + 1, file, line));
                let def = EzWidgetDefinition::new(
                    proto_type.to_string(),
                    file.clone(),
                    indentation_offset + 4,
                    i + 1 + line_offset,
                );
                templates.insert(type_name.to_string(), def);
                parsing_template = Some(type_name.to_string());
            } else {
                // This is a regular widget definition
                // Add to level, all next lines that are not widget definitions append to this widget
                level.push(EzWidgetDefinition::new(
                    type_name.to_string(),
                    file.clone(),
                    indentation_offset + 4,
                    i + 1 + line_offset,
                ));
                parsing_template = None;
            }
        } else if parsing_config {
            config.push(line);
        } else {
            // Line was not a new widget definition, so it must be config/content of the current one
            let new_line = line.strip_prefix("    ").unwrap_or_else(|| {
                panic!(
                    "Error at line {} in file {}: \"{}\". Could not strip indentation.",
                    i + line_offset + 1,
                    file,
                    line
                )
            });
            if let Some(name) = &parsing_template {
                templates
                    .get_mut(name)
                    .unwrap()
                    .content
                    .push(new_line.to_string());
            } else {
                level.last_mut().unwrap().content.push(new_line.to_string());
            }
        }
    }
    Ok((config, level, templates))
}
