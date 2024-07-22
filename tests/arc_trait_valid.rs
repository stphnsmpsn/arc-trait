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