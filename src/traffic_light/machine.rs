use super::states;

#[derive(Default)]
pub struct Context {
    pub red: bool,
    pub green: bool,
    pub yellow: bool,
}

pub enum Event {
    PressButton,
}

#[derive(Debug, Clone, Copy)]
pub enum State {
    Red,
    Green,
    Yellow,
    Error,
    Done,
}

pub struct TrafficLightMachine {
}
impl TrafficLightMachine {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn run(&self, mut rx: tokio::sync::mpsc::Receiver<Event>){
        let mut context = Context::default();
        let mut state = State::Red;


        let red_state = states::red::RedState{
            on_done: State::Green,
            on_error: State::Error,
        };

        let green_state = states::green::GreenState {
            on_done: State::Yellow,
            on_error: State::Error,
        };

        let yellow_state = states::yellow::YellowState {
            on_done: State::Red,
            on_error: State::Error,
        };

        let mut specific_state = states::specific::SpecificState::new(
            states::specific::SpecificStateOptions {
                on_done: State::Done,
            }
        );

        loop {
            let previous = state;
            state = match state {
                State::Red => red_state.run(&mut context, &mut rx).await,
                State::Green => green_state.run(&mut context, &mut rx).await,
                State::Yellow => specific_state.run(&mut context, &mut rx).await,
                State::Error => State::Done,
                State::Done => break,
            };
            println!("Transition: {:?} -> {:?}", previous, state);
        }
    }
}