use crate::traffic_light::machine::Context;
use crate::traffic_light::machine::State;
use crate::traffic_light::machine::Event;

pub struct GreenState {
    pub on_done: State,
    pub on_error: State,
}
impl GreenState {
    pub async fn run(&self, context: &mut Context, rx: &mut tokio::sync::mpsc::Receiver<Event>) -> State {
        println!("[Green]");

        let sleep = tokio::time::sleep(tokio::time::Duration::from_millis(4000));
        tokio::pin!(sleep);
        loop {
            tokio::select! {
                v = rx.recv() => {/* ignore */},
                v = &mut sleep => { break },
            }
        };
        self.on_done
    }
}