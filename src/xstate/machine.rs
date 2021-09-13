use super::{EventReceiver, EventSender};
use super::xstate::{XState};
use super::{IdType, ContextType, EventType};

pub type StatesMap<'s, Id, Context, Event> = std::collections::HashMap<Id, &'s XState<Id, Context, Event>>;
pub type ParentsMap<Id> = std::collections::HashMap<Id, Option<Id>>;

pub struct MachineStructure<'s, Id: IdType, Context: ContextType, Event: EventType> {
    pub map: StatesMap<'s, Id, Context, Event>,
    pub parents: ParentsMap<Id>,
}
impl<'a, Id: IdType, Context: ContextType, Event: EventType> MachineStructure<'a, Id, Context, Event> {
    pub fn get_parent(&self, state_id: Id) -> Option<&'a XState<Id, Context, Event>> {
        let parent_id = match self.parents.get(&state_id) {
            Some(Some(parent_id)) => parent_id,
            _ => return None,
        };

        if let Some(&parent) = self.map.get(parent_id) {
            Some(parent)
        } else {
            None
        }
    }
}

pub struct Machine<'s, Id: IdType, Context: ContextType, Event: EventType> {
    context: Context,
    states: &'s Vec<XState<Id, Context, Event>>,
    structure: MachineStructure<'s, Id, Context, Event>,
    event_sender: EventSender<Event>,
    event_receiver: EventReceiver<Event>,
}
impl<Id: IdType, Context: ContextType, Event: EventType> Machine<'_, Id, Context, Event> {
    pub fn new<'s>(context: Context, states: &'s Vec<XState<Id, Context, Event>>) -> Machine<'s, Id, Context, Event> {
        let mut map = std::collections::HashMap::new();
        let mut parents_map = std::collections::HashMap::new();
        Self::map_states(&states, &mut map, &mut parents_map, None);

        let structure = MachineStructure {
            map: map,
            parents: parents_map,
        };

        let (tx, rx) = tokio::sync::mpsc::channel(10);
        let machine = Machine {
            context: context,
            states: states,
            structure: structure,
            event_sender: tx,
            event_receiver: rx,
        };

        machine
    }

    pub fn get_event_send_handle(&self) -> EventSender<Event> {
        self.event_sender.clone()
    }

    fn map_states<'s>(states: &'s Vec<XState<Id, Context, Event>>, mut map: &mut StatesMap<'s, Id, Context, Event>, mut parents_map: &mut ParentsMap<Id>, parent: Option<Id>) {
        states.into_iter().for_each(|x| {
            if let Some(_) = map.insert(x.id, &x) {
                panic!("State with Id {:?} already exists, Id can only be used once", x.id);
            }
            parents_map.insert(x.id, parent);
            // Also map substates
            Self::map_states(&x.states, &mut map, &mut parents_map, Some(x.id));
        });
    }

    pub async fn run(&mut self, initial: Id) -> () {
        let mut current_id = initial;
        loop {
            let state = if let Some(&s) = self.structure.map.get(&current_id) {
                s
            } else {
                println!("State {:?} is not mapped, exiting", current_id);
                break
            };

            match state.run(&mut self.context, &mut self.event_receiver, &self.structure).await {
                None => { break },
                Some(next) => {
                    current_id = next;
                },
            }
        }
        println!("No next state provided, exiting..");
    }
}
