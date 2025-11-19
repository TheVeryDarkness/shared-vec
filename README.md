# `shared-vec`

`shared-vec` is a Rust crate that provides immutable reference-counted vector and string types allowing sharing data (or even a part of it) without lifetime-bound references.

## Features

- Immutable reference-counted vector type `Vec<C, T>`, where `C` is a counter type implementing the `Counter` trait.
- Immutable reference-counted string type `String<C>`, where `C` is a counter type implementing the `Counter` trait.
- Common type aliases `RcVec<T>`, `ArcVec<T>`, `RcString`, and `ArcString` for ease of use.
- Safe and efficient memory management with customizable reference counting strategies.

## Usage

Add `shared-vec` to your `Cargo.toml`:

```toml
[dependencies]
shared-vec = "0.1"
```

Then, you can use the provided types in your Rust code:

```rust
use shared_vec::{RcVec, ArcString};

// Create
let vec: RcVec<i32> = RcVec::from_boxed_slice(Box::new([1, 2, 3]));
let string: ArcString = ArcString::from_boxed_str("Hello, world!".to_owned().into_boxed_str());

// Clone
let vec_clone = vec.clone();
let string_clone = string.clone();

// Access data
println!("{:?}", &vec[..]);
println!("{}", &string[..]);

// Borrow slices
let slice = vec.idx(1..3);
println!("{:?}", slice);
assert_eq!(slice.as_slice(), &[2, 3]);
let str_slice = &string[7..12];
println!("{}", str_slice);
assert_eq!(str_slice, "world");
```
