extern crate olc_pixel_game_engine;

use crate::olc_pixel_game_engine as olc;

struct Layers {}

impl olc::Application for Layers {
  fn on_user_create(&mut self) -> Result<(), olc::Error> {
    let l1 = olc::layer::create_layer();
    println!("Created layer {}", l1);
    let l2 = olc::layer::create_layer();
    println!("Created layer {}", l2);
    let l3 = olc::layer::create_layer();
    println!("Created layer {}", l3);

    Ok(())
  }

  fn on_user_update(&mut self, _elapsed_time: f32) -> Result<(), olc::Error> {
    let l1 = olc::layer::get_draw_target(1);
    // println!("layer 1: {:?}", l1);
    let l2 = olc::layer::get_draw_target(2);
    // println!("layer 2: {:?}", l2);
    let l3 = olc::layer::get_draw_target(3);
    // println!("layer 3: {:?}", l3);
    // non-existent layer
    let _l4 = olc::layer::get_draw_target(4);
    // println!("layer 4: {:?}", l4);

    olc::layer::enable_layer(l1.id, true);
    olc::layer::set_draw_target(l1.id);
    olc::clear(olc::Pixel::rgba(155, 200, 100, 100));
    olc::draw_string(20, 20, &"Layer 1", olc::WHITE)?;
    olc::draw_string(100, 20, &format!("{}", l1.get_pixel(0, 0)), olc::WHITE)?;

    olc::layer::enable_layer(l2.id, true);
    olc::layer::set_draw_target(l2.id);
    olc::clear(olc::Pixel::rgba(0, 128, 0, 100));
    olc::draw_string(30, 30, &"Layer 2", olc::WHITE)?;
    olc::draw_string(110, 30, &format!("{}", l2.get_pixel(0, 0)), olc::WHITE)?;

    olc::layer::enable_layer(l3.id, true);
    olc::layer::set_draw_target(l3.id);
    olc::clear(olc::Pixel::rgba(128, 0, 128, 100));
    olc::draw_string(40, 40, &"Layer 3", olc::WHITE)?;
    olc::draw_string(120, 40, &format!("{}", l3.get_pixel(0, 0)), olc::WHITE)?;

    olc::layer::set_primary_draw_target();
    olc::clear(olc::Pixel::rgba(128, 0, 0, 100));
    olc::draw_string(10, 10, &"Layer 0", olc::WHITE)?;
    olc::draw_string(90, 10, &format!("{}", olc::layer::get_primary_draw_target().get_pixel(0, 0)), olc::WHITE)?;

    Ok(())
  }

  fn on_user_destroy(&mut self) -> Result<(), olc::Error> {
    Ok(())
  }
}

fn main() {
  let mut app = Layers {};
  olc::start("Layers", &mut app, 400, 140, 2, 2).unwrap();
}
