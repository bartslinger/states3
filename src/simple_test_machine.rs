use tokio_stream::{StreamExt};

#[derive(Debug, Clone, Copy)]
pub enum State {
    Initial,
    Second,
    Done,
    Error,
}

#[derive(Debug)]
pub struct Context {
    pub x: u32,
}

#[derive(Debug)]
pub enum Event {
    Yolo,
    SomeAbortEvent,
    Unintersting,
}

struct InitialState {
    on_done: State,
    on_error: State,
}

impl InitialState {
    async fn run(&self, mut context: Context, mut rx: tokio::sync::mpsc::Receiver<Event>, parent_event_handler: fn(Context, Event) -> Context) -> (Context, tokio::sync::mpsc::Receiver<Event>, State) {

        // Invoke some function
        let (abort_tx, mut abort_rx) = tokio::sync::oneshot::channel::<()>();
        let handle = tokio::spawn(async move {
            loop {
                println!("In the invoked task");
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

                // Safe abort point
                if let Ok(_) = abort_rx.try_recv() { 
                    println!("ABORTIN");
                    break;
                };
            }
        });

        // Listen to events
        loop {
            let event = rx.recv().await;
            match event {
                Some(Event::Yolo) =>{
                    println!("Received Yolo");
                },
                Some(Event::SomeAbortEvent) => {
                    let _ = abort_tx.send(());
                    break
                },
                Some(e) => {
                    println!("Dispatch to parent");
                    context = parent_event_handler(context, e);
                },
                _ => { println!("Error I guess"); }
            };
        }

        // Join the task handle
        println!("Joining the invoked task...");
        let res = handle.await;

        (context, rx, self.on_done)
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

    pub fn handle_event(context: Context, event: Event) -> Context {
        println!("Handling in handle_event: {:?}", event);
        context
    }

    pub async fn run(&self, mut context: Context, mut rx: tokio::sync::mpsc::Receiver<Event>) -> State {
        let mut current_state = State::Initial;
        loop {
            let result = match current_state {
                State::Initial => self.initial.run(context, rx, Self::handle_event).await,
                State::Second => (context, rx, State::Done),
                State::Done => break,
                _ => (context, rx, State::Error), 
            };
            context = result.0;
            rx = result.1;
            current_state = result.2;

            println!("State changed to: {:?}", current_state);
        }

        State::Done
    }
}

pub async fn run() {
    let context = Context {
        x: 0,
    };

    let (tx, rx) = tokio::sync::mpsc::channel::<Event>(10);

    let _ = tx.send(Event::Yolo).await;

    let simple_test_machine = SimpleTestMachine::new();
    tokio::spawn(async move {
        simple_test_machine.run(context, rx).await
    });

    let stdin = tokio::io::stdin();
    let mut reader = tokio_util::codec::FramedRead::new(stdin, tokio_util::codec::LinesCodec::new());
    loop {
        let line = reader.next().await;
        match line {
            Some(Ok(s)) => {
                match s.as_str() {
                    "a" => { let _ = tx.send(Event::SomeAbortEvent).await; },
                    _ => {},

                }
            },
            _ => {},
        }
    }
}