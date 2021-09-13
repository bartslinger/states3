use super::{Context, EventReceiver, EventSender};
use super::xstate::{XState, IdType};

pub type StatesMap<'a, Id> = std::collections::HashMap<Id, &'a XState<Id>>;
pub type ParentsMap<Id> = std::collections::HashMap<Id, Option<Id>>;

pub struct MachineStructure<'a, Id: 'static + IdType> {
    pub map: StatesMap<'a, Id>,
    pub parents: ParentsMap<Id>,
}
impl<Id: IdType + std::cmp::Eq + std::hash::Hash> MachineStructure<'_, Id> {
    pub fn get_parent(&self, state_id: Id) -> Option<&XState<Id>> {
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

pub struct Machine<'a, Id: 'static + IdType> {
    context: Context,
    states: &'a Vec<XState<Id>>,
    structure: MachineStructure<'a, Id>,
    event_sender: EventSender,
    event_receiver: EventReceiver,
}
impl<Id: IdType + std::fmt::Debug + std::default::Default + std::cmp::Eq + std::hash::Hash + Copy> Machine<'_, Id> {
    pub fn new(context: Context, states: &Vec<XState<Id>>) -> Machine<Id> {
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

    pub fn get_event_send_handle(&self) -> EventSender {
        self.event_sender.clone()
    }

    fn map_states<'a>(states: &'a Vec<XState<Id>>, mut map: &mut StatesMap<'a, Id>, mut parents_map: &mut ParentsMap<Id>, parent: Option<Id>) {
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
