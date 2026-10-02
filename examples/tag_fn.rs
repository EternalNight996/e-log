//! 带 tag 日志的**函数式**回归（宏写法在 log 路径支持不了 `tag = …`）。
//!
//! ```text
//! cargo run --example tag_fn --no-default-features --features log
//! cargo run --example tag_fn --no-default-features --features tracing
//! cargo run --example tag_fn --no-default-features --features log,tracing
//! cargo run --example tag_fn --no-default-features --features log,_tag
//! ```

fn main() {
  let tag = "启动".to_string();

  e_log::tag::emit_tag(e_log::Level::Info, &tag, "函数式 info + tag");
  e_log::tag::emit_tag(e_log::Level::Warn, &tag, "函数式 warn + tag");
  e_log::tag::emit_tag(e_log::Level::Error, &tag, "函数式 error + tag");
  e_log::tag::emit_tag(e_log::Level::Debug, &tag, "函数式 debug + tag");
  e_log::tag::emit_tag(e_log::Level::Trace, &tag, "函数式 trace + tag");

  // 也接受 &str / String 直接传
  e_log::tag::emit_tag(e_log::Level::Info, "字面量 tag", "&str tag");
  e_log::tag::emit_tag(e_log::Level::Info, tag.clone(), "String tag");

  println!("tag_fn: ok");
}
