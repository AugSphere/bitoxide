use std::sync::mpsc;

mod error;
use error::*;

#[derive(Debug, Clone)]
pub struct Sender<T> {
    tx: mpsc::Sender<T>,
}

#[derive(Debug)]
pub struct Receiver<T> {
    rx: mpsc::Receiver<T>,
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let (tx, rx) = mpsc::channel::<T>();
    (Sender { tx }, Receiver { rx })
}

impl<T> Sender<T> {
    pub fn send(&self, t: T) -> Result<(), SendError<T>> {
        self.tx.send(t).map_err(|x| x.into())
    }
}
impl<T> Receiver<T> {
    pub fn recv(&self) -> Result<T, RecvError> {
        self.rx.recv().map_err(|_| RecvError)
    }

    pub fn try_recv(&self) -> Result<T, RecvError> {
        self.rx.try_recv().map_err(|_| RecvError)
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
