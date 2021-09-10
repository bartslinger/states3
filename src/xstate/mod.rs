use tokio_stream::{StreamExt};

mod traffic_light;

pub type AbortSender = tokio::sync::oneshot::Sender<()>;
pub type AbortReceiver = tokio::sync::oneshot::Receiver<()>;
pub type InvokeFunction = std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + Sync>>;
pub type InvokeFunctionProvider = &'static (dyn Fn(&mut Context, AbortReceiver) -> InvokeFunction + Send + Sync);

pub type EventSender = tokio::sync::mpsc::Sender<Event>;
pub type EventReceiver = tokio::sync::mpsc::Receiver<Event>;
pub type EventHandler = &'static (dyn Fn(&mut Context, &Event) -> EventHandlerResponse);

#[derive(Debug)]
pub struct Context {
    button_press_counter: u32,
}

#[derive(Debug)]
pub enum Event {
    PushButton,
    InvokedFunctionDone,
}

#[derive(Debug)]
pub enum EventHandlerResponse {
    DoNothing,    
    GoTo(Id),
    AbortInvokedFunction,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Id {
    Root,
    Initializing,
    Running,
    Idle,
    TrafficLightRed,
    TrafficLightYellow,
    TrafficLightGreen,
    Done,
    Error,
    Unknown,
}
impl Default for Id {
    fn default() -> Self { Id::Unknown }
}

pub struct XState {
    id: Id,
    invoke: Option<InvokeFunctionProvider>,
    event_handler: EventHandler,
    states: Vec<XState>,
}
impl std::fmt::Debug for XState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "XState({:?})", self.id)
    }
}
impl XState {
    pub async fn run(&self, mut context: &mut Context, event_listener: &mut EventReceiver) -> Option<Id> {

        let (abort_tx, abort_rx) = tokio::sync::oneshot::channel::<()>();
        let (invoke_result_tx, mut invoke_result_rx) = tokio::sync::oneshot::channel::<Event>();
        let join_handle = if let Some(invoke) = self.invoke {
            let future = invoke(&mut context, abort_rx);
            Some(tokio::spawn(async move {
                future.await;
                let _ = invoke_result_tx.send(Event::InvokedFunctionDone);
            }))
        } else {
            None
        };

        // Listen to events
        let event_loop_result = loop {
            println!("Listening for next event..");
            let event = tokio::select! {
                v = event_listener.recv() => {
                    match v {
                        Some(e) => e,
                        None => break None,
                    }
                },
                v = &mut invoke_result_rx => {
                    match v {
                        Ok(e) => e,
                        _ => { continue },
                    }
                },
            };

            let event_result = (self.event_handler)(&mut context, &event);
            match event_result {
                EventHandlerResponse::GoTo(next) => {
                    break Some(next)
                },
                EventHandlerResponse::AbortInvokedFunction => {
                    let res = abort_tx.send(());
                    println!("{:?}", res);
                    break None
                }
                _ => {},
            }
        };
        println!("Finish event loop");

        if let Some(handle) = join_handle {
            println!("Joining invoked function...");
            let _ = handle.await;
        }

        println!("Event loop done: {:?}", event_loop_result);

        event_loop_result
    }
}

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
                }
            }
        }
        println!("No next state provided, exiting..");
    }
}

fn empty_event_handler(context: &mut Context, event: &Event) -> EventHandlerResponse {
    EventHandlerResponse::DoNothing
}

pub async fn run() {
    let machine_structure = vec![
        XState {
            id: Id::Root,
            invoke: None,
            event_handler: &empty_event_handler,
            states: vec![
                XState {
                    id: Id::Initializing,
                    invoke: None,
                    event_handler: &empty_event_handler,
                    states: vec![],
                },
                traffic_light::red_state::new(),
            ]
        },
    ];

    let context = Context {
        button_press_counter: 0,
    };
    let mut machine = Machine::new(context, &machine_structure);
    println!("Hoi {:?}", machine.map);

    // Get an event tx handle
    let event_sender = machine.get_event_send_handle();

    // Listen to keyboard inputs to generate events
    tokio::spawn(async move {
        let stdin = tokio::io::stdin();
        let mut reader = tokio_util::codec::FramedRead::new(stdin, tokio_util::codec::LinesCodec::new());
        loop {
            let line = reader.next().await;
            match line {
                Some(Ok(s)) => {
                    match s.as_str() {
                        "" => { let _ = event_sender.send(Event::PushButton).await; },
                        _ => {},
                    }
                },
                _ => {},
            }
        }
    });

    machine.run(Id::TrafficLightRed).await;
}