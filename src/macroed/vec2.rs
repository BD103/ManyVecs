use std::fmt;
use std::fmt::Formatter;
use std::ops::*;

/// A macro for creating [`Vec2`] structs.
macro_rules! create_vec2 {
    // Match a name and a type, optionally taking documentation and metadata.
    ($(#[$meta:meta])* $name:ident, $type_:ty) => {
        $(#[$meta])*
        #[derive(Copy, Clone, Debug)]
        pub struct $name {
            pub x: $type_,
            pub y: $type_,
        }

        impl $name {
            /// Creates a new Vec2.
            pub const fn new(x: $type_, y: $type_) -> Self {
                $name {
                    x,
                    y,
                }
            }

            /// Returns `x` squared plus `y` squared. (`x^2 + y^2` where `^` represents an exponent.)
            pub fn mag2(&self) -> $type_ {
                self.x * self.x + self.y * self.y
            }

            /// Returns the larger of each number from a given Vec2.
            ///
            /// # Example
            ///
            /// ```rust,ignore
            /// # use manyvecs::macroed::Vec2;
            /// #
            /// let a = Vec2::new(3.0, 5.0);
            /// let b = Vec2::new(3.5, 4.0);
            ///
            /// assert_eq!(a.max(b), Vec2::new(3.5, 5.0));
            /// ```
            pub fn max<V: Into<Self>>(&self, other: V) -> Self {
                let other: Self = other.into();

                Self::new(
                    self.x.max(other.x),
                    self.y.max(other.y),
                )
            }

            /// Returns the smaller of each number from a given Vec2.
            ///
            /// # Example
            ///
            /// ```rust,ignore
            /// # use manyvecs::macroed::Vec2;
            /// #
            /// let a = Vec2::new(3.0, 5.0);
            /// let b = Vec2::new(3.5, 4.0);
            ///
            /// assert_eq!(a.min(b), Vec2::new(3.0, 4.0));
            /// ```
            pub fn min<V: Into<Self>>(&self, other: V) -> Self {
                let other: Self = other.into();

                Self::new(
                    self.x.min(other.x),
                    self.y.min(other.y),
                )
            }

            /// Constrains the values of a vector to be between the min and the max.
            ///
            /// # Example
            ///
            /// ```rust,ignore
            /// # use manyvecs::macroed::Vec2u8;
            /// #
            /// let min = Vec2u8::new(5, 10);
            /// let max = Vec2u8::new(10, 15);
            ///
            /// for x in 0..15 {
            ///     for y in 0..15 {
            ///         let v = Vec2u8::new(x, y).clamp(min, max);
            ///
            ///         assert!(5 <= v.x && v.x <= 10 && 10 <= v.y && v.y <= 15);
            ///     }
            /// }
            /// ```
            pub fn clamp<V: Into<Self>>(&self, min: V, max: V) -> Self {
                let min: Self = min.into();
                let max: Self = max.into();

                self.max(min).min(max)
            }
        }

        // Operators //

        // All vectors should at least support these operators
        apply_operator!($name, $type_, Add, add, +, AddAssign, add_assign, +=);
        apply_operator!($name, $type_, Sub, sub, -, SubAssign, sub_assign, -=);
        apply_operator!($name, $type_, Mul, mul, *, MulAssign, mul_assign, *=);
        apply_operator!($name, $type_, Div, div, /, DivAssign, div_assign, /=);
        apply_operator!($name, $type_, Rem, rem, %, RemAssign, rem_assign, %=);

        // And all vectors should be able to use `==`
        impl std::cmp::PartialEq<Self> for $name {
            fn eq(&self, other: &Self) -> bool {
                self.x == other.x && self.y == other.y
            }
        }

        /*
        // I wish I could do this, but for now people will have to do it themselves.

        impl<V: impl Into<Self>> std::cmp::PartialEq<V> for $name {
            fn eq(&self, other: &V) -> bool {
                let other: Self = other.into();
                self.x == other.x && self.y == other.y
            }
        }
         */

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

        // Other Compatability //

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                write!(f, concat!(stringify!($name), "({}, {})"), self.x, self.y)
            }
        }
    };
}

