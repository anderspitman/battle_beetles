use std::thread;
use std::sync::mpsc::{channel, Sender};
//use std::time::Duration;
use websocket::{Message, OwnedMessage};
use websocket::sync::Server;
use serde_json;

use simulation;
//use FieldState;

pub struct UI {
    tx: Sender<OwnedMessage>,
}

impl UI {
    pub fn new() -> UI {

        let (tx, rx) = channel();

        let mut server = Server::bind("127.0.0.1:4020").unwrap();

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

        UI {
            tx: tx
        }
    }

    pub fn update(&self, data: &simulation::FieldState) {
    //pub fn update(&self, data: &FieldState::Person) {
        let encoded_message = serde_json::to_string(data).unwrap();
        //let encodedMessage = data.descriptor_static();
        println!("Sending: {}", encoded_message);
        self.tx.send(OwnedMessage::Text(encoded_message)).unwrap();
    }

    pub fn shutdown(&self) {
        self.tx.send(OwnedMessage::Close(None)).unwrap();
    }
}
