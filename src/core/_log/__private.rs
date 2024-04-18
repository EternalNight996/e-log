// format_args!($($arg)+)
pub use log::{log as slog, Level as StdLevel};
pub use std::format_args;
