use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
use futures::future::{ok, FutureResult};
use futures::{Future, Poll};
extern crate slog;

pub struct Slogger {
    log: slog::Logger,
}

impl Slogger {
    pub fn new(log: slog::Logger) -> Slogger {
        Slogger{ log }
    }
}

// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S> for Slogger
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = SloggerMiddleware<S>;
    type Future = FutureResult<Self::Transform, Self::InitError>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(SloggerMiddleware { service, log: self.log.clone() })
    }
}

pub struct SloggerMiddleware<S> {
    service: S,
    log: slog::Logger,
}

impl<S, B> Service for SloggerMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Box<dyn Future<Item = Self::Response, Error = Self::Error>>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.service.poll_ready()
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        // println!("Hi from start. You requested: {}", req.path());
        let log = self.log.clone();
        let method = req.method().to_string();
        let path = req.path().to_string();
        let version = format!("{:?}", req.version());
        let host = req.connection_info().host().to_string();
        let remote = match req.connection_info().remote() {
            Some(x) => x.to_string(),
            _ => "-".to_string(),
        };
        let user_agent = req.headers().iter()
            .find(|(k, _)| k.as_str() == "user-agent")
            .map(|(_, v)| {
                match v.to_str() {
                    Ok(x) => x.to_string(),
                    Err(e) => { error!(log, "{}", e); "unknown".to_string() }
                }
            });
        Box::new(self.service.call(req).and_then(move |res| {
            info!(log, "request completed";
                  "status" => res.status().as_u16(),
                  "method" => method,
                  "path" => path,
                  "request_version" => version,
                  "remote" => remote,
                  "host" => host,
                  "user_agent" => user_agent,
            );
            Ok(res)
        }))
    }
}
