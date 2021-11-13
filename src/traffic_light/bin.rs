#![allow(unused_variables, dead_code, unreachable_code)]

mod machine;
mod types;
mod states;
mod keyboard_events;

#[tokio::main]
async fn main() {
    
    let yo = tokio::spawn(async {
        let _ = tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    });
    tokio::pin!(yo);
    loop {
        tokio::select!{
            _ = &mut yo => {
                println!("JO");
            }
        };
    }
    machine::run().await;
    println!("Exit main");
}
