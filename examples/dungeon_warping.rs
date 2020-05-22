extern crate olc_pixel_game_engine;

use crate::olc_pixel_game_engine as olc;

// Tile size is 32x32 in dungeon.png.
const TILE_SIZE: olc::Vi2d = olc::Vi2d { x: 32, y: 32 };

const FLOOR: usize = 0;
const TOP: usize = 1;
const NORTH: usize = 2;
const SOUTH: usize = 3;
const WEST: usize = 4;
const EAST: usize = 5;

#[derive(Clone, Debug, Default)]
struct Cell {
  wall: bool,
  id: [olc::Vi2d; 6]
}

struct World {
  cells: Vec<Cell>,
  null_cell: Cell, // dummy cell to return when the index is out of bound
  size: olc::Vi2d,
}

impl World {
  pub fn create(width: i32, height: i32) -> Self {
    let cells = vec![Cell::default(); (width * height) as usize];
    let null_cell = Cell::default();
    let size = olc::Vi2d { x: width, y: height };
    Self { cells, size, null_cell }
  }

  pub fn get_cell_mut(&mut self, v: olc::Vi2d) -> &mut Cell {
    if v.x >= 0 && v.x < self.size.x && v.y >= 0 && v.y < self.size.y {
      &mut self.cells[(v.y * self.size.x + v.x) as usize]
    } else {
      &mut self.null_cell
    }
  }
}

#[derive(Clone, Copy, Debug, Default)]
struct Vec3d {
  x: f32,
  y: f32,
  z: f32
}

#[derive(Clone, Debug)]
struct Quad {
  points: [Vec3d; 4],
  tile: olc::Vi2d
}

struct WarpedDungeon {
  world: World,
  rend_all_walls: Option<olc::Decal>,
  rend_select: Option<olc::Decal>,
  camera_pos: olc::Vf2d,
  camera_angle: f32,
  camera_angle_target: f32,
  camera_pitch: f32,
  camera_zoom: f32,
  cursor: olc::Vi2d,
  visible: [bool; 6],
  tile_cursor: olc::Vi2d
}

impl olc::Application for WarpedDungeon {
  fn on_user_create(&mut self) -> Result<(), olc::Error> {
    self.rend_all_walls =
      Some(olc::Decal::new(olc::Sprite::from_image("examples/dungeon.png").unwrap()));
    self.rend_select =
      Some(olc::Decal::new(olc::Sprite::from_image("examples/cursor.png").unwrap()));

    for x in 0..self.world.size.x {
      for y in 0..self.world.size.y {
        self.world.get_cell_mut(olc::Vi2d { x, y }).wall = false;
        self.world.get_cell_mut(olc::Vi2d { x, y }).id[FLOOR] = olc::Vi2d { x: 0, y: 0 } * TILE_SIZE;
        self.world.get_cell_mut(olc::Vi2d { x, y }).id[TOP] = olc::Vi2d { x: 2, y: 0 } * TILE_SIZE;
        self.world.get_cell_mut(olc::Vi2d { x, y }).id[NORTH] = olc::Vi2d { x: 6, y: 6 } * TILE_SIZE;
        self.world.get_cell_mut(olc::Vi2d { x, y }).id[SOUTH] = olc::Vi2d { x: 6, y: 6 } * TILE_SIZE;
        self.world.get_cell_mut(olc::Vi2d { x, y }).id[WEST] = olc::Vi2d { x: 6, y: 6 } * TILE_SIZE;
        self.world.get_cell_mut(olc::Vi2d { x, y }).id[EAST] = olc::Vi2d { x: 6, y: 6 } * TILE_SIZE;
      }
    }

    Ok(())
  }

