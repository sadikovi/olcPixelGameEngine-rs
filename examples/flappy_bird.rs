extern crate olc_pixel_game_engine;

use std::collections::LinkedList;
use crate::olc_pixel_game_engine as olc;

struct FlappyBird {
  bird: olc::Decal,
  background: olc::Decal,
  gate: olc::Decal,
  game_over_d: olc::Decal,
  bird_pos: f32,
  bird_vel: f32,
  bird_acc: f32,
  gravity: f32,
  level_pos: f32,
  section_width: f32,
  sections: LinkedList<Option<(i32, i32)>>,
  obstacles: Vec<bool>,
  game_over: bool,
}

impl FlappyBird {
  fn new() -> Self {
    Self {
      bird: olc::Decal::new(olc::Sprite::new()),
      background: olc::Decal::new(olc::Sprite::new()),
      gate: olc::Decal::new(olc::Sprite::new()),
      game_over_d: olc::Decal::new(olc::Sprite::new()),
      bird_pos: 0.0,
      bird_vel: 0.0,
      bird_acc: 0.0,
      gravity: 100.0,
      level_pos: 0.0,
      section_width: 0.0,
      sections: LinkedList::new(),
      obstacles: Vec::new(),
      game_over: false,
    }
  }

  fn reset(&mut self) {
    self.bird_pos = (olc::screen_height() / 3) as f32;
    self.bird_vel = 0.0;
    self.bird_acc = 0.0;
    self.level_pos = 0.0;

    self.sections = LinkedList::new();
    self.sections.push_back(None);
    self.sections.push_back(None);
    self.sections.push_back(get_random_gate());
    self.sections.push_back(get_random_gate());
    self.sections.push_back(get_random_gate());
    self.sections.push_back(get_random_gate());
    self.section_width = olc::screen_width() as f32 / (self.sections.len() - 1) as f32;

    self.game_over = false;

    self.obstacles = vec![false; (olc::screen_width() * olc::screen_height()) as usize];
  }
}

fn clear(v: &mut Vec<bool>) {
  for i in 0..v.len() {
    v[i] = false;
  }
}

fn set_obstacle(v: &mut [bool], x: f32, y: f32, w: f32, h: f32) {
  let x = x as i32;
  let y = y as i32;
  let w = w as i32;
  let h = h as i32;

  for i in x..x + w {
    for j in y..y + h {
      let idx = (j * olc::screen_width() + i) as usize;
      if idx < v.len() {
        v[idx] = true;
      }
    }
  }
}

fn is_collision(v: &[bool], x: f32, y: f32, w: f32, h: f32) -> bool {
  let x = x as i32;
  let y = y as i32;
  let w = w as i32;
  let h = h as i32;

  for i in x..x + w {
    for j in y..y + h {
      let idx = (j * olc::screen_width() + i) as usize;
      if idx >= v.len() || v[idx] {
        return true;
      }
    }
  }
  false
}

fn get_random_gate() -> Option<(i32, i32)> {
  if olc::c_rand() % 10 == 0 {
    None
  } else {
    let y = 10 + (olc::c_rand() % 20);
    let gap = 30 + (olc::c_rand() % 30);
    Some((y, gap))
  }
}

impl olc::Application for FlappyBird {
  fn on_user_create(&mut self) -> Result<(), olc::Error> {
    self.bird = olc::Decal::new(olc::Sprite::from_image("examples/flappy_bird.png")?);
    self.background = olc::Decal::new(olc::Sprite::from_image("examples/flappy_bird_background.png")?);
    self.gate = olc::Decal::new(olc::Sprite::from_image("examples/flappy_bird_gate.png")?);
    self.game_over_d = olc::Decal::new(olc::Sprite::from_image("examples/flappy_bird_game_over.png")?);

    self.reset();

    Ok(())
  }

