mod vec2;

#[cfg(test)]
mod tests;

pub use self::vec2::*;

/// The default Vec2 implementation.
pub type Vec2 = Vec2f32;

/// The default floating-point Vec2 implementation.
pub type Vec2f = Vec2f32;

/// The default double-precision floating-point Vec2 implementation.
pub type Vec2d = Vec2f64;

/// The default unsigned integer Vec2 implementation.
pub type Vec2u = Vec2usize;

/// The default signed integer Vec2 implementation.
pub type Vec2i = Vec2isize;