macro_rules! add_vec2_feature {
    ($name:ident, $type_:ty, "floating") => {
        impl $name {
            /// Returns the magnitude of a vector.
            ///
            /// This is equivalent to the [Pythagorean Theorem](https://en.wikipedia.org/wiki/Pythagorean_theorem),
            /// so it returns `sqrt(x^2 + y^2)` where `^` signifies an exponent.
            pub fn mag(&self) -> $type_ {
                self.mag2().sqrt()
            }

            /// Normalizes a vector so that its magnitude is 0.
            pub fn norm(&self) -> Self {
                // Figure out way to find 1.09
                let r = 1.0 / self.mag();
                Self::new(self.x * r, self.y * r)
            }

            /// Returns a vector where the `x` and `y` values are rounded down.
            pub fn floor(&self) -> Self {
                Self::new(self.x.floor(), self.y.floor())
            }

            /// Returns a vector where the `x` and `y` values are rounded up.
            pub fn ceil(&self) -> Self {
                Self::new(self.x.ceil(), self.y.ceil())
            }
        }

        // Floating point numbers are usually signed
        add_vec2_feature!($name, $type_, "signed");
    };
    ($name:ident, $type_:ty, "signed") => {
        impl $name {
            /// Returns the perpendicular slope of a line.
            pub fn perp(&self) -> Self {
                Self::new(-self.y, self.x)
            }
        }

        impl Neg for $name {
            type Output = Self;

            fn neg(self) -> Self {
                Self::new(-self.x, -self.y)
            }
        }
    };
    ($name:ident, $type_:ty, "bitwise") => {
        apply_operator!($name, $type_, BitAnd, bitand, &, BitAndAssign, bitand_assign, &=);
        apply_operator!($name, $type_, BitOr, bitor, |, BitOrAssign, bitor_assign, |=);
        apply_operator!($name, $type_, BitXor, bitxor, ^, BitXorAssign, bitxor_assign, ^=);
        apply_operator!($name, $type_, Shl, shl, <<, ShlAssign, shl_assign, <<=);
        apply_operator!($name, $type_, Shr, shr, >>, ShrAssign, shr_assign, >>=);
    };
}

macro_rules! apply_operator {
    ($name:ident, $type_:ty, $trait_name:ident, $trait_fn:ident, $op:tt, $atrait_name:ident, $atrait_fn:ident, $aop:tt) => {
        impl $trait_name<Self> for $name {
            type Output = Self;

            fn $trait_fn(self, rhs: Self) -> Self {
                Self::new(self.x $op rhs.x, self.y $op rhs.y)
            }
        }

        impl $trait_name<$type_> for $name {
            type Output = Self;

            fn $trait_fn(self, rhs: $type_) -> Self {
                Self::new(self.x $op rhs, self.y $op rhs)
            }
        }

        impl $atrait_name<Self> for $name {
            fn $atrait_fn(&mut self, rhs: Self) {
                self.x $aop rhs.x;
                self.y $aop rhs.y;
            }
        }

        impl $atrait_name<$type_> for $name {
            fn $atrait_fn(&mut self, rhs: $type_) {
                self.x $aop rhs;
                self.y $aop rhs;
            }
        }
    };
}

// Floats
create_vec2!(
    /// A Vec2 containing [`f32`]s.
    ///
    /// # Example
    ///
    /// ```
    /// # use manyvecs::macroed::Vec2f32;
    /// let _ = Vec2f32::new(-2.0, 3.0);
    /// ```
    Vec2f32,
    f32
);
add_vec2_feature!(Vec2f32, f32, "floating");

create_vec2!(
    /// A Vec2 containing [`f64`]s.
    ///
    /// # Example
    ///
    /// ```
    /// # use manyvecs::macroed::Vec2f64;
    /// let _ = Vec2f64::new(-2.0, 3.0);
    /// ```
    Vec2f64,
    f64
);
add_vec2_feature!(Vec2f64, f64, "floating");

// Unsigned ints
create_vec2!(
    /// A Vec2 containing [`u8`]s.
    ///
    /// # Example
    ///
    /// ```
    /// # use manyvecs::macroed::Vec2u8;
    /// let _ = Vec2u8::new(2, 3);
    /// ```
    Vec2u8,
    u8
);
add_vec2_feature!(Vec2u8, u8, "bitwise");

