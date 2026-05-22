use crate::IsZero;
use core::ops::{Div, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Line<T> {
    k: T,
    d: T,
}

impl<T> Line<T> {
    pub fn k(&self) -> &T {
        &self.k
    }

    pub fn d(&self) -> &T {
        &self.d
    }
}

impl<T> From<(T, T)> for Line<T> {
    fn from(value: (T, T)) -> Self {
        Self {
            k: value.0,
            d: value.1,
        }
    }
}

impl<T> From<Line<T>> for (T, T) {
    fn from(value: Line<T>) -> Self {
        (value.k, value.d)
    }
}

impl<T> Line<T> {
    pub fn from_points(x0: T, y0: T, x1: T, y1: T) -> Option<Self>
    where
        T: Sub<Output = T> + Mul<Output = T> + Div<Output = T> + IsZero + Clone,
    {
        let dx = x1 - x0.clone();
        if dx.is_zero() {
            None
        } else {
            let k = (y1 - y0.clone()) / dx;
            let d = y0 - k.clone() * x0;
            Some(Self { k, d })
        }
    }
}
