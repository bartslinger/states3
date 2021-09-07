// A specific state that uses generic

use crate::traffic_light::machine::State;
use crate::traffic_light::machine::Context;
use crate::traffic_light::machine::Event;
// use super::generic::GenericState;

pub struct SpecificStateOptions {
    pub on_done: State,
}
trait Iets {}
struct Dat;
impl Iets for Dat {

}

type InvokeFunction = std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + Sync>>;

pub struct SpecificState {
//    generic: GenericState,
    options: SpecificStateOptions,
//    test: std::pin::Pin<Box<dyn Iets + Sync + Send>>,
//    t2: Box<dyn Iets + Sync + Send>,
    t3: &'static (dyn Fn(u32) -> InvokeFunction + Send + Sync),
}

impl SpecificState {

    fn simple_function() -> u32 {
        10
    }

    fn zelfde_misschien(x: u32) -> InvokeFunction {
        Box::pin(async move {
            println!("Doing some sleep");
            tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
            println!("And printing this number: {}", x);
        })
    }

    fn function_that_returns_new_future() -> impl Fn(u32) -> InvokeFunction {
        // let sleep_fut = tokio::time::sleep(tokio::time::Duration::from_millis(1000));
        // sleep_fut().then(|| {

        //     Ok(())
        // })
        move |x| Box::pin(async move {
            println!("Just printing stuff from the closure: {}", x);
        })
    }

    pub fn new(options: SpecificStateOptions) -> Self {
        // let (tx, rx) = tokio::sync::oneshot::channel();
        // let closure = GenericState::get_pinned_future(10);
 
        // let generic = GenericState::new(closure);
        let closure = async move {
            println!("Hoi");
            //tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
            println!("Doei!");
            ()
        };
        Self {
            options,
//            test: std::pin::Pin::new(Box::new(Dat {})),
//            t2: Box::new(Dat {}),
            t3: &SpecificState::zelfde_misschien,
        }
    }
    pub async fn run(&mut self, context: &mut Context, rx: &mut tokio::sync::mpsc::Receiver<Event>) -> State {
        // try to run the closure
        let hoi = (self.t3)(114);
        hoi.await;
        State::Done
    }
}