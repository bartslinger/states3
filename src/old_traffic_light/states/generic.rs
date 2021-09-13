use crate::traffic_light::machine::State;
use crate::traffic_light::machine::Event;
use crate::traffic_light::machine::Context;

pub type InvokeFunction = std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + Sync>>;
pub type InvokeFunctionProvider = &'static (dyn Fn(&mut Context, tokio::sync::oneshot::Receiver<()>) -> InvokeFunction + Send + Sync);

pub struct GenericState {
    pub on_done: State,
    pub invoke: InvokeFunctionProvider,
}
impl GenericState {

//     fn get_sleep_future() -> impl Fn(u32) -> InvokeFunction {
//         // let sleep_fut = tokio::time::sleep(tokio::time::Duration::from_millis(1000));
//         // sleep_fut().then(|| {
//
//         //     Ok(())
//         // })
//         move |x| Box::pin(async move {
//             println!("Just printing stuff from the closure: {}", x);
//         })
//     }
//
//     pub fn get_pinned_future(arg: u32) -> InvokeFunction {
//         Box::pin(async move {
//             println!("Printing stuff from the pinned future {}", arg);
//             let square = arg * arg;
//             ()
//         })
//     }
//
//     pub fn get_abortable_future(mut abort_rx: tokio::sync::oneshot::Receiver<()>) -> InvokeFunction {
//         Box::pin(async move {
//             loop {
//                 println!("Just looping until I get aborted...");
//
//                 let sleep = tokio::time::sleep(tokio::time::Duration::from_millis(500));
//                 tokio::pin!(sleep);
//
//                 tokio::select!{
//                     v = sleep => {},
//                     v = &mut abort_rx => { break }
//                 };
//             }
//         })
//     }

    pub fn new(invoke: InvokeFunctionProvider) -> Self {
       Self {
            on_done: State::Done,
            invoke: invoke,
        }
    }

    pub async fn run(&mut self, context: &mut Context, rx: &mut tokio::sync::mpsc::Receiver<Event>) -> State {
        // Run the entry

        // Invoke some function, with an abort handle
        let (abort_tx, abort_rx) = tokio::sync::oneshot::channel::<()>();

        let invokable = (self.invoke)(context, abort_rx);


        let join_handle = tokio::spawn(invokable);
        let _ = join_handle.await;

        println!("And now context yellow is: {}", context.yellow);

//         let join_handle = tokio::spawn(async move {
//             // For now, just wait for the abort handle
//             let _ = abort_rx.await;
//         });
// 
//         let _ = join_handle.await;

        self.on_done
    }
}