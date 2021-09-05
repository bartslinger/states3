use super::context::Context;
use super::states::State;
use super::events::Event;

pub struct RedState {
    pub on_done: State,
    pub on_error: State,
}
impl RedState {
    pub async fn run(&self, context: &mut Context, rx: &mut tokio::sync::mpsc::Receiver<Event>) -> State {
        println!("[Red] Waiting for the button to be pressed");

        loop {
            match rx.recv().await {
                Some(Event::PressButton) => {
                    println!("Button press received");
                    break;
                },
                _ => {}
            }
        }

        let sleep = tokio::time::sleep(tokio::time::Duration::from_millis(1000));
        tokio::pin!(sleep);
        loop {
            tokio::select! {
                v = rx.recv() => {},
                v = &mut sleep => { break },
            };
        }
        self.on_done
    }
}