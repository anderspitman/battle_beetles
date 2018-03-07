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
    
    let ui = ui::UI::new();

    let mut sim = simulation::Simulation::new();

    while !sim.done() {
        ui.update(sim.tick());
        thread::sleep(Duration::from_millis(10));
    }

    ui.shutdown();
}
