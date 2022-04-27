use manyvecs::Vec2;

/// Example of editing X and Y after the creation of
/// a Vec2
fn main() {
    let mut v = Vec2::new(3, 5);

    // Get a mutable reference of x
    let x = v.x_mut();

    *x += 7;

    assert_eq!(*v.x(), 10);

    println!("{}", v);
    // "Vec2(10, 5)"
}
