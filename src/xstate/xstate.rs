use super::{Id, Context, Event, InvokeFunctionProvider, EventHandler, EventHandlerResponse, EventSender, EventReceiver};
use super::machine::{MachineStructure};

pub struct XState {
    pub id: Id,
    pub invoke: Option<InvokeFunctionProvider>,
    pub event_handler: EventHandler,
    pub states: Vec<XState>,
}
impl std::fmt::Debug for XState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "XState({:?})", self.id)
    }
}
impl XState {

    fn handle_event(&self, mut context: &mut Context, event: Event, mut task_event_tx: &mut EventSender, machine_structure: &MachineStructure) -> EventHandlerResponse {
        println!("Hanlding event {:?}", event);
        let event_result = (self.event_handler)(&mut context, &event, &mut task_event_tx);
        match event_result {
            EventHandlerResponse::Unhandled => {
                // Call parent event handler
                match machine_structure.get_parent(self.id) {
                    Some(parent) => {
                        // Find the parent event handler
                        let event_result = (parent.event_handler)(&mut context, &event, &mut task_event_tx);
                        event_result
                    },
                    _ => EventHandlerResponse::Unhandled,
                }
            }
            _ => event_result,
        }
    }

    pub async fn run(&self, mut context: &mut Context, event_listener: &mut EventReceiver, machine_structure: &MachineStructure<'_>) -> Option<Id> {

        let (mut task_event_tx, task_event_rx) = tokio::sync::mpsc::channel::<Event>(100);

        if let Some(invoke) = self.invoke {
            let future = invoke(&mut context, task_event_rx);
            let mut join_handle = tokio::spawn(future);
            
            // Handle both task completion and incoming events
            loop {
                tokio::select! {
                    v = event_listener.recv() => {
                        // handle event
                        let response = match v {
                            Some(event) => self.handle_event(&mut context, event, &mut task_event_tx, machine_structure),
                            None => {
                                println!("Event channel closed (maybe program should crash here?)");
                                let _ = join_handle.await;
                                break
                            },
                        };
                    },
                    v = &mut join_handle => {
                        match v {
                            Ok(Ok(res)) => { self.handle_event(&mut context, Event::TaskDone(res), &mut task_event_tx, machine_structure); },
                            Ok(Err(err)) => { self.handle_event(&mut context, Event::TaskError(err), &mut task_event_tx, machine_structure); },
                            Err(e) => {},
                        };
                        println!("Invoked task completed");
                        break
                    }
                }
            }
        }
        
        // Continue listening to events
        loop {
            let response = match event_listener.recv().await {
                Some(event) => self.handle_event(&mut context, event, &mut task_event_tx, machine_structure),
                None => {
                    println!("Event channel closed (maybe program should crash here?)");
                    break
                }
            };
            match response {
                EventHandlerResponse::TryTransition(next) => return Some(next),
                _ => {},
            };
        }

        Some(Id::Done)
    }
}
