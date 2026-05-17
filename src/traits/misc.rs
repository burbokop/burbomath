pub trait FromUSize {
    fn from_usize(v: usize) -> Self;
}

macro_rules! impl_from_usize {
    ( $($tp: ty),+ ) => {
        $(
            impl FromUSize for $tp {
                fn from_usize(v: usize) -> Self { v as $tp }
            }
        )+
    };
}

impl_from_usize! { f32, f64, i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize }
