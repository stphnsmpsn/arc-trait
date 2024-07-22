extern crate arc_trait;
extern crate async_trait;
use arc_trait::arc_trait;
use async_trait::async_trait;
use std::sync::Arc;
use std::time::Duration;

#[arc_trait]
#[async_trait]
trait ExampleAsyncTrait {
    async fn complex_signature<'a, U>(&self, entities: &'a [U], value: i32) -> i32
    where
        U: Into<String> + Send + Sync;
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
    async fn complex_signature<'a, U>(&self, _entities: &'a [U], value: i32) -> i32
    where
        U: Into<String> + Send + Sync
    {
        tokio::time::sleep(Duration::from_millis(10)).await;
        self.value + value
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
    let entities: Vec<String> = vec!["entity1".to_string(), "entity2".to_string()];
    assert_eq!(example.complex_signature(entities.as_slice(), 1).await, 1);
}