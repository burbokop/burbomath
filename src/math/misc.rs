use super::{Clamp, MinusOne, One, Zero};
use crate::range::{Range, RangeInclusive};
use std::{
    error::Error,
    fmt::{Debug, Display},
    ops::{Add, Div, Mul, Sub},
};

pub fn map_into_range<T, I, O>(x: T, input: I, output: O) -> T
where
    T: Clone + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
    I: Into<Range<T>>,
    O: Into<Range<T>>,
{
    let input: Range<T> = input.into();
    let output: Range<T> = output.into();
    (x - input.start.clone()) * (output.end - output.start.clone()) / (input.end - input.start)
        + output.start
}

#[derive(Debug)]
pub struct FitIntoRangeError<T> {
    pub x: T,
    pub input: Range<T>,
    pub output: Range<T>,
}

impl<T> Display for FitIntoRangeError<T> {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<T> Error for FitIntoRangeError<T> where T: Debug {}

pub fn fit_into_range<T, I, O>(x: T, input: I, output: O) -> Result<T, FitIntoRangeError<T>>
where
    T: Clone + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + PartialOrd,
    I: Into<Range<T>>,
    O: Into<Range<T>>,
{
    let input: Range<T> = input.into();
    if input.contains(&x) {
        Ok(map_into_range(x, input, output))
    } else {
        Err(FitIntoRangeError {
            x,
            input,
            output: output.into(),
        })
    }
}

pub fn map_into_range_inclusive<T, I, O>(x: T, input: I, output: O) -> T
where
    T: Clone + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
    I: Into<RangeInclusive<T>>,
    O: Into<RangeInclusive<T>>,
{
    let input: RangeInclusive<T> = input.into();
    let output: RangeInclusive<T> = output.into();
    (x - input.start.clone()) * (output.end - output.start.clone()) / (input.end - input.start)
        + output.start
}

#[derive(Debug)]
pub struct FitIntoRangeInclusiveError<T> {
    pub x: T,
    pub input: RangeInclusive<T>,
    pub output: RangeInclusive<T>,
}

impl<T> Display for FitIntoRangeInclusiveError<T> {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<T> Error for FitIntoRangeInclusiveError<T> where T: Debug {}

pub fn fit_into_range_inclusive<T, I, O>(
    x: T,
    input: I,
    output: O,
) -> Result<T, FitIntoRangeInclusiveError<T>>
where
    T: Clone + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + PartialOrd,
    I: Into<RangeInclusive<T>>,
    O: Into<RangeInclusive<T>>,
{
    let input: RangeInclusive<T> = input.into();
    if input.contains(&x) {
        Ok(map_into_range_inclusive(x, input, output))
    } else {
        Err(FitIntoRangeInclusiveError {
            x,
            input,
            output: output.into(),
        })
    }
}

pub fn clamp_into_range<T, I, O>(x: T, input: I, output: O) -> T
where
    T: Clone
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Clamp<Output = T>,
    I: Into<RangeInclusive<T>>,
    O: Into<RangeInclusive<T>>,
{
    let input: RangeInclusive<T> = input.into();
    let output: RangeInclusive<T> = output.into();
    (x.clamp(input.clone()) - input.start.clone()) * (output.end - output.start.clone())
        / (input.end - input.start)
        + output.start
}

pub fn sign<T>(v: T) -> T
where
    T: Zero + One + MinusOne + PartialOrd,
{
    if v >= T::zero() {
        T::one()
    } else {
        T::minus_one()
    }
}
