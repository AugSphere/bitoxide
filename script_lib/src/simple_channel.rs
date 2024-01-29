mod error;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use error::*;

#[derive(Debug, Clone)]
pub struct Sender<T> {
    tx: Rc<RefCell<VecDeque<T>>>,
}

#[derive(Debug)]
pub struct Receiver<T> {
    rx: Rc<RefCell<VecDeque<T>>>,
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let deque = VecDeque::new();
    let tx = Rc::new(RefCell::new(deque));
    let rx = tx.clone();
    (Sender { tx }, Receiver { rx })
}

impl<T> Sender<T> {
    pub fn send(&self, t: T) -> Result<(), SendError<T>> {
        match self.tx.try_borrow_mut() {
            Ok(mut tx) => {
                tx.push_back(t);
                Ok(())
            }
            Err(_) => Err(SendError(t)),
        }
    }
}
impl<T> Receiver<T> {
    pub fn recv(&self) -> Result<T, RecvError> {
        match self.rx.try_borrow_mut() {
            Ok(mut rx) => match rx.pop_front() {
                Some(v) => Ok(v),
                None => Err(RecvError::Empty),
            },
            Err(_) => Err(RecvError::FailedToBorrow),
        }
    }

    pub fn try_recv(&self) -> Result<T, RecvError> {
        self.recv()
    }

    pub fn try_iter(&self) -> TryIter<'_, T> {
        TryIter { rx: self }
    }
}

pub struct TryIter<'a, T: 'a> {
    rx: &'a Receiver<T>,
}

impl<'a, T> Iterator for TryIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.rx.try_recv().ok()
    }
}
