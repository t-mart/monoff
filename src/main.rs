// prevent a console window from flashing
#![windows_subsystem = "windows"]

use std::thread::sleep;
use std::time::Duration;

use clap::Parser;
use windows::{
    core::*, Win32::Foundation::*, Win32::System::Console::*,
    Win32::System::LibraryLoader::GetModuleHandleW,
    Win32::UI::WindowsAndMessaging::*,
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
        value_parser = clap::value_parser!(u16).range(0..))
    ]
    delay: u16,
}

fn main() -> Result<()> {
    // attach to parent console so that stdin/stdout/stderr work
    unsafe {
        let _ = AttachConsole(ATTACH_PARENT_PROCESS);
    }

    // parse args and delay
    let args = Args::parse();
    let sleep_duration = Duration::from_millis(args.delay.into());
    sleep(sleep_duration);

    // turn off monitors
    unsafe {
        let instance = GetModuleHandleW(None)?;

        let window_class = w!("monoff");

        let wc = WNDCLASSW {
            hInstance: instance.into(),
            lpszClassName: window_class,
            lpfnWndProc: Some(wndproc),
            ..Default::default()
        };

        RegisterClassW(&wc);

        let window = CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            window_class,
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
        );

        SendNotifyMessageW(
            window,
            WM_SYSCOMMAND,
            WPARAM(SC_MONITORPOWER as usize),
            LPARAM(2), // 2 = off, -1 = on, 1 = low power
        )
    }
}

extern "system" fn wndproc(
    window: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    // this function just passes through to DefWindowProcW. the lpfnWndProc
    // field of WNDCLASSW doesn't allow DefWindowProcW though, so we create a
    // new function pointer that does. this might be a bug. See:
    //   - https://github.com/microsoft/windows-rs/issues/711
    //   - https://github.com/microsoft/windows-rs/issues/2556
    unsafe { DefWindowProcW(window, message, wparam, lparam) }
}
