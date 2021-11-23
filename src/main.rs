#[macro_use]
extern crate stdweb;

mod canvas;
mod direction;
mod snake;

use canvas::Canvas;

fn main() {
    stdweb::initialize();



    stdweb::event_loop();
}
