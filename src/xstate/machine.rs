use super::{Id, Context, EventReceiver, EventSender};
use super::xstate::{XState};

pub struct Machine<'a> {
    context: Context,
    states: &'a Vec<XState>,
    map: std::collections::HashMap<Id, &'a XState>,
    event_sender: EventSender,
    event_receiver: EventReceiver,
}
impl Machine<'_> {
    pub fn new(context: Context, states: &Vec<XState>) -> Machine {
        let mut map = std::collections::HashMap::new();
        Self::map_states(&mut map, &states);

        let (tx, rx) = tokio::sync::mpsc::channel(10);
        let machine = Machine {
            context: context,
            states: states,
            map: map,
            event_sender: tx,
            event_receiver: rx,
        };

        println!("{:?}", machine.states);
        machine
    }

    pub fn get_event_send_handle(&self) -> EventSender {
        self.event_sender.clone()
    }

    fn map_states<'a>(mut map: &mut std::collections::HashMap<Id, &'a XState>, states: &'a Vec<XState>) {
        states.into_iter().for_each(|x| {
            map.insert(x.id, &x);
            // Also map substates
            Self::map_states(&mut map, &x.states);
        });
    }

    pub async fn run(&mut self, initial: Id) -> () {
        let mut current_id = initial;
        loop {
            let state = if let Some(&s) = self.map.get(&current_id) {
                s
            } else {
                println!("State {:?} is not mapped, exiting", current_id);
                break
            };

            match state.run(&mut self.context, &mut self.event_receiver).await {
                None => { break },
                Some(next) => {
                    current_id = next;
                },
            }
        }
        println!("No next state provided, exiting..");
    }
}
