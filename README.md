# ManyVecs

A library providing Vector structs inspired by GLSL. These are useful for working with coordinates, geometry, game development, and more. Some features include:

- In-depth unit testing
- Support of component operators (addition, subtraction, etc.)
- Useful methods, like magnitude and normalization

**This is not related to the standard library [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html) type.**

## Try it out!

```rust
use manyvecs::legacy::Vec2;

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

## Features

Versions 0.1.0 and 0.2.0 use a generic-based struct for its vectors. Version 0.3.0 onwards is introducing a macro-based struct. This is a breaking change, which is why you can configure these changes with features.

> **Note:** In 1.0.0, generic-based vectors will be removed entirely.

|Feature|Description|
| - | - |
|`legacy`|The legacy feature enables generic-based vectors. It is toggled on by default for backward compatability.|
|`macroed`|The macroed feature enables macro-based vectors. It is toggled **off** by default.|

To enable macro-based vectors, include the following in your `Cargo.toml`.

```toml
[dependencies]
manyvecs = { version = "~0.Y.Z", features = ["macroed"] }
```

To enable just the macro vector, include the following instead.

```toml
[dependencies]
manyvecs = { version = "~0.Y.Z", default-features = false, features = ["macroed"] }
```

### Exporting Content

The macroed and legacy features are designed to work side-by-side. Each have their own public module. If just legacy is enabled and not macroed, the contents of the legacy module will be exported to the root of the crate. Take a look at the following code in `lib.rs`.

```rust,ignore
#[cfg(feature = "legacy")]
#[cfg(not(feature = "macroed"))]
pub use self::legacy::*;
```

The same applies for using just macroed and not legacy. This is important if you know you will not be using the other feature, as you can just `use manyvecs::Vec2` instead of `manyvecs::macroed::Vec2`.

## Stability Notice

This library is currently unstable. It is recommended that you only allow patches in your dependency on ManyVecs.

```toml
# Cargo.toml
[dependencies]
manyvecs = "~0.Y.Z"
```

See more about the tilde requirement [here](https://doc.rust-lang.org/stable/cargo/reference/specifying-dependencies.html#tilde-requirements).

## License

ManyVecs is developed and distributed under the [MIT license](https://opensource.org/licenses/MIT).

## Contributing

Contributions are welcome! This is my (BD103's) first Rust crate ever published, so there is definitely room to improve.

Unless explicitly stated, any contribution intentionally submitted for inclusion to this project will be licensed as above, without any additional terms or conditions.
