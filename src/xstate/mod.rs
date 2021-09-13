// TODO: Remove
use crate::xstate_user::{Context, EventHandlerResponse};

pub mod machine;
pub mod xstate;

pub type TaskResult = Result<TaskOutput, TaskError>;
pub type InvokeFunction = std::pin::Pin<Box<dyn std::future::Future<Output = TaskResult> + Send + Sync>>;
pub type InvokeFunctionProvider<Event> = &'static (dyn Fn(&mut Context, EventReceiver<Event>) -> InvokeFunction + Send + Sync);

pub type EventSender<Event> = tokio::sync::mpsc::Sender<Event>;
pub type EventReceiver<Event> = tokio::sync::mpsc::Receiver<Event>;
pub type EventHandler<Id, Event> = &'static (dyn Fn(&mut Context, &Event, &Option<&mut EventSender<Event>>) -> EventHandlerResponse<Id>);

#[derive(Debug)]
pub enum TaskOutput {
    Ok,
    Aborted,
}

#[derive(Debug)]
pub enum TaskError {

}
