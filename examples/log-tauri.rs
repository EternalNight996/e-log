#![allow(unused)]
#[cfg(feature = "log-tauri")]
fn main() {
  use e_log::logger::{init_layer_log, TauriBuilder};
  use e_log::Level;
  use e_log::LogTarget;
  use e_utils::parse::AutoPath as _;
  use e_utils::parse::MyParseFormat as _;
  use e_log::preload::*;

  #[inline]
  fn init_layer(
    folder: &str,
    fname: Option<String>,
    level: Level,
    out_str: Vec<&str>,
  ) -> e_utils::Result<TauriBuilder> {
    folder.auto_create_dir()?;
    let out_list: e_utils::Result<Vec<LogTarget>> = out_str
      .iter()
      .map(|x| {
        LogTarget::from_str(*x).and_then(|x2| {
          Ok(if let LogTarget::Folder(_) = &x2 {
            LogTarget::Folder(std::path::Path::new(folder).to_path_buf())
          } else {
            x2
          })
        })
      })
      .collect();
    let log = init_layer_log(
      fname,
      level,
      "".to_string(),
      out_list?,
      vec!["tao::platform_impl::platform::event_loop::runner".to_string()],
    )?;
    Ok(log)
  }
  info!(target:"test","info");
  debug!("debug");
  error!("error");
  trace!("trace");
  warn!("warn");
  println!("OK");
  //   let logger = init_layer(
  //     &"{cwd}\\logs".parse_path(),
  //     Some("test.log".into()),
  //     Level::Info,
  //     vec!["Stdout", "Web", "OutDirJson", "Folder"],
  //   )
  //   .unwrap()
  //   .build();
  //   tauri::Builder::default()
  //     .plugin(logger)
  //     .run(tauri::generate_context!())
  //     .expect("error while running tauri application");
}

#[cfg(not(feature = "log-tauri"))]
fn main() {}
