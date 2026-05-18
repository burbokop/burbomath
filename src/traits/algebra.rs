use crate::NonNeg;
use crate::Positive;
use crate::range::RangeInclusive;

pub trait Sq {
    type Output;
    fn sq(self) -> Self::Output;
}

macro_rules! impl_sq {
    ( $($tp: ty),+ ) => {
        $(
            impl Sq for $tp {
                type Output = NonNeg<Self>;
                fn sq(self) -> Self::Output { unsafe { NonNeg::new_const_unchecked(self * self) } }
            }
        )+
    };
}

macro_rules! impl_unsigned_sq {
    ( $($tp: ty),+ ) => {
        $(
            impl Sq for $tp {
                type Output = Self;
                fn sq(self) -> Self::Output { self * self }
            }
        )+
    };
}

impl_sq! { f32, f64, i8, i16, i32, i64, i128, isize }
impl_unsigned_sq! { u8, u16, u32, u64, u128, usize }

impl<T: Sq<Output = NonNeg<T>>> Sq for NonNeg<T> {
    type Output = Self;

    fn sq(self) -> Self::Output {
        self.into_inner().sq()
    }
}

impl<T: Sq<Output = NonNeg<T>>> Sq for Positive<T> {
    type Output = Self;

    fn sq(self) -> Self::Output {
        unsafe { Positive::new_const_unchecked(self.into_inner().sq().into_inner()) }
    }
}

pub trait Cube {
    type Output;
    fn cube(self) -> Self::Output;
}

macro_rules! impl_cube {
    ( $($tp: ty),+ ) => {
        $(
            impl Cube for $tp {
                type Output = Self;
                fn cube(self) -> Self { self * self * self }
            }
        )+
    };
}

impl_cube! { f32, f64, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize }

impl<T: Cube> Cube for NonNeg<T> {
    type Output = NonNeg<<T as Cube>::Output>;

    fn cube(self) -> Self::Output {
        unsafe { NonNeg::new_const_unchecked(self.into_inner().cube()) }
    }
}

impl<T: Cube> Cube for Positive<T> {
    type Output = Positive<<T as Cube>::Output>;

    fn cube(self) -> Self::Output {
        unsafe { Positive::new_const_unchecked(self.into_inner().cube()) }
    }
}

pub trait SignedSq {
    type Output;
    fn ssq(self) -> Self::Output;
}

macro_rules! impl_signed_sq {
    ( $($tp: ty),+ ) => {
        $(
            impl SignedSq for $tp {
                type Output = Self;
                fn ssq(self) -> Self::Output {
                    if self >= 0 as $tp {
                        self * self
                    } else {
                        -self * self
                    }
                }
            }
        )+
    };
}

impl_signed_sq! { f32, f64, i8, i16, i32, i64, i128, isize }

pub trait Sqrt {
    type Output;
    fn sqrt(self) -> Self::Output;
}

#[cfg(feature = "std")]
impl Sqrt for NonNeg<f32> {
    type Output = Self;

    fn sqrt(self) -> Self::Output {
        unsafe { NonNeg::new_const_unchecked(f32::sqrt(self.into_inner())) }
    }
}

#[cfg(feature = "libm")]
impl Sqrt for NonNeg<f32> {
    type Output = Self;

    fn sqrt(self) -> Self::Output {
        unsafe { NonNeg::new_const_unchecked(libm::sqrtf(self.into_inner())) }
    }
}

#[cfg(feature = "std")]
impl Sqrt for NonNeg<f64> {
    type Output = Self;

    fn sqrt(self) -> Self::Output {
        unsafe { NonNeg::new_const_unchecked(f64::sqrt(self.into_inner())) }
    }
}

#[cfg(feature = "libm")]
impl Sqrt for NonNeg<f64> {
    type Output = Self;

    fn sqrt(self) -> Self::Output {
        unsafe { NonNeg::new_const_unchecked(libm::sqrt(self.into_inner())) }
    }
}

#[cfg(feature = "std")]
impl Sqrt for Positive<f32> {
    type Output = Self;

    fn sqrt(self) -> Self::Output {
        unsafe { Positive::new_const_unchecked(f32::sqrt(self.into_inner())) }
    }
}

#[cfg(feature = "libm")]
impl Sqrt for Positive<f32> {
    type Output = Self;

    fn sqrt(self) -> Self::Output {
        unsafe { Positive::new_const_unchecked(libm::sqrtf(self.into_inner())) }
    }
}

#[cfg(feature = "std")]
impl Sqrt for Positive<f64> {
    type Output = Self;

    fn sqrt(self) -> Self::Output {
        unsafe { Positive::new_const_unchecked(f64::sqrt(self.into_inner())) }
    }
}

#[cfg(feature = "libm")]
impl Sqrt for Positive<f64> {
    type Output = Self;

    fn sqrt(self) -> Self::Output {
        unsafe { Positive::new_const_unchecked(libm::sqrt(self.into_inner())) }
    }
}

pub trait SignedSqrt {
    type Output;
    fn ssqrt(self) -> Self::Output;
}

#[cfg(feature = "std")]
impl SignedSqrt for f32 {
    type Output = f32;
    fn ssqrt(self) -> Self::Output {
        if self >= 0. {
            f32::sqrt(self)
        } else {
            -f32::sqrt(-self)
        }
    }
}

#[cfg(feature = "libm")]
impl SignedSqrt for f32 {
    type Output = f32;
    fn ssqrt(self) -> Self::Output {
        if self >= 0. {
            libm::sqrtf(self)
        } else {
            -libm::sqrtf(-self)
        }
    }
}

