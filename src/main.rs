#![allow(unused_variables, dead_code, unreachable_code)]
use tokio_stream::{StreamExt};

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
pub struct Context {
    x: u32,
}

#[derive(Debug)]
pub enum Event {
    Yolo,
    SomeAbortEvent,
    Unintersting,
}

#[tokio::main]
async fn main() {
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
        //let _ = tx.send(0).await;
    }


}

// use core::fmt::Debug;
// impl Debug for dyn Series {
//     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//         write!(f, "Series{{{}}}", self.len())
//     }
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

