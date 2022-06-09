use manyvecs::Vec2d;

fn main() {
    let mut v = Vec2d::new(2.0, 3.0);
    v += 1.0;

    println!("{}", v);
}
