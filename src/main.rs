#[macro_use]
extern crate stdweb;

mod canvas;
mod direction;

use canvas::Canvas;

fn main() {
    stdweb::initialize();

    let canvas = Canvas::new("#canvas", 20, 20);

    canvas.draw(5, 5, "blue");
    canvas.draw(12, 6, "red");
    canvas.draw(12, 12, "green");
    canvas.draw(6, 12, "yellow");

    stdweb::event_loop();
}
