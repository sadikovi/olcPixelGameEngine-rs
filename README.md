# olcPixelGameEngine-rs

[![Build Status](https://www.travis-ci.org/sadikovi/olcPixelGameEngine-rs.svg?branch=master)](https://www.travis-ci.org/sadikovi/olcPixelGameEngine-rs)
[![Crates.io](https://img.shields.io/crates/v/olc_pixel_game_engine)](https://crates.io/crates/olc_pixel_game_engine)
[![](https://docs.rs/olc_pixel_game_engine/badge.svg)](https://docs.rs/olc_pixel_game_engine)

Library offers Rust API for [olcPixelGameEngine](https://github.com/OneLoneCoder/olcPixelGameEngine/).
I try keeping the methods and constants similar to the original C++ code so the API feels familiar
and it is easy to follow along the tutorial videos.

The code builds on Linux and macOS/OSX (any 10.x, including older versions, only X11 is required)
and uses my mac port of pixel game engine https://github.com/sadikovi/olcPixelGameEngine-macos.

You can link the crate as a dependency and extend `Application` trait to run the pixel game engine:
```rust
extern crate olc_pixel_game_engine;

use crate::olc_pixel_game_engine as olc;

// Very simple example application that prints "Hello, World!" on screen.

struct ExampleProgram {}

impl olc::Application for ExampleProgram {
  fn on_user_create(&mut self) -> Result<(), olc::Error> {
    // Mirrors `olcPixelGameEngine::onUserCreate`. Your code goes here.
    Ok(())
  }

  fn on_user_update(&mut self, _elapsed_time: f32) -> Result<(), olc::Error> {
    // Mirrors `olcPixelGameEngine::onUserUpdate`. Your code goes here.

    // Clears screen and sets black colour.
    olc::clear(olc::BLACK);
    // Prints the string starting at the position (40, 40) and using white colour.
    olc::draw_string(40, 40, "Hello, World!", olc::WHITE)?;
    Ok(())
  }

  fn on_user_destroy(&mut self) -> Result<(), olc::Error> {
    // Mirrors `olcPixelGameEngine::onUserDestroy`. Your code goes here.
    Ok(())
  }
}

fn main() {
  let mut example = ExampleProgram {};
  // Launches the program in 200x100 "pixels" screen, where each "pixel" is 4x4 pixel square,
  // and starts the main game loop.
  olc::start("Hello, World!", &mut example, 200, 100, 4, 4).unwrap();
}
```

I recommend checking out the documentation to see what APIs are available.

## Examples
You can look at the examples in [examples/](./examples) directory to get a sense of available APIs,  
make sure to also check out the docs.

I will be adding more examples in that directory. Some of them are direct ports of
`olcPixelGameEngine` examples and videos. Feel free to add more as you explore the pixel game engine!

## Run examples
For example, run the Isometric Tiles demo with `cargo run --example isometric_tiles`.

## Build
Run `cargo build` to build the project.

## Test
Run `cargo test` for the project tests.

## Docs
Run `cargo doc --open` to build and view the documentation locally.
