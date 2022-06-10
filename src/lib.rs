#![doc = include_str!("../README.md")]

#[cfg(feature = "legacy")]
pub mod legacy;

#[cfg(feature = "legacy")]
#[cfg(not(feature = "macroed"))]
pub use self::legacy::*;

#[cfg(feature = "macroed")]
pub mod macroed;

#[cfg(feature = "macroed")]
#[cfg(not(feature = "legacy"))]
pub use self::macroed::*;
