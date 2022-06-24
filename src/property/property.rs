use std::cmp::Ordering;
use std::ops::{Add, Sub};
use std::sync::mpsc::{Sender, Receiver, channel};
use crossterm::style::Color;
use crate::common::definitions::{GenericEzFunction};
use crate::{CallbackConfig};
use crate::scheduler::scheduler::Scheduler;
use crate::states::definitions::{HorizontalAlignment, VerticalAlignment};
use crate::property::values::EzValues;


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
