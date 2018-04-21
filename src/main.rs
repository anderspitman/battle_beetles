extern crate websocket;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate cgmath;
extern crate rand;
extern crate protobuf;
#[macro_use]
extern crate rouille;

mod utils;
mod ui;
mod game;
mod beetle;
mod beetle_genome;
mod gen;
mod simulation;
mod message_handler;

use beetle::{Beetles};
use game::Game;
use message_handler::MessageHandler;

use simulation::GeneticAlgorithm;
use simulation::speed_ga::SpeedGA;
use simulation::battle_ga::BattleGA;

use std::thread;
use std::time::{Instant, Duration};
use rouille::Response;


fn main() {

    start_web_server_thread();

    let ui = ui::UI::new();
    let mut game = Game::new();

    let max_speed =
        utils::convert_value_for_sim_period(utils::MAX_SPEED_UNITS_PER_SECOND);

    let max_rotation =
        utils::convert_value_for_sim_period(utils::ROTATION_RADIANS_PER_SECOND);


    // run battle GA
    game.set_random_population(
            utils::POPULATION_SIZE, max_speed, max_rotation);

    let battle_population;
    {
        let mut ga = SpeedGA::new(&mut game, &ui);
        ga.run();
        battle_population = ga.get_population().clone();
    }

    // run speed GA
    game.set_random_population(
            utils::POPULATION_SIZE, max_speed, max_rotation);
    let speed_population;
    {
        let mut ga = BattleGA::new(&mut game, &ui);
        ga.run();
        speed_population = ga.get_population().clone();
    }

    // reset population
    game.field_state.beetles = Beetles::new();

    // TODO: could potentially use itertools chain method to do this, but I
    // don't want an extra dependency just for that right now.
    for (_, beetle) in battle_population.into_iter() {
        game.add_beetle(beetle);
    }
    for (_, beetle) in speed_population.into_iter() {
        game.add_beetle(beetle);
    }


    let mut message_handler = MessageHandler::new();

    let mut done = false;
    while !done {
        
        ui.update_game_state(game.tick());

        let messages = ui.get_all_messages();

        for message in messages {

            let timer = Instant::now();
            done = message_handler.handle_message(&mut game, &ui, message);
            println!("Message handling time: {:?}", duration_as_float(timer.elapsed()));

            if done {
                break;
            }
        }

        thread::sleep(Duration::from_millis(utils::SIMULATION_PERIOD_MS));
    }

    ui.shutdown();
}

fn start_web_server_thread() {

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


fn duration_as_float(duration: Duration) -> f64 {
    duration.as_secs() as f64 + (duration.subsec_nanos() as f64) / 10_000_000_000.0
}
