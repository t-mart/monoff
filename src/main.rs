// prevent a console window from flashing
#![windows_subsystem = "windows"]

use std::thread::sleep;
use std::time::Duration;

use clap::Parser;
use windows::{
    core::*,
    Win32::{
        Foundation::*,
        System::{Console::*, LibraryLoader::GetModuleHandleW},
        UI::WindowsAndMessaging::*,
    },
};

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

/// Attach to the parent console so that stdin/stdout/stderr work.
fn attach_console() -> Result<()> {
    unsafe { AttachConsole(ATTACH_PARENT_PROCESS) }
}

/// Turn off the monitors.
///
/// This function creates a window and sends a message to it to turn off the
/// monitors. The window is never shown, and is destroyed when the program
/// exits.
fn turn_off_monitors() -> Result<()> {
    let window_class_name = w!("monoff");

    let instance = unsafe { GetModuleHandleW(None)? };

    let window_class = WNDCLASSW {
        hInstance: instance.into(),
        lpszClassName: window_class_name,
        lpfnWndProc: Some(wndproc),
        ..Default::default()
    };

    unsafe { RegisterClassW(&window_class) };

    let window = unsafe {
        CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            window_class_name,
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

/// this function processes messages that our window receives. we just do the
/// default and passthrough to DefWindowProcW.
///
/// why don't we just use DefWindowProcW directly, you ask? because
/// DefWindowProcW is unsafe, but the interface for the lpfnWndProc field of the
/// WNDCLASSW wants a safe function. so we have to wrap it in a safe function.
/// this might be a bug, at least questionable design. see discussion here:
///   - https://github.com/microsoft/windows-rs/issues/711
///   - https://github.com/microsoft/windows-rs/issues/2556
extern "system" fn wndproc(
    window: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    unsafe { DefWindowProcW(window, message, wparam, lparam) }
}

fn main() -> Result<()> {
    attach_console()?;

    // parse args and sleep for delay
    let args = Args::parse();
    let sleep_duration = Duration::from_millis(args.delay.into());
    sleep(sleep_duration);

    turn_off_monitors()
}
