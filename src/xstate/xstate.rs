// TODO: Remove
use crate::xstate_user::EventHandlerResponse;

use super::{super::xstate_user::Context, InvokeFunctionProvider, EventHandler, EventSender, EventReceiver, TaskError, TaskOutput};
use super::machine::{MachineStructure};

pub trait IdType: 'static + std::fmt::Debug + std::default::Default + std::cmp::Eq + std::hash::Hash + Copy {}
pub trait EventType: 'static + std::fmt::Debug {
    fn task_done(res: TaskOutput) -> Self;
    fn task_error(res: TaskError) -> Self;
}

pub struct XState<Id: IdType, Event: EventType> {
    pub id: Id,
    pub invoke: Option<InvokeFunctionProvider<Event>>,
    pub event_handler: EventHandler<Id, Event>,
    pub states: Vec<XState<Id, Event>>,
}
impl<Id: IdType, Event: EventType> std::fmt::Debug for XState<Id, Event> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "XState({:?})", self.id)
    }
}
impl<Id: IdType, Event: EventType> XState<Id, Event> {
    fn dummy_event_handler(context: &mut Context, event: &Event, task_event_sender: &mut EventSender<Event>) -> EventHandlerResponse<Id> {
        EventHandlerResponse::Unhandled
    }

    fn handle_event(&self, mut context: &mut Context, event: Event, task_event_tx: &Option<&mut EventSender<Event>>, machine_structure: &MachineStructure<Id, Event>) -> EventHandlerResponse<Id> {
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

    pub async fn run(&self, mut context: &mut Context, event_listener: &mut EventReceiver<Event>, machine_structure: &MachineStructure<'_, Id, Event>) -> Option<Id> {

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
