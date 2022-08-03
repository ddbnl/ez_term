//! # Load common properties
//!
//! Functions to load properties common to all widgets (such as size, color, position, etc.). These
//! functions do two things:
//! - Initialize the property with the user passed value or a default value
//! - Pass an update closure, which is used if that property is bound to another property
use std::io::{Error, ErrorKind};

use crate::parser::load_base_properties;
use crate::property::ez_values::EzValues;
use crate::run::definitions::StateTree;
use crate::scheduler::scheduler::{SchedulerFrontend};
use crate::states::ez_state::GenericState;
use crate::widgets::ez_object::EzObject;

pub fn load_full_pos_hint_property(state: &mut dyn GenericState, property_value: &str,
                                   scheduler: &mut SchedulerFrontend, path: String) -> Result<(), Error> {

    let (x_str, y_str) = property_value.split_once(',').unwrap();
    load_horizontal_pos_hint_property(state, x_str, scheduler, path.clone())?;
    load_vertical_pos_hint_property(state, y_str, scheduler, path.clone())?;
    Ok(())
}


pub fn load_horizontal_pos_hint_property(state: &mut dyn GenericState, property_value: &str,
                                         scheduler: &mut SchedulerFrontend, path: String) -> Result<(), Error>  {

    state.set_pos_hint_x(load_base_properties::load_horizontal_pos_hint_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.set_pos_hint_x(*val.as_horizontal_pos_hint());
            path.clone()
        }))?);
    Ok(())
}


pub fn load_vertical_pos_hint_property(state: &mut dyn GenericState, property_value: &str,
                                       scheduler: &mut SchedulerFrontend, path: String) -> Result<(), Error>  {

    state.set_pos_hint_y(load_base_properties::load_vertical_pos_hint_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.set_pos_hint_y(*val.as_vertical_pos_hint());
            path.clone()
        }))?);
    Ok(())
}


/// Convenience function use by widgets to load a size_hint property defined in a .ez file.
/// Looks like "0.33, 0.33" or "1/3, 1/3"
pub fn load_full_size_hint_property(state: &mut dyn GenericState, property_value: &str,
                                    scheduler: &mut SchedulerFrontend, path: String) -> Result<(), Error> {

    let (x_str, y_str) = property_value.split_once(',').unwrap();
    load_size_hint_x_property(state, x_str, scheduler,path.clone())?;
    load_size_hint_y_property(state, y_str, scheduler, path)?;
    Ok(())
}


pub fn load_size_hint_x_property(state: &mut dyn GenericState, property_value: &str,
                                          scheduler: &mut SchedulerFrontend, path: String) -> Result<(), Error>  {

    state.set_size_hint_x(load_base_properties::load_size_hint_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.set_size_hint_x(*val.as_size_hint());
            path.clone()
        }))?);
    Ok(())
}


pub fn load_size_hint_y_property(state: &mut dyn GenericState, property_value: &str,
                                  scheduler: &mut SchedulerFrontend, path: String) -> Result<(), Error>  {

    state.set_size_hint_y(load_base_properties::load_size_hint_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.set_size_hint_y(*val.as_size_hint());
            path.clone()
        }))?);
    Ok(())
}


pub fn load_full_auto_scale_property(state: &mut dyn GenericState, property_value: &str,
                                      scheduler: &mut SchedulerFrontend, path: String) -> Result<(), Error> {

    let (width_str, height_str) = property_value.split_once(',').unwrap();
    load_auto_scale_width_property(state, width_str, scheduler,path.clone())?;
    load_auto_scale_height_property(state, height_str, scheduler, path)?;
    Ok(())
}


pub fn load_auto_scale_width_property(state: &mut dyn GenericState, property_value: &str,
                                       scheduler: &mut SchedulerFrontend, path: String) -> Result<(), Error>  {

    state.set_auto_scale_width(load_base_properties::load_bool_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.set_auto_scale_width(val.as_bool().clone());
            path.clone()
        }))?);
    Ok(())
}


