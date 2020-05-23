extern crate olc_pixel_game_engine;

use crate::olc_pixel_game_engine as olc;

const ORIGIN_X: i32 = 10;
const ORIGIN_Y: i32 = 10;
const WIDTH: i32 = 15;
const HEIGHT: i32 = 16;
const BLOCK_SIZE: i32 = 8;
const LEVEL: &str = "\
###############\
#.............#\
#.............#\
#..111222111..#\
#..111333111..#\
#..11.....11..#\
#.....333.....#\
#.............#\
#.............#\
#.............#\
#.............#\
#.............#\
#.............#\
#.............#\
#.............#\
#.............#\
";
const BALL_RADIUS: i32 = 2;
const BAT_WIDTH: i32 = 14;
const BAT_HEIGHT: i32 = 2;
const START_ANGLES: &[f32] = &[2.4, 2.6, 0.4, 0.6];

struct Breakout {
  level: Vec<char>,
  ball_x: f32,
  ball_y: f32,
  ball_dx: f32,
  ball_dy: f32,
  ball_speed: f32,
  bat_x: f32,
  bat_y: f32,
  bat_speed: f32,
  score: i32,
  num_blocks_left: i32,
  game_over: bool
}

impl Breakout {
  fn initialise(&mut self) {
    self.num_blocks_left = 0;
    self.score = 0;
    self.game_over = false;

    self.level.clear();
    for c in LEVEL.chars() {
      self.level.push(c);
      if c != '#' && c != '.' {
        // Then it must be a block, increment the counter.
        self.num_blocks_left += 1;
      }
    }

    // Initialise the ball.
    let ball_coords = world2screen(WIDTH/2, HEIGHT/2);
    self.ball_x = ball_coords.x as f32;
    self.ball_y = ball_coords.y as f32;

    let angle: f32 = START_ANGLES[olc::c_rand() as usize % START_ANGLES.len()];
    self.ball_dx = angle.cos();
    self.ball_dy = angle.sin();

    self.ball_speed = 50.0;

    // Initialise the bat.
    let bat_coords = world2screen(WIDTH/2, HEIGHT);
    self.bat_x = bat_coords.x as f32;
    self.bat_y = bat_coords.y as f32;

    self.bat_speed = 50.0;
  }

  fn stop(&mut self) {
    self.bat_speed = 0.0;
    self.ball_speed = 0.0;
  }
}

impl olc::Application for Breakout {
  fn on_user_create(&mut self) -> Result<(), olc::Error> {
    self.initialise();
    Ok(())
  }

