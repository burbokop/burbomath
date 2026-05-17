use crate::IsNeg;
use crate::{Abs, Floor, Pi, Positive, Sqrt, Zero};
use core::ops::{DivAssign, MulAssign, SubAssign};
use core::{
    error::Error,
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Div, Mul, Sub},
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Cannot store negative numbers
///
/// Can be constructed by macro that checks at compile time that it is indeed not negative
/// ```
/// use burbomath::non_neg;
/// non_neg!(1_i32);
/// ```
/// ```compile_fail
/// use burbomath::non_neg;
/// burbomath::non_neg!(-1_i32);
/// ```
#[derive(Clone, Copy, Debug, Ord, Eq)]
pub struct NonNeg<T> {
    pub(super) value: T,
}

#[cfg(feature = "serde")]
impl<T: Serialize> Serialize for NonNeg<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.value.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de, T: Deserialize<'de> + IsNeg + Debug> Deserialize<'de> for NonNeg<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = T::deserialize(deserializer)?;
        if value.is_neg() {
            #[cfg(feature = "std")]
            let err = Err(serde::de::Error::custom(&std::format!(
                "Can not deserialize {:?} as NoNeg because it is negative.",
                value
            )));
            #[cfg(not(feature = "std"))]
            let err = Err(serde::de::Error::custom(
                "Can not deserialize a negative number as NoNeg.",
            ));
            err
        } else {
            Ok(Self { value })
        }
    }
}

impl<T: Display> Display for NonNeg<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.value.fmt(f)
    }
}

#[derive(Debug)]
pub struct NegError<T> {
    original_value: T,
}

impl<T> NegError<T> {
    pub fn original_value(self) -> T {
        self.original_value
    }
}

impl<T> Display for NegError<T> {
    fn fmt(&self, _: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        todo!()
    }
}

impl<T> Error for NegError<T> where T: Debug {}

impl<T> NonNeg<T> {
    pub fn new(value: T) -> Result<Self, NegError<T>>
    where
        T: IsNeg,
    {
        if value.is_neg() {
            Err(NegError {
                original_value: value,
            })
        } else {
            Ok(Self { value })
        }
    }

    pub const unsafe fn new_const_unchecked(value: T) -> Self {
        Self { value }
    }

    pub fn into_inner(self) -> T {
        self.value
    }

    #[deprecated(note = "Better use NonNeg::into_inner")]
    pub fn unwrap(self) -> T {
        self.value
    }

    pub fn sqrt(self) -> NonNeg<<T as Sqrt>::Output>
    where
        T: Sqrt,
    {
        NonNeg {
            value: self.value.sqrt(),
        }
    }

    pub fn floor(self) -> NonNeg<<T as Floor>::Output>
    where
        T: Floor,
    {
        NonNeg {
            value: self.value.floor(),
        }
    }

    pub fn limited_sub<U>(self, rhs: NonNeg<U>) -> NonNeg<<T as Sub<U>>::Output>
    where
        T: Sub<U>,
        <T as Sub<U>>::Output: IsNeg,
        NonNeg<<T as Sub<U>>::Output>: Zero,
    {
        NonNeg::new(self.value - rhs.value).unwrap_or(Zero::zero())
    }

    pub fn sub_assign(&mut self, rhs: Self) -> Result<(), Self>
    where
        T: SubAssign + PartialOrd,
    {
        if self.value >= rhs.value {
            Ok(self.value -= rhs.value)
        } else {
            Err(rhs)
        }
    }
}

impl<T, U> PartialEq<NonNeg<U>> for NonNeg<T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &NonNeg<U>) -> bool {
        self.value.eq(&other.value)
    }
}

impl<T, U> PartialEq<Positive<U>> for NonNeg<T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &Positive<U>) -> bool {
        self.value.eq(&other.value)
    }
}

