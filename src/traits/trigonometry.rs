use crate::Angle;

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

pub trait Tan {
    type Output;
    fn tan(self) -> Self::Output;
}

#[cfg(feature = "std")]
impl Tan for f32 {
    type Output = f32;

    fn tan(self) -> Self::Output {
        f32::tan(self)
    }
}

#[cfg(feature = "libm")]
impl Tan for f32 {
    type Output = f32;

    fn tan(self) -> Self::Output {
        libm::tanf(self)
    }
}

#[cfg(feature = "std")]
impl Tan for f64 {
    type Output = f64;

    fn tan(self) -> Self::Output {
        f64::tan(self)
    }
}

#[cfg(feature = "libm")]
impl Tan for f64 {
    type Output = f64;

    fn tan(self) -> Self::Output {
        libm::tan(self)
    }
}

pub trait Atan {
    type Output;
    fn atan(self) -> Self::Output;
}

#[cfg(feature = "std")]
impl Atan for f32 {
    type Output = f32;

    fn atan(self) -> Self::Output {
        f32::atan(self)
    }
}

#[cfg(feature = "libm")]
impl Atan for f32 {
    type Output = f32;

    fn atan(self) -> Self::Output {
        libm::atanf(self)
    }
}

#[cfg(feature = "std")]
impl Atan for f64 {
    type Output = f64;

    fn atan(self) -> Self::Output {
        f64::atan(self)
    }
}

#[cfg(feature = "libm")]
impl Atan for f64 {
    type Output = f64;

    fn atan(self) -> Self::Output {
        libm::atan(self)
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
