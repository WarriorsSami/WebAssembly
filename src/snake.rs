use crate::canvas::Canvas;
use stdweb::unstable::TryInto;
use crate::direction::Direction;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Block(u32, u32);

impl Block {
    pub fn generate_block(width: u32, height: u32) -> Block {
        let block_x = js! {
            return Math.floor(Math.random() * @{width});
        }.try_into().unwrap();

        let block_y = js! {
            return Math.floor(Math.random() * @{height});
        }.try_into().unwrap();

        Block(block_x, block_y)
    }
}

#[derive(Debug)]
pub struct Snake {
    // snake is a vector of blocks
    head: Block,
    tail: Vec<Block>,
    food: Block,

    // scaling factor
    width: u32,
    height: u32,

    // direction of movement
    current_direction: Option<Direction>,
    next_direction: Option<Direction>,
    last_direction: Direction,
}

impl Snake {
    pub fn new(width: u32, height: u32) -> Snake {
        // create a snake with a head at a random position
        let head = Block::generate_block(width, height);

        // create some food at a random position
        let food = Block::generate_block(width, height);

        // create an empty tail for our snake
        let tail = vec![];
        
        Snake {
            head,
            tail,
            food,
            width,
            height,
            current_direction: None,
            next_direction: None,
            last_direction: Direction::Right,
        }
    }

    pub fn change_direction(&mut self, direction: Direction) {
        // if the snake isn't moving in the opposite direction, change current direction
        if !self.last_direction.opposite(direction) && self.current_direction.is_none() {
            self.current_direction = Some(direction);
        } else if self.current_direction.iter().any(|d| !d.opposite(direction)) {
            self.next_direction = Some(direction);
        }
    }

    pub fn update(&mut self) {
        let direction = self.current_direction.unwrap_or(self.last_direction);
        self.last_direction = direction;

        let new_head = match direction {
            Direction::Up => Block(
                (self.head.0) % self.width,
                (self.head.1.checked_sub(1).unwrap_or(self.height - 1)) % self.height
            ),
            Direction::Down => Block((self.head.0) % self.width, (self.head.1 + 1) % self.height),
            Direction::Left => Block(
                (self.head.0.checked_sub(1).unwrap_or(self.width - 1)) % self.width,
                (self.head.1) % self.height
            ),
            Direction::Right => Block((self.head.0 + 1) % self.width, (self.head.1) % self.height),
        };

        self.tail.insert(0, self.head);
        let last_end = self.tail.pop();

        // restart the game if the snake hits itself
        if self.tail.contains(&new_head) {
            *self = Snake::new(self.width, self.height);
        }

        self.head = new_head;
        // if the snake eats food, grow it
        if self.head == self.food {
            let mut food = self.food;
            // keep generating food until it's not on the snake
            while self.head == food || self.tail.contains(&food) {
                food = Block::generate_block(self.width, self.height);
            }
            self.food = food;
            // grow the snake by one block
            last_end.map(|x| self.tail.push(x));
        }

        // update the current direction
        self.current_direction = self.next_direction.take();
    }

    pub fn draw(&self, canvas: &Canvas) {
        canvas.clear_all();
        canvas.draw(self.head.0, self.head.1, "green");

        for &Block(x, y) in &self.tail {
            canvas.draw(x, y, "lightgreen");
        }

        canvas.draw(self.food.0, self.food.1, "red");
    }
}