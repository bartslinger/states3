#![allow(unused_variables, dead_code, unreachable_code)]

mod simple_test_machine;
mod traffic_light;
mod xstate;
mod xstate_user;

#[tokio::main]
async fn main() {
    xstate_user::run().await;
    println!("Exit main");
}