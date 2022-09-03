//! # Load common properties
//!
//! Functions to load properties common to all widgets (such as size, color, position, etc.). These
//! functions do two things:
//! - Initialize the property with the user passed value or a default value
//! - Pass an update closure, which is used if that property is bound to another property
use std::io::{Error, ErrorKind};

use crate::parser::load_base_properties;
use crate::scheduler::scheduler::SchedulerFrontend;
use crate::widgets::ez_object::EzObject;

/// Load a property common to all [EzObjects]. Returns a bool representing whether the property
/// was consumed. If not consumed it should be a property specific to a widget.
pub fn load_common_property(
    property_name: &str,
    property_value: String,
    obj: &mut dyn EzObject,
    scheduler: &mut SchedulerFrontend,
) -> Result<bool, Error> {
    let path = obj.get_path();
    let state = obj.get_state_mut();
    let property_name = property_name.trim();
    match property_name {
        "id" => obj.set_id(property_value.trim()),
        "x" => {
            load_base_properties::load_usize_property(
                property_value.trim(),
                scheduler,
                path,
                property_name,
                state,
            )?;
            state.get_position_mut().x.locked = property_value.trim().parse::<usize>().is_ok();
        }
        "y" => {
            load_base_properties::load_usize_property(
                property_value.trim(),
                scheduler,
                path,
                property_name,
                state,
            )?;
            state.get_position_mut().y.locked = property_value.trim().parse::<usize>().is_ok();
        }
        "pos" => {
            let (x, y) = match property_value.trim().split_once(',') {
                Some((i, j)) => (i, j),
                None => {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        format!(
                            "Could not load pos parameter: \"{}\". It must be in the form \
                \"pos: 5, 10\"",
                            property_value
                        ),
                    ))
                }
            };
            load_base_properties::load_usize_property(
                x.trim(),
                scheduler,
                path.clone(),
                "x",
                state,
            )?;
            state.get_position_mut().x.locked = x.trim().parse::<usize>().is_ok();
            load_base_properties::load_usize_property(y.trim(), scheduler, path, "y", state)?;
            state.get_position_mut().y.locked = y.trim().parse::<usize>().is_ok();
        }
        "size_hint" => {
            let (x, y) = match property_value.trim().split_once(',') {
                Some((i, j)) => (i, j),
                None => {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        format!(
                            "Could not load pos parameter: \"{}\". \
                                              It must be in the form \"size_hint: 0.3, 0.3\"",
                            property_value
                        ),
                    ))
                }
            };
            load_base_properties::load_size_hint_property(
                x.trim(),
                scheduler,
                path.clone(),
                "size_hint_x",
                state,
            )?;
            load_base_properties::load_size_hint_property(
                y.trim(),
                scheduler,
                path,
                "size_hint_y",
                state,
            )?;
        }
        "size_hint_x" => load_base_properties::load_size_hint_property(
            property_value.trim(),
            scheduler,
            path.clone(),
            property_name,
            state,
        )?,
        "size_hint_y" => load_base_properties::load_size_hint_property(
            property_value.trim(),
            scheduler,
            path.clone(),
            property_name,
            state,
        )?,
        "size" => {
            let (width, height) = match property_value.trim().split_once(',') {
                Some((i, j)) => (i, j),
                None => {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        format!(
                            "Could not size parameter: \"{}\". \
                               It must be in the form \"size: 5, 10\"",
                            property_value
                        ),
                    ))
                }
            };
            load_base_properties::load_usize_property(
                width.trim(),
                scheduler,
                path.clone(),
                "width",
                state,
            )?;
            state.get_size_mut().width.locked = width.trim().parse::<usize>().is_ok();
            load_base_properties::load_usize_property(
                height.trim(),
                scheduler,
                path,
                "height",
                state,
            )?;
            state.get_size_mut().height.locked = height.trim().parse::<usize>().is_ok();
        }
        "width" => {
            load_base_properties::load_usize_property(
                property_value.trim(),
                scheduler,
                path,
                property_name,
                state,
            )?;
            state.get_size_mut().width.locked = property_value.trim().parse::<usize>().is_ok();
        }
        "height" => {
            load_base_properties::load_usize_property(
                property_value.trim(),
                scheduler,
                path,
                property_name,
                state,
            )?;
            state.get_size_mut().height.locked = property_value.trim().parse::<usize>().is_ok();
        }
        "pos_hint" => {
            let (x_str, y_str) = property_value.split_once(',').unwrap();
            load_base_properties::load_horizontal_pos_hint_property(
                x_str,
                scheduler,
                path.clone(),
                "pos_hint_x",
                state,
            )?;
            load_base_properties::load_vertical_pos_hint_property(
                y_str,
                scheduler,
                path,
                "pos_hint_y",
                state,
            )?;
        }
        "pos_hint_x" => load_base_properties::load_horizontal_pos_hint_property(
            property_value.trim(),
            scheduler,
            path,
            property_name,
            state,
        )?,
        "pos_hint_y" => load_base_properties::load_vertical_pos_hint_property(
            property_value.trim(),
            scheduler,
            path,
            property_name,
            state,
        )?,
        "auto_scale" => {
            let (width_str, height_str) = property_value.split_once(',').unwrap_or_else(
                || panic!("The auto_scale property requires two \
        bool values. If you want to set only one, use auto_scale_height or auto_scale_width instead."));
            load_base_properties::load_bool_property(
                width_str.trim(),
                scheduler,
                path.clone(),
                "auto_scale_width",
                state,
            )?;
            load_base_properties::load_bool_property(
                height_str.trim(),
                scheduler,
                path,
                "auto_scale_height",
                state,
            )?;
        }
        "auto_scale_width" => load_base_properties::load_bool_property(
            property_value.trim(),
            scheduler,
            path,
            property_name,
            state,
        )?,
        "auto_scale_height" => load_base_properties::load_bool_property(
            property_value.trim(),
            scheduler,
            path,
            property_name,
            state,
        )?,
        "padding" => {
            let padding_params: Vec<&str> = property_value.trim().split(',').collect();
            load_base_properties::load_usize_property(
                padding_params[0],
                scheduler,
                path.clone(),
                "padding_top",
                state,
            )?;
            load_base_properties::load_usize_property(
                padding_params[1],
                scheduler,
                path.clone(),
                "padding_bottom",
                state,
            )?;
            load_base_properties::load_usize_property(
                padding_params[2],
                scheduler,
                path.clone(),
                "padding_left",
                state,
            )?;
            load_base_properties::load_usize_property(
                padding_params[3],
                scheduler,
                path.clone(),
                "padding_right",
                state,
            )?;
        }
        "padding_x" => {
            let (left, right) = match property_value.split_once(',') {
                Some((i, j)) => (i, j),
                None => {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        format!(
                            "Could not load padding_x parameter: \"{}\". \
                               It must be in the form \"pos: 5, 10\"",
                            property_value
                        ),
                    ))
                }
            };
            load_base_properties::load_usize_property(
                left.trim(),
                scheduler,
                path.clone(),
                "padding_left",
                state,
            )?;
            load_base_properties::load_usize_property(
                right.trim(),
                scheduler,
                path.clone(),
                "padding_right",
                state,
            )?;
        }
        "padding_y" => {
            let (top, bottom) = match property_value.split_once(',') {
                Some((i, j)) => (i, j),
                None => {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        format!(
                            "Could not load pos parameter: \"{}\". It must be in the form \
                        \"pos: 5, 10\"",
                            property_value
                        ),
                    ))
                }
            };
            load_base_properties::load_usize_property(
                top.trim(),
                scheduler,
                path.clone(),
                "padding_top",
                state,
            )?;
            load_base_properties::load_usize_property(
                bottom.trim(),
                scheduler,
                path.clone(),
                "padding_bottom",
                state,
            )?;
        }
        "padding_top" => load_base_properties::load_usize_property(
            property_value.trim(),
            scheduler,
            path.clone(),
            property_name,
            state,
        )?,
        "padding_bottom" => load_base_properties::load_usize_property(
            property_value.trim(),
            scheduler,
            path.clone(),
            property_name,
            state,
        )?,
        "padding_left" => load_base_properties::load_usize_property(
            property_value.trim(),
            scheduler,
            path.clone(),
            property_name,
            state,
        )?,
        "padding_right" => load_base_properties::load_usize_property(
            property_value.trim(),
            scheduler,
            path.clone(),
            property_name,
            state,
        )?,
        "disabled" => load_base_properties::load_bool_property(
            property_value.trim(),
            scheduler,
            path.clone(),
            property_name,
            state,
        )?,
        "selection_order" => load_base_properties::load_usize_property(
            property_value.trim(),
            scheduler,
            path.clone(),
            property_name,
            state,
        )?,
        "halign" => load_base_properties::load_halign_property(
            property_value.trim(),
            scheduler,
            path.clone(),
            property_name,
            state,
        )?,
        "valign" => load_base_properties::load_valign_property(
            property_value.trim(),
            scheduler,
            path.clone(),
            property_name,
            state,
        )?,
        "fg_color" => load_base_properties::load_color_property(
            property_value.trim(),
            scheduler,
            path.clone(),
            property_name,
            state,
        )?,
        "bg_color" => load_base_properties::load_color_property(
            property_value.trim(),
            scheduler,
            path.clone(),
            property_name,
            state,
        )?,
        "disabled_fg_color" => load_base_properties::load_color_property(
            property_value.trim(),
            scheduler,
            path.clone(),
            property_name,
            state,
        )?,
        "disabled_bg_color" => load_base_properties::load_color_property(
            property_value.trim(),
            scheduler,
            path.clone(),
            property_name,
            state,
        )?,
        "tab_header_active_fg_color" => load_base_properties::load_color_property(
            property_value.trim(),
            scheduler,
            path.clone(),
            property_name,
            state,
        )?,
        "tab_header_active_bg_color" => load_base_properties::load_color_property(
            property_value.trim(),
            scheduler,
            path.clone(),
            property_name,
            state,
        )?,
        "selection_fg_color" => load_base_properties::load_color_property(
            property_value.trim(),
            scheduler,
            path.clone(),
            property_name,
            state,
        )?,
        "selection_bg_color" => load_base_properties::load_color_property(
            property_value.trim(),
            scheduler,
            path.clone(),
            property_name,
            state,
        )?,
        "flash_fg_color" => load_base_properties::load_color_property(
            property_value.trim(),
            scheduler,
            path.clone(),
            property_name,
            state,
        )?,
        "flash_bg_color" => load_base_properties::load_color_property(
            property_value.trim(),
            scheduler,
            path.clone(),
            property_name,
            state,
        )?,
        "tab_header_fg_color" => load_base_properties::load_color_property(
            property_value.trim(),
            scheduler,
            path.clone(),
            property_name,
            state,
        )?,
        "tab_header_bg_color" => load_base_properties::load_color_property(
            property_value.trim(),
            scheduler,
            path.clone(),
            property_name,
            state,
        )?,
        "tab_header_border_fg_color" => load_base_properties::load_color_property(
            property_value.trim(),
            scheduler,
            path.clone(),
            property_name,
            state,
        )?,
        "tab_header_border_bg_color" => load_base_properties::load_color_property(
            property_value.trim(),
            scheduler,
            path.clone(),
            property_name,
            state,
        )?,
        "filler_fg_color" => load_base_properties::load_color_property(
            property_value.trim(),
            scheduler,
            path.clone(),
            property_name,
            state,
        )?,
        "filler_bg_color" => load_base_properties::load_color_property(
            property_value.trim(),
            scheduler,
            path.clone(),
            property_name,
            state,
        )?,
        "cursor_color" => load_base_properties::load_color_property(
            property_value.trim(),
            scheduler,
            path.clone(),
            property_name,
            state,
        )?,
        "border" => load_base_properties::load_bool_property(
            property_value.trim(),
            scheduler,
            path.clone(),
            property_name,
            state,
        )?,
        "border_horizontal_symbol" => load_base_properties::load_string_property(
            property_value.trim(),
            scheduler,
            path.clone(),
            property_name,
            state,
        )?,
        "border_vertical_symbol" => load_base_properties::load_string_property(
            property_value.trim(),
            scheduler,
            path.clone(),
            property_name,
            state,
        )?,
        "border_top_right_symbol" => load_base_properties::load_string_property(
            property_value.trim(),
            scheduler,
            path.clone(),
            property_name,
            state,
        )?,
        "border_top_left_symbol" => load_base_properties::load_string_property(
            property_value.trim(),
            scheduler,
            path.clone(),
            property_name,
            state,
        )?,
        "border_bottom_left_symbol" => load_base_properties::load_string_property(
            property_value.trim(),
            scheduler,
            path.clone(),
            property_name,
            state,
        )?,
        "border_bottom_right_symbol" => load_base_properties::load_string_property(
            property_value.trim(),
            scheduler,
            path.clone(),
            property_name,
            state,
        )?,
        "border_fg_color" => load_base_properties::load_color_property(
            property_value.trim(),
            scheduler,
            path.clone(),
            property_name,
            state,
        )?,
        "border_bg_color" => load_base_properties::load_color_property(
            property_value.trim(),
            scheduler,
            path.clone(),
            property_name,
            state,
        )?,
        _ => return Ok(false),
    }
    Ok(true)
}
