
use std::sync::Arc;
use tokio::sync::Mutex;

pub type InvokeFunction = std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + Sync>>;
pub type InvokeFunctionProvider = &'static (dyn Fn(&mut Context, tokio::sync::oneshot::Receiver<()>) -> InvokeFunction + Send + Sync);

pub type EventHandler = &'static (dyn Fn(&mut Context, &mut Event));

#[derive(Debug)]
pub struct Context {

}

#[derive(Debug)]
pub struct Event {

}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum StateName {
    Root,
    Initializing,
    Running,
    Idle,
    Done,
    Error,
    Unknown,
}
impl Default for StateName {
    fn default() -> Self { StateName::Unknown }
}

#[derive(Default)]
pub struct XState {
    name: StateName,
    invoke: Option<InvokeFunctionProvider>,
    on: Vec<EventHandler>,
    states: Vec<XState>,
}
impl std::fmt::Debug for XState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "XState({:?})", self.name)
    }
}

pub struct Machine<'a> {
    context: Context,
    states: &'a Vec<XState>,
    map: std::collections::HashMap<StateName, &'a XState>,
}
impl Machine<'_> {
    pub async fn run() -> () {

    }
}

fn map_states<'a>(mut map: &mut std::collections::HashMap<StateName, &'a XState>, states: &'a Vec<XState>) {
    states.into_iter().for_each(|x| {
        map.insert(x.name, &x);
        // Also map substates
        map_states(&mut map, &x.states);
    });
}

pub fn create_machine(context: Context, states: &Vec<XState>) -> Machine {

    let mut map = std::collections::HashMap::new();
    map_states(&mut map, &states);

    let machine = Machine {
        context: context,
        states: states,
        map: map,
    };

    println!("{:?}", machine.states);

    // machine.map.insert(states[0].name, &states[0]);
    // Iterate over states and add them to the map
    machine
}

pub async fn run() {
    let machine_structure = vec![
        XState {
            name: StateName::Root,
            invoke: None,
            on: vec![],
            states: vec![
                XState {
                    name: StateName::Initializing,
                    invoke: None,
                    on: vec![],
                    states: vec![],
                },
                XState {
                    name: StateName::Running,
                    invoke: None,
                    on: vec![],
                    states: vec![],
                },
            ]
        },
    ];

    let context = Context {};
    let machine = create_machine(context, &machine_structure);
    println!("Hoi {:?}", machine.map);
}