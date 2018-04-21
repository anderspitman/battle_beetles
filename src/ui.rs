use std::thread;
use std::sync::mpsc::{channel, Sender, Receiver};
//use std::time::Duration;
use websocket;
use websocket::{OwnedMessage};
use websocket::sync::Server;
use gen::messages::{
    UiMessage, UiUpdate, UiBeetle, UiGameState, UiCharts, FloatWrapper,
    UiChartsIncremental, Color,
};
use protobuf::{parse_from_bytes, RepeatedField, Message};

use beetle::Beetles;
use game;
use utils::{mean, max};
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
            new_beetle.set_size(beetle.size());
            new_beetle.set_selected(beetle.selected);

            let mut color = Color::new();
            color.set_r(beetle.color.r as i32);
            color.set_g(beetle.color.g as i32);
            color.set_b(beetle.color.b as i32);
            color.set_a(beetle.color.a as i32);

            new_beetle.set_color(color);

            beetles.push(new_beetle);
        }

        ui_game_state.set_beetles(beetles);
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
        let mut speeds_sum = 0.0;
        let mut max_health_sum = 0.0;
        let mut sizes_sum = 0.0;
        let mut densities_sum = 0.0;
        let mut strengths_sum = 0.0;
        let mut quicknesses_sum = 0.0;

        for beetle in beetles.values() {
            speeds_sum += beetle.speed();
            max_health_sum += beetle.max_health() as f32;

            sizes_sum += beetle.genome.size();
            densities_sum += beetle.genome.carapace_density();
            strengths_sum += beetle.genome.strength();
            quicknesses_sum += beetle.genome.quickness();
        }

        let mut message = UiChartsIncremental::new();

        message.set_avg_speed(speeds_sum / len);
        message.set_avg_max_health(max_health_sum / len);
        message.set_avg_size(sizes_sum / len);
        message.set_avg_carapace_density(densities_sum / len);
        message.set_avg_strength(strengths_sum / len);
        message.set_avg_quickness(quicknesses_sum / len);


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

    pub fn update_charts(
            &self,
            avg_speed_data: Vec<f32>,
            max_fitness_data: Vec<f32>,
            average_sizes_data: Vec<f32>,
            average_densities_data: Vec<f32>,
            average_strengths_data: Vec<f32>,
            average_quicknesses_data: Vec<f32>) {

        let mut ui_update = UiUpdate::new();
        let mut ui_charts = UiCharts::new();
        let mut avg_speeds = RepeatedField::new();
        let mut max_fitnesses = RepeatedField::new();
        let mut average_sizes = RepeatedField::new();
        let mut average_densities = RepeatedField::new();
        let mut average_strengths = RepeatedField::new();
        let mut average_quicknesses = RepeatedField::new();

        for i in 0..avg_speed_data.len() {
            let mut avg_speed = FloatWrapper::new();
            avg_speed.set_value(avg_speed_data[i]);
            avg_speeds.push(avg_speed);

            let mut max_fitness = FloatWrapper::new();
            max_fitness.set_value(max_fitness_data[i]);
            max_fitnesses.push(max_fitness);

            let mut average_size = FloatWrapper::new();
            average_size.set_value(average_sizes_data[i]);
            average_sizes.push(average_size);

            let mut average_density = FloatWrapper::new();
            average_density.set_value(average_densities_data[i]);
            average_densities.push(average_density);

            let mut average_strength = FloatWrapper::new();
            average_strength.set_value(average_strengths_data[i]);
            average_strengths.push(average_strength);

            let mut average_quickness = FloatWrapper::new();
            average_quickness.set_value(average_quicknesses_data[i]);
            average_quicknesses.push(average_quickness);
        }

        ui_charts.set_avg_speeds(avg_speeds);
        ui_charts.set_max_fitnesses(max_fitnesses);
        ui_charts.set_average_sizes(average_sizes);
        ui_charts.set_average_densities(average_densities);
        ui_charts.set_average_strengths(average_strengths);
        ui_charts.set_average_quicknesses(average_quicknesses);
        ui_update.set_charts(ui_charts);

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
