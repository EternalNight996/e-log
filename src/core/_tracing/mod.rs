///
pub mod __private;

cfg_tracing_actix! {
    /// support actix
    #[path="./actix.rs"]
    pub mod logger;
}

cfg_tracing_appender! {
    /// tracing-appender = { version = "0.2" }
    pub mod appender;
}