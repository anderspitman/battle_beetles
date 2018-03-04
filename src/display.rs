extern crate websocket;

use std::thread;
use std::sync::mpsc::{channel, Sender};
//use std::time::Duration;
use self::websocket::{Message, OwnedMessage};
use self::websocket::sync::Server;

pub struct Display {
    tx: Sender<OwnedMessage>,
}

impl Display {
    pub fn new() -> Display {

        let (tx, rx) = channel();

        let mut server = Server::bind("127.0.0.1:2794").unwrap();

        let upgrade = server.accept().ok().unwrap();

        thread::spawn(move || {

            let mut client = upgrade.use_protocol("battle-beetles")
                .accept().unwrap();

            let ip = client.peer_addr().unwrap();

            println!("Connection from {}", ip);

            loop {
                let message = match rx.recv() {
                    Ok(m) => m,
                    Err(e) => {
                        //client.send_message(&message).unwrap();
                        println!("Error: {:?}", e);
                        return;
                    }
                };
                // TODO: not sure why this is necessary
                match message {
                    OwnedMessage::Close(_) => {
                        let _ = client.send_message(&message);
                        return;
                    }
                    _ => (),
                }
                match client.send_message(&message) {
                    Ok(()) => (),
                    Err(e) => {
                        println!("Send Loop: {:?}", e);
                        let _ = client.send_message(&Message::close());
                        return;
                    }
                }
            }
        });

        Display{
            tx: tx
        }
    }

    pub fn update(&self, data: &str) {
        println!("{}", data);
        self.tx.send(OwnedMessage::Text(data.to_string())).unwrap();
    }

    pub fn close(&self) {
        // TODO: implement
    }
}
