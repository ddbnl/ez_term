use crossterm::style::Color;
use crate::states::definitions::{HorizontalAlignment, VerticalAlignment};
use crate::property::property::EzProperty;


#[derive(Clone, Debug)]
pub enum EzProperties {
    Usize(EzProperty<usize>),
    Bool(EzProperty<bool>),
    String(EzProperty<String>),
    Color(EzProperty<Color>),
    VerticalAlignment(EzProperty<VerticalAlignment>),
    HorizontalAlignment(EzProperty<HorizontalAlignment>),
    VerticalPosHint(EzProperty<Option<(VerticalAlignment, f64)>>),
    HorizontalPosHint(EzProperty<Option<(HorizontalAlignment, f64)>>),
    SizeHint(EzProperty<Option<f64>>),
}
impl EzProperties {

    pub fn as_usize(&self) -> &EzProperty<usize> {
        if let EzProperties::Usize(i) = self { i }
            else {panic!("Wrong property, expected UsizeProperty") }
    }

    pub fn as_usize_mut(&mut self) -> &mut EzProperty<usize> {
        if let EzProperties::Usize(i) = self { i }
            else { panic!("Wrong property, expected UsizeProperty") }
    }

    pub fn as_string(&self) -> &EzProperty<String> {
        if let EzProperties::String(i) = self { i }
            else { panic!("Wrong property, expected StringProperty") }
    }

    pub fn as_string_mut(&mut self) -> &mut EzProperty<String> {
        if let EzProperties::String(i) = self { i }
            else { panic!("Wrong property, expected StringProperty") }
    }

    pub fn as_bool(&self) -> &EzProperty<bool> {
        if let EzProperties::Bool(i) = self { i }
        else { panic!("Wrong property, expected BoolProperty") }
    }

    pub fn as_bool_mut(&mut self) -> &mut EzProperty<bool> {
        if let EzProperties::Bool(i) = self { i }
        else { panic!("Wrong property, expected BoolProperty") }
    }

    pub fn as_color(&self) -> &EzProperty<Color> {
        if let EzProperties::Color(i) = self { i }
        else { panic!("Wrong property, expected ColorProperty") }
    }

    pub fn as_color_mut(&mut self) -> &mut EzProperty<Color> {
        if let EzProperties::Color(i) = self { i }
        else { panic!("Wrong property, expected ColorProperty") }
    }

    pub fn as_vertical_alignment(&self) -> &EzProperty<VerticalAlignment> {
        if let EzProperties::VerticalAlignment(i) = self { i }
        else { panic!("Wrong property, expected VerticalAlignmentProperty") }
    }

    pub fn as_vertical_alignment_mut(&mut self) -> &mut EzProperty<VerticalAlignment> {
        if let EzProperties::VerticalAlignment(i) = self { i }
        else { panic!("Wrong property, expected VerticalAlignmentProperty") }
    }

    pub fn as_horizontal_alignment(&self) -> &EzProperty<HorizontalAlignment> {
        if let EzProperties::HorizontalAlignment(i) = self { i }
        else { panic!("Wrong property, expected HorizontalAlignmentProperty") }
    }

    pub fn as_horizontal_alignment_mut(&mut self) -> &mut EzProperty<HorizontalAlignment> {
        if let EzProperties::HorizontalAlignment(i) = self { i }
        else { panic!("Wrong property, expected HorizontalAlignmentProperty") }
    }

    pub fn as_horizontal_pos_hint(&self) -> &EzProperty<Option<(HorizontalAlignment, f64)>> {
        if let EzProperties::HorizontalPosHint(i) = self { i }
        else { panic!("Wrong property, expected HorizontalPosHintProperty") }
    }

    pub fn as_horizontal_pos_hint_mut(&mut self)
        -> &mut EzProperty<Option<(HorizontalAlignment, f64)>> {
        if let EzProperties::HorizontalPosHint(i) = self { i }
        else { panic!("Wrong property, expected HorizontalPosHintProperty") }
    }

    pub fn as_vertical_pos_hint(&self) -> &EzProperty<Option<(VerticalAlignment, f64)>> {
        if let EzProperties::VerticalPosHint(i) = self { i }
        else { panic!("Wrong property, expected VerticalPosHintProperty") }
    }

    pub fn as_vertical_pos_hint_mut(&mut self)
                                      -> &mut EzProperty<Option<(VerticalAlignment, f64)>> {
        if let EzProperties::VerticalPosHint(i) = self { i }
        else { panic!("Wrong property, expected VerticalPosHintProperty") }
    }

    pub fn as_size_hint(&self) -> &EzProperty<Option<f64>> {
        if let EzProperties::SizeHint(i) = self { i }
        else { panic!("Wrong property, expected SizeHintProperty") }
    }

    pub fn as_size_hint_mut(&mut self) -> &mut EzProperty<Option<f64>> {
        if let EzProperties::SizeHint(i) = self { i }
        else { panic!("Wrong property, expected SizeHintProperty") }
    }
}