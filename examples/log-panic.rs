fn main() -> e_utils::Result<()> {
  e_log::utils::reattach_windows_terminal();
  e_log::utils::set_panic_hook("logs/test.log")?;
  panic!("RUST测试")
}
