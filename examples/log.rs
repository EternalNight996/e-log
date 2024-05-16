#[cfg(feature = "_log")]
fn main() {
    use e_log::preload::*;
    info!("info");
    debug!("debug");
    error!("error");
    trace!("trace");
    warn!("warn");
}
#[cfg(not(feature = "_log"))]
fn main() {}