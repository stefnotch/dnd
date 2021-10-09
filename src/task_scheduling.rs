use std::{path::PathBuf, process::Command};

use chrono::{Duration, Local};

const SCHEDULED_TASK_NAME: &str = "DoNotDisturbEnd";

pub fn schedule_run_self(after: Duration) {
    let task_scheduler = "SCHTASKS.exe";

    /*
    Examples
    Ok(Output { status: ExitStatus(ExitStatus(out0)), stdout: "SUCCESS: The scheduled task \"DoNotDisturbEnd\" was successfully deleted.\r\n", stderr: "" })               s\S
    12:44                                                        \co
    Create: Ok(Output { status: ExitStatus(ExitStatus(0)), stdout: "SUCCESS: The scheduled task \"DoNotDist
    */

    let cleanup_output = Command::new(task_scheduler)
        .args(["/Delete", "/TN", SCHEDULED_TASK_NAME, "/F"])
        .output();

    log::debug!("Clean up old task: {:?}", cleanup_output);

    let path_to_self = std::env::current_exe()
        .ok()
        .map(PathBuf::into_os_string)
        .and_then(|p| p.into_string().ok())
        .unwrap_or("dnd.exe".to_string());

    let local_time = Local::now();
    let end_time = local_time + after.max(Duration::minutes(1));

    let create_output = Command::new(task_scheduler)
        .args([
            "/Create",
            "/ST",
            &end_time.format("%H:%M").to_string(),
            "/TR",
            &path_to_self,
            "/SC",
            "once",
            "/TN",
            SCHEDULED_TASK_NAME,
        ])
        .output();

    log::debug!(
        "Creating new task at {} outputted {:?}",
        &end_time.format("%H:%M").to_string(),
        create_output
    );
}
