use std::cmp::Ordering;
use std::ops::{Add, Sub};
use std::sync::mpsc::{Sender, Receiver, channel};

#[derive(Clone, Debug)]
pub struct UsizeProperty {
    pub name: String,
    pub value: usize,
    tx: Sender<usize>,
}
impl UsizeProperty {

    pub fn new(name: String, value: usize) -> (Self, Receiver<usize>) {

        let (tx, rx): (Sender<usize>, Receiver<usize>) = channel();
        let property = UsizeProperty {
            name,
            value,
            tx
        };
        (property, rx)
    }

    pub fn get(&self) -> usize { self.value }

    pub fn set(&mut self, new: usize) {
        if new != self.value {
            self.value = new;
            self.tx.send(new).unwrap();
        }
    }

    pub fn add(&mut self, rhs: usize) {
        self.set(self.value + rhs);
    }
}
impl PartialEq for UsizeProperty {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
impl PartialEq<usize> for UsizeProperty {
    fn eq(&self, other: &usize) -> bool { &self.value == other }
}

impl PartialOrd<usize> for UsizeProperty {
    fn partial_cmp(&self, other: &usize) -> Option<Ordering> {
        Some(self.value.cmp(other))
    }
}

impl Add<usize> for UsizeProperty {
    type Output = usize;
    fn add(self, rhs: usize) -> Self::Output {
        self.value + rhs
    }
}
impl Sub<usize> for UsizeProperty {
    type Output = usize;
    fn sub(self, rhs: usize) -> Self::Output {
        self.value - rhs
    }
}


#[derive(Clone, Debug)]
pub struct StringProperty {
    pub name: String,
    pub value: String,
    tx: Sender<String>,
}
impl StringProperty {

    pub fn new(name: String, value: String) -> (Self, Receiver<String>) {

        let (tx, rx): (Sender<String>, Receiver<String>) = channel();
        let property = StringProperty {
            name,
            value,
            tx
        };
        (property, rx)
    }

    pub fn get(&self) -> &String { &self.value }

    pub fn set(&mut self, new: String) {
        if new != self.value {
            self.value = new.clone();
            self.tx.send(new).unwrap();
        }
    }
}
impl PartialEq for StringProperty {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
impl PartialEq<String> for StringProperty {
    fn eq(&self, other: &String) -> bool {
        &self.value == other
    }
}