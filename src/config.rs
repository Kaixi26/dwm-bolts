mod modules;
use std::future::Future;
use std::pin::Pin;

#[derive(Debug, Clone)]
pub struct Bolt {
    pub command: fn() -> Pin<Box<dyn Future<Output = String> + Send>>,
    pub interval: Option<u64>,
}

pub const DELIM: &str = " | ";

pub const BOLTS: [Bolt; 4] = [
    Bolt {
        command: || Box::pin(modules::weather()),
        interval: Some(3600),
    },
    Bolt {
        command: || Box::pin(modules::temp()),
        interval: Some(5),
    },
    Bolt {
        command: || Box::pin(modules::bat()),
        interval: Some(30),
    },
    Bolt {
        command: || Box::pin(modules::date()),
        interval: Some(60),
    },
];
