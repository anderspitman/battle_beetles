#[macro_use]
extern crate serde_derive;

mod display;
mod simulation;

use std::thread;
use std::time::Duration;


fn main() {

    let display = display::Display::new();
    let beetle = simulation::Beetle::new(10.0, 10.0);
    display.update(&beetle);
    display.close();

    simulation::test_print(&beetle);

    thread::sleep(Duration::from_secs(1));
}
