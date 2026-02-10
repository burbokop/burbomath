use crate::math::One;
use core::ops::{Add, Mul, Sub};

pub fn lerp<V, T>(a: V, b: V, t: T) -> V
where
    V: Add<<<V as Sub>::Output as Mul<T>>::Output, Output = V>,
    V: Sub,
    <V as Sub>::Output: Mul<T>,
    V: Clone,
{
    a.clone() + (b - a) * t
}

/// Second version of lerp.
/// Both versions work the same with integers and real numbers but result may differ when working with other types.
/// The main differance it that here `t` is copied instead of `a`. One or another version may be prefered depending on cloning ability of `V` and `T` types.
pub fn lerp2<V, T>(a: V, b: V, t: T) -> V
where
    V: Mul<T, Output = V>,
    V: Add<Output = V>,
    T: Clone + One + Sub<Output = T>,
{
    a * (T::one() - t.clone()) + b * t
}

pub struct LerpIntegrator<V, T> {
    t: T,
    prev: Option<V>,
}

impl<V, T> LerpIntegrator<V, T> {
    pub fn new(t: T) -> Self {
        Self { t, prev: None }
    }

    pub fn proceed(&mut self, v: V) -> &V
    where
        V: Mul<T, Output = V>,
        V: Add<Output = V>,
        V: Clone,
        T: Clone + One + Sub<Output = T>,
    {
        let prev = self.prev.get_or_insert(v.clone());
        *prev = lerp2(prev.clone(), v, self.t.clone());
        prev
    }
}
