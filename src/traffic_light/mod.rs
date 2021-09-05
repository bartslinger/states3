use tokio_stream::{StreamExt};

mod context;
mod events;
mod states;
mod runner;
mod machine;
mod red_state;
mod green_state;
mod yellow_state;

pub async fn run() {
    println!("Run function");
    let traffic_light = machine::TrafficLightMachine::new();

    let (tx, rx) = tokio::sync::mpsc::channel::<events::Event>(10);

    tokio::spawn(async move {
        traffic_light.run(rx).await;
    });

    let stdin = tokio::io::stdin();
    let mut reader = tokio_util::codec::FramedRead::new(stdin, tokio_util::codec::LinesCodec::new());
    loop {
        let line = reader.next().await;
        match line {
            Some(Ok(s)) => {
                match s.as_str() {
                    "b" => { let _ = tx.send(events::Event::PressButton).await; },
                    _ => {},

                }
            },
            _ => {},
        }
    }
}