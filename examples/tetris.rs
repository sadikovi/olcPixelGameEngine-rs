extern crate olc_pixel_game_engine;

use crate::olc_pixel_game_engine as olc;

const SCREEN_W: i32 = 180;
const SCREEN_H: i32 = 180;
const ORIGIN_X: i32 = 10;
const ORIGIN_Y: i32 = 10;
const STEP: i32 = 4;

const GAME_W: i32 = 12;
const GAME_H: i32 = 18;

const TICK: f32 = 10.0;

const TETROMINO: [[i32; 16]; 7] = [
  [
    0, 0, 0, 0,
    0, 1, 1, 0,
    0, 1, 1, 0,
    0, 0, 0, 0
  ],
  [
    0, 2, 0, 0,
    0, 2, 0, 0,
    0, 2, 0, 0,
    0, 2, 0, 0
  ],
  [
    0, 0, 0, 0,
    0, 3, 3, 0,
    0, 0, 3, 3,
    0, 0, 0, 0
  ],
  [
    0, 0, 0, 0,
    0, 4, 4, 0,
    4, 4, 0, 0,
    0, 0, 0, 0
  ],
  [
    0, 0, 5, 0,
    0, 5, 5, 0,
    0, 0, 5, 0,
    0, 0, 0, 0
  ],
  [
    0, 6, 0, 0,
    0, 6, 0, 0,
    0, 6, 6, 0,
    0, 0, 0, 0
  ],
  [
    0, 0, 7, 0,
    0, 0, 7, 0,
    0, 7, 7, 0,
    0, 0, 0, 0
  ]
];

// returns index with rotation included
fn idx(x: i32, y: i32, rot: i32) -> usize {
  match rot % 4 {
    0 => (y * 4 + x) as usize, // 0 degrees
    1 => (12 + y - x * 4) as usize, // 90 degrees
    2 => (15 - y * 4 - x) as usize, // 180 degrees
    3 => (3 - y + x * 4) as usize, // 270 degrees
    _ => panic!("invalid rotation!"),
  }
}

fn colour(num: i32) -> olc::Pixel {
  match num {
    1 => olc::RED,
    2 => olc::YELLOW,
    3 => olc::GREEN,
    4 => olc::BLUE,
    5 => olc::MAGENTA,
    6 => olc::CYAN,
    7 => olc::WHITE,
    _ => olc::GREY
  }
}

struct Tetris {
  board: Vec<i32>,
  tick: f32,
  num_ticks: i32,
  num_pieces: i32,
  game_level: i32,
  score: i32,
  can_rotate: bool,
  game_over: bool,
  piece: usize,
  x: i32,
  y: i32,
  r: i32,
  next_piece: usize,
  next_r: i32,
}

impl Tetris {
  fn new() -> Self {
    Self {
      board: Vec::new(),
      tick: 0.0,
      num_ticks: 0,
      num_pieces: 0,
      game_level: 0,
      score: 0,
      can_rotate: true,
      game_over: false,
      piece: 0,
      x: 0,
      y: 0,
      r: 0,
      next_piece: 0,
      next_r: 0,
    }
  }

  fn reset(&mut self) {
    self.board = vec![0; (GAME_W * GAME_H) as usize];
    self.tick = 0.0;
    self.num_ticks = 0;
    self.num_pieces = 0;
    self.game_level = 1;
    self.score = 0;
    self.can_rotate = true;
    self.game_over = false;

    self.next_piece = (olc::c_rand() % 7) as usize;
    self.next_r = olc::c_rand() % 4;
  }

  fn create_new_piece(&mut self) {
    self.piece = self.next_piece;
    self.x = GAME_W / 2;
    self.y = 0;
    self.r = self.next_r;

    self.next_piece = (olc::c_rand() % 7) as usize;
    self.next_r = olc::c_rand() % 4;

    self.num_pieces += 1;
  }

  fn lock_piece(&mut self) {
    for i in 0..4 {
      for j in 0..4 {
        let value = TETROMINO[self.piece][idx(i, j, self.r)];
        if value != 0 {
          self.board[((self.y + j) * GAME_W + self.x + i) as usize] = value;
        }
      }
    }
  }

  fn remove_complete_lines(&mut self) -> i32 {
    let mut lines = Vec::new();

    for y in 0..GAME_H {
      let mut complete = true;
      for x in 0..GAME_W {
        complete = complete && self.board[(y * GAME_W + x) as usize] != 0;
      }
      if complete {
        lines.push(y);
      }
    }

    let num_lines = lines.len() as i32;

    for i in &lines {
      for y in (0..*i).rev() {
        for x in 0..GAME_W {
          self.board[((y + 1) * GAME_W + x) as usize] = self.board[(y * GAME_W + x) as usize];
        }
      }
    }

    num_lines
  }

