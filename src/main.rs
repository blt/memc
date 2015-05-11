extern crate memc;
// extern crate mio;

use memc::memc::protocol;

// use mio::*;
// use mio::tcp::{TcpListener, TcpStream};

fn main() {
    println!("Bye? {}", protocol::hello());
    // println!("Bye? {}", memc::protocol::hello());
    // let addr: std::net::SocketAddr = "127.0.0.1:13265".parse().unwrap();

    // // Setup the server socket
    // let server = TcpListener::bind(&addr).unwrap();

    // // Create an event loop
    // let mut event_loop = EventLoop::new().unwrap();

    // // Start listening for incoming connections
    // event_loop.register(&server, SERVER).unwrap();

    // // Setup the client socket
    // let sock = TcpStream::connect(&addr).unwrap();

    // // Register the socket
    // event_loop.register(&sock, CLIENT).unwrap();

    // // Start handling events
    // event_loop.run(&mut MyHandler(server)).unwrap();
}
