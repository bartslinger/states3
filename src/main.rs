#![allow(unused_variables, dead_code, unreachable_code)]

mod simple_test_machine;
mod traffic_light;
mod xstate;

#[tokio::main]
async fn main() {
    xstate::run().await;
}