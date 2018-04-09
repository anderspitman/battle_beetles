extern crate websocket;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate cgmath;
extern crate rand;
extern crate protobuf;
#[macro_use]
extern crate rouille;

mod ui;
mod game;
mod beetle;
mod gen;
mod simulation;
mod message_handler;

use simulation::Simulate;
use simulation::speed_simulation::SpeedSimulation;
use message_handler::MessageHandler;

use std::thread;
use std::time::Duration;
use rouille::Response;

const SIMULATION_PERIOD_MS: u64 = 20;


fn main() {

    run_web_server();

    let ui = ui::UI::new();

    let mut speed_simulation = SpeedSimulation::new(&ui);
    speed_simulation.mutate = false;
    speed_simulation.run();

    let mut message_handler = MessageHandler::new();

    let mut done = false;
    while !done {
        {
            let game = speed_simulation.get_game();
            ui.update_game_state(game.tick());
        }
        let messages = ui.get_all_messages();

        for message in messages {
            done = message_handler.handle_message(&mut speed_simulation, message);
            if done {
                break;
            }
        }
        thread::sleep(Duration::from_millis(SIMULATION_PERIOD_MS));
    }

    ui.shutdown();
}

fn run_web_server() {

    thread::spawn(move || {
        let index = include_str!("../ui/dist/index.html");
        let bundle = include_str!("../ui/dist/bundle.js");
        // TODO: figure out how to separately load CSS
        //let styles = include_str!("../ui/dist/styles.css");
        rouille::start_server("0.0.0.0:8000", move |request| {

            let response = router!(request,
                (GET) ["/"] => {
                    Response::html(index)
                },
                (GET) ["/bundle.js"] => {
                    Response::text(bundle)
                },
                //(GET) ["/styles.css"] => {
                //    Response::text(styles)
                //},
                _ => {
                    Response::empty_404()
                }
            );

            response
        });
    });
}

