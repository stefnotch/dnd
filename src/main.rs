use ntapi::{ntzwapi::ZwUpdateWnfStateData, winapi::shared::ntdef::WNF_STATE_NAME};
use std::{ptr, thread, time::Duration};

use crate::time_parsing::parse_time;

mod time_parsing;

fn main() {
    // Relies on some stuff being enabled in the focus mode settings
    // (Game or fullscreen, not sure)

    println!("{:?}", std::env::args());

    let time_arg = std::env::args().nth(1);
    let time_in_seconds = match &time_arg {
        Some(time) => parse_time(time),
        _ => Duration::from_secs(0),
    };

    // Everything after that is the project name
    let project_name_arg = std::env::args().skip(2).collect::<String>();

    // Nice output:
    print!("Working on {}", project_name_arg);
    if let Some(time_text) = time_arg {
        print!(" for {}", time_text);
    }

    // TODO: Log every invocation of this including the command line args

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
        thread::sleep(Duration::from_millis(1000));

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
}
