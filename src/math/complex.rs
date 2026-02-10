use super::{Angle, Cos, Point, Sin};
use crate::math::{Sq, Vector};
use std::ops::{Add, Div, Mul, Neg, Not, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Complex<T> {
    real: T,
    imag: T,
}

impl<T> From<(T, T)> for Complex<T> {
    fn from(value: (T, T)) -> Self {
        Self {
            real: value.0,
            imag: value.1,
        }
    }
}

impl<T> Complex<T> {
    pub fn from_cartesian(real: T, imag: T) -> Self {
        Self { real, imag }
    }

    pub fn from_polar(r: T, a: Angle<T>) -> Self
    where
        T: Clone + Cos<Output = T> + Sin<Output = T> + Mul<Output = T>,
    {
        Self {
            real: a.clone().cos() * r.clone(),
            imag: a.clone().sin() * r.clone(),
        }
    }

    pub fn real(&self) -> &T {
        &self.real
    }

    pub fn imag(&self) -> &T {
        &self.imag
    }

    pub fn into_cartesian(self) -> Point<T> {
        (self.real, self.imag).into()
    }

    pub fn div(v0: Vector<T>, v1: Vector<T>) -> Self
    where
        T: Sq<Output = T>,
        T: Add<Output = T>,
        T: Sub<Output = T>,
        T: Mul<Output = T>,
        T: Div<Output = T>,
        T: Neg<Output = T>,
        T: Clone,
    {
        let (a, b) = v0.into();
        let (c, d) = v1.into();
        let len_sq = c.clone().sq() + d.clone().sq();

        (
            (a.clone() * c.clone() + b.clone() * d.clone()) / len_sq.clone(),
            (b * c - a * d) / len_sq,
        )
            .into()
    }
}

impl<T> Add for Complex<T>
where
    T: Add,
{
    type Output = Complex<<T as Add>::Output>;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            real: self.real + rhs.real,
            imag: self.imag + rhs.imag,
        }
    }
}

impl<T> Mul for Complex<T>
where
    T: Mul<Output = T> + Clone + Add<Output = T> + Sub<Output = T>,
{
    type Output = Complex<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        let (a, b) = self.into_cartesian().into();
        let (c, d) = (rhs.real, rhs.imag);
        (a.clone() * c.clone() - b.clone() * d.clone(), a * d + b * c).into()
    }
}

impl<T> Not for Complex<T>
where
    T: Sq<Output = T>,
    T: Add<Output = T>,
    T: Div<Output = T>,
    T: Neg<Output = T>,
    T: Clone,
{
    type Output = Self;

    fn not(self) -> Self::Output {
        let len_sq = self.real.clone().sq() + self.imag.clone().sq();
        (self.real / len_sq.clone(), -self.imag / len_sq).into()
    }
}
