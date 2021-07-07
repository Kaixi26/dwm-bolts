use std::sync::Arc;
use std::{env, io, process};
use tokio::task;
mod config;
use tokio::time::{sleep, Duration};

struct GlobalState {
    bolt_output: [Option<String>; config::BOLTS.len()],
    dwm: bool,
}

impl Default for GlobalState {
    fn default() -> Self {
        GlobalState {
            bolt_output: Default::default(),
            dwm: false,
        }
    }
}

impl GlobalState {
    fn show(self: &Self) -> String {
        let mut ret = String::new();
        for out in self.bolt_output.iter() {
            match out {
                Some(out) => {
                    ret.push_str(out.as_str());
                    ret.push_str(config::DELIM);
                }
                None => {}
            }
        }
        ret.trim_end_matches(config::DELIM).to_string()
    }

    async fn output(self: &Self) {
        if self.dwm {
            process::Command::new("xsetroot")
                .args(&["-name", self.show().as_str()])
                .output()
                .expect("Unable to run xsetroot.");
        } else {
            println!("{}", self.show());
        }
    }
}

async fn handle_bolt(
    state: Arc<tokio::sync::Mutex<GlobalState>>,
    i_bolt: usize,
) -> Result<(), io::Error> {
    let bolt = config::BOLTS
        .get(i_bolt)
        .expect("Passed index overflew arraw.");
    loop {
        let res = (bolt.command)().await;
        {
            let mut state = state.lock().await;
            let bolt_out = state
                .bolt_output
                .get_mut(i_bolt)
                .expect("Passed index overflew array.");
            *bolt_out = Some(res);
            state.output().await;
        };
        sleep(Duration::from_secs(bolt.interval.unwrap_or(3600))).await;
    }
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let mut gstate = GlobalState::default();

    for arg in args {
        match arg.as_str() {
            "--dwm" => {
                gstate.dwm = true;
            }
            _ => {}
        }
    }

    let gstate = Arc::new(tokio::sync::Mutex::new(gstate));
    let mut tasks: Vec<task::JoinHandle<Result<(), io::Error>>> = Vec::new();

    for i_bolt in 0..config::BOLTS.len() {
        let gstate = gstate.clone();
        let task = task::spawn(async move { handle_bolt(gstate, i_bolt).await });
        tasks.push(task);
    }

    for task in tasks.iter_mut() {
        task.await??;
    }

    Ok(())
}
