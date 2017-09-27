extern crate futures;
extern crate hyper;
extern crate pretty_env_logger;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate tokio_core;

use futures::Stream;
use hyper::server::Http;
use tokio_core::reactor::Core;
use tokio_core::net::TcpListener;

use todo::service::TodoService;

mod todo;

fn main() {
    pretty_env_logger::init().unwrap();
    let mut core = Core::new().expect("Failed to create a new event loop");
    let handle = core.handle();

    let http_server = Http::new();
    let sockaddr = "0.0.0.0:8000".parse().unwrap();

    let listener = TcpListener::bind(&sockaddr, &handle).expect("Failed to bind socket address");
    let server = listener.incoming().for_each(move |(socket, addr)| {
        let service_handle = handle.clone();
        http_server.bind_connection(
            &handle, socket, addr, TodoService::new(service_handle));
        Ok(())
    });

    println!("Listening to addr 127.0.0.1 on port 8000");

    if let Err(e) = core.run(server) {
        println!("Server failed with {}", e);
    }
}
