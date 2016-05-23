extern crate mio;
use mio::*;
use mio::tcp::*;
use std::net::SocketAddr;
struct WebSocketServer;

const SERVER: mio::Token = mio::Token(0);

struct Pong {
    server: TcpListener,
    connections: Slab<Connection>,
}

impl mio::Handler for Pong {
    type Timeout = ();
    type Message = ();

    fn ready(&mut self,
             event_loop: &mut mio::EventLoop<Pong>,
             token: mio::Token,
             events: mio::EventSet) {

        match token {
            SERVER => {
                assert!(events.is_readable());

                println!("server socket is ready to accept connections");

                match self.server.accept() {
                    Ok(Some(socket)) => {
                        println!("accepted a connection. created socket");
                        event_loop.shutdown();
                    }
                    Ok(None) => {
                        println!("the server socket wasn't actually ready");
                    }
                    Err(e) => {
                        println!("listener.accept() errored: {}", e);
                        event_loop.shutdown();
                    }
                }
            }
            _ => panic!("Unknown token received"),
        }

    }
}

fn main() {
    let address = "0.0.0.0:6768".parse::<SocketAddr>().unwrap();
    let server = TcpListener::bind(&address).unwrap();

    let mut event_loop = mio::EventLoop::new().unwrap();
    event_loop.register(&server, SERVER, EventSet::readable(), PollOpt::edge())
              .unwrap();

    println!("running pingpong server");
    event_loop.run(&mut Pong { server: server });
}
