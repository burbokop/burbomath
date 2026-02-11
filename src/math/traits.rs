use super::Angle;
use crate::range::RangeInclusive;

pub trait Sq {
    type Output;
    fn sq(self) -> Self::Output;
}

impl Sq for f32 {
    type Output = f32;
    fn sq(self) -> Self::Output {
        self * self
    }
}

impl Sq for f64 {
    type Output = f64;
    fn sq(self) -> Self::Output {
        self * self
    }
}

pub trait SignedSq {
    fn ssq(self) -> Self;
}

 #[cfg(any(feature = "std", feature = "libm"))]
 impl SignedSq for f32 {
    fn ssq(self) -> Self {
        if self >= 0. {
            self * self
        } else {
            -self * self
        }
    }
}

 #[cfg(any(feature = "std", feature = "libm"))]
 impl SignedSq for f64 {
    fn ssq(self) -> Self {
        if self >= 0. {
            self * self
        } else {
            -self * self
        }
    }
}

pub trait Sqrt {
    type Output;
    fn sqrt(self) -> Self::Output;
}

#[cfg(feature = "std")]
impl Sqrt for f32 {
    type Output = f32;

    fn sqrt(self) -> Self::Output {
        f32::sqrt(self)
    }
}

#[cfg(feature = "libm")]
impl Sqrt for f32 {
    type Output = f32;

    fn sqrt(self) -> Self::Output {
        libm::sqrtf(self)
    }
}

#[cfg(feature = "std")]
impl Sqrt for f64 {
    type Output = f64;

    fn sqrt(self) -> Self::Output {
        f64::sqrt(self)
    }
}

#[cfg(feature = "libm")]
impl Sqrt for f64 {
    type Output = f64;

    fn sqrt(self) -> Self::Output {
        libm::sqrt(self)
    }
}

pub trait SignedSqrt {
    fn ssqrt(self) -> Self;
}

 #[cfg(any(feature = "std", feature = "libm"))]
 impl SignedSqrt for f32 {
    fn ssqrt(self) -> Self {
        if self >= 0. {
            f32::sqrt(self)
        } else {
            -f32::sqrt(-self)
        }
    }
}

 #[cfg(any(feature = "std", feature = "libm"))]
 impl SignedSqrt for f64 {
    fn ssqrt(self) -> Self {
        if self >= 0. {
            f64::sqrt(self)
        } else {
            -f64::sqrt(-self)
        }
    }
}

pub trait Cos {
    type Output;
    fn cos(self) -> Self::Output;
}

#[cfg(feature = "std")]
impl Cos for f32 {
    type Output = f32;

    fn cos(self) -> Self::Output {
        f32::cos(self)
    }
}

#[cfg(feature = "libm")]
impl Cos for f32 {
    type Output = f32;

    fn cos(self) -> Self::Output {
        libm::cosf(self)
    }
}

#[cfg(feature = "std")]
impl Cos for f64 {
    type Output = f64;

    fn cos(self) -> Self::Output {
        f64::cos(self)
    }
}

#[cfg(feature = "libm")]
impl Cos for f64 {
    type Output = f64;

    fn cos(self) -> Self::Output {
        libm::cos(self)
    }
}

pub trait Sin {
    type Output;
    fn sin(self) -> Self::Output;
}

#[cfg(feature = "std")]
impl Sin for f32 {
    type Output = f32;

    fn sin(self) -> Self::Output {
        f32::sin(self)
    }
}

#[cfg(feature = "libm")]
impl Sin for f32 {
    type Output = f32;

    fn sin(self) -> Self::Output {
        libm::sinf(self)
    }
}

#[cfg(feature = "std")]
impl Sin for f64 {
    type Output = f64;

    fn sin(self) -> Self::Output {
        f64::sin(self)
    }
}

#[cfg(feature = "libm")]
impl Sin for f64 {
    type Output = f64;

    fn sin(self) -> Self::Output {
        libm::sin(self)
    }
}

