use std::thread;
use std::time::Duration;

mod display;

fn main() {

    let display = display::Display::new();
    display.update("Hi there");
    display.update("Hi there");
    display.close();

    thread::sleep(Duration::from_secs(1));
}
