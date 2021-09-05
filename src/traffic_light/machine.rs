use super::states::State;
use super::events::Event;
use super::context::Context;

use super::{red_state, green_state, yellow_state};

pub struct TrafficLightMachine {
}
impl TrafficLightMachine {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn run(&self, mut rx: tokio::sync::mpsc::Receiver<Event>){
        let mut context = Context::default();
        let mut state = State::Red;


        let red_state = red_state::RedState{
            on_done: State::Green,
            on_error: State::Error,
        };

        let green_state = green_state::GreenState {
            on_done: State::Yellow,
            on_error: State::Error,
        };

        let yellow_state = yellow_state::YellowState {
            on_done: State::Red,
            on_error: State::Error,
        };

        loop {
            let previous = state;
            state = match state {
                State::Red => red_state.run(&mut context, &mut rx).await,
                State::Green => green_state.run(&mut context, &mut rx).await,
                State::Yellow => yellow_state.run(&mut context, &mut rx).await,
                State::Error => State::Done,
                State::Done => break,
            };
            println!("Transition: {:?} -> {:?}", previous, state);
        }
    }
}