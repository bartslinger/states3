#![allow(unused_variables, dead_code, unreachable_code)]

mod machine;
mod types;
mod states;

#[tokio::main]
async fn main() {
    machine::run().await;
    println!("Exit main");
}
