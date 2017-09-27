use futures::future::{FutureResult, ok as ok_future};
use hyper::header::{ContentLength, ContentType};
use hyper::Error as HyperError;
use hyper::Method;
use hyper::server::{Service, Request, Response};
use serde_json;
use tokio_core::reactor::Handle;

use super::repository::TodoRepository;

pub struct TodoService{
    handle: Handle
}

impl TodoService {
    pub fn new(handle: Handle) -> Self {
       TodoService{handle: handle}
    }
}

impl Service for TodoService {
    type Request = Request;
    type Response = Response;
    type Error = HyperError;
    type Future = FutureResult<Response, HyperError>;

    fn call(&self, request: Request) -> Self::Future {
        // let repository = match request.method()  {
        //     &Method::Get => {
        //         let query = request.query().unwrap();
        //         let id = query.get("id").unwrap();
        //         TodoRepository::Read(id[0].parse().unwrap())
        //     },
        //     &Method::Post => {
        //         TodoRepository::Create(serde_json::from_slice(request.body_ref().unwrap()).unwrap())
        //     },
        //     &Method::Put =>  {
        //         TodoRepository::Update(serde_json::from_slice(request.body_ref().unwrap()).unwrap())
        //     },
        //     &Method::Delete => {
        //         let query = request.query().unwrap();
        //         let id = query.get("id").unwrap();
        //         TodoRepository::Delete(id[0].parse().unwrap())
        //     },
        //     _ => TodoRepository::Error
        // };

        let res = "Hello world";

        let response = Response::new()
            .with_header(ContentLength(res.len() as u64))
            .with_header(ContentType::plaintext())
            .with_body(res);

        ok_future(response)
    }
}
