use std::sync::Arc;

use actix_web::{
    web, App, HttpServer, HttpRequest, HttpResponse, Responder, FromRequest,
};
use actix_web::error::{Error as ActixError, JsonPayloadError, InternalError};
#[macro_use] extern crate slog;
extern crate slog_async;
extern crate slog_json;
use slog::Drain;
use serde_derive::{Deserialize};
use serde_json::json;

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

#[derive(Deserialize)]
struct TimesReq {
    x: f64,
    y: f64,
}

fn times(item: web::Json<TimesReq>) -> impl Responder {
    let res = item.x * item.y;
    HttpResponse::Ok().json(json!({ "result": res }))
}

fn handle_json_error<'r>(
    err: JsonPayloadError, _req: &'r HttpRequest,
) -> ActixError {
    let e = &err;
    let res = match e {
        JsonPayloadError::Deserialize(e) => {
            let msg = match e.is_data() {
                true => "invalid field".to_string(),
                false => e.to_string(),
            };
            json!({
                "error": msg,
                "details": {
                    "column": e.column(),
                    "line": e.line(),
                },
            })
        }
        _ => json!({ "error": format!("{}", err) })
    };
    InternalError::from_response(
        err, HttpResponse::UnprocessableEntity().json(res)
    ).into()
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
        .service(
            web::resource("/times").data(
                web::Json::<TimesReq>::configure(|cfg| {
                    cfg.error_handler(handle_json_error)
                })
            ).route(web::post().to(times)))
        .service(web::resource("/{id}/{name}").to(index))
    )
        .bind("127.0.0.1:8080")?
        .run()
}
