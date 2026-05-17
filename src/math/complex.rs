use super::{Angle, Point};
use crate::{Cos, NonNeg, Sin, Sq, math::Vector};
use core::ops::{Add, Div, Mul, Neg, Not, Sub};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex<T> {
    pub(crate) real: T,
    pub(crate) imag: T,
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
            imag: a.sin() * r,
        }
    }

    pub fn from_uneven_polar(r: Vector<T>, a: Angle<T>) -> Self
    where
        T: Clone + Cos<Output = T> + Sin<Output = T> + Mul<Output = T>,
    {
        let (x, y) = r.into();
        Self {
            real: a.clone().cos() * x,
            imag: a.sin() * y,
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

impl<T> From<(T, T)> for Complex<T> {
    fn from(value: (T, T)) -> Self {
        Self {
            real: value.0,
            imag: value.1,
        }
    }
}

impl<T> From<Complex<T>> for (T, T) {
    fn from(value: Complex<T>) -> Self {
        (value.real, value.imag)
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
    T: Sq<Output = NonNeg<T>>,
    T: Add<Output = T>,
    T: Div<Output = T>,
    T: Neg<Output = T>,
    T: Clone,
{
    type Output = Self;

    fn not(self) -> Self::Output {
        let len_sq = self.real.clone().sq() + self.imag.clone().sq();
        (
            self.real / len_sq.clone().into_inner(),
            -self.imag / len_sq.into_inner(),
        )
            .into()
    }
}

#[cfg(test)]
mod tests {
    use crate::Complex;

    impl<T: approx::AbsDiffEq<Epsilon = T> + Clone> approx::AbsDiffEq for Complex<T> {
        type Epsilon = T;

        fn default_epsilon() -> Self::Epsilon {
            T::default_epsilon()
        }

        fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
            self.real.abs_diff_eq(&other.real, epsilon.clone())
                && self.imag.abs_diff_eq(&other.imag, epsilon)
        }
    }

    #[test]
    #[cfg(any(feature = "std", feature = "libm"))]
    fn not() {
        use crate::Angle;
        use approx::assert_abs_diff_eq;
        let rot: Complex<f32> = Complex::from_polar(1., Angle::from_degrees(60_f32));
        assert_abs_diff_eq!(
            !rot,
            Complex::from_polar(1., Angle::from_degrees(-60_f32)),
            epsilon = 0.001
        );
        assert_abs_diff_eq!(!(!rot), rot, epsilon = 0.001);
    }
}
