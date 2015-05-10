extern crate mio;

use mio::*;
use mio::tcp::{TcpListener, TcpStream};

// Setup some tokens to allow us to identify which event is
// for which socket.
const SERVER: Token = Token(0);
const CLIENT: Token = Token(1);

fn main() {
    let addr: std::net::SocketAddr = "127.0.0.1:13265".parse().unwrap();

    // Setup the server socket
    let server = TcpListener::bind(&addr).unwrap();

    // Create an event loop
    let mut event_loop = EventLoop::new().unwrap();

    // Start listening for incoming connections
    event_loop.register(&server, SERVER).unwrap();

    // Setup the client socket
    let sock = TcpStream::connect(&addr).unwrap();

    // Register the socket
    event_loop.register(&sock, CLIENT).unwrap();

    // Start handling events
    event_loop.run(&mut MyHandler(server)).unwrap();
}

// Define a handler to process the events
struct MyHandler(TcpListener);

impl Handler for MyHandler {
    type Timeout = ();
    type Message = ();

    fn readable(&mut self, event_loop: &mut EventLoop<MyHandler>, token: Token, _: ReadHint) {
        match token {
            SERVER => {
                let MyHandler(ref mut server) = *self;
                // Accept and drop the socket immediately, this will close
                // the socket and notify the client of the EOF.
                let _ = server.accept();
            }
            CLIENT => {
                // The server just shuts down the socket, let's just
                // shutdown the event loop
                println!("Aww shucks!");
                event_loop.shutdown();
            }
            _ => panic!("unexpected token"),
        }
    }
}
