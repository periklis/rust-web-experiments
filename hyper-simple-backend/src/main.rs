extern crate hyper;
extern crate futures;
extern crate pretty_env_logger;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::str::FromStr;

use futures::future::FutureResult;

use hyper::header::{ContentLength, ContentType};
use hyper::server::{Http, Service, Request, Response};

enum Routes {
    Blog,
    Admin,
    Index
}

impl FromStr for Routes {
    type Err = ();

    fn from_str(s: &str) -> Result<Routes, ()> {
        match s {
            "/" => Ok(Routes::Index),
            "/admin" => Ok(Routes::Admin),
            "/blog" => Ok(Routes::Blog),
            _ => Err(())
        }
    }
}

#[derive(Serialize, Debug)]
struct ResponsePayload {
    body: &'static str
}

struct FrontService;

impl Service for FrontService {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = FutureResult<Response, hyper::Error>;

    fn call(&self, _req: Request) -> Self::Future {
        let api =  match Routes::from_str(_req.path()) {
            Ok(Routes::Index) => ResponsePayload{body: "Index"},
            Ok(Routes::Blog) => ResponsePayload{body: "Blog entries"},
            Ok(Routes::Admin) => ResponsePayload{body: "Admin Frontend"},
            Err(_) => ResponsePayload{body: "404: Site not found"}
        };

        let res = serde_json::to_string(&api).unwrap();

        futures::future::ok(
            Response::new()
                .with_header(ContentLength(res.len() as u64))
                .with_header(ContentType::plaintext())
                .with_body(res)
        )
    }
}

fn main() {
    pretty_env_logger::init().unwrap();
    let addr = "127.0.0.1:3000".parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(FrontService)).unwrap();
    println!("Listening on http://{} with 1 thread.", server.local_addr().unwrap());
    server.run().unwrap();
}
