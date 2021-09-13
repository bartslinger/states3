pub mod machine;
pub mod xstate;

pub type TaskResult = Result<TaskOutput, TaskError>;
pub type InvokeFunction = std::pin::Pin<Box<dyn std::future::Future<Output = TaskResult> + Send + Sync>>;
pub type InvokeFunctionProvider<Context, Event> = &'static (dyn Fn(&mut Context, EventReceiver<Event>) -> InvokeFunction + Send + Sync);

pub type EventSender<Event> = tokio::sync::mpsc::Sender<Event>;
pub type EventReceiver<Event> = tokio::sync::mpsc::Receiver<Event>;
pub type EventHandler<Id, Context, Event> = &'static (dyn Fn(&mut Context, &Event, &Option<&mut EventSender<Event>>) -> EventHandlerResponse<Id>);

pub trait IdType: 'static + std::fmt::Debug + std::default::Default + std::cmp::Eq + std::hash::Hash + Copy {}
pub trait EventType: 'static + std::fmt::Debug {
    fn task_done(res: TaskOutput) -> Self;
    fn task_error(res: TaskError) -> Self;
}
pub trait ContextType {}

#[derive(Debug)]
pub enum TaskOutput {
    Ok,
    Aborted,
}

#[derive(Debug)]
pub enum TaskError {

}

#[derive(Debug)]
pub enum EventHandlerResponse<Id: IdType> {
    Unhandled,
    DoNothing,    
    TryTransition(Id),
}