impl<T, U> PartialOrd<NonNeg<U>> for NonNeg<T>
where
    T: PartialOrd<U>,
{
    fn partial_cmp(&self, other: &NonNeg<U>) -> Option<core::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<T, U> PartialOrd<Positive<U>> for NonNeg<T>
where
    T: PartialOrd<U>,
{
    fn partial_cmp(&self, other: &Positive<U>) -> Option<core::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

impl<T, U> Add<NonNeg<U>> for NonNeg<T>
where
    T: Add<U>,
{
    type Output = NonNeg<<T as Add<U>>::Output>;

    fn add(self, rhs: NonNeg<U>) -> Self::Output {
        Self::Output {
            value: self.value + rhs.value,
        }
    }
}

impl<T, U> Add<Positive<U>> for NonNeg<T>
where
    T: Add<U>,
{
    type Output = Positive<<T as Add<U>>::Output>;

    fn add(self, rhs: Positive<U>) -> Self::Output {
        Self::Output {
            value: self.value + rhs.value,
        }
    }
}

impl<T, U> AddAssign<NonNeg<U>> for NonNeg<T>
where
    T: AddAssign<U>,
{
    fn add_assign(&mut self, rhs: NonNeg<U>) {
        self.value += rhs.value
    }
}

impl<T, U> Sub<NonNeg<U>> for NonNeg<T>
where
    T: Sub<U>,
{
    type Output = <T as Sub<U>>::Output;

    fn sub(self, rhs: NonNeg<U>) -> Self::Output {
        self.value - rhs.value
    }
}

impl<T, U> Sub<Positive<U>> for NonNeg<T>
where
    T: Sub<U>,
{
    type Output = <T as Sub<U>>::Output;

    fn sub(self, rhs: Positive<U>) -> Self::Output {
        self.value - rhs.value
    }
}

impl<T, U> Mul<NonNeg<U>> for NonNeg<T>
where
    T: Mul<U>,
{
    type Output = NonNeg<<T as Mul<U>>::Output>;

    fn mul(self, rhs: NonNeg<U>) -> Self::Output {
        Self::Output {
            value: self.value * rhs.value,
        }
    }
}

impl<T, U> MulAssign<NonNeg<U>> for NonNeg<T>
where
    T: MulAssign<U>,
{
    fn mul_assign(&mut self, rhs: NonNeg<U>) {
        self.value *= rhs.value;
    }
}

impl<T, U> Div<NonNeg<U>> for NonNeg<T>
where
    T: Div<U>,
{
    type Output = NonNeg<<T as Div<U>>::Output>;

    fn div(self, rhs: NonNeg<U>) -> Self::Output {
        Self::Output {
            value: self.value / rhs.value,
        }
    }
}

impl<T, U> DivAssign<NonNeg<U>> for NonNeg<T>
where
    T: DivAssign<U>,
{
    fn div_assign(&mut self, rhs: NonNeg<U>) {
        self.value /= rhs.value;
    }
}

pub trait AbsAsNonNeg
where
    Self: Sized,
{
    type Output;
    fn abs_as_non_neg(self) -> NonNeg<Self::Output>;
}

impl<T> AbsAsNonNeg for T
where
    T: Abs,
{
    type Output = <T as Abs>::Output;
    fn abs_as_non_neg(self) -> NonNeg<Self::Output> {
        NonNeg { value: self.abs() }
    }
}

impl<T> Pi for NonNeg<T>
where
    T: Pi,
{
    fn pi() -> Self {
        Self { value: T::pi() }
    }
}

impl<T: Zero> Zero for NonNeg<T> {
    fn zero() -> Self {
        Self { value: T::zero() }
    }
}

impl<T: Zero> Default for NonNeg<T> {
    fn default() -> Self {
        Zero::zero()
    }
}

impl<T> From<Positive<T>> for NonNeg<T> {
    fn from(value: Positive<T>) -> Self {
        Self { value: value.value }
    }
}

impl From<u32> for NonNeg<i64> {
    fn from(value: u32) -> Self {
        Self {
            value: value as i64,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::NonNeg;

    #[test]
    fn eq() {
        assert!(NonNeg::new(0_i32).unwrap().eq(&NonNeg::new(0_i32).unwrap()));
        assert!(!NonNeg::new(0_i32).unwrap().eq(&NonNeg::new(1_i32).unwrap()));
    }

    #[test]
    fn limited_sub() {
        assert_eq!(
            NonNeg::new(0.5)
                .unwrap()
                .limited_sub(NonNeg::new(1.0).unwrap()),
            NonNeg::new(0.0).unwrap()
        );
        assert_eq!(
            NonNeg::new(1.0)
                .unwrap()
                .limited_sub(NonNeg::new(0.5).unwrap()),
            NonNeg::new(0.5).unwrap()
        );
    }
}
