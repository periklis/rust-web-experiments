use std::io::Error as IoError;

use futures::future::Future;
use serde::ser::Error as SerializeError;
use serde_json;
use tokio_http2::http::{Request, Response};
use tokio_http2::method::Method;
use tokio_service::Service;

use super::repository::TodoRepository;

pub struct TodoService;

impl Service for TodoService {
    type Request = Request;
    type Response = Response;
    type Error = IoError;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, request: Request) -> Self::Future {
        // TODO Eliminate unwraps and add proper error propagation
        let repository = match request.method()  {
            Method::Get => {
                let query = request.query().unwrap();
                let id = query.get("id").unwrap();
                TodoRepository::Read(id[0].parse().unwrap())
            },
            Method::Post => {
                TodoRepository::Create(serde_json::from_slice(request.payload().unwrap()).unwrap())
            },
            Method::Put =>  {
                TodoRepository::Update(serde_json::from_slice(request.payload().unwrap()).unwrap())
            },
            Method::Delete => {
                let query = request.query().unwrap();
                let id = query.get("id").unwrap();
                TodoRepository::Delete(id[0].parse().unwrap())
            },
            _ => TodoRepository::Error
        };

        repository.map(|todo| {
            let todo_str = todo
                .ok_or(serde_json::error::Error::custom("Error: Something went wrong"))
                .and_then(|t| serde_json::to_string(&t))
                .and_then(|j| Ok(j.into_bytes()));

            let (response_body, content_len) = match todo_str {
                Err(_) => ("".to_string().into_bytes(), "0".to_string()),
                Ok(body) => {
                    let len = body.len();
                    (body, len.to_string())
                }
            };

            Response::new()
                .with_header("Connection", "close")
                .with_header("Content-Length", content_len.as_str())
                .with_body(response_body)

        }).boxed()
    }
}
