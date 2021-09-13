use xstate;

#[derive(Debug)]
pub struct Context {
    pub button_press_counter: u32,
}
impl xstate::ContextType for Context {}

#[derive(Debug)]
pub enum Event {
    PushButton,
    Abort,
    TaskDone(xstate::TaskOutput),
    TaskError(xstate::TaskError),
}
impl xstate::EventType for Event {
    fn task_done(res: xstate::TaskOutput) -> Self {
        Event::TaskDone(res)
    }
    
    fn task_error(err: xstate::TaskError) -> Self {
        Event::TaskError(err)
    }
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
impl xstate::IdType for Id {}
impl Default for Id {
    fn default() -> Self { Id::Unknown }
}

