extern crate websocket;
extern crate serde_json;
extern crate protobuf;

#[macro_use]
extern crate serde_derive;

//mod FieldState;
mod ui;
mod simulation;

use std::thread;
use std::time::Duration;


fn main() {
    
    //let mut person = FieldState::Person::new();
    //person.set_name("Old Gregg".to_string());
    //println!("{:?}", person);

    let ui = ui::UI::new();

    let mut sim = simulation::Simulation::new();

    sim.tick();
    sim.tick();
    sim.tick();
    sim.tick();
    sim.tick();
    sim.tick();
    let state = sim.tick();
    ui.update(&state);
    //ui.update(&person);
    ui.shutdown();

    thread::sleep(Duration::from_secs(1));
}
