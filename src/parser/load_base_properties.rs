//! # Base property loaders
//!
//! This module contains functions that load base [EzProperty] objects from a .ez file. An
//! EzProperty can contain an actual concrete value or a reference to another EzProperty of the
//! same type. If the value is concrete, it must be parsed and set. If the value is a reference,
//! it must be bound to the referenced EzProperty and a default value must be set.
use std::io::{Error, ErrorKind};

use crossterm::style::Color;

use crate::parser::parse_properties;
use crate::scheduler::definitions::EzPropertyUpdater;
use crate::scheduler::scheduler::{SchedulerFrontend};
use crate::states::definitions::{HorizontalAlignment, VerticalAlignment};

/// Bind an [EzProperty] to another EzProperty if the user-passed property declares it. Returns true
/// if the property was bound, else false. Referenced can contain ".parent" which resolved to the
/// parent widget of the widget the value belong to, ".properties" which refers to the complete
/// collection of EzProperties currently active (which allows users to access their custom
/// properties) or a reference to a widget id in the current context, e.g. "parent.my_label" to
/// reference and ID in the parent layout.
pub fn bind_ez_property(value: &str, scheduler: &mut SchedulerFrontend, path: String,
                        update_func: EzPropertyUpdater) -> bool {

    if value.starts_with("self.") {
        let new_path = value.replace("self.", &path).replace('.', "/");
        scheduler.subscribe_to_ez_property(new_path.as_str(), update_func.clone());
        true
    } else if value.starts_with("root.") {
        let new_path = value.replace(".root", "/root").replace('.', "/");
        scheduler.subscribe_to_ez_property(new_path.as_str(), update_func.clone());
        true
    } else if value.starts_with("parent.") {
        let new_path = resolve_parent_path(path, value);
        scheduler.subscribe_to_ez_property(new_path.as_str(), update_func);
        true
    } else if value.starts_with("properties.") {
        let new_path = value.strip_prefix("properties.").unwrap();
        scheduler.subscribe_to_ez_property(new_path, update_func);
        true
    } else {
        false
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
    path
}


/// Load a usize [EzProperty]. It is either bound to another usize property and initialized with 0
/// or parsed from the user defined string from the .ez file.
pub fn load_ez_usize_property(value: &str, scheduler: &mut SchedulerFrontend, path: String,
                              update_func: EzPropertyUpdater) -> Result<usize, Error> {
    return if bind_ez_property(value, scheduler, path, update_func) {
        Ok(0)
    } else {
        match value.trim().parse() {
            Ok(i) => Ok(i),
            Err(e) => Err(Error::new(ErrorKind::InvalidData, format!(
                "Could not parse usize property with error: {}", e)))
        }
    }
}


/// Load a bool [EzProperty]. It is either bound to another bool property and initialized with false
/// or parsed from the user defined string from the .ez file.
pub fn load_ez_bool_property(value: &str, scheduler: &mut SchedulerFrontend, path: String,
                             update_func: EzPropertyUpdater) -> Result<bool, Error> {

    if bind_ez_property(value, scheduler, path, update_func) {
        Ok(false)
    } else {
        parse_properties::parse_bool_property(value)
    }
}


/// Load a string [EzProperty]. It is either bound to another string property and initialized with 
/// "" or parsed from the user defined string from the .ez file.
pub fn load_ez_string_property(value: &str, scheduler: &mut SchedulerFrontend, path: String,
                               update_func: EzPropertyUpdater) -> Result<String, Error> {

    if bind_ez_property(value, scheduler, path, update_func) {
        Ok(String::new())
    } else {
        Ok(value.trim().to_string())
    }
}


/// Load a [Color] [EzProperty]. It is either bound to another Color property and initialized with 
/// [Color::Black] or parsed from the user defined string from the .ez file.
pub fn load_ez_color_property(value: &str, scheduler: &mut SchedulerFrontend, path: String,
                              update_func: EzPropertyUpdater) -> Result<Color, Error> {

    if bind_ez_property(value, scheduler, path, update_func) {
        Ok(Color::Black)
    } else {
        parse_properties::parse_color_property(value)
    }
}


/// Load a [VerticalAlignment] [EzProperty]. It is either bound to another valign property and
/// initialized with [VerticalAlignment::Top] or parsed from the user defined string from the
/// .ez file.
pub fn load_ez_valign_property(value: &str, scheduler: &mut SchedulerFrontend, path: String,
                               update_func: EzPropertyUpdater)
    -> Result<VerticalAlignment, Error> {

    if bind_ez_property(value, scheduler, path, update_func) {
        Ok(VerticalAlignment::Top)
    } else {
        parse_properties::parse_valign_property(value)
    }
}


/// Load a [HorizontalAlignment] [EzProperty]. It is either bound to another halign property and
/// initialized with [HorizontalAlignment::Left] or parsed from the user defined string from the
/// .ez file.
pub fn load_ez_halign_property(value: &str, scheduler: &mut SchedulerFrontend, path: String,
                               update_func: EzPropertyUpdater)
    -> Result<HorizontalAlignment, Error> {

    if bind_ez_property(value, scheduler, path, update_func) {
        Ok(HorizontalAlignment::Left)
    } else {
        parse_properties::parse_halign_property(value)
    }
}


/// Load a horizontal position hint [EzProperty]. It is either bound to another hposhint property and
/// initialized with None or parsed from the user defined string from the .ez file.
pub fn load_ez_horizontal_pos_hint_property(value: &str, scheduler: &mut SchedulerFrontend, path: String,
                                            update_func: EzPropertyUpdater)
                                            -> Result<Option<(HorizontalAlignment, f64)>, Error> {

    if bind_ez_property(value, scheduler, path, update_func) {
        Ok(None)
    } else {
        parse_properties::parse_horizontal_pos_hint_property(value)
    }
}


/// Load a vertical position hint [EzProperty]. It is either bound to another vposhint property and
/// initialized with None or parsed from the user defined string from the .ez file.
pub fn load_ez_vertical_pos_hint_property(value: &str, scheduler: &mut SchedulerFrontend, path: String,
                                          update_func: EzPropertyUpdater)
                                          -> Result<Option<(VerticalAlignment, f64)>, Error> {

    if bind_ez_property(value, scheduler, path, update_func) {
        Ok(None)
    } else {
        parse_properties::parse_vertical_pos_hint_property(value)
    }
}


/// Load a [SizeHint] [EzProperty]. It is either bound to another SizeHint property and
/// initialized with Some(1.0) or parsed from the user defined string from the .ez file.
pub fn load_ez_size_hint_property(value: &str, scheduler: &mut SchedulerFrontend, path: String,
                                  update_func: EzPropertyUpdater) -> Result<Option<f64>, Error> {
    if bind_ez_property(value, scheduler, path, update_func) {
        Ok(Some(1.0))
    } else {
        parse_properties::parse_size_hint_property(value)
    }
}