  fn on_user_update(&mut self, elapsed_time: f32) -> Result<(), olc::Error> {
    // Handle win or lose
    if self.game_over {
      self.stop();

      let msg = if self.num_blocks_left == 0 {
        "You Won!"
      } else {
        "Game Over"
      };

      // Display a message.
      olc::draw_string(
        ORIGIN_X + WIDTH * BLOCK_SIZE / 2 - 34,
        ORIGIN_Y + HEIGHT * BLOCK_SIZE / 2 + 10,
        msg,
        olc::WHITE
      )?;

      if olc::get_key(olc::Key::SPACE).released {
        self.initialise();
      }

      return Ok(());
    }

    olc::clear(olc::BLACK);

    // Draw the level.
    for y in 0..HEIGHT {
      for x in 0..WIDTH {
        let screen_coords = world2screen(x, y);
        match self.level[(y * WIDTH + x) as usize] {
          '#' => {
            olc::fill_rect(screen_coords.x, screen_coords.y, BLOCK_SIZE, BLOCK_SIZE, olc::GREY);
          },
          '1' => {
            olc::fill_rect(screen_coords.x, screen_coords.y, BLOCK_SIZE, BLOCK_SIZE, olc::GREEN);
          },
          '2' => {
            olc::fill_rect(screen_coords.x, screen_coords.y, BLOCK_SIZE, BLOCK_SIZE, olc::MAGENTA);
          },
          '3' => {
            olc::fill_rect(screen_coords.x, screen_coords.y, BLOCK_SIZE, BLOCK_SIZE, olc::RED);
          },
          _ => { }
        }
      }
    }

    // Compute bat position.
    let old_bat_x = self.bat_x;

    if olc::get_key(olc::Key::LEFT).held {
      self.bat_x -= self.bat_speed * elapsed_time;
    } else if olc::get_key(olc::Key::RIGHT).held {
      self.bat_x += self.bat_speed * elapsed_time;
    }

    let bat_x_left = (self.bat_x as i32 - ORIGIN_X) / BLOCK_SIZE;
    let bat_x_right = (self.bat_x as i32 + BAT_WIDTH - ORIGIN_X) / BLOCK_SIZE;
    if bat_x_left <= 0 || bat_x_right >= WIDTH - 1 {
      self.bat_x = old_bat_x;
    }

    // Compute ball position.
    let old_ball_x = self.ball_x;
    let old_ball_y = self.ball_y;

    self.ball_x += self.ball_dx * self.ball_speed * elapsed_time;
    self.ball_y += self.ball_dy * self.ball_speed * elapsed_time;

    let old_c = ball_coordinates(old_ball_x, self.ball_dx, old_ball_y, self.ball_dy);
    let new_c = ball_coordinates(self.ball_x, self.ball_dx, self.ball_y, self.ball_dy);
    let idx = (new_c.y * WIDTH + new_c.x) as usize;

    // Compute collision.
    let is_collision =
      new_c.x >= bat_x_left && new_c.x <= bat_x_right && new_c.y == HEIGHT ||
      idx < self.level.len() && (
        self.level[idx] == '#' ||
        self.level[idx] == '1' ||
        self.level[idx] == '2' ||
        self.level[idx] == '3'
      );

    if is_collision {
      if new_c.x != old_c.x {
        self.ball_dx *= -1.0;
      } else {
        self.ball_dy *= -1.0;
      }

      // Handle block removals.
      if idx < self.level.len() {
        match self.level[idx] {
          '1' => {
            self.level[idx] = '.';
            self.score += 100;
            self.num_blocks_left -= 1;
          },
          '2' => {
            self.level[idx] = '1';
            self.score += 200;
          },
          '3' => {
            self.level[idx] = '2';
            self.score += 300;
          },
          _ => { }
        }
      }

      // You have won!
      self.game_over = self.num_blocks_left == 0;
    } else if idx >= self.level.len() {
      // Game over!
      self.game_over = true;
    }

    // Draw the ball.
    olc::fill_circle(self.ball_x as i32, self.ball_y as i32, BALL_RADIUS, olc::YELLOW);

    // Draw the bat.
    olc::fill_rect(self.bat_x as i32, self.bat_y as i32, BAT_WIDTH, BAT_HEIGHT, olc::WHITE);

    // Print the score.
    olc::draw_string(
      ORIGIN_X,
      ORIGIN_Y + HEIGHT * BLOCK_SIZE + 10,
      &format!("Score: {}", self.score),
      olc::WHITE
    )?;

    Ok(())
  }

  fn on_user_destroy(&mut self) -> Result<(), olc::Error> {
    Ok(())
  }
}

// Converts world coordinates into screen coordinates.
fn world2screen(x: i32, y: i32) -> olc::Vi2d {
  let sx = ORIGIN_X + x * BLOCK_SIZE;
  let sy = ORIGIN_Y + y * BLOCK_SIZE;
  olc::Vi2d::new(sx, sy)
}

// Converts screen coordinates into world coordinates.
fn screen2world(x: i32, y: i32) -> olc::Vi2d {
  let wx = (x - ORIGIN_X) / BLOCK_SIZE;
  let wy = (y - ORIGIN_Y) / BLOCK_SIZE;
  olc::Vi2d::new(wx, wy)
}

fn ball_coordinates(mut x: f32, dx: f32, mut y: f32, dy: f32) -> olc::Vi2d {
  if dx > 0.0 {
    x += BALL_RADIUS as f32;
  } else if dx < 0.0 {
    x -= BALL_RADIUS as f32;
  }

  if dy > 0.0 {
    y += BALL_RADIUS as f32;
  } else if dy < 0.0 {
    y -= BALL_RADIUS as f32;
  }

  screen2world(x as i32, y as i32)
}

fn main() {
  let mut app = Breakout {
    level: Vec::new(),
    ball_x: 0.0,
    ball_y: 0.0,
    ball_dx: 0.0,
    ball_dy: 0.0,
    ball_speed: 0.0,
    bat_x: 0.0,
    bat_y: 0.0,
    bat_speed: 0.0,
    score: 0,
    num_blocks_left: 0,
    game_over: false
  };
  olc::start("Breakout", &mut app, 140, 180, 4, 4).unwrap();
}
