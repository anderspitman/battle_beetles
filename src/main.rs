extern crate websocket;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate cgmath;
extern crate rand;
extern crate protobuf;
#[macro_use]
extern crate rouille;

//mod FieldState;
mod ui;
mod game;
mod beetle;
mod gen;

use beetle::BeetleBuilder;

use std::thread;
use std::time::Duration;
use cgmath::Rad;
use rouille::Response;

const SIMULATION_PERIOD_MS: u64 = 10;
const MS_PER_SECOND: f32 = 1000.0;
const SPEED_PIXELS_PER_SECOND: f32 = 100.0;
const ROTATION_RADIANS_PER_SECOND: f32 = 3.14159;


fn main() {

    run_web_server();

    let ui = ui::UI::new();

    let mut game = game::Game::new();

    let converted_speed =
        convert_value_for_sim_period(SPEED_PIXELS_PER_SECOND);

    let converted_rotation =
        convert_value_for_sim_period(ROTATION_RADIANS_PER_SECOND);

    let mut beetle = BeetleBuilder::new()
        .speed_pixels_per_tick(converted_speed)
        .rotation_radians_per_tick(Rad(converted_rotation))
        .x_pos(10.0)
        .y_pos(130.0)
        .build();
    game.add_beetle(beetle);

    beetle = BeetleBuilder::new()
        .speed_pixels_per_tick(converted_speed)
        .rotation_radians_per_tick(Rad(converted_rotation))
        .x_pos(100.0)
        .y_pos(200.0)
        .build();
    game.add_beetle(beetle);
    game.add_food(100.0, 100.0);

    game.select_beetle(1);

    game.selected_move_command(200.0, 300.0);

    ui.update(&game.field_state);

    let mut done = false;
    while !done {
        ui.update(game.tick());
        let messages = ui.get_all_messages();

        for message in messages {

            if message.has_select_beetle() {
                game.select_beetle(message.get_select_beetle().get_beetle_id());
            }
            else if message.has_selected_move_command() {
                game.selected_move_command(
                    message.get_selected_move_command().get_x(),
                    message.get_selected_move_command().get_y());
            }
            else if message.has_deselect_all_beetles() {
                game.deselect_all_beetles();
            }
            else if message.has_create_beetle() {
                let beetle = BeetleBuilder::new()
                    .speed_pixels_per_tick(converted_speed)
                    .rotation_radians_per_tick(Rad(converted_rotation))
                    .x_pos(message.get_create_beetle().get_x())
                    .y_pos(message.get_create_beetle().get_y())
                    .build();
                game.add_beetle(beetle);
            }
            else if message.has_selected_interact_command() {
                game.selected_interact_command(
                    message.get_selected_interact_command().get_beetle_id());
            }
            else if message.has_terminate() {
                done = true;
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
        rouille::start_server("0.0.0.0:8000", move |request| {

            let response = router!(request,
                (GET) ["/"] => {
                    Response::html(index)
                },
                (GET) ["/bundle.js"] => {
                    Response::text(bundle)
                },
                _ => {
                    Response::empty_404()
                }
            );

            response
        });
    });
}

fn convert_value_for_sim_period(value: f32) -> f32 {
    return value * ((SIMULATION_PERIOD_MS as f32) / MS_PER_SECOND);
}