pub trait Atan2<Rhs = Self> {
    type Output;
    fn atan2(self, rhs: Rhs) -> Angle<Self::Output>;
}

#[cfg(feature = "std")]
impl Atan2 for f32 {
    type Output = f32;

    fn atan2(self, rhs: Self) -> Angle<Self::Output> {
        Angle::from_radians(f32::atan2(self, rhs))
    }
}

#[cfg(feature = "libm")]
impl Atan2 for f32 {
    type Output = f32;

    fn atan2(self, rhs: Self) -> Angle<Self::Output> {
        Angle::from_radians(libm::atan2f(self, rhs))
    }
}

#[cfg(feature = "std")]
impl Atan2 for f64 {
    type Output = f64;

    fn atan2(self, rhs: Self) -> Angle<Self::Output> {
        Angle::from_radians(f64::atan2(self, rhs))
    }
}

#[cfg(feature = "libm")]
impl Atan2 for f64 {
    type Output = f64;

    fn atan2(self, rhs: Self) -> Angle<Self::Output> {
        Angle::from_radians(libm::atan2(self, rhs))
    }
}

pub trait RemEuclid<Rhs = Self> {
    type Output;
    fn rem_euclid(self, rhs: Rhs) -> Self::Output;
}

#[cfg(feature = "std")]
impl RemEuclid for f32 {
    type Output = f32;

    fn rem_euclid(self, rhs: Self) -> Self::Output {
        f32::rem_euclid(self, rhs)
    }
}

#[cfg(feature = "libm")]
impl RemEuclid for f32 {
    type Output = f32;

    fn rem_euclid(self, rhs: Self) -> Self::Output {
        let result = libm::fmodf(self, rhs);
        if result >= 0. {
            result
        } else {
            result + rhs
        }
    }
}

#[cfg(feature = "std")]
impl RemEuclid for f64 {
    type Output = f64;

    fn rem_euclid(self, rhs: Self) -> Self::Output {
        f64::rem_euclid(self, rhs)
    }
}

#[cfg(feature = "libm")]
impl RemEuclid for f64 {
    type Output = f64;

    fn rem_euclid(self, rhs: Self) -> Self::Output {
        let result = libm::fmod(self, rhs);
        if result >= 0. {
            result
        } else {
            result + rhs
        }
    }
}

pub trait Abs {
    type Output;
    fn abs(self) -> Self::Output;
}

impl Abs for f32 {
    type Output = f32;

    fn abs(self) -> Self::Output {
        f32::abs(self)
    }
}

impl Abs for f64 {
    type Output = f64;

    fn abs(self) -> Self::Output {
        f64::abs(self)
    }
}

pub trait Floor {
    type Output;
    fn floor(self) -> Self::Output;
}

#[cfg(feature = "std")]
impl Floor for f32 {
    type Output = f32;

    fn floor(self) -> Self::Output {
        f32::floor(self)
    }
}

#[cfg(feature = "libm")]
impl Floor for f32 {
    type Output = f32;

    fn floor(self) -> Self::Output {
        libm::floorf(self)
    }
}

#[cfg(feature = "std")]
impl Floor for f64 {
    type Output = f64;

    fn floor(self) -> Self::Output {
        f64::floor(self)
    }
}

#[cfg(feature = "libm")]
impl Floor for f64 {
    type Output = f64;

    fn floor(self) -> Self::Output {
        libm::floor(self)
    }
}

pub trait Round {
    type Output;
    fn round(self) -> Self::Output;
}

#[cfg(feature = "std")]
impl Round for f32 {
    type Output = f32;

    fn round(self) -> Self::Output {
        f32::round(self)
    }
}

#[cfg(feature = "libm")]
impl Round for f32 {
    type Output = f32;

    fn round(self) -> Self::Output {
        libm::roundf(self)
    }
}

#[cfg(feature = "std")]
impl Round for f64 {
    type Output = f64;

    fn round(self) -> Self::Output {
        f64::round(self)
    }
}

