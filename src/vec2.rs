use num_traits::{real::Real, sign::Signed, Num};

use std::cmp::PartialEq;
use std::fmt;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

/////////////////
// Main Struct //
/////////////////

/// Provides a vector type similar to GLSL's Vec2.
///
/// # Useful Methods
///
/// - [Vec2::x] and [Vec2::y]
/// - [Vec2::mag]
/// - [Vec2::max], [Vec2::min], and [Vec2::clamp]
///
/// # Example
///
/// ```
/// # use manyvecs::Vec2;
/// let v = Vec2::new(5.0, 3.5);
/// ```
#[derive(Copy, Clone, Debug)]
pub struct Vec2<T>
where
    // There should be a better way than requiring Copy
    T: Num + Copy,
{
    x: T,
    y: T,
}

/////////////////////
// Generic Number //
///////////////////

impl<T> Vec2<T>
where
    T: Num + Copy,
{
    /// Creates a new [Vec2].
    pub fn new(x: T, y: T) -> Vec2<T> {
        Vec2 { x, y }
    }

    /// Returns the X value of the vector.
    pub fn x(&self) -> &T {
        &self.x
    }

    pub fn x_mut(&mut self) -> &mut T {
        &mut self.x
    }

    /// Returns the Y value of the vector.
    pub fn y(&self) -> &T {
        &self.y
    }

    pub fn y_mut(&mut self) -> &mut T {
        &mut self.y
    }

    /// Returns the X squared plus Y squared. (`x^2 + y^2`.)
    pub fn mag2(&self) -> T {
        self.x * self.x + self.y * self.y
    }
}

//////////////////////
// Decimal Numbers //
////////////////////

impl<T> Vec2<T>
where
    T: Num + Copy + Real,
{
    /// Returns the magnitude of X and Y.
    ///
    /// This uses the Pythagorean Theorem,
    /// so it returns `sqrt(x^2 + y^2)`.
    pub fn mag(&self) -> T {
        self.mag2().sqrt()
    }

    /// Normalizes the values of the vector.
    ///
    /// Returns a new [Vec2] as if the [magnitude](Vec2.mag)
    /// were equal to 1. This constrains X and Y to be
    /// between -1 and 1.
    pub fn norm(&self) -> Self
    where
        Self: Sized,
        T: Div<T>,
    {
        let r = T::one() / self.mag();
        Self::new(self.x * r, self.y * r)
    }

    /// Floors the values of X and Y. (Round down.)
    pub fn floor(&self) -> Self
    where
        Self: Sized,
    {
        Self::new(self.x.floor(), self.y.floor())
    }

    /// Ceilings the values of X and Y. (Round up.)
    pub fn ceil(&self) -> Self
    where
        Self: Sized,
    {
        Self::new(self.x.ceil(), self.y.ceil())
    }
}

//////////////////////
// Ordered Numbers //
////////////////////

impl<T> Vec2<T>
where
    T: Num + Copy + PartialOrd,
{
    /// Finds the greater value of X and Y compared to
    /// another number.
    pub fn max(&self, v: T) -> Self
    where
        Self: Sized,
    {
        let x = if self.x > v { self.x } else { v };
        let y = if self.y > v { self.y } else { v };

        Self::new(x, y)
    }

    /// Finds the greater value of X and Y compared to
    /// another [Vec2].
    #[cold]
    pub fn max_v(&self, v: &Self) -> Self
    where
        Self: Sized,
    {
        let x = if self.x > v.x { self.x } else { v.x };
        let y = if self.y > v.y { self.y } else { v.y };

        Self::new(x, y)
    }

    /// Finds the lesser value of X and Y compared to
    /// another number.
    pub fn min(&self, v: T) -> Self
    where
        Self: Sized,
    {
        let x = if self.x < v { self.x } else { v };
        let y = if self.y < v { self.y } else { v };

        Self::new(x, y)
    }

    /// Finds the lesser value of X and Y compared to
    /// another [Vec2].
    #[cold]
    pub fn min_v(&self, v: &Self) -> Self
    where
        Self: Sized,
    {
        let x = if self.x < v.x { self.x } else { v.x };
        let y = if self.y < v.y { self.y } else { v.y };

        Self::new(x, y)
    }

    /// Constrains X and Y to be between min and max.
    ///
    /// If the value is between the min and max, it
    /// uses that number. If it is below, it instead
    /// uses the minimum value. If it is above, it
    /// uses the maximum value.
    pub fn clamp(&self, min: T, max: T) -> Self
    where
        Self: Sized,
    {
        self.max(min).min(max)
    }

    // Used to have a clamp_v method here
    // But it was buggy
    // Oh well :P
}

/////////////////////
// Signed Numbers //
///////////////////

impl<T> Vec2<T>
where
    T: Num + Copy + Signed,
{
    /// Finds the perpendicular slope of X and Y.
    pub fn perp(&self) -> Self
    where
        Self: Sized,
    {
        Self::new(-self.y, self.x)
    }
}

///////////////////////////////
// Operator Implementations //
/////////////////////////////

// Addition
impl<T> Add for Vec2<T>
where
    T: Num + Copy,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T> Add<T> for Vec2<T>
