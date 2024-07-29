fn main() -> e_utils::Result<()> {
  e_log::panic::reattach_windows_terminal();
  e_log::panic::set_panic_hook("logs/test.log")?;
  panic!("RUST测试")
}
