use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver};
//use std::time::Duration;
use websocket;
use websocket::{OwnedMessage};
use websocket::sync::Server;
use gen::messages::{UiMessage, UiUpdate, UiBeetle};
use protobuf::{parse_from_bytes, RepeatedField, Message};

use game;
//use FieldState;

pub struct UI {
    tx_sender: Sender<OwnedMessage>,
    rx_receiver: Receiver<UiMessage>,
}

impl UI {
    pub fn new() -> UI {

        let (tx_sender, tx_receiver) = channel();

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
                    if let OwnedMessage::Binary(message) = message {
                        if let Ok(ui_message) = parse_from_bytes::<UiMessage>(&message) {

                            println!("{:?}", ui_message);

                            match rx_sender.send(ui_message) {
                                Ok(()) => (),
                                Err(_) => (),
                            }
                        }
                    }
                    //if let OwnedMessage::Text(message) = message {

                    //    match serde_json::from_str::<UIMessage>(&message) {
                    //        Ok(ui_message) => {
                    //            match rx_sender.send(ui_message) {
                    //                Ok(()) => (),
                    //                Err(_) => (),
                    //            }
                    //        },
                    //        Err(_) => {},
                    //    }
                    //}
                }
            }
        });

        thread::spawn(move || {

            loop {
                let message = match tx_receiver.recv() {
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
                        let _ = sender.send_message(&websocket::Message::close());
                        return;
                    }
                }
            }
        });

        UI {
            tx_sender: tx_sender,
            rx_receiver: rx_receiver,
        }
    }

    pub fn update(&self, data: &game::FieldState) {

        let mut beetles = RepeatedField::new();
        let mut state = UiUpdate::new();

        for (_, beetle) in &data.beetles {
            let mut new_beetle = UiBeetle::new();

            new_beetle.set_id(beetle.id);
            new_beetle.set_x(beetle.position.x);
            new_beetle.set_y(beetle.position.y);
            new_beetle.set_angle(beetle.angle.0);
            new_beetle.set_size(beetle.size());
            new_beetle.set_selected(beetle.selected);

            beetles.push(new_beetle);
        }

        state.set_beetles(beetles);

        //println!("{:?}", state.get_beetles());

        //let encoded_message = serde_json::to_string(data).unwrap();
        //self.tx_sender.send(OwnedMessage::Text(encoded_message)).unwrap();
        match state.write_to_bytes() {
            Ok(encoded_message) => {
                self.tx_sender.send(OwnedMessage::Binary(encoded_message)).unwrap();
            },
            Err(e) => {
                println!("encode error: {}", e);
            }
        }
    }

    pub fn shutdown(&self) {
        self.tx_sender.send(OwnedMessage::Close(None)).unwrap();
    }

    pub fn get_all_messages(&self) -> Vec<UiMessage> {
        let mut messages = Vec::new();

        for message in self.rx_receiver.try_iter() {
            messages.push(message);
        }

        return messages;
    }
}