where
    T: Num + Copy,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self {
        Self::new(self.x + rhs, self.y + rhs)
    }
}

impl<T> AddAssign for Vec2<T>
where
    T: Num + Copy + AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> AddAssign<T> for Vec2<T>
where
    T: Num + Copy + AddAssign,
{
    fn add_assign(&mut self, rhs: T) {
        self.x += rhs;
        self.y += rhs;
    }
}

// Subtraction
impl<T> Sub for Vec2<T>
where
    T: Num + Copy,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T> Sub<T> for Vec2<T>
where
    T: Num + Copy,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self {
        Self::new(self.x - rhs, self.y - rhs)
    }
}

impl<T> SubAssign for Vec2<T>
where
    T: Num + Copy + SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T> SubAssign<T> for Vec2<T>
where
    T: Num + Copy + SubAssign,
{
    fn sub_assign(&mut self, rhs: T) {
        self.x -= rhs;
        self.y -= rhs;
    }
}

// Multiplications
impl<T> Mul for Vec2<T>
where
    T: Num + Copy,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl<T> Mul<T> for Vec2<T>
where
    T: Num + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl<T> MulAssign for Vec2<T>
where
    T: Num + Copy + MulAssign,
{
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl<T> MulAssign<T> for Vec2<T>
where
    T: Num + Copy + MulAssign,
{
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

// Division
impl<T> Div for Vec2<T>
where
    T: Num + Copy,
{
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self::new(self.x / rhs.x, self.y / rhs.y)
    }
}

impl<T> Div<T> for Vec2<T>
where
    T: Num + Copy,
{
    type Output = Self;

    fn div(self, rhs: T) -> Self {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

impl<T> DivAssign for Vec2<T>
where
    T: Num + Copy + DivAssign,
{
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl<T> DivAssign<T> for Vec2<T>
where
    T: Num + Copy + DivAssign,
{
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

// Remainder / Modulus
impl<T> Rem for Vec2<T>
where
    T: Num + Copy,
{
    type Output = Self;

    fn rem(self, rhs: Self) -> Self {
        Self::new(self.x % rhs.x, self.y % rhs.y)
    }
}

impl<T> Rem<T> for Vec2<T>
where
    T: Num + Copy,
{
    type Output = Self;

    fn rem(self, rhs: T) -> Self {
        Self::new(self.x % rhs, self.y % rhs)
    }
}

impl<T> RemAssign for Vec2<T>
where
    T: Num + Copy + RemAssign,
{
    fn rem_assign(&mut self, rhs: Self) {
        self.x %= rhs.x;
        self.y %= rhs.y;
    }
}

impl<T> RemAssign<T> for Vec2<T>
where
    T: Num + Copy + RemAssign,
{
    fn rem_assign(&mut self, rhs: T) {
        self.x %= rhs;
        self.y %= rhs;
    }
}

// Negating the Value
impl<T> Neg for Vec2<T>
where
    T: Num + Copy + Signed,
{
    type Output = Self;

    fn neg(self) -> Self {
        // Can we just re-use the object instead of creating a new one?
        Self::new(-self.x, -self.y)
    }
}

//////////////////
// Equivalence //
////////////////

impl<T> PartialEq for Vec2<T>
where
    T: Num + Copy + PartialEq,
{
    fn eq(&self, rhs: &Self) -> bool {
        self.x == rhs.x && self.y == rhs.y
    }
}

/////////////////
// Conversion //
///////////////

// Tuple
impl<T> From<(T, T)> for Vec2<T>
where
    T: Num + Copy,
{
    fn from(v: (T, T)) -> Vec2<T> {
        Vec2::new(v.0, v.1)
    }
}

impl<T> From<Vec2<T>> for (T, T)
where
    T: Num + Copy,
{
    fn from(v: Vec2<T>) -> (T, T) {
        (v.x, v.y)
    }
}

// Array
impl<T> From<[T; 2]> for Vec2<T>
where
    T: Num + Copy,
{
    fn from(v: [T; 2]) -> Vec2<T> {
        Vec2::new(v[0], v[1])
    }
}

impl<T> From<Vec2<T>> for [T; 2]
where
    T: Num + Copy,
{
    fn from(v: Vec2<T>) -> [T; 2] {
        [v.x, v.y]
    }
}

// Vec
impl<T> TryFrom<Vec<T>> for Vec2<T>
where
    T: Num + Copy,
{
    type Error = String;

    fn try_from(v: Vec<T>) -> Result<Vec2<T>, Self::Error> {
        let len = v.len();

        if len == 2 {
            Ok(Vec2::new(v[0], v[1]))
        } else {
            Err(format!("Given vec must have size 2, has size '{}'", len))
        }
    }
}

impl<T> From<Vec2<T>> for Vec<T>
where
    T: Num + Copy,
{
    fn from(v: Vec2<T>) -> Vec<T> {
        vec![v.x, v.y]
    }
}

//////////////
// Display //
////////////

impl<T> fmt::Display for Vec2<T>
where
    T: Num + Copy + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Vec2({}, {})", self.x, self.y)
    }
}

//////////////
// Default //
////////////

impl<T> Default for Vec2<T>
where
    T: Num + Copy,
{
    fn default() -> Self {
        Vec2::new(T::zero(), T::zero())
    }
}
