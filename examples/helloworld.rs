extern crate olc_pixel_game_engine;

use crate::olc_pixel_game_engine as olc;

struct ExampleProgram {}

impl olc::Application for ExampleProgram {
  fn on_user_create(&mut self) -> Result<(), olc::Error> {
    Ok(())
  }

  fn on_user_update(&mut self, _elapsed_time: f32) -> Result<(), olc::Error> {
    for x in 0..olc::screen_width() {
      for y in 0..olc::screen_height() {
        let p = olc::Pixel::rgb(
          (olc::c_rand() % 255) as u8,
          (olc::c_rand() % 255) as u8,
          (olc::c_rand() % 255) as u8
        );
        olc::draw(x, y, p);
      }
    }
    Ok(())
  }

  fn on_user_destroy(&mut self) -> Result<(), olc::Error> {
    Ok(())
  }
}

fn main() {
  let mut example = ExampleProgram {};
  olc::start("Hello, World!", &mut example, 256, 240, 4, 4).unwrap();
}
