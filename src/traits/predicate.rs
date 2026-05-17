pub trait IsZero {
    fn is_zero(&self) -> bool;
}

macro_rules! impl_is_zero {
    ( $($tp: ty),+ ) => {
        $(
            impl IsZero for $tp {
                fn is_zero(&self) -> bool { *self == (0 as $tp) }
            }
        )+
    };
}

impl_is_zero! { f32, f64, i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize }

pub trait IsNeg {
    fn is_neg(&self) -> bool;
}

macro_rules! impl_is_neg {
    ( $($tp: ty),+ ) => {
        $(
            impl IsNeg for $tp {
                fn is_neg(&self) -> bool { *self < (0 as $tp) }
            }
        )+
    };
}

impl_is_neg! { f32, f64, i8, i16, i32, i64, i128, isize }

pub trait IsPositive {
    fn is_positive(&self) -> bool;
}

macro_rules! impl_is_positive {
    ( $($tp: ty),+ ) => {
        $(
            impl IsPositive for $tp {
                fn is_positive(&self) -> bool { *self > (0 as $tp) }
            }
        )+
    };
}

impl_is_positive! { f32, f64, i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize }

pub trait IsNan {
    fn is_nan(self) -> bool;
}

macro_rules! impl_is_nan {
    ( $($tp: ty),+ ) => {
        $(
            impl IsNan for $tp {
                fn is_nan(self) -> bool { <$tp>::is_nan(self) }
            }
        )+
    };
}

macro_rules! impl_is_nan_false {
    ( $($tp: ty),+ ) => {
        $(
            impl IsNan for $tp {
                fn is_nan(self) -> bool { false }
            }
        )+
    };
}

impl_is_nan! { f32, f64 }
impl_is_nan_false! { i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize }
