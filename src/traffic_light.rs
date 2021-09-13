use crate::xstate::xstate::XState;
use crate::xstate::{InvokeFunction, EventReceiver, EventSender, TaskOutput, EventHandlerResponse};
use crate::xstate_user::{Id, Context, Event};

pub mod red_state {
    use super::*;

    pub fn new<'a>() -> XState<Id, Context, Event> {
        XState {
            id: Id::TrafficLightRed,
            invoke: Some(&invoke),
            event_handler: &event_handler,
            states: vec![],
        }
    }

    fn invoke(context: &mut Context, mut events: EventReceiver<Event>) -> InvokeFunction {
        Box::pin(async move {
            let mut cnt = 0;
            loop {
                cnt += 1;
                if cnt == 5 {
                    break
                }
                println!("Still red... ({})", cnt);
                let sleep = tokio::time::sleep(tokio::time::Duration::from_millis(2000));
                tokio::pin!(sleep);
                tokio::select! {
                    v = sleep => {},
                    v = events.recv() => { 
                        // Simulate need for some time to cleanup
                        println!("Aborting");
                        tokio::time::sleep(tokio::time::Duration::from_millis(2000)).await;
                        return Ok(TaskOutput::Aborted)
                    },
                }
            }
            println!("Aborted");
            Ok(TaskOutput::Ok)
        })
    }

    fn event_handler(context: &mut Context, event: &Event, task_event_sender: &Option<&mut EventSender<Event>>) -> EventHandlerResponse<Id> {
        // Increment the counter. If counter reaches 5, abort the invoked function
        match event {
            Event::Abort => {
                if let Some(task_event_sender) = task_event_sender {
                    let _ = task_event_sender.try_send(Event::Abort);
                }
                EventHandlerResponse::DoNothing
            },
            Event::PushButton => {
                context.button_press_counter += 1;
                if context.button_press_counter == 5 {
                    println!("Reached 5, aborting!!");
                    if let Some(task_event_sender) = task_event_sender {
                        let _ = task_event_sender.try_send(Event::Abort);
                    }
                    EventHandlerResponse::DoNothing
                } else {
                    EventHandlerResponse::Unhandled
                }
            },
            _ => {
                EventHandlerResponse::Unhandled
            },
        }
    }
}