pub fn load_auto_scale_height_property(state: &mut dyn GenericState, property_value: &str,
                                        scheduler: &mut SchedulerFrontend, path: String) -> Result<(), Error>  {

    state.set_auto_scale_height(load_base_properties::load_bool_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.set_auto_scale_height(*val.as_bool());
            path.clone()
        }))?);
    Ok(())
}


pub fn load_border_enable_property(state: &mut dyn GenericState, property_value: &str,
                                    scheduler: &mut SchedulerFrontend, path: String) -> Result<(), Error>  {

    state.get_border_config_mut().set_border(load_base_properties::load_bool_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.get_border_config_mut().set_border(*val.as_bool());
            path.clone()
        }))?);
    Ok(())
}


pub fn load_x_property(state: &mut dyn GenericState, property_value: &str,
                        scheduler: &mut SchedulerFrontend, path: String) -> Result<(), Error>  {

    state.set_x(load_base_properties::load_usize_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.set_x(*val.as_usize());
            path.clone()
        }))?);
    Ok(())
}


pub fn load_y_property(state: &mut dyn GenericState, property_value: &str,
                        scheduler: &mut SchedulerFrontend, path: String) -> Result<(), Error>  {

    state.set_y(load_base_properties::load_usize_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.set_y(*val.as_usize());
            path.clone()
        }))?);
    Ok(())
}


pub fn load_width_property(state: &mut dyn GenericState, property_value: &str,
                            scheduler: &mut SchedulerFrontend, path: String) -> Result<(), Error>  {

    state.set_width(load_base_properties::load_usize_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.set_width(*val.as_usize());
            path.clone()
        }))?);
    Ok(())
}


pub fn load_height_property(state: &mut dyn GenericState, property_value: &str,
                            scheduler: &mut SchedulerFrontend, path: String) -> Result<(), Error>  {

    state.set_height(load_base_properties::load_usize_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.set_height(*val.as_usize());
            path.clone()
        }))?);
    Ok(())
}


pub fn load_padding_top_property(state: &mut dyn GenericState, property_value: &str,
                             scheduler: &mut SchedulerFrontend, path: String) -> Result<(), Error>  {

    state.set_padding_top(load_base_properties::load_usize_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.set_padding_top(*val.as_usize());
            path.clone()
        }))?);
    Ok(())
}


pub fn load_padding_bottom_property(state: &mut dyn GenericState, property_value: &str,
                                  scheduler: &mut SchedulerFrontend, path: String) -> Result<(), Error>  {

    state.set_padding_bottom(load_base_properties::load_usize_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.set_padding_bottom(*val.as_usize());
            path.clone()
        }))?);
    Ok(())
}


pub fn load_padding_left_property(state: &mut dyn GenericState, property_value: &str,
                                  scheduler: &mut SchedulerFrontend, path: String) -> Result<(), Error>  {

    state.set_padding_left(load_base_properties::load_usize_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.set_padding_left(*val.as_usize());
            path.clone()
        }))?);
    Ok(())
}


pub fn load_padding_right_property(state: &mut dyn GenericState, property_value: &str,
                                  scheduler: &mut SchedulerFrontend, path: String) -> Result<(), Error>  {

    state.set_padding_right(load_base_properties::load_usize_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.set_padding_right(*val.as_usize());
            path.clone()
        }))?);
    Ok(())
}


pub fn load_border_horizontal_property(state: &mut dyn GenericState, property_value: &str,
                                        scheduler: &mut SchedulerFrontend, path: String) -> Result<(), Error>  {

    state.get_border_config_mut().set_horizontal_symbol(load_base_properties::load_string_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path)
                .as_generic_mut();
            state.get_border_config_mut().set_horizontal_symbol(val.as_string().clone());
            path.to_string()
        }))?);
    Ok(())
}


pub fn load_border_vertical_property(state: &mut dyn GenericState, property_value: &str,
                                        scheduler: &mut SchedulerFrontend, path: String) -> Result<(), Error>  {

    state.get_border_config_mut().set_vertical_symbol(load_base_properties::load_string_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.get_border_config_mut().set_vertical_symbol(val.as_string().clone());
            path.clone()
        }))?);
    Ok(())
}


