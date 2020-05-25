extern crate olc_pixel_game_engine;

use crate::olc_pixel_game_engine as olc;

const ORIGIN_X: i32 = 10;
const ORIGIN_Y: i32 = 10;
const SCREEN_W: i32 = 120;
const SCREEN_H: i32 = 180;

const GAME_W: i32 = 12;
const GAME_H: i32 = 18;
const STEP: i32 = 4;

const SPEED: f32 = 14.0;

const START_SLOWNESS: i32 = 14;

// List of tetromino pieces in the game
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

fn can_move(board: &[i32], p: usize, x: i32, y: i32, rot: i32) -> bool {
  for i in 0..4 {
    for j in 0..4 {
      if TETROMINO[p][idx(i, j, rot)] != 0 {
        if y + j < 0 || y + j >= GAME_H || x + i < 0 || x + i >= GAME_W {
          return false;
        }
        if board[((y + j) * GAME_W + x + i) as usize] != 0 {
          return false;
        }
      }
    }
  }
  true
}

struct Tetris {
  board: Vec<i32>,
  curr: usize, // piece index
  curr_x: i32, // game coordinate x of the current piece
  curr_y: i32, // game coordinate y of the current piece
  curr_rot: i32, // piece rotation
  tick: f32, // game tick
  tick_counter: i32, // number of ticks so far
  can_rotate: bool, // whether or not we can rotate the piece
  is_locked: bool, // whether or not the current piece is locked, so we need to reset it
  complete_lines: Vec<i32>, // complete lines
  game_over: bool,
  score: i32,
  slowness: i32, // difficulty, the smaller number is the more difficult it is to play
  pieces: i32, // number of pieces so far
}

impl Tetris {
  fn new_piece(&mut self) {
    self.curr = (olc::c_rand() % 7) as usize;
    self.curr_x = GAME_W / 2;
    self.curr_y = 0;
    self.curr_rot = olc::c_rand() % 4;
    self.pieces += 1;
    // Increase difficulty with every 20 pieces
    if self.slowness > 1 && self.pieces % 20 == 0 {
      self.slowness -= 1;
    }
  }

  fn restart_the_game(&mut self) {
    self.board = vec![0; (GAME_W * GAME_H) as usize];
    self.tick = 0.0;
    self.tick_counter = 0;
    self.can_rotate = true;
    self.is_locked = false;
    self.complete_lines.clear();
    self.game_over = false;
    self.score = 0;
    self.slowness = START_SLOWNESS;
    self.pieces = 0;

    self.new_piece();
  }
}

impl olc::Application for Tetris {
  fn on_user_create(&mut self) -> Result<(), olc::Error> {
    self.restart_the_game();
    Ok(())
  }

