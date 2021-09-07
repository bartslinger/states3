// A specific state that uses generic

use crate::traffic_light::machine::State;
use crate::traffic_light::machine::Context;
use crate::traffic_light::machine::Event;
use super::generic::{GenericState,InvokeFunction};

pub struct SpecificStateOptions {
    pub on_done: State,
}

pub struct SpecificState {
    generic: GenericState,
    options: SpecificStateOptions,
}

impl SpecificState {

    fn zelfde_misschien(context: &mut Context, rx: tokio::sync::oneshot::Receiver<()>) -> InvokeFunction {
        println!("Invoking with context yellow: {}", context.yellow);
        Box::pin(async move {
            println!("Doing some sleep");
            tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
            println!("And this is the abort handle: {:?}", rx);
        })
    }

    pub fn new(options: SpecificStateOptions) -> Self {
        let generic = GenericState {
            invoke: &SpecificState::zelfde_misschien,
            on_done: options.on_done,
        };
        Self {
            generic: generic,
            options,
        }
    }
    pub async fn run(&mut self, mut context: &mut Context, mut rx: &mut tokio::sync::mpsc::Receiver<Event>) -> State {
        // try to run the closure
        self.generic.run(&mut context, &mut rx).await;
        State::Done
    }
}