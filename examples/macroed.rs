#[cfg(feature = "macroed")]
fn main() {
    use manyvecs::macroed::Vec2f;

    let mut v = Vec2f::new(2.0, 3.0);

    v += 1.0;
    v *= 2.0;
    v -= 2.0;
    v /= 4.0;

    println!("{}", v);
    // "Vec2f32(1, 1.5)"
}

#[cfg(not(feature = "macroed"))]
fn main() {
    println!("Please run example with --features macroed");
}
