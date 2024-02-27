#![warn(clippy::pedantic)]
// prevent a console window from flashing
#![windows_subsystem = "windows"]

use clap::Parser;
use std::{ops::Deref, process::ExitCode, thread::sleep, time::Duration};
use windows::{
    core::{Result, HSTRING, PCWSTR},
    Win32::{
        Foundation::{HWND, LPARAM, LRESULT, WPARAM},
        System::{
            Console::{AttachConsole, ATTACH_PARENT_PROCESS},
            LibraryLoader::GetModuleHandleW,
        },
        UI::WindowsAndMessaging::{
            CreateWindowExW, DefWindowProcW, MessageBoxW, RegisterClassW,
            SendNotifyMessageW, CW_USEDEFAULT, MB_ICONERROR,
            MB_ICONINFORMATION, MB_OK, SC_MONITORPOWER, WINDOW_EX_STYLE,
            WM_SYSCOMMAND, WNDCLASSW, WS_OVERLAPPED,
        },
    },
};

const APPLICATION_NAME: &str = "monoff";
const DEFAULT_DELAY_MS: u16 = 100;

// See https://learn.microsoft.com/en-us/windows/win32/menurc/wm-syscommand
const OFF_MONITORPOWER: LPARAM = LPARAM(2);

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of millseconds to delay before turning off the monitors
    #[arg(
        short,
        long,
        default_value_t = DEFAULT_DELAY_MS,
        value_parser = clap::value_parser!(u16).range(0..)
    )]
    delay: u16,
}

/// Turn off the monitors.
fn turn_off_monitors() -> Result<()> {
    // This function creates an invisible window that will accept a message to
    // turn off the monitors. Then, we send that message. (If this sounds
    // strange, then know that this is just how Windows works.)
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
            OFF_MONITORPOWER,
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
    // the WNDCLASSW wants a safe function. So, essentially, this is just a safe
    // wrapper function on an unsafe function. This might be a bug, at least
    // questionable design. See discussion here:
    //   - https://github.com/microsoft/windows-rs/issues/711
    //   - https://github.com/microsoft/windows-rs/issues/2556
    unsafe { DefWindowProcW(window, message, wparam, lparam) }
}

/// A wrapper for [`clap::Error`] that better exposes the kind of error.
enum ClapError<'err> {
    /// An error that should be printed to stderr
    ParseError(&'err clap::Error),
    /// An error that should be printed to stdout, such as when the user runs with `--version` or
    /// `--help` options.
    Informational(&'err clap::Error),
}

impl ClapError<'_> {
    fn exit_code(&self) -> ExitCode {
        match self {
            Self::ParseError(_) => ExitCode::FAILURE,
            Self::Informational(_) => ExitCode::SUCCESS,
        }
    }

    fn show(&self, message_console: &MessageConsole) {
        match message_console {
            MessageConsole::Textual => {
                // unwrap here because there's nothing we can really do to recover here. this is
                // would be an error with the error handling itself.
                self.print().unwrap();
            }

            MessageConsole::Graphical => {
                message_console.show_message(
                    &self.to_string(),
                    matches!(self, ClapError::ParseError(_)),
                );
            }
        }
    }
}

impl Deref for ClapError<'_> {
    type Target = clap::Error;

    /// Easy access to the underlying error
    fn deref(&self) -> &Self::Target {
        match self {
            Self::ParseError(err) | Self::Informational(err) => err,
        }
    }
}

impl<'err> From<&'err clap::Error> for ClapError<'err> {
    /// Create a [`ClapError`] from a [`clap::Error`].
    fn from(err: &'err clap::Error) -> Self {
        // To differentiate between parse and information errors, we check if the error is destined
        // to stdout or stderr. Unfortunately, this it the best way way I could find to do this with
        // clap's API.
        if err.use_stderr() {
            Self::ParseError(err)
        } else {
            Self::Informational(err)
        }
    }
}

/// A "console" to display messages to the user
enum MessageConsole {
    /// A console that has stdout/err. This should be chosen when the user is running the program
    /// from a console to keep messaging "in-band".
    Textual,
    /// A console that displays messages in a message box. This should be chosen when the user is
    /// running the program from "explorer"-like interfaces, such as double-clicking an icon.
    Graphical,
}

impl MessageConsole {
    /// Try to attach to the parent console (which means stdout/err are can be used), or return an
    /// error. This will be an error if the parent doesn't have a console, like if the program is
    /// double-clicked in explorer, run from the Run dialog (Win+R), etc.
    ///
    /// First attempt to attach to a textual console (that has stdout/err) if available, otherwise
    /// use a "graphical" console, which displays messages with a explorer message box.
    fn attach_to_available() -> MessageConsole {
        let res = unsafe { AttachConsole(ATTACH_PARENT_PROCESS) };
        match res {
            Ok(()) => Self::Textual,
            Err(_) => Self::Graphical,
        }
    }

    fn show_message(&self, message: &str, is_error: bool) {
        match self {
            Self::Textual => {
                if is_error {
                    eprintln!("{message}");
                } else {
                    println!("{message}");
                }
            }
            Self::Graphical => {
                Self::show_message_box(message, APPLICATION_NAME, is_error);
            }
        }
    }

    /// Show a message box with text and a title. If error, the message
    /// box will have an error icon, otherwise it will have an information icon.
    fn show_message_box(text: &str, title: &str, error: bool) {
        let messagebox_style = if error {
            MB_ICONERROR | MB_OK
        } else {
            MB_ICONINFORMATION | MB_OK
        };

        unsafe {
            let _ = MessageBoxW(
                None,
                &HSTRING::from(text),
                &HSTRING::from(title),
                messagebox_style,
            );
        }
    }
}

#[cfg(target_os = "windows")]
fn main() -> ExitCode {
    // this result dictates whether we will show output in the console or in a
    // message box
    let message_console = MessageConsole::attach_to_available();

    let args = match Args::try_parse() {
        Ok(args) => args,

        Err(err) => {
            let clap_error = ClapError::from(&err);
            clap_error.show(&message_console);
            return clap_error.exit_code();
        }
    };

    let delay_duration = Duration::from_millis(args.delay.into());
    sleep(delay_duration);

    match turn_off_monitors() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            let error_message = format!("Error turning off monitors: {err}");
            message_console.show_message(&error_message, true);
            ExitCode::FAILURE
        }
    }
}

#[cfg(not(target_os = "windows"))]
fn main() {
    eprintln!("This program is only intended to run on Windows.");
}
