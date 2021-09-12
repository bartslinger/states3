use super::{Id, Context, EventReceiver, EventSender};
use super::xstate::{XState};

pub type StatesMap<'a> = std::collections::HashMap<Id, &'a XState>;
pub type ParentsMap = std::collections::HashMap<Id, Option<Id>>;

pub struct MachineStructure<'a> {
    pub map: StatesMap<'a>,
    pub parents: ParentsMap,
}
impl MachineStructure<'_> {
    pub fn get_parent(&self, state_id: Id) -> Option<&XState> {
        let parent_id = match self.parents.get(&state_id) {
            Some(Some(parent_id)) => parent_id,
            _ => return None,
        };

        if let Some(&parent) = self.map.get(&parent_id) {
            Some(parent)
        } else {
            None
        }
    }
}

pub struct Machine<'a> {
    context: Context,
    states: &'a Vec<XState>,
    structure: MachineStructure<'a>,
    event_sender: EventSender,
    event_receiver: EventReceiver,
}
impl Machine<'_> {
    pub fn new(context: Context, states: &Vec<XState>) -> Machine {
        let mut map = std::collections::HashMap::new();
        let mut parents_map = std::collections::HashMap::new();
        Self::map_states(&states, &mut map, &mut parents_map, None);
        println!("{:?}", parents_map);

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

        println!("{:?}", machine.states);
        machine
    }

    pub fn get_event_send_handle(&self) -> EventSender {
        self.event_sender.clone()
    }

    fn map_states<'a>(states: &'a Vec<XState>, mut map: &mut StatesMap<'a>, mut parents_map: &mut ParentsMap, parent: Option<Id>) {
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