  fn on_user_update(&mut self, elapsed_time: f32) -> Result<(), olc::Error> {
    // Timing =======================

    self.tick += SPEED * elapsed_time;
    let can_run = self.tick >= 1.0;
    if can_run {
      self.tick -= 1.0;
      self.tick_counter += 1;
    }

    // Input ========================

    self.can_rotate = self.can_rotate || olc::get_key(olc::Key::SPACE).released;

    if can_run {
      self.curr_x += if olc::get_key(olc::Key::LEFT).held &&
        can_move(&self.board, self.curr, self.curr_x - 1, self.curr_y, self.curr_rot) { -1 } else { 0 };
      self.curr_x += if olc::get_key(olc::Key::RIGHT).held &&
        can_move(&self.board, self.curr, self.curr_x + 1, self.curr_y, self.curr_rot) { 1 } else { 0 };
      self.curr_y += if olc::get_key(olc::Key::DOWN).held &&
        can_move(&self.board, self.curr, self.curr_x, self.curr_y + 1, self.curr_rot) { 1 } else { 0 };
      if self.can_rotate && olc::get_key(olc::Key::SPACE).held &&
        can_move(&self.board, self.curr, self.curr_x, self.curr_y, self.curr_rot + 1) {
        self.curr_rot += 1;
        self.can_rotate = false;
      }
    }

    // Game Logic ===================

    if can_run && self.tick_counter % self.slowness == 0 {
      self.game_over = !can_move(&self.board, self.curr, self.curr_x, self.curr_y, self.curr_rot);

      if !self.game_over {
        // Move or lock the piece
        if can_move(&self.board, self.curr, self.curr_x, self.curr_y + 1, self.curr_rot) {
          self.curr_y += 1
        } else {
          self.is_locked = true;
          for i in 0..4 {
            for j in 0..4 {
              let value = TETROMINO[self.curr][idx(i, j, self.curr_rot)];
              if value != 0 {
                self.board[((self.curr_y + j) * GAME_W + (self.curr_x + i)) as usize] = value;
              }
            }
          }
        }

        // Check if the piece is locked
        if self.is_locked {
          self.is_locked = false;
          self.new_piece();
        }

        if self.complete_lines.len() > 0 {
          self.score += (1 << self.complete_lines.len() - 1) * 100;
        }

        // Remove lines
        for l in &self.complete_lines {
          for y in (0..*l).rev() {
            for x in 0..GAME_W {
              self.board[((y + 1) * GAME_W + x) as usize] = self.board[(y * GAME_W + x) as usize];
            }
          }
        }
        self.complete_lines.clear();

        // Check the lines
        for y in 0..GAME_H {
          let mut is_complete = true;
          for x in 0..GAME_W {
            if self.board[(y * GAME_W + x) as usize] == 0 {
              is_complete = false;
            }
          }
          if is_complete {
            self.complete_lines.push(y);
          }
        }
      }
    }

    if self.game_over && olc::get_key(olc::Key::SPACE).released {
      self.restart_the_game();
    }

    // Display ======================

    olc::clear(olc::BLACK);

    // Draw the board state
    for x in 0..GAME_W {
      for y in 0..GAME_H {
        let col = colour(self.board[(y * GAME_W + x) as usize]);
        olc::fill_rect(ORIGIN_X + x * STEP, ORIGIN_Y + y * STEP, STEP, STEP, col);
      }
    }

    // Draw the current piece
    for x in 0..4 {
      for y in 0..4 {
        if TETROMINO[self.curr][idx(x, y, self.curr_rot)] != 0 {
          let col = colour(TETROMINO[self.curr][idx(x, y, self.curr_rot)]);
          olc::fill_rect(ORIGIN_X + (self.curr_x + x) * STEP, ORIGIN_Y + (self.curr_y + y) * STEP, STEP, STEP, col);
        }
      }
    }

    // Draw animation of complete lines
    for y in &self.complete_lines {
      for x in 0..GAME_W {
        let col = if self.tick_counter % 2 == 0 {
          olc::MAGENTA
        } else {
          olc::CYAN
        };
        olc::fill_rect(ORIGIN_X + x * STEP, ORIGIN_Y + y * STEP, STEP, STEP, col);
      }
    }

    // Display the score
    olc::draw_string(ORIGIN_X + GAME_W * STEP + 10, ORIGIN_Y, "Score:", olc::WHITE)?;
    olc::draw_string(ORIGIN_X + GAME_W * STEP + 10, ORIGIN_Y + 10, &format!("{}", self.score), olc::WHITE)?;

    // Display the level
    olc::draw_string(ORIGIN_X + GAME_W * STEP + 10, ORIGIN_Y + 30, "Level:", olc::WHITE)?;
    olc::draw_string(ORIGIN_X + GAME_W * STEP + 10, ORIGIN_Y + 40, &format!("{}", START_SLOWNESS - self.slowness), olc::WHITE)?;

    // Display message when the game is over
    if self.game_over {
      olc::draw_string(ORIGIN_X + 10, ORIGIN_Y + 10 + GAME_H * STEP, "GAME OVER!", olc::WHITE)?;
      olc::draw_string(ORIGIN_X + 10, ORIGIN_Y + 30 + GAME_H * STEP, "Press Space", olc::WHITE)?;
      olc::draw_string(ORIGIN_X + 10, ORIGIN_Y + 40 + GAME_H * STEP, "to restart", olc::WHITE)?;
    }

    Ok(())
  }

  fn on_user_destroy(&mut self) -> Result<(), olc::Error> {
    Ok(())
  }
}

fn main() {
  let mut app = Tetris {
    board: Vec::new(),
    curr: 0,
    curr_x: 0,
    curr_y: 0,
    curr_rot: 0,
    tick: 0.0,
    tick_counter: 0,
    can_rotate: true,
    is_locked: false,
    complete_lines: Vec::new(),
    game_over: false,
    score: 0,
    slowness: 14,
    pieces: 0,
  };
  olc::start("Tetris", &mut app, SCREEN_W, SCREEN_H, STEP, STEP).unwrap();
}
