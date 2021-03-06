use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver};
//use std::time::Duration;
use websocket;
use websocket::{OwnedMessage};
use websocket::sync::Server;
use gen::messages::{
    UiMessage, UiUpdate, UiBeetle, UiGameState, UiChartsIncremental, Color,
    UiFoodSource, UiHomeBase
};
use protobuf::{parse_from_bytes, RepeatedField, Message};

use entities::{Entity, Beetles};
use utils::Positioned;
use game;
use beetle_genome::BeetleGeneIndex as Gene;
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

    pub fn update_game_state(&self, data: &game::FieldState) {

        let mut ui_update = UiUpdate::new();
        let mut ui_game_state = UiGameState::new();

        let mut beetles = RepeatedField::new();

        for (_, beetle) in data.beetles.iter() {
            let mut new_beetle = UiBeetle::new();

            new_beetle.set_id(beetle.id);
            new_beetle.set_x(beetle.position.x);
            new_beetle.set_y(beetle.position.y);
            new_beetle.set_angle(beetle.angle.0);
            new_beetle.set_selected(beetle.selected);
            new_beetle.set_body_width(beetle.body_width());
            new_beetle.set_body_length(beetle.body_length());
            new_beetle.set_food_carrying(beetle.food_carrying);

            let mut color = Color::new();
            color.set_r(beetle.color.r as i32);
            color.set_g(beetle.color.g as i32);
            color.set_b(beetle.color.b as i32);
            color.set_a(beetle.color.a as i32);

            new_beetle.set_color(color);

            beetles.push(new_beetle);
        }
        ui_game_state.set_beetles(beetles);

        let mut food_sources = RepeatedField::new();
        for food_source in data.get_food_sources().values() {
            let mut new_food_source = UiFoodSource::new();

            new_food_source.set_id(food_source.get_id());
            new_food_source.set_amount(food_source.amount());
            new_food_source.set_x(food_source.get_position().x);
            new_food_source.set_y(food_source.get_position().y);

            food_sources.push(new_food_source);
        }
        ui_game_state.set_food_sources(food_sources);

        let mut home_bases = RepeatedField::new();
        for home_base in data.get_home_bases().values() {
            let mut new_home_base = UiHomeBase::new();

            new_home_base.set_id(home_base.get_id());
            new_home_base.set_x(home_base.get_position().x);
            new_home_base.set_y(home_base.get_position().y);
            new_home_base.set_food_stored_amount(home_base.get_food_stored_amount());

            home_bases.push(new_home_base);
        }
        ui_game_state.set_home_bases(home_bases);

        ui_update.set_game_state(ui_game_state);

        match ui_update.write_to_bytes() {
            Ok(encoded_message) => {
                // prepend message type byte
                //let mut final_message = vec![0];
                //final_message.append(&mut encoded_message);
                self.tx_sender.send(OwnedMessage::Binary(encoded_message)).unwrap();
            },
            Err(e) => {
                println!("encode error: {}", e);
            }
        }
    }

    pub fn update_charts_incremental(&self, beetles: &Beetles) {

        let len = beetles.len() as f32;

        // phenotype
        let mut speeds_sum = 0.0;
        let mut max_health_sum = 0.0;
        let mut attack_power_sum = 0.0;
        let mut food_collected_sum = 0.0;
        let mut sizes_sum = 0.0;

        // genotype
        let mut densities_sum = 0.0;
        let mut strengths_sum = 0.0;
        let mut quicknesses_sum = 0.0;
        let mut venomosities_sum = 0.0;
        let mut mandible_sharpness_sum = 0.0;
        let mut body_width_sum = 0.0;
        let mut body_length_sum = 0.0;

        for beetle in beetles.values() {

            speeds_sum += beetle.speed();
            max_health_sum += beetle.max_health() as f32;
            attack_power_sum += beetle.attack_power() as f32;
            food_collected_sum += beetle.food_collected as f32;
            sizes_sum += beetle.size();

            densities_sum +=
                beetle.genome.get_gene(Gene::CarapaceDensity);
            strengths_sum +=
                beetle.genome.get_gene(Gene::Strength);
            quicknesses_sum +=
                beetle.genome.get_gene(Gene::Quickness);
            venomosities_sum +=
                beetle.genome.get_gene(Gene::Venomosity);
            mandible_sharpness_sum += 
                beetle.genome.get_gene(Gene::MandibleSharpness);
            body_width_sum += 
                beetle.genome.get_gene(Gene::BodyWidth);
            body_length_sum += 
                beetle.genome.get_gene(Gene::BodyLength);
        }

        let mut message = UiChartsIncremental::new();

        message.set_avg_speed(speeds_sum / len);
        message.set_avg_max_health(max_health_sum / len);
        message.set_avg_attack_power(attack_power_sum / len);
        message.set_avg_food_collected(food_collected_sum / len);
        message.set_avg_size(sizes_sum / len);

        message.set_avg_carapace_density(densities_sum / len);
        message.set_avg_strength(strengths_sum / len);
        message.set_avg_quickness(quicknesses_sum / len);
        message.set_avg_venomosity(venomosities_sum / len);
        message.set_avg_mandible_sharpness(mandible_sharpness_sum / len);
        message.set_avg_body_width(body_width_sum / len);
        message.set_avg_body_length(body_length_sum / len);


        let mut ui_update = UiUpdate::new();
        ui_update.set_charts_incremental(message);

        match ui_update.write_to_bytes() {
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
