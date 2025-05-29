#[cfg(feature = "log")]
fn main() {
  use e_log::preload::*;
  use e_utils::fs::options::{FileExt as _, FileOptionsExt};
  use std::{fs::OpenOptions, io::Write as _, time::Duration};
  info!("test");
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
      // 按照要求使用完全限定语法调用 unlock 方法
      let _ = <_ as e_utils::fs::options::FileExt>::unlock(&f).unwrap();
    }
    let x = f.allocated_size().unwrap();
    println!("allocated -> {x}");
    std::thread::sleep(Duration::from_millis(500));
  }
}
#[cfg(not(feature = "log"))]
fn main() {}
