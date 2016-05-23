#[macro_use]
extern crate mioco;

use std::net::SocketAddr;
use std::io::{Read, Write};
use mioco::tcp::TcpListener;
use std::thread;

fn main() {

    let (mail_send, mail_recv) = mioco::sync::mpsc::channel::<Vec<u8>>();

    thread::spawn(move || {
        loop {
            let v = mail_recv.recv().unwrap();
            println!("{:?}", v);
        }
    });

    mioco::start(move || {
            let addr = "0.0.0.0:9998".parse::<SocketAddr>().unwrap();

            let listener = TcpListener::bind(&addr).unwrap();

            println!("Starting tcp echo server on {:?}",
                     listener.local_addr().unwrap());


            let mut conn = listener.accept().unwrap();

            mioco::spawn(move || {
                let mut buf = [0u8; 1024 * 16];
                let mut timer = mioco::timer::Timer::new();

                loop {
                    timer.set_timeout(5000);
                    select!(
                        r:conn => {
                            let size = conn.read(&mut buf).unwrap();
                            if size == 0 {
                                break;
                            }
                            conn.write_all(&mut buf[0..size]).unwrap();
                            let b = buf.to_vec();
                            let _ = mail_send.send(b);
                        },
                        r:timer => {
                            conn.shutdown(mioco::tcp::Shutdown::Both).unwrap();
                        },
                        );
                }
            });

        })
        .unwrap();
}
