extern crate olc_pixel_game_engine;

use crate::olc_pixel_game_engine as olc;

// "Matrix rain".
// Source code: https://github.com/OneLoneCoder/videos/blob/master/OneLoneCoder_Matrix.cpp
// Video: https://www.youtube.com/watch?v=s7EbrvA188A

// Width and height of the characters in the built-in font.
const CHAR_WIDTH: f32 = 15.0; // controls spacing between streams
const CHAR_HEIGHT: f32 = 10.0;

// Random ASCII
#[inline]
fn rand_character() -> char {
  let random_char = (olc::c_rand() % 93 + 33) as u8;
  random_char as char
}

struct Stream {
  column: f32,
  pos_delta: f32,
  pos: f32,
  speed: f32,
  text: String
}

impl Stream {
  fn new() -> Self {
    Self { column: 0.0, pos_delta: 0.0, pos: 0.0, speed: 0.0, text: String::new() }
  }

  fn prepare(&mut self) {
    self.column = (olc::c_rand() % (olc::screen_width() / CHAR_WIDTH as i32)) as f32;
    self.pos_delta = 0.0;
    self.pos = 0.0;
    self.speed = (olc::c_rand() % 50 + 15) as f32;
    self.text.clear();

    let len = olc::c_rand() % 80 + 10;
    for _ in 0..len {
      self.text.push(rand_character());
    }
  }
}

struct Matrix {
  streams: Vec<Stream>
}

impl olc::Application for Matrix {
  fn on_user_create(&mut self) -> Result<(), olc::Error> {
    let num_streams = olc::screen_width() / CHAR_WIDTH as i32;
    for _ in 0..num_streams {
      let mut stream = Stream::new();
      stream.prepare();
      self.streams.push(stream);
    }
    Ok(())
  }

  fn on_user_update(&mut self, elapsed_time: f32) -> Result<(), olc::Error> {
    olc::clear(olc::BLACK);

    for stream in &mut self.streams {
      // println!("speed: {}", CHAR_HEIGHT * elapsed_time * stream.speed);
      stream.pos_delta += elapsed_time * stream.speed;
      if stream.pos_delta >= CHAR_HEIGHT {
        stream.pos += 1.0;
        stream.pos_delta = 0.0;
      }

      for i in 0..stream.text.len() {
        // Select the colour for the character
        let col = if i == 0 {
          olc::WHITE
        } else if i <= 3 {
          olc::GREY
        } else if i >= stream.text.len() - 3 {
          olc::DARK_GREEN // fades away
        } else if stream.speed < 30.0 {
          olc::DARK_GREEN
        } else {
          olc::GREEN
        };

        let char_index = (i as i32 - stream.pos as i32) % stream.text.len() as i32;
        let char_index = if char_index < 0 {
          (stream.text.len() as i32 + char_index) as usize // wrap index if it is negative
        } else {
          char_index as usize
        };

        olc::draw_string(
          (stream.column * CHAR_WIDTH) as i32,
          ((stream.pos - i as f32) * CHAR_HEIGHT) as i32,
          &stream.text[char_index..char_index + 1],
          col
        )?;

        // Occasionally glitch a character
        if olc::c_rand() % 1000 < 5 {
          unsafe {
            let bytes = stream.text.as_bytes_mut();
            bytes[i] = rand_character() as u8;
          }
        }
      }

      // If stream goes out of screen, reset it
      if (stream.pos - stream.text.len() as f32) * CHAR_HEIGHT >= olc::screen_height() as f32 {
        stream.prepare();
      }
    }

    Ok(())
  }

  fn on_user_destroy(&mut self) -> Result<(), olc::Error> {
    Ok(())
  }
}

fn main() {
  let mut app = Matrix { streams: Vec::new() };
  olc::start("Matrix", &mut app, 800, 600, 1, 1).unwrap();
}
