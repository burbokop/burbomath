use super::Vector;
use crate::{
    NonNeg, Sq, Sqrt, Zero,
    math::{self, Complex},
};
use core::ops::{Add, Mul, Sub};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Point<T> {
    x: T,
    y: T,
}

impl<T> From<(T, T)> for Point<T> {
    fn from(value: (T, T)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl<T> From<Point<T>> for (T, T) {
    fn from(value: Point<T>) -> Self {
        (value.x, value.y)
    }
}

impl<T> From<Point<T>> for [T; 2] {
    fn from(value: Point<T>) -> Self {
        [value.x, value.y]
    }
}

impl<T: Sub> Sub for Point<T> {
    type Output = Vector<<T as Sub>::Output>;
    fn sub(self, rhs: Self) -> Self::Output {
        (self.x - rhs.x, self.y - rhs.y).into()
    }
}

impl<T: Add> Add<Vector<T>> for Point<T> {
    type Output = Point<<T as Add>::Output>;
    fn add(self, rhs: Vector<T>) -> Self::Output {
        let (x, y) = rhs.into();
        (self.x + x, self.y + y).into()
    }
}

impl<T: Sub> Sub<Vector<T>> for Point<T> {
    type Output = Point<<T as Sub>::Output>;
    fn sub(self, rhs: Vector<T>) -> Self::Output {
        let (x, y) = rhs.into();
        (self.x - x, self.y - y).into()
    }
}

impl<T> Point<T> {
    pub fn origin() -> Self
    where
        T: Zero,
    {
        Self {
            x: T::zero(),
            y: T::zero(),
        }
    }

    pub fn x(&self) -> &T {
        &self.x
    }
    pub fn y(&self) -> &T {
        &self.y
    }

    pub fn absolute(self, origin: Point<T>) -> Self
    where
        T: Add<Output = T>,
    {
        (origin.x + self.x, origin.y + self.y).into()
    }

    pub fn relative(self, origin: Point<T>) -> Self
    where
        T: Sub<Output = T>,
    {
        (self.x - origin.x, self.y - origin.y).into()
    }

    pub fn distance(self, rhs: Point<T>) -> NonNeg<T>
    where
        T: Sub<Output = T>,
        T: Sq<Output = NonNeg<T>>,
        T: Add<Output = T>,
        NonNeg<T>: Sqrt<Output = NonNeg<T>>,
    {
        (self - rhs).len()
    }

    pub fn rotated(&self, center: Point<T>, rotor: Complex<T>) -> Point<T>
    where
        T: Add<Output = T>,
        T: Sub<Output = T>,
        T: Mul<Output = T>,
        T: Clone,
    {
        math::lerp(center, self.clone(), rotor)
    }

    pub fn map<F, R>(self, mut f: F) -> Point<R>
    where
        F: FnMut(T) -> R,
    {
        Point {
            x: f(self.x),
            y: f(self.y),
        }
    }
}

impl Point<f32> {
    pub fn as_f64(self) -> Point<f64> {
        Point {
            x: self.x as f64,
            y: self.y as f64,
        }
    }
}

impl Point<i32> {
    pub fn as_f64(self) -> Point<f64> {
        Point {
            x: self.x as f64,
            y: self.y as f64,
        }
    }

    pub fn as_f32(self) -> Point<f32> {
        Point {
            x: self.x as f32,
            y: self.y as f32,
        }
    }
}

impl Point<f64> {
    pub fn as_f32(self) -> Point<f32> {
        Point {
            x: self.x as f32,
            y: self.y as f32,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Point;

    impl<T: approx::AbsDiffEq<Epsilon = T> + Clone> approx::AbsDiffEq for Point<T> {
        type Epsilon = T;

        fn default_epsilon() -> Self::Epsilon {
            T::default_epsilon()
        }

        fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
            self.x.abs_diff_eq(&other.x, epsilon.clone()) && self.y.abs_diff_eq(&other.y, epsilon)
        }
    }

    #[test]
    fn map() {
        use approx::assert_abs_diff_eq;
        let point: Point<_> = (12, 9).into();
        assert_abs_diff_eq!(
            point.map(|x| x as f32),
            (12., 9.).into(),
            epsilon = 0.000001
        );
    }
}
