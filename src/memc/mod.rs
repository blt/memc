pub mod protocol;

// mod memc; // {
    // use mio::*;
    // use mio::tcp::{TcpListener};

    // // Setup some tokens to allow us to identify which event is
    // // for which socket.
    // const SERVER: Token = Token(0);
    // const CLIENT: Token = Token(1);

    // // Define a handler to process the events
    // struct MyHandler(TcpListener);

    // impl Handler for MyHandler {
    //     type Timeout = ();
    //     type Message = ();

    //     fn readable(&mut self, event_loop: &mut EventLoop<MyHandler>, token: Token, _: ReadHint) {
    //         match token {
    //             SERVER => {
    //                 let MyHandler(ref mut server) = *self;
    //                 // Accept and drop the socket immediately, this will close
    //                 // the socket and notify the client of the EOF.
    //                 let _ = server.accept();
    //             }
    //             CLIENT => {
    //                 // The server just shuts down the socket, let's just
    //                 // shutdown the event loop
    //                 println!("Aww shucks!");
    //                 event_loop.shutdown();
    //             }
    //             _ => panic!("unexpected token"),
    //         }
    //     }
    // }
//}
