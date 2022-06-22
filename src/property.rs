use std::cmp::Ordering;
use std::ops::{Add, Sub};
use std::sync::mpsc::{Sender, Receiver, channel};
use crate::common::definitions::{GenericEzFunction};
use crate::{CallbackConfig};
use crate::scheduler::Scheduler;


#[derive(Clone, Debug)]
pub enum EzValues {
    Usize(usize),
    Bool(bool),
    String(String),
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


#[derive(Clone, Debug)]
pub enum EzProperties {
    Usize(EzProperty<usize>),
    Bool(EzProperty<bool>),
    String(EzProperty<String>)
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
