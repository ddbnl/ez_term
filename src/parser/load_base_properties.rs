use std::io::{Error, ErrorKind};
use crossterm::style::{Color};
use crate::states::definitions::{HorizontalAlignment, VerticalAlignment};
use crate::scheduler::scheduler::Scheduler;
use crate::parser::parse_properties;
use crate::scheduler::definitions::EzPropertyUpdater;


/* Base property loaders */
/// Bind a property to another property if the user-passed property declares it. Returns true
/// if the property was bound, else false.
pub fn bind_ez_property(value: &str, scheduler: &mut Scheduler, path: String,
                        update_func: EzPropertyUpdater) -> bool {

    if value.starts_with("parent.") {
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
    path.to_string()
}


pub fn load_ez_usize_property(value: &str, scheduler: &mut Scheduler, path: String,
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


pub fn load_ez_bool_property(value: &str, scheduler: &mut Scheduler, path: String,
                             update_func: EzPropertyUpdater) -> Result<bool, Error> {

    if bind_ez_property(value, scheduler, path, update_func) {
        Ok(false)
    } else {
        parse_properties::parse_bool_property(value)
    }
}


pub fn load_ez_string_property(value: &str, scheduler: &mut Scheduler, path: String,
                               update_func: EzPropertyUpdater) -> Result<String, Error> {

    if bind_ez_property(value, scheduler, path, update_func) {
        Ok(String::new())
    } else {
        Ok(value.trim().to_string())
    }
}


pub fn load_ez_color_property(value: &str, scheduler: &mut Scheduler, path: String,
                              update_func: EzPropertyUpdater) -> Result<Color, Error> {

    if bind_ez_property(value, scheduler, path, update_func) {
        Ok(Color::Black)
    } else {
        parse_properties::parse_color_property(value)
    }
}


pub fn load_ez_valign_property(value: &str, scheduler: &mut Scheduler, path: String,
                               update_func: EzPropertyUpdater)
    -> Result<VerticalAlignment, Error> {

    if bind_ez_property(value, scheduler, path, update_func) {
        Ok(VerticalAlignment::Top)
    } else {
        parse_properties::parse_valign_property(value)
    }
}


pub fn load_ez_halign_property(value: &str, scheduler: &mut Scheduler, path: String,
                               update_func: EzPropertyUpdater)
    -> Result<HorizontalAlignment, Error> {

    if bind_ez_property(value, scheduler, path, update_func) {
        Ok(HorizontalAlignment::Left)
    } else {
        parse_properties::parse_halign_property(value)
    }
}


pub fn load_ez_horizontal_pos_hint_property(value: &str, scheduler: &mut Scheduler, path: String,
                                            update_func: EzPropertyUpdater)
                                            -> Result<Option<(HorizontalAlignment, f64)>, Error> {

    if bind_ez_property(value, scheduler, path, update_func) {
        Ok(None)
    } else {
        parse_properties::parse_horizontal_pos_hint_property(value)
    }
}


pub fn load_ez_vertical_pos_hint_property(value: &str, scheduler: &mut Scheduler, path: String,
                                          update_func: EzPropertyUpdater)
                                          -> Result<Option<(VerticalAlignment, f64)>, Error> {

    if bind_ez_property(value, scheduler, path, update_func) {
        Ok(None)
    } else {
        parse_properties::parse_vertical_pos_hint_property(value)
    }
}


pub fn load_ez_size_hint_property(value: &str, scheduler: &mut Scheduler, path: String,
                                  update_func: EzPropertyUpdater) -> Result<Option<f64>, Error> {
    if bind_ez_property(value, scheduler, path, update_func) {
        Ok(None)
    } else {
        parse_properties::parse_size_hint_property(value)
    }
}