#[cfg(feature = "std")]
impl SignedSqrt for f64 {
    type Output = f64;
    fn ssqrt(self) -> Self::Output {
        if self >= 0. {
            f64::sqrt(self)
        } else {
            -f64::sqrt(-self)
        }
    }
}

#[cfg(feature = "libm")]
impl SignedSqrt for f64 {
    type Output = f64;
    fn ssqrt(self) -> Self::Output {
        if self >= 0. {
            libm::sqrt(self)
        } else {
            -libm::sqrt(-self)
        }
    }
}

pub trait Pow<R = Self> {
    type Output;
    fn pow(self, rhs: R) -> Self::Output;
}

#[cfg(feature = "std")]
impl Pow<f32> for f32 {
    type Output = f32;

    fn pow(self, rhs: f32) -> Self::Output {
        f32::powf(self, rhs)
    }
}

#[cfg(feature = "libm")]
impl Pow<f32> for f32 {
    type Output = f32;

    fn pow(self, rhs: f32) -> Self::Output {
        libm::powf(self, rhs)
    }
}

#[cfg(feature = "std")]
impl Pow<f64> for f64 {
    type Output = f64;

    fn pow(self, rhs: f64) -> Self::Output {
        f64::powf(self, rhs)
    }
}

#[cfg(feature = "libm")]
impl Pow<f64> for f64 {
    type Output = f64;

    fn pow(self, rhs: f64) -> Self::Output {
        libm::pow(self, rhs)
    }
}

#[cfg(feature = "std")]
impl Pow<f32> for Positive<f32> {
    type Output = Self;

    fn pow(self, rhs: f32) -> Self::Output {
        unsafe { Positive::new_const_unchecked(f32::powf(self.into_inner(), rhs)) }
    }
}

#[cfg(feature = "libm")]
impl Pow<f32> for Positive<f32> {
    type Output = Self;

    fn pow(self, rhs: f32) -> Self::Output {
        unsafe { Positive::new_const_unchecked(libm::powf(self.into_inner(), rhs)) }
    }
}

#[cfg(feature = "std")]
impl Pow<f64> for Positive<f64> {
    type Output = Self;

    fn pow(self, rhs: f64) -> Self::Output {
        unsafe { Positive::new_const_unchecked(f64::powf(self.into_inner(), rhs)) }
    }
}

#[cfg(feature = "libm")]
impl Pow<f64> for Positive<f64> {
    type Output = Self;

    fn pow(self, rhs: f64) -> Self::Output {
        unsafe { Positive::new_const_unchecked(libm::pow(self.into_inner(), rhs)) }
    }
}

pub trait Log2 {
    type Output;
    fn log2(self) -> Self::Output;
}

#[cfg(feature = "std")]
impl Log2 for Positive<f32> {
    type Output = f32;

    fn log2(self) -> Self::Output {
        f32::log2(self.into_inner())
    }
}

#[cfg(feature = "libm")]
impl Log2 for Positive<f32> {
    type Output = f32;

    fn log2(self) -> Self::Output {
        libm::log2f(self.into_inner())
    }
}

#[cfg(feature = "std")]
impl Log2 for Positive<f64> {
    type Output = f64;

    fn log2(self) -> Self::Output {
        f64::log2(self.into_inner())
    }
}

#[cfg(feature = "libm")]
impl Log2 for Positive<f64> {
    type Output = f64;

    fn log2(self) -> Self::Output {
        libm::log2(self.into_inner())
    }
}

pub trait RemEuclid<Rhs = Self> {
    type Output;
    fn rem_euclid(self, rhs: Rhs) -> Self::Output;
}

#[cfg(feature = "std")]
impl RemEuclid for f32 {
    type Output = NonNeg<f32>;

    fn rem_euclid(self, rhs: Self) -> Self::Output {
        unsafe { NonNeg::new_const_unchecked(f32::rem_euclid(self, rhs)) }
    }
}

#[cfg(feature = "libm")]
impl RemEuclid for f32 {
    type Output = NonNeg<f32>;

    fn rem_euclid(self, rhs: Self) -> Self::Output {
        let result = libm::fmodf(self, rhs);
        unsafe { NonNeg::new_const_unchecked(if result >= 0. { result } else { result + rhs }) }
    }
}

#[cfg(feature = "std")]
impl RemEuclid for f64 {
    type Output = NonNeg<f64>;

    fn rem_euclid(self, rhs: Self) -> Self::Output {
        unsafe { NonNeg::new_const_unchecked(f64::rem_euclid(self, rhs)) }
    }
}

#[cfg(feature = "libm")]
impl RemEuclid for f64 {
    type Output = NonNeg<f64>;

    fn rem_euclid(self, rhs: Self) -> Self::Output {
        let result = libm::fmod(self, rhs);
        unsafe { NonNeg::new_const_unchecked(if result >= 0. { result } else { result + rhs }) }
    }
}

pub trait Abs {
    type Output;
    fn abs(self) -> Self::Output;
}

impl Abs for f32 {
    type Output = NonNeg<f32>;

    fn abs(self) -> Self::Output {
        unsafe { NonNeg::new_const_unchecked(f32::abs(self)) }
    }
}

impl Abs for f64 {
    type Output = NonNeg<f64>;

    fn abs(self) -> Self::Output {
        unsafe { NonNeg::new_const_unchecked(f64::abs(self)) }
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
