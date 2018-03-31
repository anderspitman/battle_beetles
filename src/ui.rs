use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver};
//use std::time::Duration;
use websocket::{Message, OwnedMessage};
use websocket::sync::Server;
use serde_json;

use simulation;
//use FieldState;

pub struct UI {
    tx: Sender<OwnedMessage>,
    rx_receiver: Receiver<UIMessage>,
}

#[derive(Deserialize, Debug)]
pub struct UIMessage {
    pub message_type: String,
    pub beetle_id: i32,
    pub x: f32,
    pub y: f32,
}

impl UI {
    pub fn new() -> UI {

        let (tx, rx) = channel();

        let (rx_sender, rx_receiver) = channel();

        let mut server = Server::bind("127.0.0.1:4020").unwrap();

        let upgrade = server.accept().ok().unwrap();

        let client = upgrade.use_protocol("battle-beetles")
            .accept().unwrap();

        let ip = client.peer_addr().unwrap();
        println!("Connection from {}", ip);

        let (mut receiver, mut sender) = client.split().unwrap();

        thread::spawn(move || {
            for message in receiver.incoming_messages() {
                if let Ok(message) = message {
                    if let OwnedMessage::Text(message) = message {

                        match serde_json::from_str::<UIMessage>(&message) {
                            Ok(ui_message) => {
                                match rx_sender.send(ui_message) {
                                    Ok(()) => (),
                                    Err(_) => (),
                                }
                            },
                            Err(_) => {},
                        }
                    }
                }
            }
        });

        thread::spawn(move || {

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
                        let _ = sender.send_message(&message);
                        return;
                    }
                    _ => (),
                }
                match sender.send_message(&message) {
                    Ok(()) => (),
                    Err(e) => {
                        println!("Send Loop: {:?}", e);
                        let _ = sender.send_message(&Message::close());
                        return;
                    }
                }
            }
        });

        UI {
            tx: tx,
            rx_receiver: rx_receiver,
        }
    }

    pub fn update(&self, data: &simulation::FieldState) {
    //pub fn update(&self, data: &FieldState::Person) {
        let encoded_message = serde_json::to_string(data).unwrap();
        //let encodedMessage = data.descriptor_static();
        //println!("Sending: {}", encoded_message);
        self.tx.send(OwnedMessage::Text(encoded_message)).unwrap();
    }

    pub fn shutdown(&self) {
        self.tx.send(OwnedMessage::Close(None)).unwrap();
    }

    pub fn get_all_messages(&self) -> Vec<UIMessage> {
        let mut messages = Vec::new();

        for message in self.rx_receiver.try_iter() {
            messages.push(message);
        }

        return messages;
    }
}
