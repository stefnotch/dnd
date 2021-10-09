use chrono::Duration;
use flexi_logger::{Duplicate, FileSpec, Logger, WriteMode};

use crate::{focus_mode::set_focus_mode, time_parsing::parse_time};

mod focus_mode;
mod task_scheduling;
mod time_parsing;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut log_file_spec = FileSpec::default().suppress_timestamp();

    // Always log nex to the .exe
    if let Some(mut path_to_exe) = std::env::current_exe().ok() {
        path_to_exe.pop();
        log_file_spec = log_file_spec.directory(path_to_exe);
    }

    let _logger = Logger::try_with_str("trace")?
        .log_to_file(log_file_spec)
        .append()
        .write_mode(WriteMode::BufferAndFlush)
        .duplicate_to_stderr(Duplicate::Warn)
        .start()?;

    // Parse time
    let time_arg = std::env::args().nth(1);
    let time_in_seconds = match &time_arg {
        Some(time) => parse_time(time),
        _ => Duration::seconds(0),
    };

    // Parse project name
    let project_name_arg = std::env::args().skip(2).collect::<Vec<String>>().join(" ");

    // Output info
    let time_text = time_arg
        .map(|v| format!("for {}.", v))
        .unwrap_or("".to_string());
    log::info!("Working on {} {}", project_name_arg, time_text);

    if time_in_seconds >= Duration::seconds(1) {
        set_focus_mode(true);
        task_scheduling::schedule_run_self(time_in_seconds);
    } else {
        set_focus_mode(false);
    }

    let _keep_logger_alive = _logger;
    Ok(())
}
