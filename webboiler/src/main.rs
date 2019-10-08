use std::sync::Arc;

use actix_web::{web, App, HttpServer, Responder};
#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_json;
use slog::Drain;

mod logging;

struct AppData {
    log: slog::Logger,
}

impl AppData {
    pub fn new(log: slog::Logger) -> AppData {
        AppData { log: log }
    }
}


fn index(data: web::Data<AppData>, info: web::Path<(u32, String)>) -> impl Responder {
    info!(data.log, "hello");
    format!("Hello {}! id:{}", info.1, info.0)
}

fn main() -> std::io::Result<()> {
    let drain = slog_json::Json::default(std::io::stdout()).fuse();
    let drain = Arc::new(slog_async::Async::new(drain).build().fuse());
    let root = slog::Logger::root(
        drain, o!("version" => env!("CARGO_PKG_VERSION")));
    HttpServer::new(
        move ||
        App::new()
        .data(AppData::new(root.clone()))
        .wrap(logging::Slogger::new(root.clone()))
        .service(web::resource("/{id}/{name}").to(index))
    )
        .bind("127.0.0.1:8080")?
        .run()
}
