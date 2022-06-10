mod vec2;

#[cfg(test)]
mod tests;

pub use self::vec2::Vec2;

#[cfg(not(feature = "macroed"))]
pub type Vec2f = Vec2<f32>;

#[cfg(not(feature = "macroed"))]
pub type Vec2d = Vec2<f64>;

#[cfg(not(feature = "macroed"))]
pub type Vec2u = Vec2<usize>;

#[cfg(not(feature = "macroed"))]
pub type Vec2i = Vec2<isize>;
