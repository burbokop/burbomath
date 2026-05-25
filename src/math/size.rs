use crate::{Point, Two};
use core::ops::{Div, Mul};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Size<T> {
    w: T,
    h: T,
}

impl<T> Size<T> {
    pub fn center(&self) -> Point<T>
    where
        T: Two + Div<Output = T> + Clone,
    {
        (self.w.clone() / T::two(), self.h.clone() / T::two()).into()
    }

    pub fn map<F, R>(self, mut f: F) -> Size<R>
    where
        F: FnMut(T) -> R,
    {
        Size {
            w: f(self.w),
            h: f(self.h),
        }
    }
}

impl<T> From<(T, T)> for Size<T> {
    fn from(value: (T, T)) -> Self {
        Self {
            w: value.0,
            h: value.1,
        }
    }
}

impl<T> From<Size<T>> for (T, T) {
    fn from(value: Size<T>) -> Self {
        (value.w, value.h)
    }
}

impl<T> From<Size<T>> for [T; 2] {
    fn from(value: Size<T>) -> Self {
        [value.w, value.h]
    }
}

impl Size<f32> {
    pub fn as_f64(self) -> Size<f64> {
        Size {
            w: self.w as f64,
            h: self.h as f64,
        }
    }
}

impl Size<f64> {
    pub fn as_f32(self) -> Size<f32> {
        Size {
            w: self.w as f32,
            h: self.h as f32,
        }
    }
}

impl Size<u32> {
    pub fn as_f32(self) -> Size<f32> {
        Size {
            w: self.w as f32,
            h: self.h as f32,
        }
    }

    pub fn as_f64(self) -> Size<f64> {
        Size {
            w: self.w as f64,
            h: self.h as f64,
        }
    }
}

impl<T> Size<T> {
    pub fn w(&self) -> &T {
        &self.w
    }
    pub fn h(&self) -> &T {
        &self.h
    }
}

impl<T> Div<T> for Size<T>
where
    T: Div<Output = T> + Clone,
{
    type Output = Size<T>;

    fn div(self, rhs: T) -> Self::Output {
        Self::Output {
            w: self.w / rhs.clone(),
            h: self.h / rhs,
        }
    }
}

impl<T> Mul<T> for Size<T>
where
    T: Mul<Output = T> + Clone,
{
    type Output = Size<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Self::Output {
            w: self.w * rhs.clone(),
            h: self.h * rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Size;

    impl<T: approx::AbsDiffEq<Epsilon = T> + Clone> approx::AbsDiffEq for Size<T> {
        type Epsilon = T;

        fn default_epsilon() -> Self::Epsilon {
            T::default_epsilon()
        }

        fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
            self.w.abs_diff_eq(&other.w, epsilon.clone()) && self.h.abs_diff_eq(&other.h, epsilon)
        }
    }

    #[test]
    fn map() {
        use approx::assert_abs_diff_eq;
        let size: Size<_> = (12, 9).into();
        assert_abs_diff_eq!(size.map(|x| x as f32), (12., 9.).into(), epsilon = 0.000001);
    }
}
