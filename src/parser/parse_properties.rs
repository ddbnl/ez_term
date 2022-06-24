//! # Ez Parser
//! Module containing structs and functions for paring a .ez file into a root layout.
use crossterm::style::{Color};
use std::str::FromStr;
use crate::states::definitions::{HorizontalAlignment, VerticalAlignment};


pub fn parse_color_property(value: &str) -> Color {
    if value.contains(',') {
        let rgb: Vec<&str> = value.split(',').collect();
        if rgb.len() != 3 {
            panic!("Invalid rgb data in Ez file: {:?}. Must be in format: '255, 0, 0'", rgb)
        }
        Color::from(
            (rgb[0].trim().parse().unwrap_or_else(
                |_| panic!("Could not parse the first number in this RGB value: {}", value)),
            rgb[1].trim().parse().unwrap_or_else(
                |_| panic!("Could not parse the second number in this RGB value: {}", value)),
            rgb[2].trim().parse().unwrap_or_else(
                |_| panic!("Could not parse the third number in this RGB value: {}", value)),
            ))
    } else {
        Color::from_str(value.trim()).unwrap()
    }
}


/// Convenience function use by widgets to load a bool property defined in a .ez file.
/// Looks like "false".
pub fn parse_bool_property(value: &str) -> bool {

    if value.to_lowercase() == "true" { true }
    else if value.to_lowercase() == "false" { false }
    else {
        panic!("Ez file bool property must be true/false, not: {}", value) }
}


/// Convenience function use by widgets to load a size_hint property defined in a .ez file.
/// Looks like "0.33" or "1/3"
pub fn parse_size_hint_property(value: &str) -> Option<f64> {

    let to_parse = value.trim();
    // Size hint can be None
    if to_parse.to_lowercase() == "none" {
        None
    }
    // Size hint can be a fraction
    else if to_parse.contains('/') {
        let (left_str, right_str) = to_parse.split_once('/').unwrap_or_else(
            || panic!("Size hint contains an invalid fraction: {}. Must be in format '1/3'",
                      value));
        let left: f64 = left_str.trim().parse().unwrap_or_else(
            |_| panic!("Could not parse left side of size hint fraction: {}", value));
        let right: f64 = right_str.trim().parse().unwrap_or_else(
            |_| panic!("Could not parse right side of size hint fraction: {}", value));
        let result = left / right;
        Some(result)
    }
    // Size hint can be a straight number
    else {
        let size_hint = value.parse().unwrap_or_else(
            |_| panic!("Could not parse this size hint number: {}", value));
        Some(size_hint)
    }
}


/// Convenience function use by widgets to load a pos_hint property defined in a .ez file.
/// Looks like "pos_hint_x: right: 0.9"
pub fn parse_horizontal_pos_hint_property(value: &str) -> Option<(HorizontalAlignment, f64)> {

    let to_parse = value.trim();
    // Pos hint can be None
    if to_parse.to_lowercase() == "none" {
        return None
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
        _ => panic!("This value is not allowed for pos_hint_x: {}. Use left/right/center",
                    value)
    };
    Some((pos, fraction))
}


/// Convenience function use by widgets to load a pos_hint_y property defined in a .ez file
/// Looks like "pos_hint_y: bottom: 0.9"
pub fn parse_vertical_pos_hint_property(value: &str)
                                         -> Option<(VerticalAlignment, f64)> {

    let to_parse = value.trim();
    // Pos hint can be None
    if to_parse.to_lowercase() == "none" {
        return None
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
        fraction= 1.0;  // Default fraction
    }
    let pos = match keyword {
        "top" => VerticalAlignment::Top,
        "bottom" => VerticalAlignment::Bottom,
        "middle" => VerticalAlignment::Middle,
        _ => panic!("This value is not allowed for pos_hint_y: {}. Use top/bottom/middle",
                    value)
    };
    Some((pos, fraction))
}


/// Convenience function use by widgets to load a horizontal alignment defined in a .ez file.
/// Looks like: "left"
pub fn parse_halign_property(value: &str) -> HorizontalAlignment {

    if value.to_lowercase() == "left" { HorizontalAlignment::Left }
    else if value.to_lowercase() == "right" { HorizontalAlignment::Right }
    else if value.to_lowercase() == "center" { HorizontalAlignment::Center }
    else { panic!("halign property must be left/right/center: {}", value) }
}


/// Convenience function use by widgets to load a vertical alignment defined in a .ez file
/// Looks like: "bottom"
pub fn parse_valign_property(value: &str) -> VerticalAlignment {

    if value.to_lowercase() == "top" { VerticalAlignment::Top }
    else if value.to_lowercase() == "bottom" { VerticalAlignment::Bottom }
    else if value.to_lowercase() == "middle" { VerticalAlignment::Middle }
    else { panic!("valign property must be left/right/center: {}", value) }
}