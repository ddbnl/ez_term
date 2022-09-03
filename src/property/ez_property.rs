//! # Ez Property
//!
//! A module implementing the generic [EzProperty] struct.
use std::cmp::Ordering;
use std::ops::{Add, Sub};
use std::sync::mpsc::{channel, Receiver, Sender};

use crossterm::style::Color;

use crate::property::ez_values::EzValues;
use crate::scheduler::definitions::GenericFunction;
use crate::scheduler::scheduler::SchedulerFrontend;
use crate::states::definitions::{
    HorizontalAlignment, HorizontalPosHint, LayoutMode, LayoutOrientation, VerticalAlignment,
    VerticalPosHint,
};

/// A struct wrapping a property of a widget state.
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

    /// Value is locked and cannot changed. Used e.g. when user manually sets a property such as
    /// height.
    pub locked: bool,

    /// Sender for the channel belonging to this property. When a new value is set, the new value
    /// will be send over this channel. At runtime the [Scheduler] will own the receiver of this
    /// channel; if any other properties are subscribed to this property, new values received by
    /// the scheduler will be synced to the subscribers.
    tx: Sender<EzValues>,
}
impl<T> EzProperty<T>
where
    EzValues: From<T>,
{
    /// Create a new EzProperty. If this property belongs to a widget state the name must be a path
    /// to the widget state, followed by the property name.
    pub fn new(name: String, value: T) -> (Self, Receiver<EzValues>) {
        let (tx, rx): (Sender<EzValues>, Receiver<EzValues>) = channel();
        let property = EzProperty {
            name,
            value,
            tx,
            locked: false,
        };
        (property, rx)
    }

    /// Get a ref to the name of this property
    pub fn get_name(&self) -> &String {
        &self.name
    }

    /// Get a ref to the value of this property
    pub fn get(&self) -> &T {
        &self.value
    }

    /// Set the value of this property. If any other properties are subscribed to this property,
    /// they will automatically be updated as well.
    pub fn set(&mut self, new: T) -> bool
    where
        T: PartialEq + Clone,
    {
        if self.locked {
            return false;
        }
        if new != self.value {
            self.value = new.clone();
            self.tx
                .send(EzValues::from(new))
                .unwrap_or_else(|e| panic!("Error setting value \"{}\": {}.", self.name, e));
            true
        } else {
            false
        }
    }

    pub fn copy_from(&mut self, other: &EzProperty<T>)
    where
        T: PartialEq + Clone,
    {
        self.set(other.value.clone());
        self.locked = other.locked;
    }

    /// Bind a custom callback to this property which will be called when the value changes
    pub fn bind(&self, callback: GenericFunction, scheduler: &mut SchedulerFrontend) {
        scheduler.bind_property_callback(self.name.as_str(), callback);
    }
}
impl EzProperty<usize> {
    pub fn set_from_ez_value(&mut self, value: EzValues) -> bool {
        self.set(value.as_usize().to_owned())
    }
}
impl EzProperty<bool> {
    pub fn set_from_ez_value(&mut self, value: EzValues) -> bool {
        self.set(value.as_bool().to_owned())
    }
}
impl EzProperty<f64> {
    pub fn set_from_ez_value(&mut self, value: EzValues) -> bool {
        self.set(value.as_f64().to_owned())
    }
}
impl EzProperty<String> {
    pub fn set_from_ez_value(&mut self, value: EzValues) -> bool {
        self.set(value.as_string().to_owned())
    }
}
impl EzProperty<Color> {
    pub fn set_from_ez_value(&mut self, value: EzValues) -> bool {
        self.set(value.as_color().to_owned())
    }
}
impl EzProperty<LayoutMode> {
    pub fn set_from_ez_value(&mut self, value: EzValues) -> bool {
        self.set(value.as_layout_mode().to_owned())
    }
}
impl EzProperty<LayoutOrientation> {
    pub fn set_from_ez_value(&mut self, value: EzValues) -> bool {
        self.set(value.as_layout_orientation().to_owned())
    }
}
impl EzProperty<HorizontalAlignment> {
    pub fn set_from_ez_value(&mut self, value: EzValues) -> bool {
        self.set(value.as_horizontal_alignment().to_owned())
    }
}
impl EzProperty<VerticalAlignment> {
    pub fn set_from_ez_value(&mut self, value: EzValues) -> bool {
        self.set(value.as_vertical_alignment().to_owned())
    }
}
impl EzProperty<Option<f64>> {
    pub fn set_from_ez_value(&mut self, value: EzValues) -> bool {
        self.set(value.as_size_hint())
    }
}
impl EzProperty<VerticalPosHint> {
    pub fn set_from_ez_value(&mut self, value: EzValues) -> bool {
        self.set(value.as_vertical_pos_hint())
    }
}
impl EzProperty<HorizontalPosHint> {
    pub fn set_from_ez_value(&mut self, value: EzValues) -> bool {
        self.set(value.as_horizontal_pos_hint())
    }
}
impl<T> PartialEq for EzProperty<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
impl PartialEq<usize> for EzProperty<usize> {
    fn eq(&self, other: &usize) -> bool {
        &self.value == other
    }
}

impl PartialOrd<usize> for EzProperty<usize> {
    fn partial_cmp(&self, other: &usize) -> Option<Ordering> {
        Some(self.value.cmp(other))
    }
}
impl Add<usize> for EzProperty<usize> {
    type Output = usize;
    fn add(self, rhs: usize) -> Self::Output {
        self.value + rhs
    }
}
impl Sub<usize> for EzProperty<usize> {
    type Output = usize;
    fn sub(self, rhs: usize) -> Self::Output {
        self.value - rhs
    }
}
impl PartialEq<String> for EzProperty<String> {
    fn eq(&self, other: &String) -> bool {
        &self.value == other
    }
}
impl PartialEq<bool> for EzProperty<bool> {
    fn eq(&self, other: &bool) -> bool {
        &self.value == other
    }
}
impl PartialEq<Color> for EzProperty<Color> {
    fn eq(&self, other: &Color) -> bool {
        &self.value == other
    }
}
impl PartialEq<VerticalAlignment> for EzProperty<VerticalAlignment> {
    fn eq(&self, other: &VerticalAlignment) -> bool {
        &self.value == other
    }
}
impl PartialEq<HorizontalAlignment> for EzProperty<HorizontalAlignment> {
    fn eq(&self, other: &HorizontalAlignment) -> bool {
        &self.value == other
    }
}
impl PartialEq<VerticalPosHint> for EzProperty<VerticalPosHint> {
    fn eq(&self, other: &VerticalPosHint) -> bool {
        &self.value == other
    }
}
impl PartialEq<HorizontalPosHint> for EzProperty<HorizontalPosHint> {
    fn eq(&self, other: &HorizontalPosHint) -> bool {
        &self.value == other
    }
}
impl PartialEq<Option<f64>> for EzProperty<Option<f64>> {
    fn eq(&self, other: &Option<f64>) -> bool {
        &self.value == other
    }
}
