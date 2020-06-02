extern crate olc_pixel_game_engine;

use std::time::SystemTime;
use crate::olc_pixel_game_engine as olc;

const UNKNOWN: i32 = -1;
const MINE: i32 = -2;
const BLOWN: i32 = -3;

struct MineSweeper {
  width: i32,
  height: i32,
  time: f32,
  mines: i32,
  board: Vec<(i32, bool)>,
  game_over: i32,
  sprite: olc::Sprite,
}

impl MineSweeper {
  fn new() -> Self {
    Self {
      width: 10,
      height: 10,
      time: 0.0,
      mines: 15,
      board: Vec::new(),
      game_over: 0,
      sprite: olc::Sprite::new(),
    }
  }

  fn gen_board(&mut self) {
    self.board = vec![(UNKNOWN, false); (self.width * self.height) as usize];
    let mut mine_pos = Vec::new();
    for x in 0..self.width {
      for y in 0..self.height {
        mine_pos.push((x, y));
      }
    }

    let mut left = self.mines;
    while left > 0 {
      let idx = olc::c_rand() as usize % mine_pos.len();
      let (x, y) = mine_pos[idx];
      self.board[(y * self.width + x) as usize] = (MINE, false);
      if idx < mine_pos.len() - 1 {
        mine_pos[idx] = mine_pos[mine_pos.len() - 1];
      }
      mine_pos.resize(mine_pos.len() - 1, (0, 0));
      left -= 1;
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
  fn explore(&mut self, x: i32, y: i32) -> bool {
    if x >= 0 && x < self.width && y >= 0 && y < self.height {
      let idx = (y * self.width + x) as usize;

      if self.board[idx].0 == UNKNOWN && !self.board[idx].1 {
        self.board[idx] = (0, self.board[idx].1);

        for i in -1..2 {
          for j in -1..2 {
            if self.get(x + i, y + j).0 == MINE {
              self.board[idx] = (self.board[idx].0 + 1, self.board[idx].1);
            }
          }
        }

        if self.board[idx].0 == 0 {
          for i in -1..2 {
            for j in -1..2 {
              self.explore(x + i, y + j);
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
    let secs = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    println!("secs: {}", secs);
    olc::c_srand(secs as u32);
    self.sprite = olc::Sprite::from_image("examples/minesweeper.png")?;
    self.gen_board();

    Ok(())
  }

  fn on_user_update(&mut self, elapsed_time: f32) -> Result<(), olc::Error> {
    olc::clear(olc::BLACK);

    if self.game_over == 0 {
      self.time += elapsed_time;

      let (mx, my) = (olc::get_mouse_x(), olc::get_mouse_y());
      let (gx, gy) = (mx / 8, my / 8); // game coordinates

      // Setting a mine flag
      if olc::get_mouse(1).released {
        let (v, safe) = self.get(gx, gy);
        if safe || self.mines > 0 {
          self.set(gx, gy, v, !safe);
          self.mines += if safe { 1 } else { -1 };
        }
      }

      if olc::get_mouse(0).released && !self.get(gx, gy).1 && !self.explore(gx, gy) {
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

    olc::draw_string(85, 10, &format!("Time: {}", self.time as i32), olc::WHITE)?;
    olc::draw_string(85, 20, &format!("Flags: {}", self.mines), olc::WHITE)?;

    if self.game_over == 1 {
      olc::draw_string(85, 40, &"GAME OVER", olc::RED)?;
    } else if self.game_over == 2 {
      olc::draw_string(85, 40, &"YOU WON!", olc::GREEN)?;
    }

    Ok(())
  }

  fn on_user_destroy(&mut self) -> Result<(), olc::Error> {
    Ok(())
  }
}

fn main() {
  let mut app = MineSweeper::new();
  olc::start("Mine Sweeper", &mut app, 200, 80, 4, 4).unwrap();
}
