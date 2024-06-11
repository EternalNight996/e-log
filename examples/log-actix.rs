#[macro_use]
extern crate e_log;
/// # Format
/// Variable | Description
/// -------- | -----------
/// `%%` | The percent sign
/// `%a` | Peer IP address (or IP address of reverse proxy if used)
/// `%t` | Time when the request started processing (in RFC 3339 format)
/// `%r` | First line of request (Example: `GET /test HTTP/1.1`)
/// `%s` | Response status code
/// `%b` | Size of response in bytes, including HTTP headers
/// `%T` | Time taken to serve the request, in seconds to 6 decimal places
/// `%D` | Time taken to serve the request, in milliseconds
/// `%U` | Request URL
/// `%{r}a` | "Real IP" remote address **\***
/// `%{FOO}i` | `request.headers["FOO"]`
/// `%{FOO}o` | `response.headers["FOO"]`
/// `%{FOO}e` | `env_var["FOO"]`
/// `%{FOO}xi` | [Custom request replacement](Logger::custom_request_replace) labelled "FOO"
/// `%{FOO}xo` | [Custom response replacement](Logger::custom_response_replace) labelled "FOO"
///
/// # Security
/// **\*** "Real IP" remote address is calculated using
/// [`ConnectionInfo::realip_remote_addr()`](crate::dev::ConnectionInfo::realip_remote_addr())
///
/// If you use this value, ensure that all requests come from trusted hosts. Otherwise, it is
/// trivial for the remote client to falsify their source IP address.
#[cfg(feature = "log-actix")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
  use actix_web::middleware::Logger;
  use actix_web::{web, App, HttpServer};
  use e_log::logger::init_log_layer;
  use e_log::Level;

  async fn hello() -> &'static str {
    info!(target:"test","hello");
    warn!("hello");
    debug!("hello");
    error!("hello");
    trace!("hello");
    "Hello world!"
  }

  init_log_layer(Level::Info);

  HttpServer::new(move || {
    fn parse_jwt_id(_req: Option<&actix_web::http::header::HeaderValue>) -> String {
      "jwt_uid".to_owned()
    }
    let logger = Logger::new("%% %a %t %P %r %s %b %T %D %{User-Agent}i %{JWT_ID}xi")
      .custom_request_replace("JWT_ID", |req| {
        parse_jwt_id(req.headers().get("Authorization"))
      })
      .log_target("http_log");
    App::new()
      // [2024-05-15T07:58:00Z INFO  actix_web::middleware::logger] % 127.0.0.1 2024-05-15T07:58:00.3089077Z P GET /hello
      // HTTP/1.1 200 12 0.001117 1.134500 PostmanRuntime-ApipostRuntime/1.1.0
      // [2024-05-15T07:55:45Z INFO  actix_web::middleware::logger] 127.0.0.1 "GET /hello HTTP/1.1" 200 12 "-" "PostmanRuntime-ApipostRuntime/1.1.0" 0.001208
      .wrap(logger)
      .service(web::resource("/hello").to(hello))
  })
  .bind("127.0.0.1:12345")?
  .run()
  .await?;

  Ok(())
}

#[cfg(not(feature = "log-actix"))]
fn main() {}
