use std::{ffi, str};

async fn run_cmd<I, S>(cmd: &str, args: I) -> String
where
    I: IntoIterator<Item = S>,
    S: AsRef<ffi::OsStr>,
{
    let output = tokio::process::Command::new(cmd).args(args).output().await;
    match output {
        Ok(output) => {
            let mut output_string =
                String::from(str::from_utf8(&output.stdout).unwrap_or_default());
            output_string.pop();
            output_string
        }
        Err(_e) => String::from("ERR"),
    }
}

pub async fn date() -> String {
    run_cmd("date", &["+%d/%m %H:%M"]).await
}

pub async fn temp() -> String {
    run_cmd(
        "sed",
        &["s/000$/°C/", "/sys/class/thermal/thermal_zone0/temp"],
    )
    .await
}

pub async fn bat() -> String {
    run_cmd("sed", &["s/$/%/", "/sys/class/power_supply/BAT0/capacity"]).await
}

pub async fn weather() -> String {
    run_cmd("curl", &["wttr.in/Ponte de Lima?format=1"]).await
}
