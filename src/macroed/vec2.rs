use std::ops::*;
use std::fmt;
use std::fmt::Formatter;

macro_rules! create_vec2 {
    ($name:ident, $type_:ty) => {
        #[derive(Copy, Clone, Debug)]
        pub struct $name {
            pub x: $type_,
            pub y: $type_,
        }

        impl $name {
            pub fn new(x: $type_, y: $type_) -> Self {
                $name {
                    x,
                    y,
                }
            }

            pub fn mag2(&self) -> $type_ {
                self.x * self.x + self.y * self.y
            }

            pub fn max<V: Into<$name>>(&self, other: V) -> Self {
                let other: $name = other.into();

                Self::new(
                    self.x.max(other.x),
                    self.y.max(other.y),
                )
            }

            pub fn min<V: Into<$name>>(&self, other: V) -> Self {
                let other: $name = other.into();

                Self::new(
                    self.x.min(other.x),
                    self.y.min(other.y),
                )
            }

            pub fn clamp<V: Into<$name>>(&self, min: V, max: V) -> Self {
                let min: $name = min.into();
                let max: $name = max.into();

                self.max(min).min(max)
            }
        }

        // Operators //

        impl Add<$name> for $name {
            type Output = Self;

            fn add(self, rhs: Self) -> Self {
                Self::new(self.x + rhs.x, self.y + rhs.y)
            }
        }

        impl Add<$type_> for $name {
            type Output = Self;

            fn add(self, rhs: $type_) -> Self {
                Self::new(self.x + rhs, self.y + rhs)
            }
        }

        impl AddAssign<$name> for $name {
            fn add_assign(&mut self, rhs: Self) {
                self.x += rhs.x;
                self.y += rhs.y;
            }
        }

        impl AddAssign<$type_> for $name {
            fn add_assign(&mut self, rhs: $type_) {
                self.x += rhs;
                self.y += rhs;
            }
        }

        // Conversion //

        // Tuple
        impl From<($type_, $type_)> for $name {
            fn from(v: ($type_, $type_)) -> Self {
                Self::new(v.0, v.1)
            }
        }

        impl From<$name> for ($type_, $type_) {
            fn from(v: $name) -> Self {
                (v.x, v.y)
            }
        }

        // Array
        impl From<[$type_; 2]> for $name {
            fn from(v: [$type_; 2]) -> Self {
                Self::new(v[0], v[1])
            }
        }

        impl From<$name> for [$type_; 2] {
            fn from(v: $name) -> Self {
                [v.x, v.y]
            }
        }

        // Vec
        impl TryFrom<Vec<$type_>> for $name {
            type Error = String;

            fn try_from(v: Vec<$type_>) -> Result<$name, Self::Error> {
                if v.len() == 2 {
                    Ok(Self::new(v[0], v[1]))
                } else {
                    Err(format!("Given Vec must have size of 2, but instead has size of '{}'", v.len()))
                }
            }
        }

        impl From<$name> for Vec<$type_> {
            fn from(v: $name) -> Self {
                vec![v.x, v.y]
            }
        }
    };
}

macro_rules! add_vec2_feature {
    ($name:ident, $type_:ty, floating) => {
        impl $name {
            pub fn mag(&self) -> $type_ {
                self.mag2().sqrt()
            }

            pub fn norm(&self) -> Self {
                // Figure out way to find 1.09
                let r = 1.0 / self.mag();
                Self::new(self.x * r, self.y * r)
            }

            pub fn floor(&self) -> Self {
                Self::new(self.x.floor(), self.y.floor())
            }

            pub fn ceil(&self) -> Self {
                Self::new(self.x.ceil(), self.y.ceil())
            }
        }

        // Floating point numbers are usually signed
        add_vec2_feature!($name, $type_, signed);
    };
    ($name:ident, $type_:ty, signed) => {
        impl $name {
            pub fn perp(&self) -> Self {
                Self::new(-self.y, self.x)
            }
        }
    }
}

create_vec2!(Vec2f, f32);
add_vec2_feature!(Vec2f, f32, floating);

impl fmt::Display for Vec2f {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Vec2f({}, {})", self.x, self.y)
    }
}

create_vec2!(Vec2d, f64);
add_vec2_feature!(Vec2d, f64, floating);

impl fmt::Display for Vec2d {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Vec2d({}, {})", self.x, self.y)
    }
}
