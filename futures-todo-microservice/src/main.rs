extern crate bytes;
extern crate futures;
extern crate futures_cpupool;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate tokio_io;
extern crate tokio_core;
extern crate tokio_http2;
extern crate tokio_proto;
extern crate tokio_service;

// Extern crates
use futures::Stream;
use tokio_core::reactor::Core;
use tokio_core::net::TcpListener;
use tokio_http2::http::{HttpProto};
use tokio_proto::{BindServer};

// Application
mod todo;
use todo::api::TodoService;

fn main() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let address = "127.0.0.1:8000".parse().unwrap();

    let listener = TcpListener::bind(&address, &handle).unwrap();
    let server = listener.incoming().for_each(move |(socket, _)| {
        HttpProto::default().bind_server(&handle, socket, TodoService);
        Ok(())
    });

    println!("Listening to addr 127.0.0.1 on port 8000");

    if let Err(e) = core.run(server) {
        println!("Server failed with {}", e);
    }
}
