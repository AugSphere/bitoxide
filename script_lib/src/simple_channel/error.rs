use std::sync::mpsc;
use std::{error, fmt};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SendError<T>(T);

impl<T> fmt::Debug for SendError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SendError").finish_non_exhaustive()
    }
}

impl<T> fmt::Display for SendError<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "failed to send through the channel".fmt(f)
    }
}

impl<T> error::Error for SendError<T> {}

impl<T> From<mpsc::SendError<T>> for SendError<T> {
    fn from(value: mpsc::SendError<T>) -> Self {
        SendError(value.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RecvError;

impl fmt::Display for RecvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "failed to receive".fmt(f)
    }
}

impl error::Error for RecvError {}