pub fn load_border_top_left_property(state: &mut dyn GenericState, property_value: &str,
                                     scheduler: &mut SchedulerFrontend, path: String) -> Result<(), Error>  {

    state.get_border_config_mut().set_top_left_symbol(load_base_properties::load_string_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.get_border_config_mut().set_top_left_symbol(val.as_string().clone());
            path.clone()
        }))?);
    Ok(())
}


pub fn load_border_top_right_property(state: &mut dyn GenericState, property_value: &str,
                                      scheduler: &mut SchedulerFrontend, path: String) -> Result<(), Error>  {

    state.get_border_config_mut().set_top_right_symbol(load_base_properties::load_string_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.get_border_config_mut().set_top_right_symbol(val.as_string().clone());
            path.clone()
        }))?);
    Ok(())
}


pub fn load_border_bottom_left_property(state: &mut dyn GenericState, property_value: &str,
                                        scheduler: &mut SchedulerFrontend, path: String) -> Result<(), Error>  {
    state.get_border_config_mut().set_bottom_left_symbol(load_base_properties::load_string_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.get_border_config_mut().set_bottom_left_symbol(val.as_string().clone());
            path.clone()
        }))?);
    Ok(())
}


pub fn load_border_bottom_right_property(state: &mut dyn GenericState, property_value: &str,
                                         scheduler: &mut SchedulerFrontend, path: String) -> Result<(), Error>  {

    state.get_border_config_mut().set_bottom_right_symbol(load_base_properties::load_string_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.get_border_config_mut().set_bottom_right_symbol(val.as_string().clone());
            path.clone()
        }))?);
    Ok(())
}


pub fn load_border_foreground_color_property(state: &mut dyn GenericState, property_value: &str,
                                      scheduler: &mut SchedulerFrontend, path: String) -> Result<(), Error>  {

    state.get_color_config_mut().set_border_fg_color(load_base_properties::load_color_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.get_color_config_mut().set_border_fg_color(*val.as_color());
            path.clone()
        }))?);
    Ok(())
}


pub fn load_border_background_color_property(state: &mut dyn GenericState, property_value: &str,
                                      scheduler: &mut SchedulerFrontend, path: String) -> Result<(), Error>  {

    state.get_color_config_mut().set_border_bg_color(load_base_properties::load_color_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.get_color_config_mut().set_border_bg_color(*val.as_color());
            path.clone()
        }))?);
    Ok(())
}


pub fn load_foreground_color_property(state: &mut dyn GenericState, property_value: &str,
                                             scheduler: &mut SchedulerFrontend, path: String) -> Result<(), Error>  {

    state.get_color_config_mut().set_fg_color(load_base_properties::load_color_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.get_color_config_mut().set_fg_color(*val.as_color());
            path.clone()
        }))?);
    Ok(())
}


pub fn load_background_color_property(state: &mut dyn GenericState, property_value: &str,
                                             scheduler: &mut SchedulerFrontend, path: String) -> Result<(), Error>  {

    state.get_color_config_mut().set_bg_color(load_base_properties::load_color_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.get_color_config_mut().set_bg_color(*val.as_color());
            path.clone()
        }))?);
    Ok(())
}


pub fn load_selection_foreground_color_property(state: &mut dyn GenericState, property_value: &str,
                                             scheduler: &mut SchedulerFrontend, path: String) -> Result<(), Error>  {

    state.get_color_config_mut().set_selection_fg_color(load_base_properties::load_color_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.get_color_config_mut().set_selection_fg_color(*val.as_color());
            path.clone()
        }))?);
    Ok(())
}


pub fn load_selection_background_color_property(state: &mut dyn GenericState, property_value: &str,
                                             scheduler: &mut SchedulerFrontend, path: String) -> Result<(), Error>  {

    state.get_color_config_mut().set_selection_bg_color(load_base_properties::load_color_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.get_color_config_mut().set_selection_bg_color(*val.as_color());
            path.clone()
        }))?);
    Ok(())
}


pub fn load_disabled_foreground_color_property(state: &mut dyn GenericState, property_value: &str,
                                             scheduler: &mut SchedulerFrontend, path: String) -> Result<(), Error>  {

    state.get_color_config_mut().set_disabled_fg_color(load_base_properties::load_color_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.get_color_config_mut().set_disabled_fg_color(*val.as_color());
            path.clone()
        }))?);
    Ok(())
}


