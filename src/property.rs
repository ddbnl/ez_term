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
            self.tx.send(self.value).unwrap();
        }
    }
}
impl PartialEq for UsizeProperty {
    fn eq(&self, other: &Self) -> bool { self.value == other.value }
}
