extern crate olc_pixel_game_engine;

use crate::olc_pixel_game_engine as olc;

// Number of tiles in world
const WORLD_SIZE: (i32, i32) = (14, 10);
// Size of single tile graphic
const TILE_SIZE: (i32, i32) = (40, 20);
// Where to place tile (0, 0) on screen (in tile size steps)
const ORIGIN: (i32, i32) = (5, 1);

struct IsometricTiles {
  sprite: olc::Sprite,
  world: Vec<i32>
}

impl IsometricTiles {
  pub fn new() -> Self {
    Self {
      sprite: olc::Sprite::new(),
      world: Vec::new()
    }
  }
}

impl olc::Application for IsometricTiles {
  fn on_user_create(&mut self) -> Result<(), olc::Error> {
    // Load sprites
    self.sprite = olc::Sprite::from_image("examples/isometric_demo.png")?;
    // Create the world
    self.world = vec![0; WORLD_SIZE.0 as usize * WORLD_SIZE.1 as usize];
    Ok(())
  }

  fn on_user_update(&mut self, _elapsed_time: f32) -> Result<(), olc::Error> {
    olc::clear(olc::WHITE);

    // Get mouse coordinates
    let mouse_coords = (olc::get_mouse_x(), olc::get_mouse_y());

    // Work out the active cell
    let cell = (mouse_coords.0 / TILE_SIZE.0, mouse_coords.1 / TILE_SIZE.1);

    // Work out the mouse offset into the cell
    let offset = (mouse_coords.0 % TILE_SIZE.0, mouse_coords.1 % TILE_SIZE.1);

    // Sample into cell offset colour
    let col = self.sprite.get_pixel(3 * TILE_SIZE.0 + offset.0, offset.1);

    // Work out selected cell by transforming screen cell
    let mut selected = (
      (cell.1 - ORIGIN.1) + (cell.0 - ORIGIN.0),
      (cell.1 - ORIGIN.1) - (cell.0 - ORIGIN.0)
    );

    // "Bodge" selected cell by sampling corners
    let delta = match col {
      olc::RED => (-1, 0),
      olc::BLUE => (0, -1),
      olc::GREEN => (0, 1),
      olc::YELLOW => (1, 0),
      _ => (0, 0)
    };

    selected.0 += delta.0;
    selected.1 += delta.1;

    // Handle mouse click to toggle if a tile is visible or not
    if olc::get_mouse(0).pressed {
      if selected.0 >= 0 && selected.0 < WORLD_SIZE.0 && selected.1 >= 0 && selected.1 < WORLD_SIZE.1 {
        let idx = (selected.1 * WORLD_SIZE.0 + selected.0) as usize;
        self.world[idx] += 1;
        self.world[idx] %= 6;
      }
    }

    // Draw World - has binary transparancy so enable masking
    olc::set_pixel_mode(olc::PixelMode::MASK);

    // Labmda function to convert "world" coordinate into screen space
    fn to_screen(x: i32, y: i32) -> (i32, i32) {
      (
        ORIGIN.0 * TILE_SIZE.0 + (x - y) * (TILE_SIZE.0 / 2),
        ORIGIN.1 * TILE_SIZE.1 + (x + y) * (TILE_SIZE.1 / 2)
      )
    }

    // (0, 0) is at top, defined by ORIGIN, so draw from top to bottom
    // to ensure tiles closest to camera are drawn last
    for y in 0..WORLD_SIZE.1 {
      for x in 0..WORLD_SIZE.0 {
        // Convert cell coordinate to world space

        let world_coords = to_screen(x, y);

        match self.world[(y * WORLD_SIZE.0 + x) as usize] {
          0 => {
            // Invisble Tile
            olc::draw_partial_sprite(world_coords.0, world_coords.1, &self.sprite, 1 * TILE_SIZE.0, 0, TILE_SIZE.0, TILE_SIZE.1);
          },
          1 => {
            // Visible Tile
            olc::draw_partial_sprite(world_coords.0, world_coords.1, &self.sprite, 2 * TILE_SIZE.0, 0, TILE_SIZE.0, TILE_SIZE.1);
          },
          2 => {
            // Tree
            olc::draw_partial_sprite(world_coords.0, world_coords.1 - TILE_SIZE.1, &self.sprite, 0 * TILE_SIZE.0, 1 * TILE_SIZE.1, TILE_SIZE.0, TILE_SIZE.1 * 2);
          },
          3 => {
            // Spooky Tree
            olc::draw_partial_sprite(world_coords.0, world_coords.1 - TILE_SIZE.1, &self.sprite, 1 * TILE_SIZE.0, 1 * TILE_SIZE.1, TILE_SIZE.0, TILE_SIZE.1 * 2);
          },
          4 => {
            // Beach
            olc::draw_partial_sprite(world_coords.0, world_coords.1 - TILE_SIZE.1, &self.sprite, 2 * TILE_SIZE.0, 1 * TILE_SIZE.1, TILE_SIZE.0, TILE_SIZE.1 * 2);
          },
          5 => {
            // Water
            olc::draw_partial_sprite(world_coords.0, world_coords.1 - TILE_SIZE.1, &self.sprite, 3 * TILE_SIZE.0, 1 * TILE_SIZE.1, TILE_SIZE.0, TILE_SIZE.1 * 2);
          },
          _ => { }
        }
      }
    }

    // Draw Selected Cell - Has varying alpha components
    olc::set_pixel_mode(olc::PixelMode::ALPHA);

    // Convert selected cell coordinate to world space
    let selected_world_coords = to_screen(selected.0, selected.1);

    // Draw "highlight" tile
    olc::draw_partial_sprite(selected_world_coords.0, selected_world_coords.1, &self.sprite, 0 * TILE_SIZE.0, 0, TILE_SIZE.0, TILE_SIZE.1);

    // Draw Hovered Cell Boundary
    // olc::draw_rect(cell.0 * TILE_SIZE.0, cell.1 * TILE_SIZE.1, TILE_SIZE.0, TILE_SIZE.1, olc::RED);

    // Draw Debug Info
    olc::draw_string(4, 4, &format!("Mouse   : {}, {}", mouse_coords.0, mouse_coords.1), olc::BLACK)?;
    olc::draw_string(4, 14, &format!("Cell    : {}, {}", cell.0, cell.1), olc::BLACK)?;
    olc::draw_string(4, 24, &format!("Selected: {}, {}", selected.0, selected.1), olc::BLACK)?;

    Ok(())
  }

  fn on_user_destroy(&mut self) -> Result<(), olc::Error> {
    Ok(())
  }
}

fn main() {
  let mut app = IsometricTiles::new();
  olc::start("Isometric Tiles", &mut app, 512, 480, 2, 2).unwrap();
}
