use super::{XState, Id, Context, Event, InvokeFunction, EventHandlerResponse, AbortReceiver};

pub mod red_state {
    use super::*;

    pub fn new() -> XState {
        XState {
            id: Id::TrafficLightRed,
            invoke: Some(&invoke),
            event_handler: &event_handler,
            states: vec![],
        }
    }

    fn invoke(context: &mut Context, mut abort: AbortReceiver) -> InvokeFunction {
        Box::pin(async move {
            loop {
                println!("Still red...");
                let sleep = tokio::time::sleep(tokio::time::Duration::from_millis(2000));
                tokio::pin!(sleep);
                tokio::select! {
                    v = sleep => {},
                    v = &mut abort => { 
                        // Simulate need for some time to cleanup
                        println!("Aborting");
                        tokio::time::sleep(tokio::time::Duration::from_millis(2000)).await;
                        break
                    },
                }
            }
            println!("Aborted");
        })
    }

    fn event_handler(context: &mut Context, event: &Event) -> EventHandlerResponse {
        // Increment the counter. If counter reaches 5, abort the invoked function
        context.button_press_counter += 1;
        if context.button_press_counter == 5 {
            println!("Reached 5, aborting!!");
            return EventHandlerResponse::AbortInvokedFunction
        }
        EventHandlerResponse::DoNothing
    }
}
