//! # Base property loaders
//!
//! This module contains functions that load base [EzProperty] objects from a .ez file. An
//! EzProperty can contain an actual concrete value or a reference to another EzProperty of the
//! same type. If the value is concrete, it must be parsed and set. If the value is a reference,
//! it must be bound to the referenced EzProperty and a default value must be set.
use std::io::{Error, ErrorKind};

use meval;

use crossterm::style::Color;

use crate::parser::parse_properties;
use crate::property::ez_values::EzValues;
use crate::scheduler::scheduler::{SchedulerFrontend};
use crate::states::definitions::{HorizontalAlignment, LayoutMode, LayoutOrientation,
                                 VerticalAlignment};
use crate::{GenericState, StateTree};
use crate::parser::parse_properties::parse_layout_orientation_property;


pub fn resolve_property(value: &str, path: String) -> Option<String> {

    if value.starts_with("self.") {
        Some(value.replace("self.", &path).replace('.', "/"))
    } else if value.starts_with("root.") {
        Some(value.replace(".root", "/root").replace('.', "/"))
    } else if value.starts_with("parent.") {
        Some(resolve_parent_path(path, value))
    } else if value.starts_with("properties.") {
        Some(value.strip_prefix("properties.").unwrap().to_string())
    } else {
        None
    }
}


