#![deny(warnings)]
#![no_std]

#[cfg(all(feature = "std", feature = "libm"))]
compile_error!("Features 'std' and 'libm' are mutually exclusive.");

#[cfg(feature = "std")]
extern crate std;

mod math;
mod traits;
pub use math::*;
pub use traits::*;

pub mod camera;
pub mod physics;
pub mod range;
pub mod time;

pub use burbomath_macro::*;
