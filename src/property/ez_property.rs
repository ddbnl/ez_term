//! # Ez Property
//!
//! A module implementing the generic [EzProperty] struct.
use std::cmp::Ordering;
use std::ops::{Add, Sub};
use std::sync::mpsc::{channel, Receiver, Sender};

use crossterm::style::Color;

use crate::CallbackConfig;
use crate::property::ez_values::EzValues;
use crate::scheduler::definitions::GenericEzFunction;
use crate::scheduler::scheduler::Scheduler;
use crate::states::definitions::{HorizontalAlignment, VerticalAlignment};


/// A struct wrapping a property of a widget state.
///
/// A widget has a state, which has properties (such as size, position, color, etc.). This struct
/// wraps around those properties to allow binding one property to another. If property A is bound
/// to property B, then whenever property B gets a new value, that value is set for property A as
/// well.
#[derive(Clone, Debug)]
pub struct EzProperty<T> {

    /// Name of the value. When a user creates a custom value this can be anything. If it's a
    /// widget state value then the name will be the path to the widget followed by the name of thev
    /// value, e.g. "/root/layout_1/label/height".
    pub name: String,

    /// Current value of the property.
    pub value: T,

    /// Sender for the channel belonging to this property. When a new value is set, the new value
    /// will be send over this channel. At runtime the [Scheduler] will own the receiver of this
    /// channel; if any other properties are subscribed to this property, new values received by
    /// the scheduler will be synced to the subscribers.
    tx: Sender <EzValues>,
}
impl<T> EzProperty<T> where EzValues: From<T> {

    /// Create a new EzProperty. If this property belongs to a widget state the name must be a path
    /// to the widget state, followed by the property name.
    pub fn new(name: String, value: T) -> (Self, Receiver<EzValues>) {

        let (tx, rx): (Sender<EzValues>, Receiver<EzValues>) = channel();
        let property = EzProperty {
            name,
            value,
            tx
        };
        (property, rx)
    }

    /// Get a ref to the name of this property
    pub fn get_name(&self) -> &String { &self.name }

    /// Get a ref to the value of this property
    pub fn get(&self) -> &T { &self.value }

    /// Set the value of this property. If any other properties are subscribed to this property,
    /// they will automatically be updated as well.
    pub fn set(&mut self, new: T) where T: PartialEq + Clone {

        if new != self.value {
            self.value = new.clone();
            self.tx.send(EzValues::from(new)).unwrap();
        }
    }

    /// Bind a custom callback to this property which will be called when the value changes. This
    /// is unrelated to binding widgets to one another. This is only used to allow end-users to
    /// bind a callback to an EzProperty.
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
