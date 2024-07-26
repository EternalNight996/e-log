cfg_panic! {
    /// Panic
    pub mod crash_handler;
    pub use crash_handler::*;
    /// Panic
    pub mod bad_modules;
    pub use bad_modules::*;
}
/// ``WINDOWS ONLY``: Have the application write to the terminal even with
/// ``[windows_subsystem = "windows"]``
///
/// This allows logs to be displayed when launched from the terminal.
#[cfg(feature = "panic")]
pub fn reattach_windows_terminal() {
    #[cfg(windows)]
    {
        use windows_sys::Win32::System::Console::{AttachConsole, ATTACH_PARENT_PROCESS};
        let _ = unsafe { AttachConsole(ATTACH_PARENT_PROCESS) };
    }
}

// 如果panic特性被启用，但log和tracing都没有被启用，则编译错误
#[cfg(feature = "panic")]
#[cfg(panic = "abort")]
#[cfg(not(any(feature = "log", feature = "tracing")))]
compile_error!("Feature 'panic' requires either 'log' or 'tracing' to be enabled.");