  fn on_user_update(&mut self, elapsed_time: f32) -> Result<(), olc::Error> {
    // Edit mode - Selection from tile sprite sheet
    if olc::get_key(olc::Key::TAB).held {
      let mouse = olc::Vi2d { x: olc::get_mouse_x(), y: olc::get_mouse_y() };
      olc::draw_sprite(0, 0, self.rend_all_walls.as_ref().unwrap().sprite());
      olc::draw_rect(self.tile_cursor.x * TILE_SIZE.x, self.tile_cursor.y * TILE_SIZE.y, TILE_SIZE.x, TILE_SIZE.y, olc::YELLOW);
      if olc::get_mouse(0).pressed {
        self.tile_cursor = mouse / TILE_SIZE;
      }

      return Ok(())
    }

    // WS keys to tilt camera
    if olc::get_key(olc::Key::W).held { self.camera_pitch += 1.0 * elapsed_time; }
    if olc::get_key(olc::Key::S).held { self.camera_pitch -= 1.0 * elapsed_time; }

    // DA Keys to manually rotate camera
    if olc::get_key(olc::Key::D).held { self.camera_angle_target += 1.0 * elapsed_time; }
    if olc::get_key(olc::Key::A).held { self.camera_angle_target -= 1.0 * elapsed_time; }

    // QZ Keys to zoom in or out
    if olc::get_key(olc::Key::Q).held { self.camera_zoom += 5.0 * elapsed_time; }
    if olc::get_key(olc::Key::Z).held { self.camera_zoom -= 5.0 * elapsed_time; }

    // Arrow keys to move the selection cursor around map (boundary checked)
    if olc::get_key(olc::Key::LEFT).pressed { self.cursor.x -= 1; }
    if olc::get_key(olc::Key::RIGHT).pressed { self.cursor.x += 1; }
    if olc::get_key(olc::Key::UP).pressed { self.cursor.y -= 1; }
    if olc::get_key(olc::Key::DOWN).pressed { self.cursor.y += 1; }
    if self.cursor.x < 0 { self.cursor.x = 0; }
    if self.cursor.y < 0 { self.cursor.y = 0; }
    if self.cursor.x >= self.world.size.x { self.cursor.x = self.world.size.x - 1; }
    if self.cursor.y >= self.world.size.y { self.cursor.y = self.world.size.y - 1; }

    // Place block with space
    if olc::get_key(olc::Key::SPACE).pressed {
      let is_wall = self.world.get_cell_mut(self.cursor).wall;
      self.world.get_cell_mut(self.cursor).wall = !is_wall;
    }

    // Position camera in the world
    self.camera_pos = olc::Vf2d::new(self.cursor.x as f32 + 0.5, self.cursor.y as f32 + 0.5);
    self.camera_pos.x *= self.camera_zoom;
    self.camera_pos.y *= self.camera_zoom;

    // Numpad keys used to rotate camera to fixed angles
    if olc::get_key(olc::Key::U).pressed { self.camera_angle_target = 3.14159 * 0.0; }
    if olc::get_key(olc::Key::I).pressed { self.camera_angle_target = 3.14159 * 0.25; }
    if olc::get_key(olc::Key::O).pressed { self.camera_angle_target = 3.14159 * 0.5; }
    if olc::get_key(olc::Key::J).pressed { self.camera_angle_target = 3.14159 * 0.75; }
    if olc::get_key(olc::Key::K).pressed { self.camera_angle_target = 3.14159 * 1.0; }
    if olc::get_key(olc::Key::L).pressed { self.camera_angle_target = 3.14159 * 1.25; }
    if olc::get_key(olc::Key::N).pressed { self.camera_angle_target = 3.14159 * 1.5; }
    if olc::get_key(olc::Key::M).pressed { self.camera_angle_target = 3.14159 * 1.75; }

    // Numeric keys apply selected tile to specific face
    if olc::get_key(olc::Key::K1).pressed { self.world.get_cell_mut(self.cursor).id[NORTH] = self.tile_cursor * TILE_SIZE; }
    if olc::get_key(olc::Key::K2).pressed { self.world.get_cell_mut(self.cursor).id[EAST] = self.tile_cursor * TILE_SIZE; }
    if olc::get_key(olc::Key::K3).pressed { self.world.get_cell_mut(self.cursor).id[SOUTH] = self.tile_cursor * TILE_SIZE; }
    if olc::get_key(olc::Key::K4).pressed { self.world.get_cell_mut(self.cursor).id[WEST] = self.tile_cursor * TILE_SIZE; }
    if olc::get_key(olc::Key::K5).pressed { self.world.get_cell_mut(self.cursor).id[FLOOR] = self.tile_cursor * TILE_SIZE; }
    if olc::get_key(olc::Key::K6).pressed { self.world.get_cell_mut(self.cursor).id[TOP] = self.tile_cursor * TILE_SIZE; }

    // Smooth camera
		self.camera_angle += (self.camera_angle_target - self.camera_angle) * 10.0 * elapsed_time;

    let mut quads: Vec<Quad> = Vec::new();

    // Create dummy cube to extract visible face information.
    // Cull faces that cannot be seen.
    let cull_cube = Self::create_cube(
      olc::Vi2d { x: 0, y: 0 },
      self.camera_angle,
      self.camera_pitch,
      self.camera_zoom,
      Vec3d { x: self.camera_pos.x, y: 0.0, z: self.camera_pos.y}
    );
    self.calculate_visible_faces(&cull_cube);

    for y in 0..self.world.size.y {
      for x in 0..self.world.size.x {
        self.get_quads(
          olc::Vi2d { x, y },
          self.camera_angle,
          self.camera_pitch,
          self.camera_zoom,
          Vec3d { x: self.camera_pos.x, y: 0.0, z: self.camera_pos.y },
          &mut quads
        );
      }
    }

    quads.sort_by(|q1, q2| {
      let z1 = (q1.points[0].z + q1.points[1].z + q1.points[2].z + q1.points[3].z) * 0.25;
      let z2 = (q2.points[0].z + q2.points[1].z + q2.points[2].z + q2.points[3].z) * 0.25;
      z1.partial_cmp(&z2).unwrap()
    });

    olc::clear(olc::BLACK);

    for quad in &quads {
      olc::draw_partial_warped_decal(
        self.rend_all_walls.as_ref().unwrap(),
        &[
          olc::Vf2d::new(quad.points[0].x, quad.points[0].y),
          olc::Vf2d::new(quad.points[1].x, quad.points[1].y),
          olc::Vf2d::new(quad.points[2].x, quad.points[2].y),
          olc::Vf2d::new(quad.points[3].x, quad.points[3].y)
        ],
        &olc::Vf2d { x: quad.tile.x as f32, y: quad.tile.y as f32 },
        &olc::Vf2d { x: TILE_SIZE.x as f32, y: TILE_SIZE.y as f32 } // TODO: fix this inconsistency
      );
    }

    // 6) Draw selection "tile cube"
    quads.clear();
    self.get_quads(
      self.cursor,
      self.camera_angle,
      self.camera_pitch,
      self.camera_zoom,
      Vec3d { x: self.camera_pos.x, y: 0.0, z: self.camera_pos.y },
      &mut quads
    );

    for quad in &quads {
      olc::draw_partial_warped_decal(
        self.rend_select.as_ref().unwrap(),
        &[
          olc::Vf2d::new(quad.points[0].x, quad.points[0].y),
          olc::Vf2d::new(quad.points[1].x, quad.points[1].y),
          olc::Vf2d::new(quad.points[2].x, quad.points[2].y),
          olc::Vf2d::new(quad.points[3].x, quad.points[3].y)
        ],
        &olc::Vf2d { x: quad.tile.x as f32, y: quad.tile.y as f32 },
        &olc::Vf2d { x: TILE_SIZE.x as f32, y: TILE_SIZE.y as f32 } // TODO: fix this inconsistency
      );
    }

    Ok(())
  }

