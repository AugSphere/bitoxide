use std::{error, fmt};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SendError<T>(pub T);

impl<T> fmt::Debug for SendError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SendError").finish_non_exhaustive()
    }
}

impl<T> fmt::Display for SendError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "failed to borrow for sending".fmt(f)
    }
}

impl<T> error::Error for SendError<T> {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RecvError {
    Empty,
    FailedToBorrow,
}

impl fmt::Display for RecvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            RecvError::Empty => "receiving from an empty queue".fmt(f),
            RecvError::FailedToBorrow => "failed to borrow the queue".fmt(f),
        }
    }
}

impl error::Error for RecvError {}
