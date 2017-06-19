use std::convert::{Into};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::{Error as IoError, ErrorKind};
use std::option::Option;
use std::result::Result;
use std::time::SystemTime;

use futures::{Async, Poll};
use futures::future::{Future, FutureResult, err, ok};
use httpdate::fmt_http_date;
use tk_http::{Status};
use tk_http::server::buffered::{Request};
use tk_http::server::{Encoder, EncoderDone, Error};

use super::Todo;

pub enum Api {
    Error,
    Get(Request),
    Post(Request),
    Put(Request),
    Delete(Request)
}

impl Api {
    pub fn serve<S>(req: Request, mut e: Encoder<S>)
                    -> FutureResult<EncoderDone<S>, Error> {

        e.status(Status::Ok);

        let api: Api = req.into();

        let response = match api {
            Api::Get(req) => {
                let parts: Vec<&str> = req.path().split('/').collect();

                if parts.len() == 0 {
                    None
                } else {
                    println!("{:?}", parts);
                    Todo::get(parts[1].parse().unwrap())
                }
            },
            Api::Post(req) => None,
            Api::Put(req) => None,
            Api::Delete(req) => None,
            Api::Error => {
                e.status(Status::InternalServerError);
                None
            }
        };

        match response {
            Some(t) => {
                let todo = t.to_string();
                e.add_length(todo.as_bytes().len() as u64).unwrap();
                e.format_header("Date", fmt_http_date(SystemTime::now())).unwrap();

                match e.done_headers() {
                    Ok(_) => {
                        e.write_body(todo.as_bytes());
                        ok(e.done())
                    },
                    Err(e) => err(Error::custom(e))
                }
            },
            None => err(Error::custom(IoError::new(ErrorKind::Other, "oh no!")))
        }
    }
}

impl Into<Api> for Request {
    fn into(self) -> Api {
        match self.method().to_uppercase().as_str() {
            "GET" => Api::Get(self),
            "POST" => Api::Post(self),
            "PUT" => Api::Put(self),
            "DELETE" => Api::Delete(self),
            _ => Api::Error
        }
    }
}

// impl From<Request> for Api {
//     fn from(value: Request) -> Self {
//         value.into()
//     }
// }

impl Display for Api {

    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            &Api::Error => write!(f, "Api Error"),
            &Api::Get(ref r) => write!(f, "Request existing todo: {:?}", r),
            &Api::Post(ref r) => write!(f, "Create new todo: {:?}", r),
            &Api::Put(ref r) => write!(f, "Update new todo: {:?}", r),
            &Api::Delete(ref r) => write!(f, "Delete existing todo: {:?}", r)
        }
    }
}

// impl<T> Future for Api<T> {
//     type Item = super::Todo;
//     type Error = io::Error;

//     fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
//         match *self {
//             Api::Get =>  Ok(Todo::get(req)),
//             _ => Ok(Async::NotReady)
//         }
//     }
// }
