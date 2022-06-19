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

    pub fn get(&self) -> usize {
        self.value
    }

    pub fn set(&mut self, new: usize) {
        if new != self.value {
            self.value = new;
            self.tx.send(self.value.clone()).unwrap();
        }
    }
}
impl PartialEq for UsizeProperty {
    fn eq(&self, other: &Self) -> bool {
       self.value == other.value
    }

    fn ne(&self, other: &Self) -> bool {
       self.value != other.value
    }
}


#[derive(Clone, Debug)]
pub struct IsizeProperty {
    pub name: String,
    pub value: isize,
    tx: Sender<isize>,
}
impl IsizeProperty {

    pub fn new(name: String, value: isize) -> (Self, Receiver<isize>) {

        let (tx, rx): (Sender<isize>, Receiver<isize>) = channel();
        let property = IsizeProperty {
            name,
            value,
            tx
        };
        (property, rx)
    }

    pub fn get(&self) -> isize {
        self.value
    }

    pub fn set(&mut self, new: isize) {
        if new != self.value {
            self.value = new;
            self.tx.send(self.value.clone()).unwrap();
        }
    }
}
impl PartialEq for IsizeProperty {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }

    fn ne(&self, other: &Self) -> bool {
        self.value != other.value
    }
}
