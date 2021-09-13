use xstate::{Machine, XState, EventSender, EventHandlerResponse};

use super::types::{Id, Context, Event};
use super::states;
use super::keyboard_events;

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

    let (event_sender, event_receiver) = tokio::sync::mpsc::channel::<Event>(10);
    tokio::spawn(keyboard_events::listener(event_sender));

    let mut machine = Machine::new(context, event_receiver, &machine_states);
    machine.run(Id::TrafficLightRed).await;
}