pub fn load_disabled_background_color_property(state: &mut dyn GenericState, property_value: &str,
                                             scheduler: &mut SchedulerFrontend, path: String) -> Result<(), Error>  {

    state.get_color_config_mut().set_disabled_bg_color(load_base_properties::load_color_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.get_color_config_mut().set_disabled_bg_color(*val.as_color());
            path.clone()
        }))?);
    Ok(())
}


pub fn load_tab_foreground_color_property(state: &mut dyn GenericState, property_value: &str,
                                             scheduler: &mut SchedulerFrontend, path: String) -> Result<(), Error>  {

    state.get_color_config_mut().set_tab_fg_color(load_base_properties::load_color_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.get_color_config_mut().set_tab_fg_color(*val.as_color());
            path.clone()
        }))?);
    Ok(())
}


pub fn load_tab_background_color_property(state: &mut dyn GenericState, property_value: &str,
                                             scheduler: &mut SchedulerFrontend, path: String) -> Result<(), Error>  {

    state.get_color_config_mut().set_tab_bg_color(load_base_properties::load_color_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.get_color_config_mut().set_tab_bg_color(*val.as_color());
            path.clone()
        }))?);
    Ok(())
}


pub fn load_flash_foreground_color_property(state: &mut dyn GenericState, property_value: &str,
                                             scheduler: &mut SchedulerFrontend, path: String)
    -> Result<(), Error>  {

    state.get_color_config_mut().set_flash_fg_color(load_base_properties::load_color_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.get_color_config_mut().set_flash_fg_color(*val.as_color());
            path.clone()
        }))?);
    Ok(())
}


pub fn load_flash_background_color_property(state: &mut dyn GenericState, property_value: &str,
                                             scheduler: &mut SchedulerFrontend, path: String)
    -> Result<(), Error>  {

    state.get_color_config_mut().set_flash_bg_color(load_base_properties::load_color_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.get_color_config_mut().set_flash_bg_color(*val.as_color());
            path.clone()
        }))?);
    Ok(())
}


pub fn load_fill_foreground_color_property(state: &mut dyn GenericState, property_value: &str,
                                             scheduler: &mut SchedulerFrontend, path: String)
    -> Result<(), Error>  {

    state.get_color_config_mut().set_filler_fg_color(load_base_properties::load_color_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.get_color_config_mut().set_filler_fg_color(*val.as_color());
            path.clone()
        }))?);
    Ok(())
}


pub fn load_fill_background_color_property(state: &mut dyn GenericState, property_value: &str,
                                             scheduler: &mut SchedulerFrontend, path: String)
    -> Result<(), Error>  {

    state.get_color_config_mut().set_filler_bg_color(
        load_base_properties::load_color_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.get_color_config_mut().set_filler_bg_color(*val.as_color());
            path.clone()
        }))?);
    Ok(())
}


pub fn load_cursor_background_color_property(state: &mut dyn GenericState, property_value: &str,
                                           scheduler: &mut SchedulerFrontend, path: String)
    -> Result<(), Error>  {

    state.get_color_config_mut().set_cursor_color(load_base_properties::load_color_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.get_color_config_mut().set_cursor_color(*val.as_color());
            path.clone()
        }))?);
    Ok(())
}


pub fn load_valign_property(state: &mut dyn GenericState, property_value: &str,
                            scheduler: &mut SchedulerFrontend, path: String) -> Result<(), Error>  {

    state.set_vertical_alignment(load_base_properties::load_valign_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.set_vertical_alignment(*val.as_vertical_alignment());
            path.clone()
        }))?);
    Ok(())
}


pub fn load_halign_property(state: &mut dyn GenericState, property_value: &str,
                            scheduler: &mut SchedulerFrontend, path: String) -> Result<(), Error>  {

    state.set_horizontal_alignment(load_base_properties::load_halign_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.set_horizontal_alignment(*val.as_horizontal_alignment());
            path.clone()
        }))?);
    Ok(())
}


