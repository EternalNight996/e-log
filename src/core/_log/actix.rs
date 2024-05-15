use env_logger::{init_from_env, Env};

use crate::Level;

/// # Example
/// ```rust
/// use actix_web::middleware::Logger;
/// use actix_web::{web, App, HttpServer};
/// use e_log::{preload::*, Level, init_log_layer};
/// async fn hello() -> &'static str {
///   info!("hello");
///   warn!("hello");
///   debug!("hello");
///   error!("hello");
///   trace!("hello");
///   "Hello world!"
/// }
/// #[actix_web::main]
/// async fn main() -> std::io::Result<()> {
///   init_log_layer(Level::Info);
///   HttpServer::new(move || {
///     App::new()
///       .wrap(Logger::default())
///       // %% The percent sign
///       // %a Remote IP-address (IP-address of proxy if using reverse proxy)
///       // %t Time when the request was started to process
///       // %P The process ID of the child that serviced the request
///       // %r First line of request
///       // %s Response status code
///       // %b Size of response in bytes, including HTTP headers
///       // %T Time taken to serve the request, in seconds with floating fraction in .06f format
///       // %D Time taken to serve the request, in milliseconds
///       // %{FOO}i request.headers['FOO']
///       // %{FOO}o response.headers['FOO']
///       // %{FOO}e os.environ['FOO']
///       // [2024-05-15T07:58:00Z INFO  actix_web::middleware::logger] % 127.0.0.1 2024-05-15T07:58:00.3089077Z P GET /hello
///       // HTTP/1.1 200 12 0.001117 1.134500 PostmanRuntime-ApipostRuntime/1.1.0
///       // [2024-05-15T07:55:45Z INFO  actix_web::middleware::logger] 127.0.0.1 "GET /hello HTTP/1.1" 200 12 "-" "PostmanRuntime-ApipostRuntime/1.1.0" 0.001208
///       .wrap(Logger::new("%% %a %t %P %r %s %b %T %D %{User-Agent}i"))
///       .service(web::resource("/hello").to(hello))
///   })
///   .bind("127.0.0.1:12345")?
///   .run()
///   .await?;
///   Ok(())
/// }
///```
pub fn init_log_layer(level: Level) {
  let env_level = Env::new().default_filter_or(level.to_string());
  init_from_env(env_level);
}
