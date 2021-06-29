mod modules;

#[derive(Debug, Copy, Clone)]
pub struct Bolt {
    pub command: fn() -> String,
    pub interval: Option<u64>,
}

impl Default for Bolt {
    fn default() -> Self {
        Bolt {
            command: || String::default(),
            interval: None,
        }
    }
}

pub const DELIM: &str = " | ";

pub const BOLTS: [Bolt; 3] = [
    Bolt {
        command: modules::weather,
        interval: Some(3600),
    },
    Bolt {
        command: modules::temp,
        interval: Some(5),
    },
    Bolt {
        command: modules::date,
        interval: Some(30),
    },
];
