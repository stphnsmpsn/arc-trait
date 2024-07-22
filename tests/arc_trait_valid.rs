use arc_trait::arc_trait;
use async_trait::async_trait;
use std::{sync::Arc, time::Duration};

#[arc_trait]
#[async_trait]
trait ExampleAsyncTrait {
    async fn get_value_async(&self) -> i32;
}

#[arc_trait]
trait ExampleTrait {
    fn get_value(&self) -> i32;
}

#[derive(Default)]
struct Example {
    pub value: i32,
}

#[async_trait]
impl ExampleAsyncTrait for Example {
    async fn get_value_async(&self) -> i32 {
        tokio::time::sleep(Duration::from_millis(10)).await;
        self.value
    }
}

impl ExampleTrait for Example {
    fn get_value(&self) -> i32 {
        self.value
    }
}

#[tokio::main]
async fn main() {
    let example = Arc::new(Example::default());

    // Sync methods
    assert_eq!(example.get_value(), 0);

    // Async methods
    assert_eq!(example.get_value_async().await, 0);
}