create_vec2!(
    /// A Vec2 containing [`u16`]s.
    ///
    /// # Example
    ///
    /// ```
    /// # use manyvecs::macroed::Vec2u16;
    /// let _ = Vec2u16::new(2, 3);
    /// ```
    Vec2u16,
    u16
);
add_vec2_feature!(Vec2u16, u16, "bitwise");

create_vec2!(
    /// A Vec2 containing [u32`]s.
    ///
    /// # Example
    ///
    /// ```
    /// # use manyvecs::macroed::Vec2u32;
    /// let _ = Vec2u32::new(2, 3);
    /// ```
    Vec2u32,
    u32
);
add_vec2_feature!(Vec2u32, u32, "bitwise");

create_vec2!(
    /// A Vec2 containing [`u64`]s.
    ///
    /// # Example
    ///
    /// ```
    /// # use manyvecs::macroed::Vec2u64;
    /// let _ = Vec2u64::new(2, 3);
    /// ```
    Vec2u64,
    u64
);
add_vec2_feature!(Vec2u64, u64, "bitwise");

create_vec2!(
    /// A Vec2 containing [`u128`]s.
    ///
    /// # Example
    ///
    /// ```
    /// # use manyvecs::macroed::Vec2u128;
    /// let _ = Vec2u128::new(2, 3);
    /// ```
    Vec2u128,
    u128
);
add_vec2_feature!(Vec2u128, u128, "bitwise");

create_vec2!(
    /// A Vec2 containing [`usize`]s.
    ///
    /// # Example
    ///
    /// ```
    /// # use manyvecs::macroed::Vec2usize;
    /// let _ = Vec2usize::new(2, 3);
    /// ```
    Vec2usize,
    usize
);
add_vec2_feature!(Vec2usize, usize, "bitwise");

// Signed ints
create_vec2!(
    /// A Vec2 containing [`i8`]s.
    ///
    /// # Example
    ///
    /// ```
    /// # use manyvecs::macroed::Vec2i8;
    /// let _ = Vec2i8::new(-2, 3);
    /// ```
    Vec2i8,
    i8
);
add_vec2_feature!(Vec2i8, i8, "bitwise");
add_vec2_feature!(Vec2i8, i8, "signed");

create_vec2!(
    /// A Vec2 containing [`i16`]s.
    ///
    /// # Example
    ///
    /// ```
    /// # use manyvecs::macroed::Vec2i16;
    /// let _ = Vec2i16::new(-2, 3);
    /// ```
    Vec2i16,
    i16
);
add_vec2_feature!(Vec2i16, i16, "bitwise");
add_vec2_feature!(Vec2i16, i16, "signed");

create_vec2!(
    /// A Vec2 containing [`i32`]s.
    ///
    /// # Example
    ///
    /// ```
    /// # use manyvecs::macroed::Vec2i32;
    /// let _ = Vec2i32::new(-2, 3);
    /// ```
    Vec2i32,
    i32
);
add_vec2_feature!(Vec2i32, i32, "bitwise");
add_vec2_feature!(Vec2i32, i32, "signed");

create_vec2!(
    /// A Vec2 containing [`i64`]s.
    ///
    /// # Example
    ///
    /// ```
    /// # use manyvecs::macroed::Vec2i64;
    /// let _ = Vec2i64::new(-2, 3);
    /// ```
    Vec2i64,
    i64
);
add_vec2_feature!(Vec2i64, i64, "bitwise");
add_vec2_feature!(Vec2i64, i64, "signed");

create_vec2!(
    /// A Vec2 containing [`i128`]s.
    ///
    /// # Example
    ///
    /// ```
    /// # use manyvecs::macroed::Vec2i128;
    /// let _ = Vec2i128::new(-2, 3);
    /// ```
    Vec2i128,
    i128
);
add_vec2_feature!(Vec2i128, i128, "bitwise");
add_vec2_feature!(Vec2i128, i128, "signed");

create_vec2!(
    /// A Vec2 containing [`isize`]s.
    ///
    /// # Example
    ///
    /// ```
    /// # use manyvecs::macroed::Vec2isize;
    /// let _ = Vec2isize::new(-2, 3);
    /// ```
    Vec2isize,
    isize
);
add_vec2_feature!(Vec2isize, isize, "bitwise");
add_vec2_feature!(Vec2isize, isize, "signed");
