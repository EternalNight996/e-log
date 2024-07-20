/// Panic
pub mod crash_handler;
pub use crash_handler::*;
pub mod bad_modules;
pub use bad_modules::*;
/// ``WINDOWS ONLY``: Have the application write to the terminal even with
/// ``[windows_subsystem = "windows"]``
///
/// This allows logs to be displayed when launched from the terminal.
pub fn reattach_windows_terminal() {
    #[cfg(windows)]
    {
        use windows_sys::Win32::System::Console::{AttachConsole, ATTACH_PARENT_PROCESS};
        let _ = unsafe { AttachConsole(ATTACH_PARENT_PROCESS) };
    }
}