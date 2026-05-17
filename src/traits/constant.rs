pub trait UnsignedContstant<const N: usize> {
    fn unsigned_contstant() -> Self;
}

/// For shorter
pub fn uconst<const N: usize, T: UnsignedContstant<N>>() -> T {
    UnsignedContstant::unsigned_contstant()
}

macro_rules! impl_unsigned_contstant {
    ( $($tp: ty),+ ) => {
        $(
            impl<const N: usize> UnsignedContstant<N> for $tp {
                fn unsigned_contstant() -> Self { N as Self }
            }
        )+
    };
}

impl_unsigned_contstant! { f32, f64, i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize }

pub trait Zero {
    fn zero() -> Self;
}

impl<T: UnsignedContstant<0>> Zero for T {
    fn zero() -> Self {
        UnsignedContstant::unsigned_contstant()
    }
}

pub trait One {
    fn one() -> Self;
}

impl<T: UnsignedContstant<1>> One for T {
    fn one() -> Self {
        UnsignedContstant::unsigned_contstant()
    }
}

pub trait Two {
    fn two() -> Self;
}

impl<T: UnsignedContstant<2>> Two for T {
    fn two() -> Self {
        UnsignedContstant::unsigned_contstant()
    }
}

pub trait MinusOne {
    fn minus_one() -> Self;
}

macro_rules! impl_minus_one {
    ( $($tp: ty),+ ) => {
        $(
            impl MinusOne for $tp {
                fn minus_one() -> Self { (-1) as $tp }
            }
        )+
    };
}

impl_minus_one! { f32, f64, i8, i16, i32, i64, i128, isize }

pub trait Pi {
    fn pi() -> Self;
}

macro_rules! impl_pi {
    ( $($tp: ident),+ ) => {
        $(
            impl Pi for $tp {
                fn pi() -> Self { core::$tp::consts::PI }
            }
        )+
    };
}

impl_pi! { f32, f64 }

pub trait TopLimit {
    fn top_limit() -> Self;
}

macro_rules! impl_top_limit {
    ( $($tp: ty),+ ) => {
        $(
            impl TopLimit for $tp {
                fn top_limit() -> Self { <$tp>::MAX }
            }
        )+
    };
}

impl_top_limit! { f32, f64, i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize }

pub trait BottomLimit {
    fn bottom_limit() -> Self;
}

macro_rules! impl_bottom_limit {
    ( $($tp: ty),+ ) => {
        $(
            impl BottomLimit for $tp {
                fn bottom_limit() -> Self { <$tp>::MIN }
            }
        )+
    };
}

impl_bottom_limit! { f32, f64, i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize }
