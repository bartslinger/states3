use tokio_stream::{StreamExt};

use xstate::EventSender;
use super::types::Event;

// Listen to keyboard inputs to generate events
pub async fn listener(event_sender: EventSender<Event>) {
    let stdin = tokio::io::stdin();
    let mut reader = tokio_util::codec::FramedRead::new(stdin, tokio_util::codec::LinesCodec::new());
    loop {
        let line = reader.next().await;
        match line {
            Some(Ok(s)) => {
                match s.as_str() {
                    "" => { let _ = event_sender.send(Event::PushButton).await; },
                    "d" => { println!("dropping the event sender"); break },
                    _ => {},
                }
            },
            _ => {},
        }
    }
    drop(event_sender);
    println!("Event sender dropped");
}