  fn can_move(&self, x: i32, y: i32, r: i32) -> bool {
    for i in 0..4 {
      for j in 0..4 {
        let value = TETROMINO[self.piece][idx(i, j, r)];
        if value != 0 {
          if x + i < 0 || x + i >= GAME_W || y + j < 0 || y + j >= GAME_H {
            return false;
          }
          if self.board[((y + j) * GAME_W + x + i) as usize] != 0 {
            return false;
          }
        }
      }
    }
    true
  }
}

impl olc::Application for Tetris {
  fn on_user_create(&mut self) -> Result<(), olc::Error> {
    self.reset();
    self.create_new_piece();
    Ok(())
  }

  fn on_user_update(&mut self, elapsed_time: f32) -> Result<(), olc::Error> {
    // Timing =======================

    self.tick += TICK * elapsed_time;
    let is_tick = self.tick >= 1.0;
    if is_tick {
      self.tick -= 1.0;
      self.num_ticks += 1;
    }

    // Input ========================

    self.can_rotate = self.can_rotate || olc::get_key(olc::Key::SPACE).released;
    if is_tick && !self.game_over {
      if olc::get_key(olc::Key::UP).held && self.can_move(self.x, self.y - 1, self.r) { self.y -= 1; }
      if olc::get_key(olc::Key::DOWN).held && self.can_move(self.x, self.y + 1, self.r) { self.y += 1; }
      if olc::get_key(olc::Key::LEFT).held && self.can_move(self.x - 1, self.y, self.r) { self.x -= 1; }
      if olc::get_key(olc::Key::RIGHT).held && self.can_move(self.x + 1, self.y, self.r) { self.x += 1; }
      if olc::get_key(olc::Key::SPACE).held && self.can_rotate && self.can_move(self.x, self.y, self.r + 1) {
        self.can_rotate = false;
        self.r += 1;
      }
    }

    // Game Logic ===================

    self.game_over = !self.can_move(self.x, self.y, self.r);

    if is_tick && !self.game_over && self.num_ticks % (11 - self.game_level) == 0 {
      // Game updates
      if self.can_move(self.x, self.y + 1, self.r) {
        self.y += 1;
      } else {
        self.lock_piece();
        let num_lines = self.remove_complete_lines();

        if num_lines > 0 {
          self.score += (1 << (num_lines - 1)) * 100;
        }

        self.create_new_piece();

        self.tick = 0.0;
        if self.num_pieces % 10 == 0 {
          self.game_level += 1;
          if self.game_level > 10 {
            self.game_level = 10; // So we don't overflow the modulo above
          }
        }
      }
    }

    // Display ======================

    olc::clear(olc::BLACK);

    for x in 0..GAME_W {
      for y in 0..GAME_H {
        let col = colour(self.board[(y * GAME_W + x) as usize]);
        olc::fill_rect(ORIGIN_X + x * STEP, ORIGIN_Y + y * STEP, STEP, STEP, col);
      }
    }

    for x in 0..4 {
      for y in 0..4 {
        let value = TETROMINO[self.piece][idx(x, y, self.r)];
        if value != 0 {
          let col = colour(value);
          olc::fill_rect(ORIGIN_X + (self.x + x) * STEP, ORIGIN_Y + (self.y + y) * STEP, STEP, STEP, col);
        }
      }
    }

    olc::draw_string(ORIGIN_X + GAME_W * STEP + 5, ORIGIN_Y, &format!("Level: {}", self.game_level), olc::WHITE)?;
    olc::draw_string(ORIGIN_X + GAME_W * STEP + 5, ORIGIN_Y + 15, &format!("Score: {}", self.score), olc::WHITE)?;
    olc::draw_string(ORIGIN_X + GAME_W * STEP + 5, ORIGIN_Y + 30, &format!("Count: {}", self.num_pieces), olc::WHITE)?;
    olc::draw_string(ORIGIN_X + GAME_W * STEP + 5, ORIGIN_Y + 45, &"Next:", olc::WHITE)?;
    for i in 0..4 {
      for j in 0..4 {
        let value = TETROMINO[self.next_piece][idx(i, j, self.next_r)];
        if value != 0 {
          let col = colour(value);
          olc::fill_rect(ORIGIN_X + GAME_W * STEP + 10 + i * STEP, ORIGIN_Y + 55 + j * STEP, STEP, STEP, col);
        }
      }
    }
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
  let mut app = Tetris::new();
  olc::start("Tetris", &mut app, SCREEN_W, SCREEN_H, STEP, STEP).unwrap();
}
