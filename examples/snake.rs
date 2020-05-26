extern crate olc_pixel_game_engine;

use std::collections::LinkedList;
use crate::olc_pixel_game_engine as olc;

const ORIGIN_X: i32 = 10;
const ORIGIN_Y: i32 = 10;
const SCREEN_W: i32 = 120;
const SCREEN_H: i32 = 180;

const GAME_W: i32 = 12;
const GAME_H: i32 = 18;
const STEP: i32 = 4;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
  UP,
  DOWN,
  LEFT,
  RIGHT
}

struct SnakeGame {
  tick: f32,
  speed: f32,
  direction: Direction,
  snake: LinkedList<olc::Vi2d>,
  point: olc::Vi2d,
  game_over: bool,
}

impl SnakeGame {
  fn new() -> Self {
    Self {
      tick: 0.0,
      speed: 0.0,
      direction: Direction::RIGHT,
      snake: LinkedList::new(),
      point: olc::Vi2d::new(0, 0),
      game_over: false,
    }
  }

  fn can_move(&self, dx: i32, dy: i32) -> bool {
    match self.snake.front() {
      Some(pos) => {
        if pos.x + dx < 0 || pos.x + dx >= GAME_W || pos.y + dy < 0 || pos.y + dy >= GAME_H {
          return false;
        }

        let mut iter = self.snake.iter();
        iter.next(); // drop the first one

        for n in iter {
          if n.x == pos.x + dx && n.y == pos.y + dy {
            return false
          }
        }

        true
      },
      None => false
    }
  }

  fn move_snake(&mut self, dx: i32, dy: i32) {
    if self.snake.len() > 1 {
      let head = self.snake.front().unwrap();
      let new_x = head.x + dx;
      let new_y = head.y + dy;

      let mut tail = self.snake.pop_back().unwrap();
      tail.x = new_x;
      tail.y = new_y;
      self.snake.push_front(tail);
    } else if self.snake.len() == 1 {
      let head = self.snake.front_mut().unwrap();
      head.x += dx;
      head.y += dy;
    }
  }
}

impl olc::Application for SnakeGame {
  fn on_user_create(&mut self) -> Result<(), olc::Error> {
    self.tick = 0.0;
    self.speed = 3.0;
    self.direction = Direction::RIGHT;
    self.snake = LinkedList::new();

    self.snake.push_front(olc::Vi2d::new(0, 0));

    self.point = olc::Vi2d::new(olc::c_rand() % GAME_W, olc::c_rand() % GAME_H);
    self.game_over = false;

    Ok(())
  }

  fn on_user_update(&mut self, elapsed_time: f32) -> Result<(), olc::Error> {
    // Timing =======================

    self.tick += self.speed * elapsed_time;
    let can_run = self.tick >= 1.0;
    if can_run {
      self.tick -= 1.0;
    }

    // Input ========================

    if olc::get_key(olc::Key::LEFT).held && self.direction != Direction::RIGHT { self.direction = Direction::LEFT; }
    if olc::get_key(olc::Key::RIGHT).held && self.direction != Direction::LEFT { self.direction = Direction::RIGHT; }
    if olc::get_key(olc::Key::UP).held && self.direction != Direction::DOWN { self.direction = Direction::UP; }
    if olc::get_key(olc::Key::DOWN).held && self.direction != Direction::UP { self.direction = Direction::DOWN; }

    // Game Logic ===================

    if can_run && !self.game_over {
      let (dx, dy) = if self.direction == Direction::LEFT {
        (-1, 0)
      } else if self.direction == Direction::RIGHT {
        (1, 0)
      } else if self.direction == Direction::UP {
        (0, -1)
      } else if self.direction == Direction::DOWN {
        (0, 1)
      } else {
        (0, 0)
      };

      self.game_over = !self.can_move(dx, dy);
      if !self.game_over { self.move_snake(dx, dy); }

      if let Some(pos) = self.snake.front() {
        if pos.x == self.point.x && pos.y == self.point.y {
          // Eat the point and generate a new one
          self.snake.push_back(olc::Vi2d::new(self.point.x, self.point.y));
          self.point = olc::Vi2d::new(olc::c_rand() % GAME_W, olc::c_rand() % GAME_H);

          // Increase the difficulty/speed
          if self.snake.len() % 4 == 0 {
            self.speed += 1.0;
          }
        }
      }
    }

    // Display ======================

    olc::clear(olc::BLACK);
    olc::fill_rect(ORIGIN_X, ORIGIN_Y, GAME_W * STEP, GAME_H * STEP, olc::GREY);

    olc::fill_rect(ORIGIN_X + self.point.x * STEP, ORIGIN_Y + self.point.y * STEP, STEP, STEP, olc::WHITE);

    for pos in &self.snake {
      olc::fill_rect(ORIGIN_X + pos.x * STEP, ORIGIN_Y + pos.y * STEP, STEP, STEP, olc::DARK_GREY);
    }

    olc::draw_string(ORIGIN_X + GAME_W * STEP + 4, ORIGIN_Y, &"Speed:", olc::WHITE)?;
    olc::draw_string(ORIGIN_X + GAME_W * STEP + 4, ORIGIN_Y + 10, &format!("{}", self.speed), olc::WHITE)?;

    olc::draw_string(ORIGIN_X + GAME_W * STEP + 4, ORIGIN_Y + 30, &"Length:", olc::WHITE)?;
    olc::draw_string(ORIGIN_X + GAME_W * STEP + 4, ORIGIN_Y + 40, &format!("{}", self.snake.len()), olc::WHITE)?;

    if self.game_over {
      olc::draw_string(ORIGIN_X + 10, ORIGIN_Y + GAME_H * STEP + 10, &"GAME OVER!", olc::WHITE)?;
    }

    Ok(())
  }

  fn on_user_destroy(&mut self) -> Result<(), olc::Error> {
    Ok(())
  }
}

fn main() {
  let mut app = SnakeGame::new();
  olc::start("Snake", &mut app, SCREEN_W, SCREEN_H, STEP, STEP).unwrap();
}
