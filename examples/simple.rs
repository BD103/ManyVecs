use manyvecs::legacy::Vec2;

/// A very simple example of using a Vec2
fn main() {
    // Vec2 stores two numbers, X and Y
    let v = Vec2::<f32>::new(2.0, -0.5);

    println!("{}, {}", v.x(), v.y());
    // "2, -0.5"

    // Operators work on vectors!
    // They are applied component-wise.
    println!("{}", v + 1.0);
    // "Vec2(3, 0.5)"

    // You can even create vectors from tuples
    let _ = Vec2::from((2.0, 3.0));
}
