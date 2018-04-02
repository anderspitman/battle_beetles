extern crate websocket;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate cgmath;
extern crate rand;

//mod FieldState;
mod ui;
mod simulation;
mod beetle;
use beetle::BeetleBuilder;

use std::thread;
use std::time::Duration;
use cgmath::Rad;

const SIMULATION_PERIOD_MS: u64 = 16;
const MS_PER_SECOND: f32 = 1000.0;


fn main() {
    
    let ui = ui::UI::new();

    let mut sim = simulation::Simulation::new();

    const SPEED_PIXELS_PER_SECOND: f32 = 100.0;
    let converted_speed =
        convert_value_for_sim_period(SPEED_PIXELS_PER_SECOND);

    println!("converted_speed: {}", converted_speed);

    const ROTATION_RADIANS_PER_SECOND: f32 = 3.14159;
    let converted_rotation =
        convert_value_for_sim_period(ROTATION_RADIANS_PER_SECOND);

    println!("converted_rotation: {}", converted_rotation);

    let mut beetle = BeetleBuilder::new()
        .speed_pixels_per_tick(converted_speed)
        .rotation_radians_per_tick(Rad(converted_rotation))
        .x_pos(10.0)
        .y_pos(130.0)
        .build();
    sim.add_beetle(beetle);

    beetle = BeetleBuilder::new()
        .speed_pixels_per_tick(converted_speed)
        .rotation_radians_per_tick(Rad(converted_rotation))
        .x_pos(100.0)
        .y_pos(200.0)
        .build();
    sim.add_beetle(beetle);
    sim.add_food(100.0, 100.0);

    sim.select_beetle(0);
    //sim.select_beetle(1);

    sim.selected_move_command(200.0, 300.0);

    ui.update(&sim.field_state);

    let mut done = false;
    while !done {
        ui.update(sim.tick());
        let messages = ui.get_all_messages();

        for message in messages {

            println!("{:?}", message);
            match message.message_type.as_ref() {
                "terminate" => {
                    done = true;
                    break;
                },
                "select-beetle" => {
                    println!("selected beetle {}", message.beetle_id);
                    sim.select_beetle(message.beetle_id);
                },
                "selected-move-command" => {
                    sim.selected_move_command(message.x, message.y);
                },
                "selected-interact-command" => {
                    sim.selected_interact_command(message.beetle_id);
                },
                "deselect-all-beetles" => {
                    sim.deselect_all_beetles();
                },
                message_type => {
                    println!("Invalid message_type: {}", message_type);
                },
            }
        }
        thread::sleep(Duration::from_millis(SIMULATION_PERIOD_MS));
    }

    ui.shutdown();
}

fn convert_value_for_sim_period(value: f32) -> f32 {
    return value * ((SIMULATION_PERIOD_MS as f32) / MS_PER_SECOND);
}