  fn on_user_destroy(&mut self) -> Result<(), olc::Error> {
    Ok(())
  }
}

impl WarpedDungeon {
  fn create_cube(cell: olc::Vi2d, angle: f32, pitch: f32, scale: f32, camera: Vec3d) -> [Vec3d; 8] {
    let mut unit_cube = [Vec3d::default(); 8];
    let mut rot_cube = [Vec3d::default(); 8];
    let mut world_cube = [Vec3d::default(); 8];
    let mut proj_cube = [Vec3d::default(); 8];

    // Create unit cube
    unit_cube[0] = Vec3d { x: 0.0, y: 0.0, z: 0.0 };
    unit_cube[1] = Vec3d { x: scale, y: 0.0, z: 0.0 };
    unit_cube[2] = Vec3d { x: scale, y: -scale, z: 0.0 };
    unit_cube[3] = Vec3d { x: 0.0, y: -scale, z: 0.0 };
    unit_cube[4] = Vec3d { x: 0.0, y: 0.0, z: scale };
    unit_cube[5] = Vec3d { x: scale, y: 0.0, z: scale };
    unit_cube[6] = Vec3d { x: scale, y: -scale, z: scale };
    unit_cube[7] = Vec3d { x: 0.0, y: -scale, z: scale };

    // Translate cube in X-Z plane.
    for i in 0..8 {
      unit_cube[i].x += cell.x as f32 * scale - camera.x;
      unit_cube[i].y += -camera.y;
      unit_cube[i].z += cell.y as f32 * scale - camera.z;
    }

    // Rotate cube in Y-axis around origin
    let s = angle.sin();
    let c = angle.cos();
    for i in 0..8 {
      rot_cube[i].x = unit_cube[i].x * c + unit_cube[i].z * s;
      rot_cube[i].y = unit_cube[i].y;
      rot_cube[i].z = unit_cube[i].x * (-s) + unit_cube[i].z * c;
    }

    // Rotate cube in X-axis around origin (tilt slightly overhead)
    let s = pitch.sin();
    let c = pitch.cos();
    for i in 0..8 {
      world_cube[i].x = rot_cube[i].x;
      world_cube[i].y = rot_cube[i].y * c - rot_cube[i].z * s;
      world_cube[i].z = rot_cube[i].y * s + rot_cube[i].z * c;
    }

    // Project cube orthographically - full screen centered
    for i in 0..8 {
      proj_cube[i].x = world_cube[i].x + olc::screen_width() as f32 * 0.5;
      proj_cube[i].y = world_cube[i].y + olc::screen_height() as f32 * 0.5;
      proj_cube[i].z = world_cube[i].z;
    }

    proj_cube
  }

