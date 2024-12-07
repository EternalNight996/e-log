fn main() -> e_utils::AnyResult<()> {
  #[cfg(feature = "panic")]
  {
    e_log::panic::reattach_windows_terminal();
    e_log::panic::set_panic_hook("logs", "test.log")?;
  }
  panic!("RUST测试")
}
