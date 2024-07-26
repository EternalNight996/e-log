#[cfg(feature = "log")]
fn main() {
  use std::{fs::OpenOptions, io::Write as _, time::Duration};
  use e_utils::fs::{FileExt as _, FileOptionsExt};
  info!(target: "test","info");
  debug!("debug");
  error!("error");
  trace!("trace");
  warn!("warn");
  let mut f = OpenOptions::new()
    .write(true)
    .read(true)
    .create(true)
    .lock_share(e_log::FileShare::Read)
    .open("1.txt")
    .unwrap();
  f.write("locked\n".as_bytes()).unwrap();
  for i in 0..30 {
    let _ = f
      .write(format!("{i}.locke write\n").as_bytes())
      .inspect_err(|e| eprintln!("{}", e));
    if i > 20 {
      let _ = f.unlock();
    }
    let x = f.allocated_size().unwrap();
    println!("allocated -> {x}");
    std::thread::sleep(Duration::from_millis(500));
  }
}
#[cfg(not(feature = "log"))]
fn main() {}