/// Bind an [EzProperty] to another EzProperty if the user-passed property declares it. Returns true
/// if the property was bound, else false. Referenced can contain ".parent" which resolved to the
/// parent widget of the widget the value belong to, ".properties" which refers to the complete
/// collection of EzProperties currently active (which allows users to access their custom
/// properties) or a reference to a widget id in the current context, e.g. "parent.my_label" to
/// reference and ID in the parent layout.
pub fn bind_ez_property(value: &str, scheduler: &mut SchedulerFrontend, path: String) -> bool {

    if let Some(bind_path) = resolve_property(value, path.clone()) {
        scheduler.subscribe_to_property(bind_path.as_str(), path);
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
pub fn load_usize_property(value: &str, scheduler: &mut SchedulerFrontend, path: String,
                           property_name: &str, state: &mut dyn GenericState) -> Result<(), Error> {
    return if value.find(|x| ['+', '-', '/', '*'].contains(&x)).is_some() {
        wrap_usize_property(value.to_string(), path, property_name.to_string(),
                            scheduler);
        state.update_property(property_name, EzValues::Usize(0));
        Ok(())
    } else if let Some(i) = resolve_property(value, path.clone()) {
        bind_ez_property(&i, scheduler, path);
        state.update_property(property_name, EzValues::Usize(0));
        Ok(())
    } else {
        match value.trim().parse() {
            Ok(i) => {
                state.update_property(property_name, EzValues::Usize(i));
                Ok(())
            },
            Err(e) => Err(Error::new(ErrorKind::InvalidData, format!(
                "Could not parse usize property with error: {}", e)))
        }
    }
}


pub fn wrap_usize_property(value: String, path: String, property_name: String,
                           scheduler: &mut SchedulerFrontend) {


    let mut value = value.clone();
    let mut parts: Vec<String> = value.split(|x| ['+', '-', '/', '*'].contains(&x))
        .map(|x|x.to_string()).collect();
    let mut values = Vec::new();
    let mut to_bind = Vec::new();
    for part in parts {
        let name = part.replace(".", "_").to_string();
        value = value.replace(&part, &name);

        let (mut object, mut property) = (String::new(), String::new());
        if part.trim().parse::<usize>().is_err() {
            let property_path = resolve_property(part.trim(), path.clone())
                .unwrap_or_else(|| panic!("Cannot parse this value expression: {}", value));
            let (_object, _property) = property_path.rsplit_once('/').unwrap().to_owned();
            (object, property) = (_object.to_string(), _property.to_string());
            to_bind.push(property_path.to_string());
        }

        let getter =
            move |state_tree: &mut StateTree| {
                match part.trim().parse() {
                    Err(_) => {
                        state_tree
                            .get(&object).as_generic()
                            .get_property(&property).as_usize()
                    },
                    Ok(i) => {
                        i
                    }
                }
            };
        values.push((name, getter));
    }

    let expr: meval::Expr = value.parse().unwrap();
    let values_c = values.clone();
    let property_path = format!("{}/{}", path, property_name);
    let mut update_func =
        scheduler.backend.property_updaters.remove(&property_path).unwrap();

    let wrapper = move |state_tree: &mut StateTree, val: EzValues| {
        let mut ctx = meval::Context::new(); // built-ins
        for (name, getter) in values_c.iter() {
            ctx.var(name.trim(), getter(state_tree) as f64);
        }
        let result = expr.eval_with_context(ctx).unwrap() as usize;
        update_func(state_tree, EzValues::Usize(result))
    };

    scheduler.backend.property_updaters.insert(property_path.clone(), Box::new(wrapper));

    for bind in to_bind {
        scheduler.subscribe_to_property(&bind, property_path.clone());
    }
}

/// Load a f64 [EzProperty]. It is either bound to another f64 property and initialized with 0.0
/// or parsed from the user defined string from the .ez file.
pub fn load_f64_property(value: &str, scheduler: &mut SchedulerFrontend, path: String,
                         property_name: &str, state: &mut dyn GenericState) -> Result<(), Error> {

    return if bind_ez_property(value, scheduler, path) {
        state.update_property(property_name, EzValues::F64(0.0));
        Ok(())
    } else {
        match value.trim().parse() {
            Ok(i) => {
                state.update_property(property_name, EzValues::F64(i));
                Ok(())
            },
            Err(e) => Err(Error::new(ErrorKind::InvalidData, format!(
                "Could not parse f64 property with error: {}", e)))
        }
    }
}

/// Load a bool [EzProperty]. It is either bound to another bool property and initialized with false
/// or parsed from the user defined string from the .ez file.
pub fn load_bool_property(value: &str, scheduler: &mut SchedulerFrontend, path: String,
                          property_name: &str, state: &mut dyn GenericState)
    -> Result<(), Error> {

    if bind_ez_property(value, scheduler, path) {
        state.update_property(property_name, EzValues::Bool(false));
        Ok(())
    } else {
        let val = parse_properties::parse_bool_property(value)?;
        state.update_property(property_name, EzValues::Bool(val));
        Ok(())
    }
}


/// Load a string [EzProperty]. It is either bound to another string property and initialized with 
/// "" or parsed from the user defined string from the .ez file.
pub fn load_string_property(value: &str, scheduler: &mut SchedulerFrontend, path: String,
                            property_name: &str, state: &mut dyn GenericState) -> Result<(), Error> {

    if bind_ez_property(value, scheduler, path) {
        state.update_property(property_name, EzValues::String(String::new()));
        Ok(())
    } else {
        state.update_property(property_name, EzValues::String(value.trim().to_string()));
        Ok(())
    }
}


/// Load a [Color] [EzProperty]. It is either bound to another Color property and initialized with 
/// [Color::Black] or parsed from the user defined string from the .ez file.
pub fn load_color_property(value: &str, scheduler: &mut SchedulerFrontend, path: String,
                           property_name: &str, state: &mut dyn GenericState) -> Result<(), Error> {

    if bind_ez_property(value, scheduler, path) {
        state.update_property(property_name, EzValues::Color(Color::Black));
        Ok(())
    } else {
        let val = parse_properties::parse_color_property(value)?;
        state.update_property(property_name, EzValues::Color(val));
        Ok(())
    }
}


/// Load a [LayoutMode] [EzProperty]. It is either bound to another LayoutMode property and
/// initialized with [LayoutMode::Top] or parsed from the user defined string from the
/// .ez file.
pub fn load_layout_mode_property(value: &str, scheduler: &mut SchedulerFrontend, path: String,
                                 property_name: &str, state: &mut dyn GenericState)
    -> Result<(), Error> {

    if bind_ez_property(value, scheduler, path) {
        state.update_property(property_name, EzValues::LayoutMode(LayoutMode::Box));
        Ok(())
    } else {
        let val = parse_properties::parse_layout_mode_property(value)?;
        state.update_property(property_name, EzValues::LayoutMode(val));
        Ok(())
    }
}


/// Load a [LayoutOrientation] [EzProperty]. It is either bound to another LayoutOrientation
/// property and initialized with [LayoutOrientation::Horizontal] or parsed from the user defined
/// string from the .ez file.
pub fn load_layout_orientation_property(value: &str, scheduler: &mut SchedulerFrontend, path: String,
                                        property_name: &str, state: &mut dyn GenericState) -> Result<(), Error> {

    if bind_ez_property(value, scheduler, path) {
        state.update_property(property_name,
                              EzValues::LayoutOrientation(LayoutOrientation::Horizontal));
        Ok(())
    } else {
        let val = parse_properties::parse_layout_orientation_property(value)?;
        state.update_property(property_name, EzValues::LayoutOrientation(val));
        Ok(())
    }
}


/// Load a [VerticalAlignment] [EzProperty]. It is either bound to another valign property and
/// initialized with [VerticalAlignment::Top] or parsed from the user defined string from the
/// .ez file.
pub fn load_valign_property(value: &str, scheduler: &mut SchedulerFrontend, path: String,
                            property_name: &str, state: &mut dyn GenericState) -> Result<(), Error> {

    if bind_ez_property(value, scheduler, path) {
        state.update_property(property_name,
                              EzValues::VerticalAlignment(VerticalAlignment::Top));
        Ok(())
    } else {
        let val = parse_properties::parse_valign_property(value)?;
        state.update_property(property_name,EzValues::VerticalAlignment(val));
        Ok(())
    }
}


/// Load a [HorizontalAlignment] [EzProperty]. It is either bound to another halign property and
/// initialized with [HorizontalAlignment::Left] or parsed from the user defined string from the
/// .ez file.
pub fn load_halign_property(value: &str, scheduler: &mut SchedulerFrontend, path: String,
                            property_name: &str, state: &mut dyn GenericState) -> Result<(), Error> {

    if bind_ez_property(value, scheduler, path) {
        state.update_property(property_name,
                              EzValues::HorizontalAlignment(HorizontalAlignment::Left));
        Ok(())
    } else {
        let val = parse_properties::parse_halign_property(value)?;
        state.update_property(property_name,EzValues::HorizontalAlignment(val));
        Ok(())
    }
}


/// Load a horizontal position hint [EzProperty]. It is either bound to another hposhint property and
/// initialized with None or parsed from the user defined string from the .ez file.
pub fn load_horizontal_pos_hint_property(value: &str, scheduler: &mut SchedulerFrontend, path: String,
                                         property_name: &str, state: &mut dyn GenericState) -> Result<(), Error> {

    if bind_ez_property(value, scheduler, path) {
        state.update_property(property_name,EzValues::HorizontalPosHint(None));
        Ok(())
    } else {
        let val = parse_properties::parse_horizontal_pos_hint_property(value)?;
        state.update_property(property_name,EzValues::HorizontalPosHint(val));
        Ok(())
    }
}


/// Load a vertical position hint [EzProperty]. It is either bound to another vposhint property and
/// initialized with None or parsed from the user defined string from the .ez file.
pub fn load_vertical_pos_hint_property(value: &str, scheduler: &mut SchedulerFrontend, path: String,
                                       property_name: &str, state: &mut dyn GenericState) -> Result<(), Error> {

    if bind_ez_property(value, scheduler, path) {
        state.update_property(property_name,EzValues::VerticalPosHint(None));
        Ok(())
    } else {
        let val = parse_properties::parse_vertical_pos_hint_property(value)?;
        state.update_property(property_name,EzValues::VerticalPosHint(val));
        Ok(())
    }
}


/// Load a [SizeHint] [EzProperty]. It is either bound to another SizeHint property and
/// initialized with Some(1.0) or parsed from the user defined string from the .ez file.
pub fn load_size_hint_property(value: &str, scheduler: &mut SchedulerFrontend, path: String,
                               property_name: &str, state: &mut dyn GenericState) -> Result<(), Error> {
    if bind_ez_property(value, scheduler, path) {
        state.update_property(property_name,
                              EzValues::SizeHint(Some(1.0)));
        Ok(())
    } else {
        let val = parse_properties::parse_size_hint_property(value)?;
        state.update_property(property_name,EzValues::SizeHint(val));
        Ok(())
    }
}