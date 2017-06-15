extern crate bytes;
extern crate futures;
extern crate futures_cpupool;
extern crate httpdate;
extern crate http_muncher;
extern crate tokio_io;
extern crate tokio_core;
extern crate tokio_proto;
extern crate tokio_service;
extern crate tokio_timer;

use std::fmt;
use std::io;
use std::str;
use std::time::SystemTime;

use bytes::BytesMut;
use futures::{future, BoxFuture, Future, Stream, Sink};
use futures_cpupool::CpuPool;
use httpdate::fmt_http_date;
use http_muncher::{Parser, ParserHandler};
use tokio_io::AsyncRead;
use tokio_io::codec::{Encoder, Decoder};
use tokio_core::reactor::Core;
use tokio_core::net::TcpListener;
use tokio_service::Service;

struct HttpParserHandler{
    message: HttpMessage
}

impl ParserHandler for HttpParserHandler {
    fn on_body(&mut self, _: &mut Parser, body: &[u8]) -> bool {
        match str::from_utf8(body) {
            Ok(body) => {
                self.message.body = String::from(body);
                true
            },
            Err(_) => false
        }
    }
}

struct HttpMessage {
    headers: String,
    body: String
}

impl fmt::Display for HttpMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n{}", self.headers, self.body)
    }
}

struct HttpCodec;

impl Decoder for HttpCodec {
    type Item = HttpMessage;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<HttpMessage>> {
        let message = HttpMessage{headers: "".to_string(), body: "".to_string()};

        let mut handler = HttpParserHandler{message: message};
        let mut parser = Parser::request();

        parser.parse(&mut handler, buf);

        if !parser.has_error() {
            Ok(Some(handler.message))
        } else {
            Err(io::Error::new(io::ErrorKind::Other, "invalid HTTP"))
        }
    }
}

impl Encoder for HttpCodec {
    type Item = HttpMessage;
    type Error = io::Error;

    fn encode(&mut self, msg: HttpMessage, buf: &mut BytesMut) -> io::Result<()> {
        buf.extend(msg.headers.as_bytes());
        buf.extend("\r\n".to_string().as_bytes());
        buf.extend(msg.body.as_bytes());
        Ok(())
    }
}

struct HttpService;

impl Service for HttpService {
    type Request = HttpMessage;
    type Response = HttpMessage;
    type Error = io::Error;
    type Future = BoxFuture<HttpMessage, io::Error>;

    fn call(&self, input: HttpMessage) -> Self::Future {
        let response_body = if input.body.len() > 0 {
            "{\"Request\":".to_string() + &input.body + "}"
        } else {
            "{}".to_string()
        };

        let date_field = fmt_http_date(SystemTime::now());

        let headers = format!("
HTTP/1.1 200 OK
Date: {}
Content-Type: application/json; charset=UTF-8
Content-Encoding: UTF-8
Content-Length: {}
Connection: close
", date_field, response_body.len());

        let response = HttpMessage {
            headers: headers.to_string(),
            body: response_body.to_string()
        };

        future::ok(response).boxed()
    }
}

fn main() {
    let pool = CpuPool::new(4);
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let address = "127.0.0.1:8000".parse().unwrap();
    let listener = TcpListener::bind(&address, &handle).unwrap();

    let connections = listener.incoming();
    let server = connections.for_each(move |(socket, _peer_addr)| {
        let (writer, reader) = socket.framed(HttpCodec).split();
        let service = HttpService{};

        let responses = reader.and_then(move |req| service.call(req));
        let server = writer.send_all(responses).then(|_| Ok(()));

        handle.spawn(pool.spawn(server));

        Ok(())
    });

    println!("Listening to addr 127.0.0.1 on port 8000");

    if let Err(e) = core.run(server) {
        println!("Server failed with {}", e);
    }
}