pub fn load_disabled_property(state: &mut dyn GenericState, property_value: &str,
                              scheduler: &mut SchedulerFrontend, path: String) -> Result<(), Error>  {

    state.set_disabled(load_base_properties::load_bool_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.set_disabled(*val.as_bool());
            path.clone()
        }))?);
    Ok(())
}


pub fn load_selection_order_property(state: &mut dyn GenericState, property_value: &str,
                              scheduler: &mut SchedulerFrontend, path: String) -> Result<(), Error> {

    state.set_selection_order(load_base_properties::load_usize_property(
        property_value.trim(), scheduler, path.clone(),
        Box::new(move |state_tree: &mut StateTree, val: EzValues| {
            let state = state_tree.get_by_path_mut(&path.clone())
                .as_generic_mut();
            state.set_selection_order(*val.as_usize());
            path.clone()
        }))?);
    Ok(())
}




/// Load a property common to all [EzObjects]. Returns a bool representing whether the property
/// was consumed. If not consumed it should be a property specific to a widget.
pub fn load_common_property(property_name: &str, property_value: String,
                              obj: &mut dyn EzObject, scheduler: &mut SchedulerFrontend)
    -> Result<bool, Error> {

    let path = obj.get_full_path();
    let state = obj.get_state_mut();
    match property_name {
        "id" => obj.set_id(property_value.trim().to_string()),
        "x" => load_x_property(state, property_value.trim(), scheduler, path)?,
        "y" => load_y_property(state, property_value.trim(), scheduler, path)?,
        "pos" => {
            let (x,y) = match property_value.trim().split_once(',') {
                Some((i, j)) => (i, j),
                None => return Err(Error::new(ErrorKind::InvalidData,
                format!("Could not load pos parameter: \"{}\". It must be in the form \
                \"pos: 5, 10\"", property_value)))
            };
            load_x_property(state, x.trim(), scheduler, path.clone())?;
            load_y_property(state, y.trim(), scheduler, path)?;
        },
        "size_hint" => load_full_size_hint_property(
            state,property_value.trim(), scheduler, path)?,
        "size_hint_x" => load_size_hint_x_property(
            state, property_value.trim(), scheduler, path)?,
        "size_hint_y" => load_size_hint_y_property(
            state, property_value.trim(), scheduler, path)?,
        "size" => {
            let (width, height) = match property_value.trim().split_once(',') {
                Some((i, j)) => (i, j),
                None => return Err(
                    Error::new(ErrorKind::InvalidData,
                               format!("Could not size parameter: \"{}\". \
                               It must be in the form \"pos: 5, 10\"", property_value)))
            };
            load_width_property(state, width.trim(), scheduler,path.clone())?;
            load_height_property(state, height.trim(), scheduler, path)?;
        },
        "width" => load_width_property(
            state, property_value.trim(), scheduler, path)?,
        "height" => load_height_property(
            state, property_value.trim(), scheduler, path)?,
        "pos_hint" => load_full_pos_hint_property(
            state, property_value.trim(), scheduler, path)?,
        "pos_hint_x" => load_horizontal_pos_hint_property(
            state, property_value.trim(), scheduler, path)?,
        "pos_hint_y" => load_vertical_pos_hint_property(
            state, property_value.trim(), scheduler, path)?,
        "auto_scale" => load_full_auto_scale_property(
            state, property_value.trim(), scheduler, path)?,
        "auto_scale_width" => load_auto_scale_width_property(
            state, property_value.trim(), scheduler, path)?,
        "auto_scale_height" => load_auto_scale_height_property(
            state, property_value.trim(), scheduler, path)?,
        "padding" => {
            let padding_params: Vec<&str> = property_value.trim().split(',').collect();
            load_padding_top_property(state, padding_params[0].trim(),
                                       scheduler,path.clone())?;
            load_padding_bottom_property(state, padding_params[1].trim(),
                                       scheduler,path.clone())?;
            load_padding_left_property(state, padding_params[2].trim(),
                                       scheduler,path.clone())?;
            load_padding_right_property(state, padding_params[3].trim(),
                                       scheduler,path)?;
        },
        "disabled" =>
            load_disabled_property(state, property_value.trim(), scheduler, path)?,
        "selection_order" =>
            load_selection_order_property(state, property_value.trim(), scheduler, path)?,
        "padding_x" => {
            let (left, right) = match property_value.split_once(',') {
                Some((i, j)) => (i, j),
                None => return Err(
                    Error::new(ErrorKind::InvalidData,
                               format!("Could not load padding_x parameter: \"{}\". \
                               It must be in the form \"pos: 5, 10\"", property_value)))
            };
            load_padding_left_property(state, left.trim(), scheduler,path.clone())?;
            load_padding_right_property(state, right.trim(), scheduler,path)?;
        },
        "padding_y" => {
            let (top, bottom) = match property_value.split_once(',') {
                Some((i, j)) => (i, j),
                None => return Err(
                    Error::new(
                        ErrorKind::InvalidData,
                        format!("Could not load pos parameter: \"{}\". It must be in the form \
                        \"pos: 5, 10\"", property_value)))
            };
            load_padding_left_property(state, top.trim(), scheduler,path.clone())?;
            load_padding_right_property(state, bottom.trim(), scheduler,path)?;
        },
        "padding_top" =>
            load_padding_top_property(state, property_value.trim(), scheduler, path)?,
        "padding_bottom" =>
            load_padding_bottom_property(state, property_value.trim(), scheduler, path)?,
        "padding_left" =>
            load_padding_left_property(state, property_value.trim(), scheduler, path)?,
        "padding_right" =>
            load_padding_right_property(state, property_value.trim(), scheduler, path)?,
        "halign" => load_halign_property(state, property_value.trim(), scheduler, path)?,
        "valign" => load_valign_property(state, property_value.trim(), scheduler, path)?,
        "fg_color" => load_foreground_color_property(
            state, property_value.trim(), scheduler, path)?,
        "bg_color" => load_background_color_property(
            state, property_value.trim(), scheduler, path)?,
        "disabled_fg_color" => load_disabled_foreground_color_property(
            state, property_value.trim(), scheduler, path)?,
        "disabled_bg_color" => load_disabled_background_color_property(
            state, property_value.trim(), scheduler, path)?,
        "selection_fg_color" => load_selection_foreground_color_property(
            state, property_value.trim(), scheduler, path)?,
        "selection_bg_color" => load_selection_background_color_property(
            state, property_value.trim(), scheduler, path)?,
        "flash_fg_color" => load_flash_foreground_color_property(
            state, property_value.trim(), scheduler, path)?,
        "flash_bg_color" => load_flash_background_color_property(
            state, property_value.trim(), scheduler, path)?,
        "tab_fg_color" => load_tab_foreground_color_property(
            state, property_value.trim(), scheduler, path)?,
        "tab_bg_color" => load_tab_background_color_property(
            state, property_value.trim(), scheduler, path)?,
        "fill_fg_color" => load_fill_foreground_color_property(
            state, property_value.trim(), scheduler, path)?,
        "fill_bg_color" => load_fill_background_color_property(
            state, property_value.trim(), scheduler, path)?,
        "cursor_color" => load_cursor_background_color_property(
            state, property_value.trim(), scheduler, path)?,
        "border" => load_border_enable_property(
            state, property_value.trim(), scheduler, path)?,
        "border_horizontal_symbol" => load_border_horizontal_property(
            state, property_value.trim(), scheduler, path)?,
        "border_vertical_symbol" => load_border_vertical_property(
            state, property_value.trim(), scheduler, path)?,
        "border_top_right_symbol" => load_border_top_left_property(
            state, property_value.trim(), scheduler, path)?,
        "border_top_left_symbol" => load_border_top_right_property(
            state, property_value.trim(), scheduler, path)?,
        "border_bottom_left_symbol" => load_border_bottom_left_property(
            state, property_value.trim(), scheduler, path)?,
        "border_bottom_right_symbol" => load_border_bottom_right_property(
            state, property_value.trim(), scheduler, path)?,
        "border_fg_color" => load_border_foreground_color_property(
            state, property_value.trim(), scheduler, path)?,
        "border_bg_color" => load_border_background_color_property(
            state, property_value.trim(), scheduler, path)?,
        _ => return Ok(false),
    }
    Ok(true)
}