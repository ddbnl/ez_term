use crossterm::style::Color;
use crate::states::definitions::{HorizontalAlignment, VerticalAlignment};


#[derive(Clone, Debug)]
pub enum EzValues {
    Usize(usize),
    Bool(bool),
    String(String),
    Color(Color),
    HorizontalAlignment(HorizontalAlignment),
    VerticalAlignment(VerticalAlignment),
    SizeHint(Option<f64>),
    VerticalPosHint(Option<(VerticalAlignment, f64)>),
    HorizontalPosHint(Option<(HorizontalAlignment, f64)>),
}
impl EzValues {

    pub fn as_usize(&self) -> &usize {
        if let EzValues::Usize(i) = self { i }
            else { panic!("Wrong property, expected UsizeProperty") }
    }

    pub fn as_string(&self) -> &String {
        if let EzValues::String(i) = self { i }
            else { panic!("Wrong property, expected StringProperty") }
    }

    pub fn as_bool(&self) -> &bool {
        if let EzValues::Bool(i) = self { i }
        else { panic!("Wrong property, expected BoolProperty") }
    }

    pub fn as_color(&self) -> &Color {
        if let EzValues::Color(i) = self { i }
        else { panic!("Wrong property, expected ColorProperty") }
    }

    pub fn as_vertical_alignment(&self) -> &VerticalAlignment {
        if let EzValues::VerticalAlignment(i) = self { i }
        else { panic!("Wrong property, expected VerticalAlignmentProperty") }
    }

    pub fn as_horizontal_alignment(&self) -> &HorizontalAlignment {
        if let EzValues::HorizontalAlignment(i) = self { i }
        else { panic!("Wrong property, expected HorizontalAlignmentProperty") }
    }

    pub fn as_vertical_pos_hint(&self) -> &Option<(VerticalAlignment, f64)> {
        if let EzValues::VerticalPosHint(i) = self { i }
        else { panic!("Wrong property, expected VerticalPosHintProperty") }
    }

    pub fn as_horizontal_pos_hint(&self) -> &Option<(HorizontalAlignment, f64)> {
        if let EzValues::HorizontalPosHint(i) = self { i }
        else { panic!("Wrong property, expected HorizontalPosHintProperty") }
    }

    pub fn as_size_hint(&self) -> &Option<f64> {
        if let EzValues::SizeHint(i) = self { i }
        else { panic!("Wrong property, expected SizeHintProperty") }
    }
}
impl From<usize> for EzValues {
    fn from(inner: usize) -> EzValues { EzValues::Usize(inner) }
}
impl From<bool> for EzValues {
    fn from(inner: bool) -> EzValues { EzValues::Bool(inner) }
}
impl From<String> for EzValues {
    fn from(inner: String) -> EzValues { EzValues::String(inner) }
}
impl From<Color> for EzValues {
    fn from(inner: Color) -> EzValues { EzValues::Color(inner) }
}
impl From<VerticalAlignment> for EzValues {
    fn from(inner: VerticalAlignment) -> EzValues { EzValues::VerticalAlignment(inner) }
}
impl From<HorizontalAlignment> for EzValues {
    fn from(inner: HorizontalAlignment) -> EzValues { EzValues::HorizontalAlignment(inner) }
}
impl From<Option<(VerticalAlignment, f64)>> for EzValues {
    fn from(inner: Option<(VerticalAlignment, f64)>)
        -> EzValues { EzValues::VerticalPosHint(inner) }
}
impl From<Option<(HorizontalAlignment, f64)>> for EzValues {
    fn from(inner: Option<(HorizontalAlignment, f64)>)
            -> EzValues { EzValues::HorizontalPosHint(inner) }
}
impl From<Option<f64>> for EzValues {
    fn from(inner: Option<f64>) -> EzValues { EzValues::SizeHint(inner) }
}