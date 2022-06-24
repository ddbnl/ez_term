use crossterm::style::{Color};
use crate::common::definitions::{EzPropertyUpdater};
use crate::states::definitions::{HorizontalAlignment, VerticalAlignment};
use crate::scheduler::scheduler::Scheduler;
use crate::parser::parse_properties;


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
        parse_properties::parse_bool_property(value)
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
        parse_properties::parse_color_property(value)
    }
}


pub fn load_ez_valign_property(value: &str, scheduler: &mut Scheduler, path: String,
                               update_func: EzPropertyUpdater) -> VerticalAlignment {

    if bind_ez_property(value, scheduler, path, update_func) {
        VerticalAlignment::Top
    } else {
        parse_properties::parse_valign_property(value)
    }
}


pub fn load_ez_halign_property(value: &str, scheduler: &mut Scheduler, path: String,
                               update_func: EzPropertyUpdater) -> HorizontalAlignment {

    if bind_ez_property(value, scheduler, path, update_func) {
        HorizontalAlignment::Left
    } else {
        parse_properties::parse_halign_property(value)
    }
}


pub fn load_ez_horizontal_pos_hint_property(value: &str, scheduler: &mut Scheduler, path: String,
                                            update_func: EzPropertyUpdater)
                                            -> Option<(HorizontalAlignment, f64)> {

    if bind_ez_property(value, scheduler, path, update_func) {
        None
    } else {
        parse_properties::parse_horizontal_pos_hint_property(value)
    }
}


pub fn load_ez_vertical_pos_hint_property(value: &str, scheduler: &mut Scheduler, path: String,
                                          update_func: EzPropertyUpdater)
                                          -> Option<(VerticalAlignment, f64)> {

    if bind_ez_property(value, scheduler, path, update_func) {
        None
    } else {
        parse_properties::parse_vertical_pos_hint_property(value)
    }
}


pub fn load_ez_size_hint_property(value: &str, scheduler: &mut Scheduler, path: String,
                                  update_func: EzPropertyUpdater) -> Option<f64> {

    if bind_ez_property(value, scheduler, path, update_func) {
        None
    } else {
        parse_properties::parse_size_hint_property(value)
    }
}