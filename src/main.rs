#![allow(unused_variables, dead_code, unreachable_code)]

mod simple_test_machine;

#[tokio::main]
async fn main() {
    simple_test_machine::run().await;
}