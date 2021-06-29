use std::{cmp, env, process, thread, time};
mod config;

#[derive(Default)]
struct BoltState {
    bolt: config::Bolt,
    next_exec_time: Option<u64>,
    text: String,
}

impl BoltState {
    fn update_if_time(self: &mut Self, cur_time: u64) {
        match (self.next_exec_time, self.bolt.interval) {
            (Some(next_exec_time), Some(interval)) => {
                if cur_time >= next_exec_time {
                    self.text = (self.bolt.command)();
                    self.next_exec_time = Some(cur_time + interval);
                }
            }
            _ => {}
        }
    }
}

struct State {
    bolts: Vec<BoltState>,
    dwm: bool,
}

impl Default for State {
    fn default() -> Self {
        State {
            bolts: vec![],
            dwm: false,
        }
    }
}

impl State {
    fn show(self: &Self) -> String {
        match self.bolts.get(0) {
            None => String::from(""),
            Some(bolt_state) => {
                let mut ret = bolt_state.text.clone();
                for bolt_state in self.bolts[1..].iter() {
                    ret.push_str(config::DELIM);
                    ret.push_str(bolt_state.text.as_str());
                }
                ret
            }
        }
    }

    fn update(self: &mut Self) -> u64 {
        let cur_time = time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let mut next_update = u64::MAX;
        for bolt in self.bolts.iter_mut() {
            bolt.update_if_time(cur_time);
            next_update = cmp::min(next_update, bolt.next_exec_time.unwrap_or(u64::MAX));
        }
        next_update - cur_time
    }

    fn output(self: &Self) {
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

fn main() {
    let mut state = State::default();
    let args: Vec<String> = env::args().collect();

    for arg in args {
        match arg.as_str() {
            "--dwm" => {
                state.dwm = true;
            }
            _ => {}
        }
    }

    for bolt in config::BOLTS.iter() {
        state.bolts.push(BoltState {
            bolt: bolt.clone(),
            next_exec_time: Some(0),
            text: (bolt.command)(),
        });
    }

    loop {
        let next_update = state.update();
        state.output();
        thread::sleep(time::Duration::from_secs(next_update));
    }
}
