macro_rules! define_box_functions {
  ($(($name:ident, $level:ident, $fnname:ident, $res:ty)),+) => {
      $(
          #[doc(hidden)]
          pub fn $name(title: &str, msg: impl Into<String>) -> $res {
            let msg = msg.into();
            crate::preload::$level!(msg);
            e_utils::dialog::sync::$fnname(title,msg)
          }

          paste::paste! {
              #[doc(hidden)]
              pub async fn [<a_ $name>](title: &str, msg: impl Into<String>) -> $res {
                let msg = msg.into();
                crate::preload::$level!(msg);
                e_utils::dialog::a_sync::$fnname(title,msg).await
              }
          }
      )+
  };
}

define_box_functions! {
  (error_box, error, error, ()),
  (info_box, info, info, ()),
  (warn_box, warn, warn, ()),
  (ok_zh_box, debug, ok_zh, ()),
  (ok_box, debug, ok, ()),
  (yesno_zh_box, debug, yesno_zh, bool),
  (yesno_box, debug, yesno, bool)
}
