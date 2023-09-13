// prevent a console window from flashing
#![windows_subsystem = "windows"]

use std::process::ExitCode;
use std::thread::sleep;
use std::time::Duration;

use clap::Parser;
use windows::{
    core::*,
    Win32::{
        Foundation::*,
        System::{Console::*, LibraryLoader::*},
        UI::WindowsAndMessaging::*,
    },
};

const APPLICATION_NAME: &str = "monoff";
const DEFAULT_DELAY: u16 = 50;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of millseconds to delay before turning off the monitors
    #[arg(
        short,
        long,
        default_value_t = DEFAULT_DELAY,
        value_parser = clap::value_parser!(u16).range(0..)
    )]
    delay: u16,
}

/// Try to attach to the parent console (which means stdout/err are can be
/// used), or return an error. This will be an error if the parent doesn't have
/// a console, like if the program is double-clicked in explorer, run from the
/// Run dialog (Win+R), etc.
fn try_attach_console() -> Result<()> {
    unsafe { AttachConsole(ATTACH_PARENT_PROCESS) }
}

/// Turn off the monitors.
fn turn_off_monitors() -> Result<()> {
    // This function creates an invisible window that will accept a message to
    // turn off the monitors. Then, we send that message. (If this sounds
    // strange, that's just how Windows works.)
    let window_class_name = HSTRING::from(APPLICATION_NAME);

    let instance = unsafe { GetModuleHandleW(None)? };

    let window_class = WNDCLASSW {
        hInstance: instance.into(),
        lpszClassName: PCWSTR(window_class_name.as_wide().as_ptr()),
        lpfnWndProc: Some(window_proc),
        ..Default::default()
    };

    unsafe { RegisterClassW(&window_class) };

    let window = unsafe {
        CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            &window_class_name,
            PCWSTR::null(),
            WS_OVERLAPPED,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            None,
            None,
            instance,
            None,
        )
    };

    unsafe {
        SendNotifyMessageW(
            window,
            WM_SYSCOMMAND,
            WPARAM(SC_MONITORPOWER as usize),
            LPARAM(2), // 2 = off, -1 = on, 1 = low power
        )
    }
}

/// Handler for messages sent to our window.
extern "system" fn window_proc(
    window: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    // As far as handling monitor power system command, we don't know how to do
    // it, but DefWindowProcW does, so we just passthrough to that.
    //
    // Why don't we just use DefWindowProcW directly, you ask? Because
    // DefWindowProcW is unsafe, but the interface for the lpfnWndProc field of
    // the WNDCLASSW wants a safe function. So, essentially, this is just a
    // wrapper function that changes the "safety" from unsafe to safe. This
    // might be a bug, at least questionable design. See discussion here:
    //   - https://github.com/microsoft/windows-rs/issues/711
    //   - https://github.com/microsoft/windows-rs/issues/2556
    unsafe { DefWindowProcW(window, message, wparam, lparam) }
}

/// Show a message box. If error, the message box will have an error icon,
/// otherwise it will have an information icon.
fn show_message_box(text: &str, caption: &str, error: bool) -> () {
    let utype = if error {
        MB_ICONERROR | MB_OK
    } else {
        MB_ICONINFORMATION | MB_OK
    };

    unsafe {
        let _ = MessageBoxW(
            None,
            &HSTRING::from(text),
            &HSTRING::from(caption),
            utype,
        );
    }
}

fn main() -> ExitCode {
    // this result dictates whether we will show output in the console or in a
    // message box
    let attach_con_result = try_attach_console();

    let args = match Args::try_parse() {
        Ok(args) => args,
        Err(err) => {
            // --help, for example, will have an exit_code() == 0
            let exit_code_is_error = err.exit_code() != 0;
            if attach_con_result.is_err() {
                show_message_box(
                    &err.to_string(),
                    APPLICATION_NAME,
                    exit_code_is_error,
                );
            } else {
                let _ = err.print();
            }
            return match exit_code_is_error {
                true => ExitCode::FAILURE,
                false => ExitCode::SUCCESS,
            };
        }
    };

    let delay_duration = Duration::from_millis(args.delay.into());
    sleep(delay_duration);

    match turn_off_monitors() {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            let error_message =
                format!("Error turning off monitors: {}", err.to_string());
            if attach_con_result.is_err() {
                show_message_box(&error_message, APPLICATION_NAME, true);
            } else {
                eprintln!("{}", error_message);
            }
            ExitCode::FAILURE
        }
    }
}
