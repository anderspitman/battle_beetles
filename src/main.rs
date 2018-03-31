extern crate websocket;
extern crate serde_json;
extern crate protobuf;
#[macro_use]
extern crate serde_derive;
extern crate cgmath;
extern crate rand;

//mod FieldState;
mod ui;
mod simulation;
use simulation::{Beetle, BeetleBuilder};

use std::thread;
use std::time::Duration;


fn main() {
    
    let ui = ui::UI::new();

    let mut sim = simulation::Simulation::new();

    //while !sim.done() {
    ////for i in 0..100 {
    //    ui.update(sim.tick());
    //    thread::sleep(Duration::from_millis(10));
    //}

    let beetle = BeetleBuilder::new()
        .x_pos(10.0)
        .y_pos(130.0)
        .build();
    sim.add_beetle(beetle);
    sim.add_beetle(Beetle::new());
    sim.add_food(100.0, 100.0);

    sim.select_beetle(0);
    //sim.select_beetle(1);

    sim.selected_move_command(100.0, 100.0);

    ui.update(&sim.field_state);

    for i in 0..200 {
        ui.update(sim.tick());
        thread::sleep(Duration::from_millis(10));
    }

    sim.select_beetle(1);
    sim.selected_move_command(100.0, 200.0);

    for i in 0..100 {
        ui.update(sim.tick());
        thread::sleep(Duration::from_millis(10));
    }

    //thread::sleep(Duration::from_millis(100));
    
    sim.deselect_all_beetles();

    ui.shutdown();
}
