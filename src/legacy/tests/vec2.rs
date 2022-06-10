use crate::legacy::Vec2;

/////////////////////
// Core Functions //
///////////////////

#[test]
fn new() {
    Vec2::new(10, 5);
}

#[test]
fn x_and_y() {
    let v = Vec2::new(3.0, 6.8);

    assert_eq!(*v.x(), 3.0);
    assert_eq!(*v.y(), 6.8);
}

#[test]
fn mut_x_and_y() {
    let mut v = Vec2::new(6, 8);

    *v.y_mut() /= 2;

    assert_eq!(*v.y(), 4);
}

#[test]
fn types() {
    Vec2::<u8>::new(2, 3);
    Vec2::<i8>::new(-2, 3);
    Vec2::<u16>::new(2, 3);
    Vec2::<i16>::new(-2, 3);
    Vec2::<u32>::new(2, 3);
    Vec2::<i32>::new(-2, 3);
    Vec2::<u64>::new(2, 3);
    Vec2::<i64>::new(-2, 3);
    Vec2::<u128>::new(2, 3);
    Vec2::<i128>::new(-2, 3);

    Vec2::<usize>::new(2, 4);
    Vec2::<isize>::new(-2, 4);

    Vec2::<f32>::new(5.32, -8.9);
    Vec2::<f64>::new(3.14159, 8.0);
}

#[test]
fn mag2() {
    assert_eq!(Vec2::new(2, 3).mag2(), 13);
}

// Make sure struct is thread-safe
#[test]
fn sync_send() {
    fn assert_ss<T: Sync + Send>() {}
    assert_ss::<Vec2<f32>>();
    // Probably could test other types as well...
}

#[test]
fn default() {
    assert_eq!(Vec2::<u16>::default(), Vec2::<u16>::new(0, 0));
}

////////////////////
// Floating Vecs //
//////////////////

#[test]
fn mag() {
    assert_eq!(Vec2::new(6.0, 8.0).mag(), 10.0);
}

#[test]
fn norm() {
    // Difficult to test with exact float, this works for now
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

//////////////////
// Max and Min //
////////////////

#[test]
fn max() {
    assert_eq!(Vec2::new(4, 7).max(5), Vec2::new(5, 7));
    assert_eq!(Vec2::new(4, 7).max_v(&Vec2::new(6, 5)), Vec2::new(6, 7));
}

#[test]
fn min() {
    assert_eq!(Vec2::new(4, 7).min(5), Vec2::new(4, 5));
    assert_eq!(Vec2::new(4, 7).min_v(&Vec2::new(6, 5)), Vec2::new(4, 5));
}

#[test]
fn clamp() {
    assert_eq!(Vec2::new(3, 6).clamp(4, 8), Vec2::new(4, 6));
}

/////////////////////
// Signed Numbers //
///////////////////

#[test]
fn perp() {
    let v = Vec2::<isize>::new(4, 6);
    assert_eq!(v.perp(), Vec2::<isize>::new(-6, 4));
}

////////////////
// Operators //
//////////////

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
    let v = Vec2::<isize>::new(5, -8);
    assert_eq!(-v, Vec2::<isize>::new(-5, 8));
}

//////////////////
// Equivelance //
////////////////

#[test]
fn eq() {
    let v1 = Vec2::new(2, 2);
    let v2 = Vec2::new(2, 2);

    assert!(v1 == v2);
    assert_eq!(v1, v2);
}

/////////////////
// Conversion //
///////////////

#[test]
fn conv_tuple() {
    let t1 = (3_u8, 5_u8);
    let v = Vec2::from(t1);

    assert_eq!(t1.0, *v.x());
    assert_eq!(t1.1, *v.y());

    let t2 = <(u8, u8)>::from(v);
    assert_eq!(t1, t2);
}

#[test]
fn conv_array() {
    let a1: [u8; 2] = [8, 5];
    let v = Vec2::from(a1);

    assert_eq!(a1[0], *v.x());
    assert_eq!(a1[1], *v.y());

    let a2 = <[u8; 2]>::from(v);
    assert_eq!(a1, a2);
}

#[test]
fn conv_std_vec() {
    let std_vec1: Vec<u8> = vec![4, 19];
    let v = Vec2::try_from(std_vec1.clone()).unwrap();

    assert_eq!(std_vec1[0], *v.x());
    assert_eq!(std_vec1[1], *v.y());

    let std_vec2 = Vec::from(v);
    assert_eq!(std_vec1, std_vec2);
}

#[test]
#[should_panic]
fn conv_std_vec_err() {
    let std_vec: Vec<u8> = vec![8];
    Vec2::try_from(std_vec).unwrap();
}

//////////////////////
// Display + Debug //
////////////////////

#[test]
fn display() {
    let v = Vec2::new(4.1, 8.8);

    assert_eq!(format!("{}", v), "Vec2(4.1, 8.8)");
}

#[test]
fn debug() {
    let v = Vec2::new(7, 3);

    // Output may change (because derived), but it
    // should at least still work
    format!("{:?}", v);
    format!("{:#?}", v);
}
