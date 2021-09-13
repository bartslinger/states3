use tokio_stream::{StreamExt};

use xstate::{Machine, XState, EventSender, EventHandlerResponse};

use super::types::{Id, Context, Event};
use super::states;

fn empty_event_handler(context: &mut Context, event: &Event, task_event_sender: &Option<&mut EventSender<Event>>) -> EventHandlerResponse<Id> {
    println!("Empty event handler called");
    EventHandlerResponse::Unhandled
}

pub async fn run() {

    // test
    println!("Start");

    let machine_states = vec![
        XState {
            id: Id::Root,
            invoke: None,
            event_handler: &empty_event_handler,
            states: vec![
                XState {
                    id: Id::Initializing,
                    invoke: None,
                    event_handler: &empty_event_handler,
                    states: vec![],
                },
                states::red_state::new(),
            ]
        },
    ];

    let context = Context {
        button_press_counter: 0,
    };
    let mut machine = Machine::new(context, &machine_states);

    // Get an event tx handle
    let event_sender = machine.get_event_send_handle();

    // Listen to keyboard inputs to generate events
    tokio::spawn(async move {
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
    });

    machine.run(Id::TrafficLightRed).await;
}