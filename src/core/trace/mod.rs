///
pub mod __private;
// use tracing_appender::rolling;
// use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _};
// /// 添加日志
// pub fn init_layer_log<P>(
//   fname: &str,
//   folder: P,
//   level: Level,
//   format: String,
//   output_list: Vec<String>,
// ) -> std::io::Result<()>
// where
//   P: AsRef<Path> + Clone,
// {
//   if !folder.as_ref().exists() {
//     fs::create_dir_all(&folder)?;
//   }
//   let ansi = format.contains("{ansi}");
//   let line = format.contains("{line}");
//   let file = format.contains("{file}");
//   let xlevel = format.contains("{level}");
//   let thread_ids = format.contains("{thread-ids}");
//   let thread_names = format.contains("{thread-names}");
//   let target = format.contains("{target}");
//   let format_log = rolling::never(folder, fname);
//   let (non_blocking_appender, _guard) = tracing_appender::non_blocking(format_log);
//   let file_layer = tracing_subscriber::fmt::layer()
//     .with_ansi(ansi)
//     .with_line_number(line)
//     .with_file(file)
//     .with_level(xlevel)
//     .with_thread_ids(thread_ids)
//     .with_thread_names(thread_names)
//     .with_target(target)
//     .with_writer(non_blocking_appender)
//     .json();
//   let sub = tracing_subscriber::registry()
//     .with(level.to_level_filter())
//     .with(file_layer);
//   if cfg!(debug_assertions) {
//     let terminal_layer = tracing_subscriber::fmt::layer().with_writer(io::stdout);
//     sub.with(terminal_layer).init();
//   } else {
//     sub.init();
//   };
//   Ok(())
// }
