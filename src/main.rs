#![allow(unused_variables, dead_code, unreachable_code)]

mod simple_test_machine;
use simple_test_machine::SimpleTestMachine;

#[derive(Debug, Clone, Copy)]
pub enum State {
    Initial,
    Second,
    Done,
    Error,
}

#[derive(Debug)]
pub enum Event {
    Yolo,
    SomeAbortEvent,
}

#[tokio::main]
async fn main() {

    let (tx, rx) = tokio::sync::mpsc::channel::<Event>(10);

    let _ = tx.send(Event::Yolo).await;
    let _ = tx.send(Event::Yolo).await;
    let simple_test_machine = SimpleTestMachine::new();
    simple_test_machine.run(rx).await;
}

// use core::fmt::Debug;
// impl Debug for dyn Series {
//     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//         write!(f, "Series{{{}}}", self.len())
//     }
// }


// let stdin = tokio::io::stdin();
// let mut reader = tokio_util::codec::FramedRead::new(stdin, tokio_util::codec::LinesCodec::new());
// loop {
//     let line = reader.next().await;
//     //let _ = tx.send(0).await;
// }

// use async_trait::async_trait;
// use core::fmt::Debug;

// #[async_trait]
// trait Runnable {
//     async fn run(&self) -> State;
// }
// 
// impl Debug for dyn Runnable {
//     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//         write!(f, "Runnable")
//     }
// }

