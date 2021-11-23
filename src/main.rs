#[macro_use]
extern crate stdweb;

use stdweb::traits::*;
use stdweb::web::{event::KeyDownEvent, IEventTarget, document};

use std::cell::RefCell;
use std::rc::Rc;

mod canvas;
mod direction;
mod snake;

use canvas::Canvas;
use direction::Direction;
use snake::Snake;

const TIME_PER_FRAME: u32 = 100;
const SCALE_FACTOR: (u32, u32) = (30, 30);

fn main() {
    stdweb::initialize();

    let canvas = Canvas::new("#canvas", SCALE_FACTOR.0, SCALE_FACTOR.1);
    let snake = Rc::new(RefCell::new(
        Snake::new(SCALE_FACTOR.0, SCALE_FACTOR.1)
    ));

    // draw the snake
    snake.borrow().draw(&canvas);

    // add event listener
    document().add_event_listener({
        let snake = snake.clone();
        move |event: KeyDownEvent| {
            match event.key().as_ref() {
                "ArrowLeft" => snake.borrow_mut().change_direction(Direction::Left),
                "ArrowRight" => snake.borrow_mut().change_direction(Direction::Right),
                "ArrowUp" => snake.borrow_mut().change_direction(Direction::Up),
                "ArrowDown" => snake.borrow_mut().change_direction(Direction::Down),
                _ => {
                    js! { console.log("unknown key"); }
                },
            };
        }
    });

    // define game loop as an inline function
    fn game_loop(snake: Rc<RefCell<Snake>>, canvas: Rc<Canvas>, time: u32) {
        stdweb::web::set_timeout(move || {
            game_loop(snake.clone(), canvas.clone(), time);
            snake.borrow_mut().update();
            snake.borrow().draw(&canvas);
        }, time);
    }

    game_loop(snake, Rc::new(canvas), TIME_PER_FRAME);

    stdweb::event_loop();
}
