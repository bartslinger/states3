use super::{Id, Context, Event, InvokeFunctionProvider, EventHandler, EventHandlerResponse, EventSender, EventReceiver};
use super::machine::{MachineStructure};

pub trait IdType {
    fn as_any(&self) -> &dyn std::any::Any;
}
impl std::fmt::Debug for &'static dyn IdType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "IdType({:?})", self)
    }
}
impl PartialEq for &dyn IdType {
    fn eq(&self, rhs: &&dyn IdType) -> bool {
        *self == *rhs
    }
}
impl Eq for &dyn IdType {}
impl std::hash::Hash for &dyn IdType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (*self).hash(state)
    }
}

pub struct XState {
    pub id: &'static dyn IdType,
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
    fn dummy_event_handler(context: &mut Context, event: &Event, task_event_sender: &mut EventSender) -> EventHandlerResponse {
        EventHandlerResponse::Unhandled
    }

    fn handle_event(&self, mut context: &mut Context, event: Event, task_event_tx: &Option<&mut EventSender>, machine_structure: &MachineStructure) -> EventHandlerResponse {
        println!("Hanlding event {:?}", event);
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

    pub async fn run(&self, mut context: &mut Context, event_listener: &mut EventReceiver, machine_structure: &MachineStructure<'_>) -> Option<&'static dyn IdType> {

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
                            Ok(Ok(res)) => { self.handle_event(&mut context, Event::TaskDone(res), &Some(&mut task_event_tx), machine_structure); },
                            Ok(Err(err)) => { self.handle_event(&mut context, Event::TaskError(err), &Some(&mut task_event_tx), machine_structure); },
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

        Some(&Id::Done)
    }
}
