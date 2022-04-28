# ManyVecs

A library providing Vector structs inspired by GLSL. These are useful for working with coordinates, geometry, game development, and more. Some features include:

- In-depth unit testing
- Support of component operators (addition, subtraction, etc.)
- Useful methods, like magnitude and normalization

**This is not related to the standard library [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html) type.**

## Try it out!

```rust
use manyvecs::Vec2;

// Vec2 stores two numbers, X and Y
let v = Vec2::<f32>::new(2.0, -0.5);

println!("{}, {}", v.x(), v.y());
// "2.0, -0.5"

// Operators work on vectors!
// They are applied componenet-wise.
println!("{}", v + 1.0);
// "Vec2(3.0, 0.5)"

// You can even create vecs from tuples
Vec2::from((2.0, 3.0));
```

These are just a few examples, you can find the complete documentation at [docs.rs](https://docs.rs/manyvecs).

## License

ManyVecs is developed and distributed under the [MIT license](https://opensource.org/licenses/MIT).

## Contributing

Contributions are welcome! This is my (BD103's) first Rust crate ever published, so there is definitely room to improve.

Unless explicitly stated, any contribution intentionally submitted for inclusion to this project will be licensed as above, without any additional terms or conditions.
