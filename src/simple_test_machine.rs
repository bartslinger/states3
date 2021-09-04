use crate::{State, Event};

struct InitialState {
    on_done: State,
    on_error: State,
}

impl InitialState {
    async fn run(&self, mut rx: tokio::sync::mpsc::Receiver<Event>) -> (tokio::sync::mpsc::Receiver<Event>, State) {

        // Invoke some function
        let (abort_tx, mut abort_rx) = tokio::sync::oneshot::channel::<()>();
        let handle = tokio::spawn(async move {
            loop {
                println!("In the invoked task");
                tokio::time::sleep(tokio::time::Duration::from_millis(3000)).await;

                // Safe abort point
                if let Ok(_) = abort_rx.try_recv() { break };
            }
        });

        // Listen to events
        loop {
            let event = rx.recv().await;
            match event {
                Some(Event::Yolo) =>{
                    println!("Received Yolo");
                },
                Some(Event::SomeAbortEvent) => { let _ = abort_tx.send(()); },
                v => println!("rx.recv().await returned: {:?}", v),
            };
            break;
        }

        // Join the task handle
        println!("Joining the invoked task...");
        let res = handle.await;

        (rx, self.on_done)
    }
}

pub struct SimpleTestMachine {
    initial: InitialState,
}
impl SimpleTestMachine {
    pub fn new() -> Self {
        Self {
            initial: InitialState {
                on_done: State::Second,
                on_error: State::Error,
            }
        }
    }

    pub async fn run(&self, mut rx: tokio::sync::mpsc::Receiver<Event>) -> State {
        let mut current_state = State::Initial;
        loop {
            let result = match current_state {
                State::Initial => self.initial.run(rx).await,
                State::Second => (rx, State::Done),
                State::Done => break,
                _ => (rx, State::Error), 
            };
            rx = result.0;
            current_state = result.1;

            println!("State changed to: {:?}", current_state);
        }

        State::Done
    }
}
