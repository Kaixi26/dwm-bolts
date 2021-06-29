use std::{ffi, process, str};

fn run_cmd<I, S>(cmd: &str, args: I) -> String
where
    I: IntoIterator<Item = S>,
    S: AsRef<ffi::OsStr>,
{
    let output = process::Command::new(cmd).args(args).output();
    match output {
        Ok(output) => {
            let mut output_string = String::from(str::from_utf8(&output.stdout).unwrap_or_default());
            output_string.pop();
            output_string
        }
        Err(_e) => String::from("ERR"),
    }
}

pub fn date() -> String {
    run_cmd("date", &["+%d/%m %H:%M"])
}

pub fn temp() -> String {
    run_cmd(
        "sed",
        &["s/000$/°C/", "/sys/class/thermal/thermal_zone0/temp"],
    )
}

pub fn weather() -> String {
    run_cmd("curl", &["wttr.in/Ponte de Lima?format=1"])
}
