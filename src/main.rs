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
use game::{Game, FieldState};
use message_handler::MessageHandler;

use simulation::GeneticAlgorithm;
use simulation::speed_ga::SpeedGA;
use simulation::battle_ga::BattleGA;
use simulation::Simulate;
use simulation::fight_simulation::FightSimulation;

use std::thread;
use std::time::{Instant, Duration};
use rouille::Response;
use rand::{Rng, thread_rng};
use cgmath::{Vector2};


fn main() {

    start_web_server_thread();

    let ui = ui::UI::new();
    let mut game = Game::new();

    let max_speed =
        utils::convert_value_for_sim_period(utils::MAX_SPEED_UNITS_PER_SECOND);

    let max_rotation =
        utils::convert_value_for_sim_period(utils::ROTATION_RADIANS_PER_SECOND);


    let mut rng = thread_rng();

    //// run battle GA
    //game.set_random_population(
    //        utils::POPULATION_SIZE, max_speed, max_rotation);

    //let mut battle_population;
    //{
    //    let mut ga = BattleGA::new(&mut game, &ui);
    //    ga.run();
    //    battle_population = ga.get_population().clone();
    //}
    //for (_, beetle) in battle_population.iter_mut() {
    //    let rand_x: f32 = rng.gen_range(0.0, 500.0);
    //    let rand_y: f32 = rng.gen_range(0.0, 500.0);
    //    beetle.position.x = rand_x;
    //    beetle.position.y = rand_y;
    //    beetle.team_id = 0;
    //    beetle.direction = Vector2::new(-1.0, 0.0);
    //}

    // run speed GA
    game.set_random_population(
            utils::POPULATION_SIZE, max_speed, max_rotation);
    let mut speed_population;
    {
        let mut ga = SpeedGA::new(&mut game, &ui);
        ga.run();
        speed_population = ga.get_population().clone();
    }
    for (_, beetle) in speed_population.iter_mut() {
        let rand_x: f32 = rng.gen_range(600.0, 1100.0);
        let rand_y: f32 = rng.gen_range(0.0, 500.0);
        beetle.position.x = rand_x;
        beetle.position.y = rand_y;
        beetle.team_id = 1;
        beetle.direction = Vector2::new(1.0, 0.0);
    }

    // reset population
    game.field_state.beetles = Beetles::new();

    // TODO: could potentially use itertools chain method to do this, but I
    // don't want an extra dependency just for that right now.
    //for (_, beetle) in battle_population.into_iter() {
    //    game.add_beetle(beetle);
    //}
    for (_, beetle) in speed_population.into_iter() {
        game.add_beetle(beetle);
    }

    println!("All fight");

    //for (_, beetle) in game.field_state.beetles.iter() {
    //    println!("id: {:?}", beetle.id);
    //    println!("team_id: {:?}", beetle.team_id);
    //    println!("dir: {:?}", beetle.direction);
    //    println!("pos: {:?}", beetle.position);

    //    game.select_beetle(beetle.id);
    //}

    ui.update_game_state(game.tick());

    {
        let check_done_callback = |state: &FieldState| {
            // whatever the first beetle's team is, make sure there are no
            // enemies left.
            if let Some(beetle) = state.beetles.iter().next() {
                let team_id = beetle.1.team_id;

                for (_, other) in &state.beetles {
                    if other.team_id != team_id {
                        return false;
                    }
                }

                true
            }
            else {
                true
            }
        };
        let mut sim = FightSimulation::new(&mut game, check_done_callback);
        sim.set_tick_callback(|state| {
            ui.update_game_state(&state);
            //println!("{:?}", ui);
            thread::sleep(Duration::from_millis(utils::SIMULATION_PERIOD_MS));
        });
        sim.run();
    }

    //println!("End simulation");

    // move all to center
    //let ids: Vec<i32> = game.field_state.beetles.iter().map(|(_, b)| b.id).collect();
    //for id in ids {
    //    game.select_beetle(id);
    //    game.selected_move_command(500.0, 250.0);
    //    game.deselect_all_beetles();
    //}

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
