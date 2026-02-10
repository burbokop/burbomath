use super::{Abs, Angle, Atan2, Sq, Sqrt};
use crate::math::{Complex, Cos, Sin};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector<T> {
    x: T,
    y: T,
}

impl<T> From<(T, T)> for Vector<T> {
    fn from(value: (T, T)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl<T> From<Vector<T>> for (T, T) {
    fn from(value: Vector<T>) -> Self {
        (value.x, value.y)
    }
}

impl<T> Add for Vector<T>
where
    T: Add,
{
    type Output = Vector<<T as Add>::Output>;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> Sub for Vector<T>
where
    T: Sub,
{
    type Output = Vector<<T as Sub>::Output>;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> Mul<T> for Vector<T>
where
    T: Mul + Clone,
{
    type Output = Vector<<T as Mul>::Output>;

    fn mul(self, rhs: T) -> Self::Output {
        Self::Output {
            x: self.x * rhs.clone(),
            y: self.y * rhs,
        }
    }
}

impl<T> Mul<Complex<T>> for Vector<T>
where
    T: Mul<Output = T>,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Clone,
{
    type Output = Vector<T>;
    fn mul(self, rhs: Complex<T>) -> Self::Output {
        let (a, b) = self.into();
        let (c, d) = rhs.into_cartesian().into();

        (a.clone() * c.clone() - b.clone() * d.clone(), a * d + b * c).into()
    }
}

impl<T> Mul<Vector<T>> for Complex<T>
where
    T: Mul<Output = T>,
    T: Add<Output = T>,
    T: Sub<Output = T>,
    T: Clone,
{
    type Output = Vector<T>;
    fn mul(self, rhs: Vector<T>) -> Self::Output {
        let (a, b) = self.into_cartesian().into();
        let (c, d) = rhs.into();
        (a.clone() * c.clone() - b.clone() * d.clone(), a * d + b * c).into()
    }
}

impl<T> Div<T> for Vector<T>
where
    T: Div + Clone,
{
    type Output = Vector<<T as Div>::Output>;

    fn div(self, rhs: T) -> Self::Output {
        Self::Output {
            x: self.x / rhs.clone(),
            y: self.y / rhs,
        }
    }
}

impl<T> Vector<T> {
    pub fn from_polar(r: T, a: Angle<T>) -> Self
    where
        T: Clone + Cos<Output = T> + Sin<Output = T> + Mul<Output = T>,
    {
        Self {
            x: a.clone().cos() * r.clone(),
            y: a.clone().sin() * r.clone(),
        }
    }

    pub fn len(self) -> <<<T as Sq>::Output as Add>::Output as Sqrt>::Output
    where
        T: Sq,
        <T as Sq>::Output: Add,
        <<T as Sq>::Output as Add>::Output: Sqrt,
    {
        (self.x.sq() + self.y.sq()).sqrt()
    }

    pub fn manhattan_len(self) -> <<T as Abs>::Output as Add>::Output
    where
        T: Abs,
        <T as Abs>::Output: Add,
    {
        self.x.abs() + self.y.abs()
    }

    pub fn len_sqr(self) -> <<T as Sq>::Output as Add>::Output
    where
        T: Sq,
        <T as Sq>::Output: Add,
    {
        self.x.sq() + self.y.sq()
    }

    pub fn norm(self) -> Vector<T>
    where
        T: Sq<Output = T>,
        T: Add<Output = T>,
        T: Sqrt<Output = T>,
        T: Div<Output = T>,
        T: Clone,
    {
        self.clone() / self.len()
    }

    pub fn dot(self, rhs: Self) -> T
    where
        T: Mul<Output = T>,
        T: Add<Output = T>,
    {
        let (x0, y0) = self.into();
        let (x1, y1) = rhs.into();
        x0 * x1 + y0 * y1
    }

    pub fn cross(self, rhs: Self) -> T
    where
        T: Mul<Output = T>,
        T: Sub<Output = T>,
    {
        let (x0, y0) = self.into();
        let (x1, y1) = rhs.into();
        x0 * y1 - y0 * x1
    }

    pub fn rotor(self) -> Complex<T>
    where
        T: Sq<Output = T>,
        T: Add<Output = T>,
        T: Sqrt<Output = T>,
        T: Div<Output = T>,
        T: Clone,
    {
        let (r, i) = self.norm().into();
        Complex::from_cartesian(r, i)
    }
}

impl<T> Vector<T>
where
    T: Atan2,
{
    pub fn angle(self) -> Angle<<T as Atan2>::Output> {
        self.y.atan2(self.x)
    }
}

impl<T> Vector<T> {
    pub fn x(&self) -> &T {
        &self.x
    }
    pub fn y(&self) -> &T {
        &self.y
    }
}
