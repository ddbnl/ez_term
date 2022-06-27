//! # Ez Parser
//! Module containing structs and functions for paring a .ez file into a root layout.
use std::io::{ErrorKind, Error};
use crossterm::style::{Color};
use std::str::FromStr;
use crate::states::definitions::{HorizontalAlignment, VerticalAlignment};


pub fn parse_color_property(value: &str) -> Result<Color, Error> {

    if value.contains(',') {
        let rgb: Vec<&str> = value.split(',').collect();
        if rgb.len() != 3 {
            return Err(Error::new(ErrorKind::InvalidData,
                           format!("Invalid rgb data in Ez file: {:?}. Must be in format: \
                           '255, 0, 0'", rgb)))
        }
        let r = match rgb[0].trim().parse() {
            Ok(i) => i,
            Err(_) => return Err(
                Error::new(ErrorKind::InvalidData,
                           format!("Could not parse the first number in this RGB value: {}",
                                   value)))
        };
        let g = match rgb[1].trim().parse() {
            Ok(i) => i,
            Err(_) => return Err(
                Error::new(ErrorKind::InvalidData,
                           format!("Could not parse the second number in this RGB value: {}",
                                   value)))
        };
        let b = match rgb[2].trim().parse() {
            Ok(i) => i,
            Err(_) => return Err(
                Error::new(ErrorKind::InvalidData,
                           format!("Could not parse the third number in this RGB value: {}",
                                   value)))
        };
        Ok(Color::from((r, g, b)))
    } else {
        Ok(Color::from_str(value.trim()).unwrap())
    }
}


/// Convenience function use by widgets to load a bool property defined in a .ez file.
/// Looks like "false".
pub fn parse_bool_property(value: &str) -> Result<bool, Error> {

    if value.to_lowercase() == "true" { Ok(true) }
    else if value.to_lowercase() == "false" { Ok(false) }
    else {
        Err(Error::new(ErrorKind::InvalidData,
                       format!("Ez file bool property must be true/false, not: {}", value))) }
}


/// Convenience function use by widgets to load a size_hint property defined in a .ez file.
/// Looks like "0.33" or "1/3"
pub fn parse_size_hint_property(value: &str) -> Result<Option<f64>, Error> {

    let to_parse = value.trim();
    // Size hint can be None
    if to_parse.to_lowercase() == "none" {
        Ok(None)
    }
    // Size hint can be a fraction
    else if to_parse.contains('/') {
        let (left_str, right_str) = match to_parse.split_once('/') {
            Some((i, j)) => (i, j),
            None => return Err(
                Error::new(ErrorKind::InvalidData,
                           format!("Size hint contains an invalid fraction: {}. \
                           Must be in format '1/3'", value)))
        };
        let left: f64 = match left_str.trim().parse() {
                Ok(i) => i,
                Err(_) => return Err(
                    Error::new(ErrorKind::InvalidData,
                               format!("Could not parse left side of size hint fraction: \
                               {}", value)))
        };
        let right: f64 = match right_str.trim().parse() {
            Ok(i) => i,
            Err(_) => return Err(
                Error::new(ErrorKind::InvalidData,
                           format!("Could not parse right side of size hint fraction: \
                           {}", value)))
        };
        let result = left / right;
        Ok(Some(result))
    }
    // Size hint can be a straight number
    else {
        let size_hint = match value.parse() {
            Ok(i) => i,
            Err(_) => return Err(
                Error::new(ErrorKind::InvalidData,
                           format!("This value is not allowed for pos_hint_x: {}. \
                           Use left/right/center", value)))
        };
        Ok(Some(size_hint))
    }
}


/// Convenience function use by widgets to load a pos_hint property defined in a .ez file.
/// Looks like "pos_hint_x: right: 0.9"
pub fn parse_horizontal_pos_hint_property(value: &str)
    -> Result<Option<(HorizontalAlignment, f64)>, Error> {

    let to_parse = value.trim();
    // Pos hint can be None
    if to_parse.to_lowercase() == "none" {
        return Ok(None)
    }
    // pos hint can one or two values. E.g. "top" or "top:0.8"
    let (keyword, fraction);
    if to_parse.contains(':') {
        let (keyword_str, fraction_str) = to_parse.split_once(':').unwrap();
        fraction = fraction_str.trim().parse().unwrap_or_else(
            |_| panic!("Could not parse pos hint: {}", value));
        keyword = keyword_str.trim();
    } else {
        keyword = value.trim();
        fraction = 1.0;  // Default fraction
    }
    let pos = match keyword {
        "left" => HorizontalAlignment::Left,
        "right" => HorizontalAlignment::Right,
        "center" => HorizontalAlignment::Center,
        _ => { return Err(Error::new(ErrorKind::InvalidData,
                                     format!("This value is not allowed for pos_hint_x: {}. \
                                     Use left/right/center", value))) }
    };
    Ok(Some((pos, fraction)))
}


/// Convenience function use by widgets to load a pos_hint_y property defined in a .ez file
/// Looks like "pos_hint_y: bottom: 0.9"
pub fn parse_vertical_pos_hint_property(value: &str)
    -> Result<Option<(VerticalAlignment, f64)>, Error> {

    let to_parse = value.trim();
    // Pos hint can be None
    if to_parse.to_lowercase() == "none" {
        return Ok(None)
    }
    // pos hint can one or two values. E.g. "top" or "top:0.8"
    let (keyword, fraction);
    if to_parse.contains(':') {
        let (keyword_str, fraction_str) = to_parse.split_once(':').unwrap();
        fraction = match fraction_str.trim().parse() {
            Ok(i) => i,
            Err(_) => return Err(Error::new(ErrorKind::InvalidData,
                                            format!("Could not parse pos hint: {}", value)))
        };
        keyword = keyword_str.trim();
    } else {
        keyword = value.trim();
        fraction= 1.0;  // Default fraction
    }
    let pos = match keyword {
        "top" => VerticalAlignment::Top,
        "bottom" => VerticalAlignment::Bottom,
        "middle" => VerticalAlignment::Middle,
        _ => return Err(Error::new(ErrorKind::InvalidData,
                                   format!("This value is not allowed for pos_hint_y: {}.\
                                   Use top/bottom/middle", value)))

    };
    Ok(Some((pos, fraction)))
}


/// Convenience function use by widgets to load a horizontal alignment defined in a .ez file.
/// Looks like: "left"
pub fn parse_halign_property(value: &str) -> Result<HorizontalAlignment, Error> {

    if value.to_lowercase() == "left" { Ok(HorizontalAlignment::Left) }
    else if value.to_lowercase() == "right" { Ok(HorizontalAlignment::Right) }
    else if value.to_lowercase() == "center" { Ok(HorizontalAlignment::Center) }
    else { Err(Error::new(ErrorKind::InvalidData,
                          format!("valign property must be left/center/right: {}", value))) }
}


/// Convenience function use by widgets to load a vertical alignment defined in a .ez file
/// Looks like: "bottom"
pub fn parse_valign_property(value: &str) -> Result<VerticalAlignment, Error> {

    if value.to_lowercase() == "top" { Ok(VerticalAlignment::Top) }
    else if value.to_lowercase() == "bottom" { Ok(VerticalAlignment::Bottom) }
    else if value.to_lowercase() == "middle" { Ok(VerticalAlignment::Middle) }
    else { Err(Error::new(ErrorKind::InvalidData,
                          format!(
                              "valign property must be top/middle/bottom: {}", value
                          ))) }
}