  fn get_quads(&mut self, cell: olc::Vi2d, camera_angle: f32, camera_pitch: f32, camera_zoom: f32, camera_pos: Vec3d, render: &mut Vec<Quad>) {
    let cube = Self::create_cube(cell, camera_angle, camera_pitch, camera_zoom, camera_pos);
    let cell = self.world.get_cell_mut(cell);

    let mut make_face = |v1: usize, v2: usize, v3: usize, v4: usize, face: usize| {
      let quad = Quad { points: [cube[v1], cube[v2], cube[v3], cube[v4]], tile: cell.id[face] };
      render.push(quad);
    };

    if !cell.wall {
      if self.visible[FLOOR] { make_face(4, 0, 1, 5, FLOOR); }
    } else {
      if self.visible[SOUTH] { make_face(3, 0, 1, 2, SOUTH); }
      if self.visible[NORTH] { make_face(6, 5, 4, 7, NORTH); }
      if self.visible[EAST] { make_face(7, 4, 0, 3, EAST); }
      if self.visible[WEST] { make_face(2, 1, 5, 6, WEST); }
      if self.visible[TOP] { make_face(7, 3, 2, 6, TOP); }
    }
  }

  fn calculate_visible_faces(&mut self, cube: &[Vec3d; 8]) {
    let check_normal = |v1: usize, v2: usize, v3: usize| {
      let a = olc::Vf2d::new(cube[v1].x, cube[v1].y);
      let b = olc::Vf2d::new(cube[v2].x, cube[v2].y);
      let c = olc::Vf2d::new(cube[v3].x, cube[v3].y);
      (b - a).cross(c - a) > 0.0
    };

    self.visible[FLOOR] = check_normal(4, 0, 1);
    self.visible[SOUTH] = check_normal(3, 0, 1);
    self.visible[NORTH] = check_normal(6, 5, 4);
    self.visible[EAST] = check_normal(7, 4, 0);
    self.visible[WEST] = check_normal(2, 1, 5);
    self.visible[TOP] = check_normal(7, 3, 2);
  }
}

fn main() {
  let mut example = WarpedDungeon {
    world: World::create(64, 64),
    rend_all_walls: None,
    rend_select: None,
    camera_pos: olc::Vf2d::new(0.0, 0.0),
    camera_angle: 0.0,
    camera_angle_target: 0.0,
    camera_pitch: 5.5,
    camera_zoom: 16.0,
    cursor: olc::Vi2d::new(0, 0),
    visible: [false; 6],
    tile_cursor: olc::Vi2d::new(0, 0)
  };

  olc::start("Warped Dungeon", &mut example, 640, 480, 2, 2).unwrap();
}
