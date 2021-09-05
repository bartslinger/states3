use super::states::State;
use super::events::Event;
use super::context::Context;

struct GenericState {
    on_done: State,
}
impl GenericState {

    fn get_sleep_future() -> impl Fn(u32) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()>>> {
        // let sleep_fut = tokio::time::sleep(tokio::time::Duration::from_millis(1000));
        // sleep_fut().then(|| {

        //     Ok(())
        // })
        move |x| Box::pin(async move {
            println!("Just printing stuff from the closure: {}", x);
        })
    }

    fn get_pinned_future(arg: u32) -> std::pin::Pin<Box<dyn std::future::Future<Output = u32>>> {
        Box::pin(async move {
            println!("Printing stuff from the pinned future {}", arg);
            let square = arg * arg;
            square
        })
    }

    pub fn new() -> Self {
        Self {
            on_done: State::Done,
        }
    }

    pub async fn run(&mut self, context: &mut Context, rx: &mut tokio::sync::mpsc::Receiver<Event>) -> State {
        let fut = GenericState::get_pinned_future(10);
        let res = fut.await;
        println!("Squared = {}", res);
        // Run the entry

        // Invoke some function, with an abort handle
        let (abort_tx, abort_rx) = tokio::sync::oneshot::channel::<()>();

        let join_handle = tokio::spawn(async move {
            // For now, just wait for the abort handle
            let _ = abort_rx.await;
        });

        self.on_done
    }
}