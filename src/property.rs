use std::cmp::Ordering;
use std::ops::{Add, Sub};
use std::sync::mpsc::{Sender, Receiver, channel};
use crossterm::style::Color;
use crate::common::definitions::{GenericEzFunction};
use crate::{CallbackConfig};
use crate::scheduler::Scheduler;
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

#[derive(Clone, Debug)]
pub struct EzProperty<T> {

    pub name: String,
    pub value: T,
    tx: Sender <EzValues>,
}
impl<T> EzProperty<T> where EzValues: From<T> {

    pub fn new(name: String, value: T) -> (Self, Receiver<EzValues>) {

        let (tx, rx): (Sender<EzValues>, Receiver<EzValues>) = channel();
        let property = EzProperty {
            name,
            value,
            tx
        };
        (property, rx)
    }

    pub fn get_name(&self) -> &String { &self.name }

    pub fn get(&self) -> &T { &self.value }

    pub fn set(&mut self, new: T) where T: PartialEq + Clone {

        if new != self.value {
            self.value = new.clone();
            self.tx.send(EzValues::from(new)).unwrap();
        }
    }

    pub fn bind(&self, callback: GenericEzFunction, scheduler: &mut Scheduler) {

        let config = CallbackConfig::from_on_value_change(callback);
        scheduler.set_callback_config(self.get_name().as_str(), config);
    }
}
impl<T> PartialEq for EzProperty<T> where T: PartialEq {
    fn eq(&self, other: &Self) -> bool { self.value == other.value }
}
impl PartialEq<usize> for EzProperty<usize> {
    fn eq(&self, other: &usize) -> bool { &self.value == other }
}

impl PartialOrd<usize> for EzProperty<usize> {
    fn partial_cmp(&self, other: &usize) -> Option<Ordering> { Some(self.value.cmp(other)) }
}
impl Add<usize> for EzProperty<usize> {
    type Output = usize;
    fn add(self, rhs: usize) -> Self::Output { self.value + rhs }
}
impl Sub<usize> for EzProperty<usize> {
    type Output = usize;
    fn sub(self, rhs: usize) -> Self::Output { self.value - rhs }
}
impl PartialEq<String> for EzProperty<String> {
    fn eq(&self, other: &String) -> bool { &self.value == other }
}
impl PartialEq<bool> for EzProperty<bool> {
    fn eq(&self, other: &bool) -> bool { &self.value == other }
}
impl PartialEq<Color> for EzProperty<Color> {
    fn eq(&self, other: &Color) -> bool { &self.value == other }
}
impl PartialEq<VerticalAlignment> for EzProperty<VerticalAlignment> {
    fn eq(&self, other: &VerticalAlignment) -> bool { &self.value == other }
}
impl PartialEq<HorizontalAlignment> for EzProperty<HorizontalAlignment> {
    fn eq(&self, other: &HorizontalAlignment) -> bool { &self.value == other }
}
impl PartialEq<Option<(VerticalAlignment, f64)>> for EzProperty<Option<(VerticalAlignment, f64)>> {
    fn eq(&self, other: &Option<(VerticalAlignment, f64)>) -> bool { &self.value == other }
}
impl PartialEq<Option<(HorizontalAlignment, f64)>> for EzProperty<Option<(HorizontalAlignment, f64)>> {
    fn eq(&self, other: &Option<(HorizontalAlignment, f64)>) -> bool { &self.value == other }
}
impl PartialEq<Option<f64>> for EzProperty<Option<f64>> {
    fn eq(&self, other: &Option<f64>) -> bool { &self.value == other }
}
