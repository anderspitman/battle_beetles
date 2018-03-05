#[macro_use]
extern crate serde_derive;

mod ui;
mod simulation;

use std::thread;
use std::time::Duration;


fn main() {

    let ui = ui::UI::new();
    let beetle = simulation::Beetle::new(10.0, 10.0);
    ui.update(&beetle);
    ui.close();

    simulation::test_print(&beetle);

    thread::sleep(Duration::from_secs(1));
}
