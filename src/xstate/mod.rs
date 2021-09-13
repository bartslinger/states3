use tokio_stream::{StreamExt};

mod traffic_light;
mod machine;
mod xstate;

use xstate::{XState, IdType};
use machine::{Machine};

pub type TaskResult = Result<TaskOutput, TaskError>;
pub type InvokeFunction = std::pin::Pin<Box<dyn std::future::Future<Output = TaskResult> + Send + Sync>>;
pub type InvokeFunctionProvider = &'static (dyn Fn(&mut Context, EventReceiver) -> InvokeFunction + Send + Sync);

pub type EventSender = tokio::sync::mpsc::Sender<Event>;
pub type EventReceiver = tokio::sync::mpsc::Receiver<Event>;
pub type EventHandler<Id> = &'static (dyn Fn(&mut Context, &Event, &Option<&mut EventSender>) -> EventHandlerResponse<Id>);

#[derive(Debug)]
pub struct Context {
    button_press_counter: u32,
}

#[derive(Debug)]
pub enum Event {
    PushButton,
    Abort,
    TaskDone(TaskOutput),
    TaskError(TaskError),
}

#[derive(Debug)]
pub enum EventHandlerResponse<Id: IdType> {
    Unhandled,
    DoNothing,    
    TryTransition(Id),
}

#[derive(Debug)]
pub enum TaskOutput {
    Ok,
    Aborted,
}

#[derive(Debug)]
pub enum TaskError {

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
impl IdType for Id {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}
impl Default for &dyn IdType {
    fn default() -> Self {
        &Id::Unknown
    }
}
impl PartialEq for &dyn IdType {
    fn eq(&self, rhs: &&dyn IdType) -> bool {
        let lhs = self.as_any().downcast_ref::<Id>();
        let rhs = rhs.as_any().downcast_ref::<Id>();
        match (lhs, rhs) {
            (Some(lhs), Some(rhs)) => lhs == rhs,
            _ => false
        }
    }
}
impl Eq for &dyn IdType {}
impl std::hash::Hash for &dyn IdType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let lhs = self.as_any().downcast_ref::<Id>();
        lhs.hash(state)
    }
}
impl std::fmt::Debug for &'static dyn IdType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.as_any().downcast_ref::<Id>() {
            Some(id) => write!(f, "{:?}", id),
            None => write!(f, "IdType()"),
        }
    }
}


fn empty_event_handler(context: &mut Context, event: &Event, task_event_sender: &Option<&mut EventSender>) -> EventHandlerResponse<Id> {
    println!("Empty event handler called");
    EventHandlerResponse::Unhandled
}

fn needs_idtype(id: impl IdType) {
    if let Some(test) = id.as_any().downcast_ref::<Id>() {
        println!("{:?}", test);
    }
}

pub async fn run() {

    // test
    println!("Start");


    needs_idtype(Id::Root);


    let machine_states = vec![
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
    let mut machine = Machine::new(context, &machine_states);

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
                        "d" => { println!("dropping the event sender"); break },
                        _ => {},
                    }
                },
                _ => {},
            }
        }
        drop(event_sender);
        println!("Event sender dropped");
    });

    machine.run(Id::TrafficLightRed).await;
}