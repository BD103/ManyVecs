#![doc = include_str!("../README.md")]

#[cfg(feature = "legacy")]
mod legacy;

#[cfg(feature = "legacy")]
pub use self::legacy::*;

#[cfg(feature = "macroed")]
mod macroed;

#[cfg(feature = "macroed")]
pub use self::macroed::*;

#[cfg(test)]
mod tests;
