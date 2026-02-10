use super::{Vector, Zero};
use crate::math::{self, Complex, Sq, Sqrt};
use serde::{Deserialize, Serialize};
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
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

    pub fn distance(self, rhs: Point<T>) -> T
    where
        T: Sub<Output = T>,
        T: Sq<Output = T>,
        T: Add<Output = T>,
        T: Sqrt<Output = T>,
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