#[cfg(feature = "libm")]
impl Round for f64 {
    type Output = f64;

    fn round(self) -> Self::Output {
        libm::round(self)
    }
}

pub trait Clamp: Sized {
    type Output;
    fn clamp<R: Into<RangeInclusive<Self>>>(self, range: R) -> Self::Output;
}

impl Clamp for f32 {
    type Output = f32;

    fn clamp<R: Into<RangeInclusive<Self>>>(self, range: R) -> Self::Output {
        let range: RangeInclusive<Self> = range.into();
        f32::clamp(self, range.start, range.end)
    }
}

impl Clamp for f64 {
    type Output = f64;

    fn clamp<R: Into<RangeInclusive<Self>>>(self, range: R) -> Self::Output {
        let range: RangeInclusive<Self> = range.into();
        f64::clamp(self, range.start, range.end)
    }
}

pub trait Zero {
    fn zero() -> Self;
}

impl Zero for f32 {
    fn zero() -> Self {
        0.
    }
}

impl Zero for f64 {
    fn zero() -> Self {
        0.
    }
}

impl Zero for u32 {
    fn zero() -> Self {
        0
    }
}

impl Zero for i64 {
    fn zero() -> Self {
        0
    }
}

pub trait One {
    fn one() -> Self;
}

impl One for f32 {
    fn one() -> Self {
        1.
    }
}

impl One for f64 {
    fn one() -> Self {
        1.
    }
}

impl One for u32 {
    fn one() -> Self {
        1
    }
}

pub trait MinusOne {
    fn minus_one() -> Self;
}

impl MinusOne for f32 {
    fn minus_one() -> Self {
        -1.
    }
}

impl MinusOne for f64 {
    fn minus_one() -> Self {
        -1.
    }
}

pub trait Two {
    fn two() -> Self;
}

impl Two for f32 {
    fn two() -> Self {
        2.
    }
}

impl Two for f64 {
    fn two() -> Self {
        2.
    }
}

pub trait Pi {
    fn pi() -> Self;
}

impl Pi for f32 {
    fn pi() -> Self {
        core::f32::consts::PI
    }
}

impl Pi for f64 {
    fn pi() -> Self {
        core::f64::consts::PI
    }
}

pub trait IsNeg {
    fn is_neg(&self) -> bool;
}

impl IsNeg for f32 {
    fn is_neg(&self) -> bool {
        *self < 0.
    }
}

impl IsNeg for f64 {
    fn is_neg(&self) -> bool {
        *self < 0.
    }
}

impl IsNeg for i64 {
    fn is_neg(&self) -> bool {
        *self < 0
    }
}

pub trait IsPositive {
    fn is_positive(&self) -> bool;
}

impl IsPositive for f32 {
    fn is_positive(&self) -> bool {
        *self > 0.
    }
}

impl IsPositive for f64 {
    fn is_positive(&self) -> bool {
        *self > 0.
    }
}

impl IsPositive for i64 {
    fn is_positive(&self) -> bool {
        *self > 0
    }
}

pub trait RadToDeg {
    type Output;
    fn rad_to_deg(self) -> Self::Output;
}

impl RadToDeg for f32 {
    type Output = f32;

    fn rad_to_deg(self) -> Self::Output {
        self / core::f32::consts::PI * 180.
    }
}

impl RadToDeg for f64 {
    type Output = f64;

    fn rad_to_deg(self) -> Self::Output {
        self / core::f64::consts::PI * 180.
    }
}

pub trait DegToRad {
    type Output;
    fn deg_to_rad(self) -> Self::Output;
}

impl DegToRad for f32 {
    type Output = f32;

    fn deg_to_rad(self) -> Self::Output {
        self / 180. * core::f32::consts::PI
    }
}

impl DegToRad for f64 {
    type Output = f64;

    fn deg_to_rad(self) -> Self::Output {
        self / 180. * core::f64::consts::PI
    }
}