  fn on_user_update(&mut self, elapsed_time: f32) -> Result<(), olc::Error> {
    olc::clear(olc::BLACK);

    let bird_pos_x = (olc::screen_width() / 3) as f32;

    if self.game_over {
      if olc::get_key(olc::Key::SPACE).released {
        self.reset();
      }
    } else {
      if olc::get_key(olc::Key::SPACE).pressed && self.bird_vel >= self.gravity / 10.0 {
        self.bird_acc = 0.0;
        self.bird_vel = -self.gravity / 4.0;
      } else {
        self.bird_acc += self.gravity * elapsed_time;
        if self.bird_acc > self.gravity {
          self.bird_acc = self.gravity;
        }
      }

      self.bird_vel += self.bird_acc * elapsed_time;
      self.bird_pos += self.bird_vel * elapsed_time;
      self.level_pos += 10.0 * elapsed_time;

      if self.level_pos >= self.section_width {
        self.level_pos -= self.section_width;
        self.sections.pop_front();
        self.sections.push_back(get_random_gate());
      }
    }

    olc::draw_decal(
      &olc::Vf2d::new(0.0, 0.0),
      &self.background
    );

    clear(&mut self.obstacles);
    let mut x = self.section_width / 2.0 - self.level_pos;
    for s in &self.sections {
      if let Some((height, gap)) = *s {
        // Draw the top part of the gate
        let mut y = height;

        for j in 0..y {
          olc::draw_partial_decal(
            &olc::Vf2d::new(x + 2.0, j as f32),
            &self.gate,
            &olc::Vf2d::new(32.0, 0.0),
            &olc::Vf2d::new(12.0, 1.0)
          );
          set_obstacle(&mut self.obstacles, x + 2.0, j as f32, 12.0, 1.0);
        }

        olc::draw_partial_decal(
          &olc::Vf2d::new(x, y as f32),
          &self.gate,
          &olc::Vf2d::new(16.0, 0.0),
          &olc::Vf2d::new(16.0, 6.0)
        );
        set_obstacle(&mut self.obstacles, x, y as f32, 16.0, 6.0);

        // Draw the bottom part of the gate
        y += gap;

        olc::draw_partial_decal(
          &olc::Vf2d::new(x, y as f32),
          &self.gate,
          &olc::Vf2d::new(0.0, 0.0),
          &olc::Vf2d::new(16.0, 6.0)
        );
        set_obstacle(&mut self.obstacles, x, y as f32, 16.0, 6.0);

        y += 6;
        for j in y..olc::screen_height() - 10 {
          olc::draw_partial_decal(
            &olc::Vf2d::new(x + 2.0, j as f32),
            &self.gate,
            &olc::Vf2d::new(32.0, 0.0),
            &olc::Vf2d::new(12.0, 1.0)
          );
          set_obstacle(&mut self.obstacles, x + 2.0, j as f32, 12.0, 1.0);
        }
      }

      x += self.section_width;
    }

    // Set the bottom part as an obstacle
    set_obstacle(&mut self.obstacles, 0.0, (olc::screen_height() - 10) as f32, olc::screen_width() as f32, 10.0);

    let bird_offset = if self.bird_vel > 0.0 { 0 } else { 10 };

    olc::draw_partial_decal(
      &olc::Vf2d::new(bird_pos_x, self.bird_pos),
      &self.bird,
      &olc::Vf2d::new(bird_offset as f32, 0.0),
      &olc::Vf2d::new(10.0, 8.0)
    );

    if self.game_over {
      olc::draw_decal(
        &olc::Vf2d::new((olc::screen_width() / 3) as f32, (olc::screen_height() / 3) as f32),
        &self.game_over_d
      );
    }

    // Make the bird a bit smaller from each side, to collision look good
    if is_collision(&self.obstacles, bird_pos_x + 1.0, self.bird_pos + 1.0, 10.0 - 1.0, 8.0 - 1.0) {
      self.game_over = true;
    }

    Ok(())
  }

  fn on_user_destroy(&mut self) -> Result<(), olc::Error> {
    Ok(())
  }
}

fn main() {
  let mut app = FlappyBird::new();
  olc::start("Flappy Bird", &mut app, 200, 100, 4, 4).unwrap();
}
