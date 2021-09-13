use crate::traffic_light::machine::Context;
use crate::traffic_light::machine::State;
use crate::traffic_light::machine::Event;

pub struct YellowState {
    pub on_done: State,
    pub on_error: State,
}
impl YellowState {
    pub async fn run(&self, context: &mut Context, rx: &mut tokio::sync::mpsc::Receiver<Event>) -> State {
        println!("[Yellow]");

        let mut flashing = tokio::spawn(async {
            for i in 0..10 {
                if i % 2 == 0 {
                    println!("[Yellow ON]");
                } else {
                    println!("[Yellow OFF]");
                }
                tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
            }
        });

        loop {
            tokio::select! {
                v = rx.recv() => {/* ignore */ println!("ignoring")},
                v = &mut flashing => { break },
            }
        };

        self.on_done
    }
}