# Arc Trait

`arc_trait` is a procedural macro that simplifies the process of implementing traits for `std::sync::Arc<T>`. By
applying this macro to a trait, you enable `Arc<T>` to implement it whenever the inner type `T` implements the trait,
saving you from writing repetitive boilerplate code.

## Features

- Automatically implements a trait for `std::sync::Arc<T>` types.
- Simplifies code and reduces boilerplate.
- Easy to use with simple attribute-based syntax.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
arc_trait = "0.1.0"
```

And this to your crate root:

```rust
extern crate arc_trait;
use arc_trait::arc_trait;
```

## Usage

Apply the arc_trait attribute to your traits as shown in the example:

```rust
extern crate arc_trait;
use arc_trait::arc_trait;
use std::sync::Arc;

#[arc_trait]
trait ExampleTrait {
    fn example_method(&self, value: i32) -> i32;
}

struct Example;

impl ExampleTrait for Example {
    fn example_method(&self, value: i32) -> i32 {
        value + 1
    }
}

fn main() {
    let example = Arc::new(Example);
    assert_eq!(example.example_method(1), 2);
}
```

## How It Works

arc_trait works by:

- Parsing the given trait to identify all methods within it.
- Generating an implementation of the trait for `std::sync::Arc<T>` that forwards method calls to the inner value
  wrapped by the `Arc`.

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests
on [GitHub](https://github.com/stphnsmpsn/arc-trait).

## License

This project is licensed under the MIT License. See the LICENSE file for details.