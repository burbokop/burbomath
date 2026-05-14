use super::{Abs, Angle, Atan2, Sq, Sqrt};
use crate::{
    NonNeg,
    math::{Complex, Cos, Sin},
};
use core::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};

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

impl<T, R> AddAssign<Vector<R>> for Vector<T>
where
    T: AddAssign<R>,
{
    fn add_assign(&mut self, rhs: Vector<R>) {
        self.x += rhs.x;
        self.y += rhs.y;
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

impl<T, R> SubAssign<Vector<R>> for Vector<T>
where
    T: SubAssign<R>,
{
    fn sub_assign(&mut self, rhs: Vector<R>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
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

impl<T> Div<NonNeg<T>> for Vector<T>
where
    T: Div + Clone,
{
    type Output = Vector<<T as Div>::Output>;

    fn div(self, rhs: NonNeg<T>) -> Self::Output {
        let rhs = rhs.into_inner();
        Self::Output {
            x: self.x / rhs.clone(),
            y: self.y / rhs,
        }
    }
}

impl<T: Neg> Neg for Vector<T> {
    type Output = Vector<<T as Neg>::Output>;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<T> Vector<T> {
    pub fn x(&self) -> &T {
        &self.x
    }

    pub fn y(&self) -> &T {
        &self.y
    }

    pub fn from_polar(r: T, a: Angle<T>) -> Self
    where
        T: Clone + Cos<Output = T> + Sin<Output = T> + Mul<Output = T>,
    {
        Self {
            x: a.clone().cos() * r.clone(),
            y: a.clone().sin() * r.clone(),
        }
    }

    pub fn angle(self) -> Angle<<T as Atan2>::Output>
    where
        T: Atan2,
    {
        self.y.atan2(self.x)
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

    pub fn norm(self) -> <Self as Div<<T as Sqrt>::Output>>::Output
    where
        T: Sq<Output = T>,
        T: Add<Output = T>,
        T: Sqrt,
        T: Clone,
        Self: Div<<T as Sqrt>::Output>,
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
        T: Sqrt,
        T: Clone,
        Self: Div<<T as Sqrt>::Output>,
        <Self as Div<<T as Sqrt>::Output>>::Output: Into<(T, T)>,
    {
        let (r, i) = self.norm().into();
        Complex::from_cartesian(r, i)
    }

    /// Left perpendicular vector
    pub fn left_perp(self) -> Self
    where
        T: Neg<Output = T>,
    {
        Self {
            x: self.y,
            y: -self.x,
        }
    }

    /// Right perpendicular vector
    pub fn right_perp(self) -> Self
    where
        T: Neg<Output = T>,
    {
        Self {
            x: -self.y,
            y: self.x,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Complex, Vector};

    impl<T: approx::AbsDiffEq<Epsilon = T> + Clone> approx::AbsDiffEq for Vector<T> {
        type Epsilon = T;

        fn default_epsilon() -> Self::Epsilon {
            T::default_epsilon()
        }

        fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
            self.x.abs_diff_eq(&other.x, epsilon.clone()) && self.y.abs_diff_eq(&other.y, epsilon)
        }
    }

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
    fn neg() {
        let vec: Vector<_> = (12, 9).into();
        assert_eq!(-(-vec), vec);
    }

    #[test]
    fn perpendicular() {
        let vec: Vector<_> = (12, 9).into();

        assert_eq!(vec.left_perp().right_perp(), vec);
        assert_eq!(vec.left_perp().left_perp().left_perp().left_perp(), vec);
        assert_eq!(vec.right_perp().right_perp().right_perp().right_perp(), vec);
        assert_eq!(vec.left_perp().left_perp(), -vec);
        assert_eq!(vec.right_perp().right_perp(), -vec);
    }

    #[test]
    fn add_assign() {
        let mut vec: Vector<_> = (12, 9).into();
        vec += (1, -1).into();
        assert_eq!(vec, (13, 8).into());
    }

    #[test]
    fn sub_assign() {
        let mut vec: Vector<_> = (12, 9).into();
        vec -= (1, -1).into();
        assert_eq!(vec, (11, 10).into());
    }

    #[test]
    #[cfg(any(feature = "std", feature = "libm"))]
    fn len() {
        use crate::NonNeg;
        use approx::assert_abs_diff_eq;
        let vec: Vector<_> = (4., 3.).into();
        let len: NonNeg<f64> = vec.len();
        assert_abs_diff_eq!(len.into_inner(), 5., epsilon = 0.001);
    }

    #[test]
    #[cfg(any(feature = "std", feature = "libm"))]
    fn norm() {
        use approx::assert_abs_diff_eq;

        let vec: Vector<_> = (4., 3.).into();
        let norm = vec.norm();
        assert_abs_diff_eq!(norm, Vector::from((0.8, 0.6)), epsilon = 0.001);
    }

    #[test]
    #[cfg(any(feature = "std", feature = "libm"))]
    fn rotor() {
        use approx::assert_abs_diff_eq;

        let vec: Vector<_> = (4., 3.).into();
        let rot = vec.rotor();
        assert_abs_diff_eq!(rot, Complex::from_cartesian(0.8, 0.6), epsilon = 0.001);
    }
}
