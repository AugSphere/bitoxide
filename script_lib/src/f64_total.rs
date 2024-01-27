use std::cmp::Ordering;
use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone, Copy)]
pub struct F64Total(f64);

impl Deref for F64Total {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for F64Total {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<f64> for F64Total {
    fn from(value: f64) -> Self {
        F64Total(value)
    }
}

impl From<F64Total> for f64 {
    fn from(value: F64Total) -> Self {
        value.0
    }
}

impl PartialEq for F64Total {
    fn eq(&self, other: &Self) -> bool {
        self.0.total_cmp(other) == Ordering::Equal
    }
}

impl Eq for F64Total {}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl PartialOrd for F64Total {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.0.total_cmp(other))
    }
}

impl Ord for F64Total {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.total_cmp(other)
    }
}
