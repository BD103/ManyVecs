use crate::macroed::*;

// General //

#[test]
fn new() {
    Vec2::new(10.0, 5.0);
}

#[test]
fn x_and_y() {
    let v = Vec2::new(3.0, 6.8);

    assert_eq!(v.x, 3.0);
    assert_eq!(v.y, 6.8);
}

#[test]
fn mut_x_and_y() {
    let mut v = Vec2::new(6.0, 8.0);

    v.x += 1.0;
    v.y /= 2.0;

    assert_eq!(v.x, 7.0);
    assert_eq!(v.y, 4.0);
}

#[test]
fn types() {
    // Type aliases
    Vec2::new(-2.0, 3.0);
    Vec2f::new(-2.0, 3.0);
    Vec2d::new(-2.0, 3.0);
    Vec2u::new(2, 3);
    Vec2i::new(-2, 3);

    // Unsigned
    Vec2u8::new(2, 3);
    Vec2u16::new(2, 3);
    Vec2u32::new(2, 3);
    Vec2u64::new(2, 3);
    Vec2u128::new(2, 3);
    Vec2usize::new(2, 3);

    // Signed
    Vec2i8::new(-2, 3);
    Vec2i16::new(-2, 3);
    Vec2i32::new(-2, 3);
    Vec2i64::new(-2, 3);
    Vec2i128::new(-2, 3);
    Vec2isize::new(-2, 3);
}

#[test]
fn mag2() {
    assert_eq!(Vec2::new(2.0, 3.0).mag2(), 13.0);
}

#[test]
fn max() {
    assert_eq!(Vec2::new(4.0, 7.0).max([5.0; 2]), Vec2::new(5.0, 7.0));
    assert_eq!(
        Vec2::new(4.0, 7.0).max(Vec2::new(6.0, 5.0)),
        Vec2::new(6.0, 7.0)
    );
}

#[test]
fn min() {
    assert_eq!(Vec2::new(4.0, 7.0).min([5.0; 2]), Vec2::new(4.0, 5.0));
    assert_eq!(
        Vec2::new(4.0, 7.0).min(Vec2::new(6.0, 5.0)),
        Vec2::new(4.0, 5.0)
    );
}

#[test]
fn clamp() {
    assert_eq!(
        Vec2::new(3.0, 6.0).clamp([4.0; 2], [8.0; 2]),
        Vec2::new(4.0, 6.0)
    );
}

// Make sure struct is thread-safe
#[test]
fn sync_send() {
    fn assert_ss<T: Sync + Send>() {}
    assert_ss::<Vec2>();
    // Probably could test other types as well...
}

// Floats //

#[test]
fn mag() {
    assert_eq!(Vec2::new(6.0, 8.0).mag(), 10.0);
}

#[test]
fn norm() {
    // Difficult to test with exact float, this works for now by comparing slopes
    assert_eq!(Vec2::new(2.0, 4.0).norm(), Vec2::new(4.0, 8.0).norm());
}

#[test]
fn floor() {
    assert_eq!(Vec2::new(3.14159, 2.0).floor(), Vec2::new(3.0, 2.0));
}

#[test]
fn ceil() {
    assert_eq!(Vec2::new(3.14159, 6.0).ceil(), Vec2::new(4.0, 6.0));
}

// Signed Integers //

#[test]
fn perp() {
    let v = Vec2i::new(4, 6);
    assert_eq!(v.perp(), Vec2i::new(-6, 4));
}

// Operators //

#[test]
fn add() {
    let mut v1 = Vec2::new(2.0, 3.0);
    let v2 = Vec2::new(1.0, 1.0);
    let out = Vec2::new(3.0, 4.0);

    assert_eq!(v1 + v2, out);

    v1 += v2;

    assert_eq!(v1, out);
}

#[test]
fn sub() {
    let mut v1 = Vec2::new(2.0, 3.0);
    let v2 = Vec2::new(1.0, 1.0);
    let out = Vec2::new(1.0, 2.0);

    assert_eq!(v1 - v2, out);

    v1 -= v2;

    assert_eq!(v1, out);
}

#[test]
fn mul() {
    let mut v1 = Vec2::new(2.0, 3.0);
    let v2 = Vec2::new(2.0, 2.0);
    let out = Vec2::new(4.0, 6.0);

    assert_eq!(v1 * v2, out);

    v1 *= v2;

    assert_eq!(v1, out);
}

#[test]
fn div() {
    let mut v1 = Vec2::new(2.0, 3.0);
    let v2 = Vec2::new(2.0, 2.0);
    let out = Vec2::new(1.0, 1.5);

    assert_eq!(v1 / v2, out);

    v1 /= v2;

    assert_eq!(v1, out);
}

#[test]
fn rem() {
    let mut v1 = Vec2::new(2.0, 3.0);
    let v2 = Vec2::new(2.0, 2.0);
    let out = Vec2::new(0.0, 1.0);

    assert_eq!(v1 % v2, out);

    v1 %= v2;

    assert_eq!(v1, out);
}

#[test]
fn neg() {
    let v = Vec2i::new(5, -8);
    assert_eq!(-v, Vec2i::new(-5, 8));
}

// Equality //

#[test]
fn eq() {
    let v1 = Vec2u::new(2, 2);
    let v2 = Vec2u::new(2, 2);

    assert_eq!(v1, v2);
}

// Conversion //

#[test]
fn conv_tuple() {
    let t1 = (3.0, 5.0);
    let v = Vec2::from(t1);

    assert_eq!(t1.0, v.x);
    assert_eq!(t1.1, v.y);

    let t2 = <(f32, f32)>::from(v);
    assert_eq!(t1, t2);
}

#[test]
fn conv_array() {
    let a1: [f32; 2] = [8.0, 5.0];
    let v = Vec2::from(a1);

    assert_eq!(a1[0], v.x);
    assert_eq!(a1[1], v.y);

    let a2 = <[f32; 2]>::from(v);
    assert_eq!(a1, a2);
}

#[test]
fn conv_std_vec() {
    let std_vec1: Vec<f32> = vec![4.0, 19.0];
    let v = Vec2::try_from(std_vec1.clone()).unwrap();

    assert_eq!(std_vec1[0], v.x);
    assert_eq!(std_vec1[1], v.y);

    let std_vec2 = Vec::from(v);
    assert_eq!(std_vec1, std_vec2);
}

#[test]
#[should_panic]
fn conv_std_vec_err() {
    let std_vec: Vec<f32> = vec![8.0];
    Vec2::try_from(std_vec).unwrap();
}

// Other //

#[test]
fn display() {
    let v = Vec2f32::new(4.1, 8.8);

    assert_eq!(format!("{}", v), "Vec2f32(4.1, 8.8)");
}

#[test]
fn debug() {
    let v = Vec2f32::new(7.4, 3.9);

    // Output may change (because derived), but it
    // should at least still work
    format!("{:?}", v);
    format!("{:#?}", v);
}
