extern crate olc_pixel_game_engine;

use crate::olc_pixel_game_engine as olc;

const UNKNOWN: i32 = -1;
const MINE: i32 = -2;
const BLOWN: i32 = -3;
const SAFE: i32 = !0x7fffffff;

struct MineSweeper {
  width: i32,
  height: i32,
  depth: i32,
  board: Vec<(i32, bool)>,
  game_over: i32,
  sprite: olc::Sprite,
}

impl MineSweeper {
  fn new() -> Self {
    Self {
      width: 10,
      height: 10,
      depth: 5,
      board: Vec::new(),
      game_over: 0,
      sprite: olc::Sprite::new(),
    }
  }

  fn gen_board(&mut self) {
    self.board = vec![(UNKNOWN, false); (self.width * self.height) as usize];
    for x in 0..self.width {
      for y in 0..self.height {
        if olc::c_rand() % 8 == 0 {
          self.board[(y * self.width + x) as usize] = (MINE, false);
        }
      }
    }
  }

  fn get(&mut self, x: i32, y: i32) -> (i32, bool) {
    if x >= 0 && x < self.width && y >= 0 && y < self.height {
      self.board[(y * self.width + x) as usize]
    } else {
      (0, false)
    }
  }

  fn set(&mut self, x: i32, y: i32, value: i32, safe: bool) {
    if x >= 0 && x < self.width && y >= 0 && y < self.height {
      self.board[(y * self.width + x) as usize] = (value, safe);
    }
  }

  // Returns true if we explored successfully, false if (x, y) is a mine.
  fn explore(&mut self, x: i32, y: i32, depth: i32) -> bool {
    if x >= 0 && x < self.width && y >= 0 && y < self.height && depth > 0 {
      let idx = (y * self.width + x) as usize;

      if self.board[idx].0 == UNKNOWN && !self.board[idx].1 {
        self.board[idx] = (0, self.board[idx].1);

        for i in -1..2 {
          for j in -1..2 {
            if !self.explore(x + i, y + j, depth - 1) {
              self.board[idx] = (self.board[idx].0 + 1, self.board[idx].1);
            }
          }
        }
      }

      self.board[idx].0 != MINE
    } else {
      true
    }
  }
}

impl olc::Application for MineSweeper {
  fn on_user_create(&mut self) -> Result<(), olc::Error> {
    self.sprite = olc::Sprite::from_image("examples/minesweeper.png")?;
    self.gen_board();

    Ok(())
  }

  fn on_user_update(&mut self, _elapsed_time: f32) -> Result<(), olc::Error> {
    olc::clear(olc::BLACK);

    if self.game_over == 0 {
      let (mx, my) = (olc::get_mouse_x(), olc::get_mouse_y());
      let (gx, gy) = (mx / 8, my / 8); // game coordinates

      if olc::get_mouse(1).released {
        let (v, safe) = self.get(gx, gy);
        self.set(gx, gy, v, !safe);
      }

      if olc::get_mouse(0).released && !self.get(gx, gy).1 && !self.explore(gx, gy, self.depth) {
        self.set(gx, gy, BLOWN, false);
        self.game_over = 1;
      }

      // Winning condition
      let mut has_won = true;
      'outer: for x in 0..self.width {
        for y in 0..self.height {
          let (v, safe) = self.get(x, y);
          if v == UNKNOWN || v == MINE && !safe || v != MINE && safe {
            has_won = false;
            break 'outer;
          }
        }
      }

      if has_won { self.game_over = 2; }
    }

    for x in 0..self.width {
      for y in 0..self.height {
        let (v, safe) = self.get(x, y);

        if v == MINE && self.game_over == 1 {
          olc::draw_partial_sprite(x * 8, y * 8, &self.sprite, 16, 0, 8, 8);
        } else if v == BLOWN && self.game_over == 1 {
          olc::draw_partial_sprite(x * 8, y * 8, &self.sprite, 24, 0, 8, 8);
        } else if safe {
          olc::draw_partial_sprite(x * 8, y * 8, &self.sprite, 32, 0, 8, 8);
        } else if v == 0 {
          olc::draw_partial_sprite(x * 8, y * 8, &self.sprite, 8, 0, 8, 8);
        } else if v == 1 {
          olc::draw_partial_sprite(x * 8, y * 8, &self.sprite, 0, 8, 8, 8);
        } else if v == 2 {
          olc::draw_partial_sprite(x * 8, y * 8, &self.sprite, 8, 8, 8, 8);
        } else if v == 3 {
          olc::draw_partial_sprite(x * 8, y * 8, &self.sprite, 16, 8, 8, 8);
        } else if v == 4 {
          olc::draw_partial_sprite(x * 8, y * 8, &self.sprite, 24, 8, 8, 8);
        } else if v == 5 {
          olc::draw_partial_sprite(x * 8, y * 8, &self.sprite, 32, 8, 8, 8);
        } else if v == 6 {
          olc::draw_partial_sprite(x * 8, y * 8, &self.sprite, 40, 8, 8, 8);
        } else if v == 7 {
          olc::draw_partial_sprite(x * 8, y * 8, &self.sprite, 48, 8, 8, 8);
        } else if v == 8 {
          olc::draw_partial_sprite(x * 8, y * 8, &self.sprite, 56, 8, 8, 8);
        } else {
          olc::draw_partial_sprite(x * 8, y * 8, &self.sprite, 0, 0, 8, 8);
        }
      }
    }

    if self.game_over == 1 {
      olc::draw_string(10, olc::screen_height() - 10, &"GAME OVER", olc::WHITE)?;
    } else if self.game_over == 2 {
      olc::draw_string(10, olc::screen_height() - 10, &"YOU WON!", olc::WHITE)?;
    }

    Ok(())
  }

  fn on_user_destroy(&mut self) -> Result<(), olc::Error> {
    Ok(())
  }
}

fn main() {
  let mut app = MineSweeper::new();
  olc::start("Mine Sweeper", &mut app, 100, 100, 4, 4).unwrap();
}
