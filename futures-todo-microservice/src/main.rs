extern crate bytes;
extern crate futures;
extern crate futures_cpupool;
extern crate httpdate;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate tk_http;
extern crate tokio_io;
extern crate tokio_core;

// Extern crates
use futures::{Future, Stream};
use tk_http::server::buffered::{BufferedDispatcher};
use tk_http::server::{Config, Proto};
use tokio_core::reactor::Core;
use tokio_core::net::TcpListener;

// Application
mod todo;
use todo::api::Api;

fn main() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let address = "127.0.0.1:8000".parse().unwrap();
    let listener = TcpListener::bind(&address, &handle).unwrap();
    let cfg = Config::new().done();

    let client_handle = core.handle();
    let connections = listener.incoming();
    let server = connections.for_each(move |(socket, addr)| {
        let responses = Proto::new(
            socket,
            &cfg,
            BufferedDispatcher::new(addr, &client_handle, || Api::serve),
            &client_handle
        ).map_err(|e| { println!("Connection error: {}", e); });

        client_handle.spawn(responses);

        Ok(())
    });

    println!("Listening to addr 127.0.0.1 on port 8000");

    if let Err(e) = core.run(server) {
        println!("Server failed with {}", e);
    }
}
