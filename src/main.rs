#![allow(unused_variables, dead_code, unreachable_code)]

mod simple_test_machine;
mod traffic_light;

#[tokio::main]
async fn main() {
    traffic_light::run().await;
}