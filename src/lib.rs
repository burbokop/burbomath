#![deny(warnings)]
#![no_std]

#[cfg(all(feature = "std", feature = "libm"))]
compile_error!("Features 'std' and 'libm' are mutually exclusive.");

#[cfg(feature = "std")]
extern crate std;

pub mod camera;
pub mod math;
pub mod physics;
pub mod range;
