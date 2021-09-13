use super::{IdType, ContextType, EventType};

use super::{InvokeFunctionProvider, EventHandler, EventSender, EventReceiver, EventHandlerResponse};
use super::machine::{MachineStructure};
pub struct XState<Id: IdType, Context: ContextType, Event: EventType> {
    pub id: Id,
    pub invoke: Option<InvokeFunctionProvider<Context, Event>>,
    pub event_handler: EventHandler<Id, Context, Event>,
    pub states: Vec<XState<Id, Context, Event>>,
}
impl<Id: IdType, Context: ContextType, Event: EventType> std::fmt::Debug for XState<Id, Context, Event> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "XState({:?})", self.id)
    }
}
impl<Id: IdType, Context: ContextType, Event: EventType> XState<Id, Context, Event> {
    fn dummy_event_handler(context: &mut Context, event: &Event, task_event_sender: &mut EventSender<Event>) -> EventHandlerResponse<Id> {
        EventHandlerResponse::Unhandled
    }

    fn handle_event(&self, mut context: &mut Context, event: Event, task_event_tx: &Option<&mut EventSender<Event>>, machine_structure: &MachineStructure<Id, Context, Event>) -> EventHandlerResponse<Id> {
        println!("Handling event {:?}", event);
        let event_result = (self.event_handler)(&mut context, &event, task_event_tx);
        match event_result {
            EventHandlerResponse::Unhandled => {
                // Call parent event handler
                match machine_structure.get_parent(self.id) {
                    Some(parent) => {
                        // Find the parent event handler
                        let event_result = (parent.event_handler)(&mut context, &event, task_event_tx);
                        event_result
                    },
                    _ => EventHandlerResponse::Unhandled,
                }
            }
            _ => event_result,
        }
    }

    pub async fn run(&self, mut context: &mut Context, event_listener: &mut EventReceiver<Event>, machine_structure: &MachineStructure<'_, Id, Context, Event>) -> Option<Id> {

        if let Some(invoke) = self.invoke {
            let (mut task_event_tx, task_event_rx) = tokio::sync::mpsc::channel::<Event>(100);
            let future = invoke(&mut context, task_event_rx);
            let mut join_handle = tokio::spawn(future);
            
            // Handle both task completion and incoming events
            loop {
                tokio::select! {
                    v = event_listener.recv() => {
                        // handle event
                        let response = match v {
                            Some(event) => self.handle_event(&mut context, event, &Some(&mut task_event_tx), machine_structure),
                            None => {
                                println!("Event channel closed (maybe program should crash here?)");
                                let _ = join_handle.await;
                                break
                            },
                        };
                    },
                    v = &mut join_handle => {
                        match v {
                            Ok(Ok(res)) => { self.handle_event(&mut context, Event::task_done(res), &Some(&mut task_event_tx), machine_structure); },
                            Ok(Err(err)) => { self.handle_event(&mut context, Event::task_error(err), &Some(&mut task_event_tx), machine_structure); },
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
                Some(event) => self.handle_event(&mut context, event, &None, machine_structure),
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

        Some(Id::default())
    }
}
