use chrono::{Duration, Local};
use flexi_logger::{Duplicate, FileSpec, Logger, WriteMode};
use ntapi::{ntzwapi::ZwUpdateWnfStateData, winapi::shared::ntdef::WNF_STATE_NAME};
use std::{path::PathBuf, process::Command, ptr, thread};

use crate::time_parsing::parse_time;

mod task_scheduling;
mod time_parsing;

const SCHEDULED_TASK_NAME: &str = "DoNotDisturbEnd";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _logger = Logger::try_with_str("trace")?
        .log_to_file(FileSpec::default())
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
    let project_name_arg = std::env::args().skip(2).collect::<String>();

    // Output info
    let time_text = time_arg
        .map(|v| format!("for {}.", v))
        .unwrap_or("".to_string());
    log::info!("Working on {} {}", project_name_arg, time_text);

    if time_in_seconds >= Duration::seconds(1) {
        task_scheduling::schedule_run_self(time_in_seconds);
    }

    return Ok(());
    // TODO: Log every invocation of this including the command line args
    // println!("{:?}", std::env::args());

    // TODO: If no args, then turn it off
    // If args, then turn it on & schedule the task

    // Taken from https://github.com/googleprojectzero/sandbox-attacksurface-analysis-tools/blob/80d7fcc8df9c3160c814c60f5121ae46c560a1b5/NtApiDotNet/NtWnfWellKnownNames.cs#L865
    // WNF_SHEL_QUIET_MOMENT_SHELL_MODE_CHANGED
    let mut wnf_shel_quiet_moment_shell_mode_changed = WNF_STATE_NAME {
        Data: [0xa3bf5075, 0xd83063e],
    };

    /*let mut wnf_shel_quiethours_active_profile_changed = WNF_STATE_NAME {
        Data: [0xA3BF1C75, 0xD83063E],
    };*/

    // Taken from https://stackoverflow.com/a/62602601/3492994
    let mut disable_buffer_data: Vec<u32> = vec![0x00, 0x00, 0x00, 0x00];
    let mut enable_buffer_data: Vec<u32> = vec![0x02, 0x00, 0x00, 0x00];

    unsafe {
        let wnf_update_state = |state_name: &mut WNF_STATE_NAME, buffer: &mut Vec<u32>| {
            ZwUpdateWnfStateData(
                state_name,
                buffer.as_mut_ptr() as *mut _,
                buffer.len() as u32,
                ptr::null_mut(),
                ptr::null_mut(),
                0,
                0,
            )
        };

        let disable_focus_mode_command_status = wnf_update_state(
            &mut wnf_shel_quiet_moment_shell_mode_changed,
            &mut disable_buffer_data,
        );

        if disable_focus_mode_command_status != 0 {
            println!(
                "Warning, disabling the state returned {:#X} (decimal: {})",
                disable_focus_mode_command_status, disable_focus_mode_command_status
            )
        }

        // This is a hack. But so is the rest
        thread::sleep(std::time::Duration::from_millis(1000));

        let enable_focus_mode_command_status = wnf_update_state(
            &mut wnf_shel_quiet_moment_shell_mode_changed,
            &mut enable_buffer_data,
        );

        if enable_focus_mode_command_status == 0 {
            println!("Success")
        } else {
            println!(
                "Failed to change state, return code was {:#X} (decimal: {})",
                enable_focus_mode_command_status, enable_focus_mode_command_status
            )
        }
    }

    Ok(())
}
