//! olcPixelGameEngine Rust API.
//!
//! See documentation on the invidual structs, enums, and functions for more information.
//! All of the root API functions and drawing routines are normally called as
//! `olc::<function>(...)` similar to C++ code.
//!
//! Here is an example that shows how to implement [`Application`](Application) trait and call
//! olcPixelGameEngine drawing functions.
//!
//! ```no_run
//! extern crate olc_pixel_game_engine;
//!
//! use crate::olc_pixel_game_engine as olc;
//!
//! // Very simple example application that prints "Hello, World!" on screen.
//!
//! struct ExampleProgram {}
//!
//! impl olc::Application for ExampleProgram {
//!   fn on_user_create(&mut self) -> Result<(), olc::Error> {
//!     // Mirrors `olcPixelGameEngine::onUserCreate`. Your code goes here.
//!     Ok(())
//!   }
//!
//!   fn on_user_update(&mut self, _elapsed_time: f32) -> Result<(), olc::Error> {
//!     // Mirrors `olcPixelGameEngine::onUserUpdate`. Your code goes here.
//!
//!     // Clears screen and sets black colour.
//!     olc::clear(olc::BLACK);
//!     // Prints the string starting at the position (40, 40) and using white colour.
//!     olc::draw_string(40, 40, "Hello, World!", olc::WHITE)?;
//!     Ok(())
//!   }
//!
//!   fn on_user_destroy(&mut self) -> Result<(), olc::Error> {
//!     // Mirrors `olcPixelGameEngine::onUserDestroy`. Your code goes here.
//!     Ok(())
//!   }
//! }
//!
//! fn main() {
//!   let mut example = ExampleProgram {};
//!   // Launches the program in 200x100 "pixels" screen, where each "pixel" is 4x4 pixel square,
//!   // and starts the main game loop.
//!   olc::start("Hello, World!", &mut example, 200, 100, 4, 4).unwrap();
//! }
//! ```

mod cpp;

// Public export of cpp module structs and enums so they can be used as an API.
pub use cpp::PixelMode;
pub use cpp::V2d;
pub use cpp::Vi2d;
pub use cpp::Vf2d;
pub use cpp::Pixel;
pub use cpp::HWButton;
pub use cpp::Key;
pub use cpp::SpriteMode;
pub use cpp::SpriteFlip;

use std::ffi::CString;
use std::fmt;
use std::ops;

//----------------------------------
// Private runnable API
//----------------------------------

/// Binding for the game.
struct Binding<'a> {
  app: &'a mut dyn Application
}

#[no_mangle]
extern "C" fn onUserCreate(binding: *mut cpp::c_void) -> bool {
  let b = unsafe { Box::from_raw(binding as *mut Binding) };
  let res = match b.app.on_user_create() {
    Err(err) => {
      println!("ERROR: {}", err);
      false
    },
    Ok(_) => true
  };
  Box::leak(b); // always leak the binding, it will be cleaned up in onUserDestroy/main function
  res
}

#[no_mangle]
extern "C" fn onUserUpdate(binding: *mut cpp::c_void, elapsed_time: cpp::c_float) -> bool {
  let b = unsafe { Box::from_raw(binding as *mut Binding) };
  let res = match b.app.on_user_update(elapsed_time) {
    Err(err) => {
      println!("ERROR: {}", err);
      false
    },
    Ok(_) => true
  };
  Box::leak(b); // always leak the binding, it will be cleaned up in onUserDestroy/main function
  res
}

#[no_mangle]
extern "C" fn onUserDestroy(binding: *mut cpp::c_void) -> bool {
  // binding goes out of scope and is dropped
  let b = unsafe { Box::from_raw(binding as *mut Binding) };
  match b.app.on_user_destroy() {
    Err(err) => println!("ERROR: {}", err),
    Ok(_) => {}
  }
  true // always return true to finish cleanup
}

//----------------------------------
// Public API
//----------------------------------

impl<T> cpp::V2d<T> {
  /// Creates new V2d struct.
  pub fn new(x: T, y: T) -> Self {
    Self { x, y }
  }
}

impl Vi2d {
  /// Returns magnitude (or length) of a vector.
  #[inline]
  pub fn mag(&self) -> i32 { (self.mag2() as f32).sqrt() as i32 }

  /// Returns magnitude squared.
  #[inline]
  pub fn mag2(&self) -> i32 { self.x * self.x + self.y * self.y }

  /// Returns vector norm.
  #[inline]
  pub fn norm(&self) -> Self { let r = 1 / self.mag(); Self { x: self.x * r, y: self.y * r } }

  /// Returns perpendicular vector.
  #[inline]
  pub fn perp(&self) -> Self { Self { x: -self.y, y: self.x } }

  /// Returns dot product of two vectors.
  #[inline]
  pub fn dot(&self, rhs: Vi2d) -> i32 { self.x * rhs.x + self.y * rhs.y }

  /// Returns cross product of two vectors.
  #[inline]
  pub fn cross(&self, rhs: Vi2d) -> i32 { self.x * rhs.y - self.y * rhs.x }
}

impl Vf2d {
  /// Returns magnitude (or length) of a vector.
  #[inline]
  pub fn mag(&self) -> f32 { self.mag2().sqrt() }

  /// Returns magnitude squared.
  #[inline]
  pub fn mag2(&self) -> f32 { self.x * self.x + self.y * self.y }

  /// Returns vector norm.
  #[inline]
  pub fn norm(&self) -> Self { let r = 1.0 / self.mag(); Self { x: self.x * r, y: self.y * r } }

  /// Returns perpendicular vector.
  #[inline]
  pub fn perp(&self) -> Self { Self { x: -self.y, y: self.x } }

  /// Returns dot product of two vectors.
  #[inline]
  pub fn dot(&self, rhs: Vf2d) -> f32 { self.x * rhs.x + self.y * rhs.y }

  /// Returns cross product of two vectors.
  #[inline]
  pub fn cross(&self, rhs: Vf2d) -> f32 { self.x * rhs.y - self.y * rhs.x }
}

impl<T> From<(T, T)> for cpp::V2d<T> {
  fn from(tuple: (T, T)) -> Self {
    Self { x: tuple.0, y: tuple.1 }
  }
}

impl<T: ops::Add<Output = T>> ops::Add for cpp::V2d<T> {
  type Output = Self;

  fn add(self, other: Self) -> Self::Output {
    Self { x: self.x + other.x, y: self.y + other.y }
  }
}

impl<T: ops::AddAssign> ops::AddAssign for cpp::V2d<T> {
  fn add_assign(&mut self, other: Self) {
    self.x += other.x;
    self.y += other.y;
  }
}

impl<T: ops::Sub<Output = T>> ops::Sub for cpp::V2d<T> {
  type Output = Self;

  fn sub(self, other: Self) -> Self::Output {
    Self { x: self.x - other.x, y: self.y - other.y }
  }
}

impl<T: ops::SubAssign> ops::SubAssign for cpp::V2d<T> {
  fn sub_assign(&mut self, other: Self) {
    self.x -= other.x;
    self.y -= other.y;
  }
}

impl<T: ops::Mul<Output = T>> ops::Mul for cpp::V2d<T> {
  type Output = Self;

  fn mul(self, other: Self) -> Self::Output {
    Self { x: self.x * other.x, y: self.y * other.y }
  }
}

impl<T: ops::MulAssign> ops::MulAssign for cpp::V2d<T> {
  fn mul_assign(&mut self, other: Self) {
    self.x *= other.x;
    self.y *= other.y;
  }
}

impl<T: ops::Div<Output = T>> ops::Div for cpp::V2d<T> {
  type Output = Self;

  fn div(self, other: Self) -> Self::Output {
    Self { x: self.x / other.x, y: self.y / other.y }
  }
}

impl<T: ops::DivAssign> ops::DivAssign for cpp::V2d<T> {
  fn div_assign(&mut self, other: Self) {
    self.x /= other.x;
    self.y /= other.y;
  }
}

impl<T: fmt::Display + fmt::Debug> fmt::Display for cpp::V2d<T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "({:?}, {:?})", self.x, self.y)
  }
}

impl Pixel {
  /// Creates a new pixel with RGBA value.
  pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
    Self { r, g, b, a }
  }

  /// Creates a new pixel with RGB value.
  pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
    Self { r, g, b, a: 0xFF }
  }
}

impl fmt::Display for Pixel {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "(R: {} G: {} B: {} A: {})", self.r, self.g, self.b, self.a)
  }
}

/// Mirror of `olc::Sprite`.
/// An image represented by a 2D array of `olc::Pixel`.
#[derive(Debug)]
pub struct Sprite {
  inner: cpp::Sprite
}

impl Sprite {
  /// Returns a new empty sprite, alias for [`Sprite::new`](Sprite::new).
  /// This can be used to initialise a sprite instead of using [`Option`](Option).
  pub fn empty() -> Self {
    Self::new()
  }

  /// Creates a new empty sprite.
  pub fn new() -> Self {
    let inner = unsafe { cpp::SpriteNullConstructor() };
    Self { inner }
  }

  /// Creates an empty sprite with dimensions `width` x `height`.
  pub fn with_dims(width: i32, height: i32) -> Self {
    let inner = unsafe { cpp::SpriteConstructor(width, height) };
    Self { inner }
  }

  /// Loads a sprite from the image.
  /// Returns error if the image could be loaded.
  pub fn from_image(path: &str) -> Result<Self, Error> {
    let image = CString::new(path)?;
    let inner = unsafe { cpp::SpriteConstructor(0, 0) };
    let res = unsafe { cpp::SpriteLoadFromFile(&inner, image.as_ptr()) };
    match res {
      cpp::RCode::FAIL =>
        Err(Error { msg: format!("Failed to load the sprite") }),
      cpp::RCode::NO_FILE =>
        Err(Error { msg: format!("Failed to load the sprite: No such file '{}'", path) }),
      cpp::RCode::OK => {
        Ok(Self { inner })
      }
    }
  }

  /// Returns width of the sprite.
  pub fn width(&self) -> i32 {
    unsafe { cpp::SpriteWidth(&self.inner) }
  }

  /// Returns height of the sprite.
  pub fn height(&self) -> i32 {
    unsafe { cpp::SpriteHeight(&self.inner) }
  }

  /// Returns true if sprite has data.
  pub fn has_data(&self) -> bool {
    unsafe { cpp::SpriteHasData(&self.inner) }
  }

  /// Returns sample mode for the sprite.
  pub fn sample_mode(&self) -> SpriteMode {
    unsafe { cpp::SpriteGetSampleMode(&self.inner) }
  }

  /// Sets sample mode.
  pub fn set_sample_mode(&mut self, mode: SpriteMode) {
    unsafe { cpp::SpriteSetSampleMode(&self.inner, mode) }
  }

  /// Returns pixel at (x, y).
  pub fn get_pixel(&self, x: i32, y: i32) -> Pixel {
    unsafe { cpp::SpriteGetPixel(&self.inner, x, y) }
  }

  /// Sets pixel at (x, y).
  pub fn set_pixel(&mut self, x: i32, y: i32, p: Pixel) -> bool {
    unsafe { cpp::SpriteSetPixel(&self.inner, x, y, p) }
  }

  /// Samples sprite for `x` and `y`,
  /// `x` and `y` should be between 0.0 and 1.0.
  pub fn sample(&self, x: f32, y: f32) -> Pixel {
    unsafe { cpp::SpriteSample(&self.inner, x, y) }
  }

  /// Sample BL for `u` and `v`.
  pub fn sample_bl(&self, u: f32, v: f32) -> Pixel {
    unsafe { cpp::SpriteSampleBL(&self.inner, u, v) }
  }
}

impl fmt::Display for Sprite {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "sprite {}x{}, mode: {:?}, has_data: {}",
      self.width(), self.height(), self.sample_mode(), self.has_data())
  }
}

impl Drop for Sprite {
  fn drop(&mut self) {
    unsafe {
      cpp::SpriteDestructor(&self.inner);
    }
  }
}

/// Mirror of `olc::Decal`.
/// A GPU resident storage of an `olc::Sprite`.
#[derive(Debug)]
pub struct Decal {
  inner: cpp::Decal,
  sprite: Sprite
}

impl Decal {
  /// Creates a new empty decal.
  /// This can be used for initialisation insted of using [`Option`](Option).
  pub fn empty() -> Self {
    Self::new(Sprite::new())
  }

  /// Creates a new decal from a sprite.
  pub fn new(sprite: Sprite) -> Self {
    let inner = unsafe { cpp::DecalConstructor(&sprite.inner) };
    Self { inner, sprite }
  }

  /// Returns id of the decal.
  pub fn id(&self) -> i32 {
    unsafe { cpp::DecalId(&self.inner) }
  }

  /// Returns scale of the decal.
  pub fn scale(&self) -> Vf2d {
    unsafe { cpp::DecalScale(&self.inner) }
  }

  /// Returns sprite reference.
  pub fn sprite(&self) -> &Sprite {
    &self.sprite
  }
}

impl fmt::Display for Decal {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "decal id {}, scale {:?}, {}", self.id(), self.scale(), self.sprite())
  }
}

impl Drop for Decal {
  fn drop(&mut self) {
    unsafe {
      cpp::DecalDestructor(&self.inner);
    }
  }
}

#[derive(Clone, Debug)]
/// olcPixelGameEngine error.
pub struct Error {
  msg: String
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.msg)
  }
}

impl From<std::ffi::NulError> for Error {
  fn from(error: std::ffi::NulError) -> Self {
    Self { msg: format!("{}", error) }
  }
}

/// Application trait, should be extended by an implementation and passed to [`start`](start)
/// function.
pub trait Application {
  /// Called on user create action.
  fn on_user_create(&mut self) -> Result<(), Error>;
  /// Called on user update action for every frame.
  fn on_user_update(&mut self, elapsed_time: f32) -> Result<(), Error>;
  /// Called on user destroy action.
  fn on_user_destroy(&mut self) -> Result<(), Error>;
}

/// Starts the main game loop.
pub fn start(
  name: &str,
  app: &mut dyn Application,
  screen_width: i32,
  screen_height: i32,
  pixel_width: i32,
  pixel_height: i32
) -> Result<(), Error>
{
  start_with_full_screen_and_vsync(
    name, app, screen_width, screen_height, pixel_width, pixel_height, false, false)
}

/// Starts the main game loop with configurable full screen and vsync.
pub fn start_with_full_screen_and_vsync(
  name: &str,
  app: &mut dyn Application,
  screen_width: i32,
  screen_height: i32,
  pixel_width: i32,
  pixel_height: i32,
  full_screen: bool,
  vsync: bool
) -> Result<(), Error>
{
  let name = CString::new(name)?;

  let binding = Binding { app };

  let res = unsafe {
    cpp::start(
      name.as_ptr(),
      Box::into_raw(Box::new(binding)) as *mut cpp::c_void,
      screen_width,
      screen_height,
      pixel_width,
      pixel_height,
      full_screen,
      vsync
    )
  };

  match res {
    cpp::RCode::FAIL =>
      Err(Error { msg: format!("Failed to start the application: FAIL") }),
    cpp::RCode::NO_FILE =>
      Err(Error { msg: format!("Failed to start the application: NO_FILE") }),
    cpp::RCode::OK =>
      Ok(())
  }
}

/// Sets seed for C++ rand function, equivalent of `srand(seed)`.
/// See http://www.cplusplus.com/reference/cstdlib/srand for more information.
pub fn c_srand(seed: u32) {
  unsafe { cpp::c_srand(seed) }
}

/// Utility C++ rand function, equivalent of `rand()`.
/// See http://www.cplusplus.com/reference/cstdlib/rand for more information.
pub fn c_rand() -> i32 {
  unsafe { cpp::c_rand() }
}

//----------------------------------
// olcPixelGameEngine API
//----------------------------------

pub const GREY: Pixel = Pixel::rgb(192, 192, 192);
pub const DARK_GREY: Pixel = Pixel::rgb(128, 128, 128);
pub const VERY_DARK_GREY: Pixel = Pixel::rgb(64, 64, 64);

pub const RED: Pixel = Pixel::rgb(255, 0, 0);
pub const DARK_RED: Pixel = Pixel::rgb(128, 0, 0);
pub const VERY_DARK_RED: Pixel = Pixel::rgb(64, 0, 0);

pub const YELLOW: Pixel = Pixel::rgb(255, 255, 0);
pub const DARK_YELLOW: Pixel = Pixel::rgb(128, 128, 0);
pub const VERY_DARK_YELLOW: Pixel = Pixel::rgb(64, 64, 0);

pub const GREEN: Pixel = Pixel::rgb(0, 255, 0);
pub const DARK_GREEN: Pixel = Pixel::rgb(0, 128, 0);
pub const VERY_DARK_GREEN: Pixel = Pixel::rgb(0, 64, 0);

pub const CYAN: Pixel = Pixel::rgb(0, 255, 255);
pub const DARK_CYAN: Pixel = Pixel::rgb(0, 128, 128);
pub const VERY_DARK_CYAN: Pixel = Pixel::rgb(0, 64, 64);

pub const BLUE: Pixel = Pixel::rgb(0, 0, 255);
pub const DARK_BLUE: Pixel = Pixel::rgb(0, 0, 128);
pub const VERY_DARK_BLUE: Pixel = Pixel::rgb(0, 0, 64);

pub const MAGENTA: Pixel = Pixel::rgb(255, 0, 255);
pub const DARK_MAGENTA: Pixel = Pixel::rgb(128, 0, 128);
pub const VERY_DARK_MAGENTA: Pixel = Pixel::rgb(64, 0, 64);

pub const WHITE: Pixel = Pixel::rgb(255, 255, 255);
pub const BLACK: Pixel = Pixel::rgb(0, 0, 0);
pub const BLANK: Pixel = Pixel::rgba(0, 0, 0, 0);

/// Whether or not the window is focused.
pub fn is_focused() -> bool {
  unsafe { cpp::IsFocused() }
}

/// Returns the state of a specific keyboard button.
pub fn get_key(k: Key) -> HWButton {
  unsafe { cpp::GetKey(k) }
}

/// Returns the state of a specific mouse button.
pub fn get_mouse(b: u32) -> HWButton {
  unsafe { cpp::GetMouse(b) }
}

/// Returns mouse X coordinate in "pixel" space.
pub fn get_mouse_x() -> i32 {
  unsafe { cpp::GetMouseX() }
}

/// Returns mouse Y coordinate in "pixel" space.
pub fn get_mouse_y() -> i32 {
  unsafe { cpp::GetMouseY() }
}

/// Returns mouse wheel delta.
pub fn get_mouse_wheel() -> i32 {
  unsafe { cpp::GetMouseWheel() }
}

/// Returns the width of the screen in "pixels".
pub fn screen_width() -> i32 {
  unsafe { cpp::ScreenWidth() }
}

/// Returns the height of the screen in "pixels".
pub fn screen_height() -> i32 {
  unsafe { cpp::ScreenHeight() }
}

/// Returns the width of the currently selected drawing target in "pixels".
pub fn get_draw_target_width() -> i32 {
  unsafe { cpp::GetDrawTargetWidth() }
}

/// Returns the height of the currently selected drawing target in "pixels".
pub fn get_draw_target_height() -> i32 {
  unsafe { cpp::GetDrawTargetHeight() }
}

/// Resizes the primary screen sprite.
pub fn set_screen_size(w: i32, h: i32) {
  unsafe { cpp::SetScreenSize(w, h) }
}

/// Gets the current Frames Per Second.
pub fn get_fps() -> u32 {
  unsafe { cpp::GetFPS() }
}

/// Layer API.
/// Allows creation and manipulation of layers including the primary draw target, a layer 0.
/// ```
/// # extern crate olc_pixel_game_engine;
/// # use crate::olc_pixel_game_engine as olc;
/// #
/// let layer_id = olc::layer::create_layer();
/// // By default, the layer is disabled.
/// olc::layer::enable_layer(1, true);
///
/// olc::layer::set_draw_target(layer_id);
///
/// // Will be drawn onto the layer 1.
/// olc::draw(0, 0, olc::RED);
///
/// // Returns the current layer description.
/// let layer_desc = olc::layer::get_draw_target(1);
///
/// // Reset to the primary draw target.
/// olc::layer::set_primary_draw_target();
/// ```
pub mod layer {
  use super::*;

  pub use cpp::LayerDesc;

  impl LayerDesc {
    /// Returns the pixel set for (x, y) coordinates.
    pub fn get_pixel(&self, x: i32, y: i32) -> Pixel {
      unsafe { cpp::SpriteGetPixel(&self.sprite, x, y) }
    }
  }

  /// Creates a new layer.
  pub fn create_layer() -> u8 {
    // Layer is supposed to be at most u8
    let layer = unsafe { cpp::CreateLayer() };
    layer as u8
  }

  /// Sets layer as the main draw target.
  /// After calling this function, all of the drawing routines will be projected onto the layer.
  pub fn set_draw_target(layer: u8) {
    unsafe { cpp::SetDrawTarget(layer) }
  }

  /// Sets the primary layer (index 0, the default layer) as the main draw target.
  /// This is equivalent to `olc::SetDrawTarget(nullptr)` in the pixel game engine.
  pub fn set_primary_draw_target() {
    unsafe { cpp::SetPrimaryDrawTarget() }
  }

  /// Returns layer description for the selected layer.
  pub fn get_draw_target(layer: u8) -> LayerDesc {
    unsafe { cpp::GetDrawTarget(layer) }
  }

  /// Returns description of the primary layer (index 0, the default layer).
  /// This is equivalent to `olc::GetDrawTarget(0)` in the pixel game engine.
  pub fn get_primary_draw_target() -> LayerDesc {
    unsafe { cpp::GetPrimaryDrawTarget() }
  }

  /// Enables/disables layer.
  pub fn enable_layer(layer: u8, b: bool) {
    unsafe { cpp::EnableLayer(layer, b) }
  }

  /// Sets layer offset.
  pub fn set_layer_offset(layer: u8, x: f32, y: f32) {
    unsafe { cpp::SetLayerOffset(layer, x, y) }
  }

  /// Sets layer scale.
  pub fn set_layer_scale(layer: u8, x: f32, y: f32) {
    unsafe { cpp::SetLayerScale(layer, x, y) }
  }

  /// Sets layer tint.
  pub fn set_layer_tint(layer: u8, tint: Pixel) {
    unsafe { cpp::SetLayerTint(layer, tint) }
  }
}

/// Changes the pixel mode for different optimisations.
///
/// - PixelMode::NORMAL = No transparency.
/// - PixelMode::MASK   = Transparent if alpha is < 255.
/// - PixelMode::ALPHA   = Full transparency.
pub fn set_pixel_mode(m: PixelMode) {
  unsafe { cpp::SetPixelMode(m) }
}

/// Returns the current pixel mode.
pub fn get_pixel_mode() -> PixelMode {
  unsafe { cpp::GetPixelMode() }
}

/// Changes the blend factor form between 0.0f to 1.0f.
pub fn set_pixel_blend(blend: f32) {
  unsafe { cpp::SetPixelBlend(blend) }
}

/// Draws pixel at coordinates (x, y).
pub fn draw(x: i32, y: i32, p: Pixel) -> bool {
  unsafe { cpp::Draw(x, y, p) }
}

/// Draws a line from (x1, y1) to (x2, y2).
#[inline]
pub fn draw_line(x1: i32, y1: i32, x2: i32, y2: i32, p: Pixel) {
  draw_line_with_pattern(x1, y1, x2, y2, p, 0xFFFFFFFF)
}

/// Draws a line from (x1, y1) to (x2, y2).
/// Allows to set pattern.
pub fn draw_line_with_pattern(x1: i32, y1: i32, x2: i32, y2: i32, p: Pixel, pattern: u32) {
  unsafe { cpp::DrawLine(x1, y1, x2, y2, p, pattern) }
}

/// Draws a circle located at (x, y) with radius.
#[inline]
pub fn draw_circle(x: i32, y: i32, radius: i32, p: Pixel) {
  draw_circle_with_mask(x, y, radius, p, 0xFF)
}

/// Draws a circle located at (x, y) with radius.
/// Allows to set mask.
pub fn draw_circle_with_mask(x: i32, y: i32, radius: i32, p: Pixel, mask: u8) {
  unsafe { cpp::DrawCircle(x, y, radius, p, mask) }
}

/// Fills a circle located at (x, y) with radius.
pub fn fill_circle(x: i32, y: i32, radius: i32, p: Pixel) {
  unsafe { cpp::FillCircle(x, y, radius, p) }
}

/// Draws a rectangle at (x, y) to (x+w, y+h).
pub fn draw_rect(x: i32, y: i32, w: i32, h: i32, p: Pixel) {
  unsafe { cpp::DrawRect(x, y, w, h, p) }
}

/// Fills a rectangle at (x, y) to (x+w, y+h).
pub fn fill_rect(x: i32, y: i32, w: i32, h: i32, p: Pixel) {
  unsafe { cpp::FillRect(x, y, w, h, p) }
}

/// Draws a triangle between points (x1, y1), (x2, y2) and (x3, y3).
pub fn draw_triangle(x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, p: Pixel) {
  unsafe { cpp::DrawTriangle(x1, y1, x2, y2, x3, y3, p) }
}

/// Flat fills a triangle between points (x1, y1), (x2, y2) and (x3, y3).
pub fn fill_triangle(x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, p: Pixel) {
  unsafe { cpp::FillTriangle(x1, y1, x2, y2, x3, y3, p) }
}

/// Draws an entire sprite at the location (x, y).
#[inline]
pub fn draw_sprite(x: i32, y: i32, sprite: &Sprite) {
  draw_sprite_ext(x, y, sprite, 1, SpriteFlip::NONE)
}

/// Draws an entire sprite at the location (x, y) with provided scale and flip.
pub fn draw_sprite_ext(x: i32, y: i32, sprite: &Sprite, scale: u32, flip: SpriteFlip) {
  unsafe { cpp::DrawSprite(x, y, &sprite.inner, scale, flip) }
}

/// Draws an area of a sprite at location (x, y), where the selected area is (ox, oy) to (ox+w, oy+h).
#[inline]
pub fn draw_partial_sprite(x: i32, y: i32, sprite: &Sprite, ox: i32, oy: i32, w: i32, h: i32) {
  draw_partial_sprite_ext(x, y, sprite, ox, oy, w, h, 1, SpriteFlip::NONE)
}

/// Draws an area of a sprite at location (x, y), where the selected area is (ox, oy) to (ox+w, oy+h)
/// with provided scale and flip.
pub fn draw_partial_sprite_ext(x: i32, y: i32, sprite: &Sprite, ox: i32, oy: i32, w: i32, h: i32, scale: u32, flip: SpriteFlip) {
  unsafe { cpp::DrawPartialSprite(x, y, &sprite.inner, ox, oy, w, h, scale, flip) }
}

/// Draws a whole decal with default scale and tinting.
#[inline]
pub fn draw_decal(pos: &Vf2d, decal: &Decal) {
  draw_decal_ext(pos, decal, &Vf2d::new(1.0, 1.0), &WHITE)
}

/// Draws a whole decal with scale and tinting.
pub fn draw_decal_ext(pos: &Vf2d, decal: &Decal, scale: &Vf2d, tint: &Pixel) {
  unsafe { cpp::DrawDecal(pos, &decal.inner, scale, tint) }
}

/// Draws a region of a decal with default scale and tint.
#[inline]
pub fn draw_partial_decal(pos: &Vf2d, decal: &Decal, source_pos: &Vf2d, source_size: &Vf2d) {
  draw_partial_decal_ext(pos, decal, source_pos, source_size, &Vf2d::new(1.0, 1.0), &WHITE)
}

/// Draws a region of a decal with scale and tinting.
pub fn draw_partial_decal_ext(pos: &Vf2d, decal: &Decal, source_pos: &Vf2d, source_size: &Vf2d, scale: &Vf2d, tint: &Pixel) {
  unsafe { cpp::DrawPartialDecal(pos, &decal.inner, source_pos, source_size, scale, tint) }
}

/// Draws warped decal with default tinting. `pos` is an array of 4 positions.
#[inline]
pub fn draw_warped_decal(decal: &Decal, pos: &[Vf2d]) {
  draw_warped_decal_ext(decal, pos, &WHITE)
}

/// Draws warped decal. `pos` is an array of 4 positions.
pub fn draw_warped_decal_ext(decal: &Decal, pos: &[Vf2d], tint: &Pixel) {
  assert_eq!(pos.len(), 4, "Expected 4 positions, received {}", pos.len());
  let pos_ptr = pos.as_ptr();
  unsafe { cpp::DrawWarpedDecal(&decal.inner, pos_ptr, tint) }
}

/// Draws partial warped decal with default tinting. `pos` is an array of 4 positions.
#[inline]
pub fn draw_partial_warped_decal(decal: &Decal, pos: &[Vf2d], source_pos: &Vf2d, source_size: &Vf2d) {
  draw_partial_warped_decal_ext(decal, pos, source_pos, source_size, &WHITE)
}

/// Draws partial warped decal. `pos` is an array of 4 positions.
pub fn draw_partial_warped_decal_ext(decal: &Decal, pos: &[Vf2d], source_pos: &Vf2d, source_size: &Vf2d, tint: &Pixel) {
  assert_eq!(pos.len(), 4, "Expected 4 positions, received {}", pos.len());
  let pos_ptr = pos.as_ptr();
  unsafe { cpp::DrawPartialWarpedDecal(&decal.inner, pos_ptr, source_pos, source_size, tint) }
}

/// Draws rotated decal with default center, scale, and tinting.
#[inline]
pub fn draw_rotated_decal(pos: &Vf2d, decal: &Decal, angle: f32) {
  draw_rotated_decal_ext(pos, decal, angle, &Vf2d::new(0.0, 0.0), &Vf2d::new(1.0, 1.0), &WHITE);
}

/// Draws rotated decal with custom center, scale, and tinting.
pub fn draw_rotated_decal_ext(pos: &Vf2d, decal: &Decal, angle: f32, center: &Vf2d, scale: &Vf2d, tint: &Pixel) {
  unsafe { cpp::DrawRotatedDecal(pos, &decal.inner, angle, center, scale, tint) }
}

/// Draws partial rotated decal with default scale and tinting.
#[inline]
pub fn draw_partial_rotated_decal(pos: &Vf2d, decal: &Decal, angle: f32, center: &Vf2d, source_pos: &Vf2d, source_size: &Vf2d) {
  draw_partial_rotated_decal_ext(pos, decal, angle, center, source_pos, source_size, &Vf2d::new(1.0, 1.0), &WHITE);
}

/// Draws partial rotated decal.
pub fn draw_partial_rotated_decal_ext(pos: &Vf2d, decal: &Decal, angle: f32, center: &Vf2d, source_pos: &Vf2d, source_size: &Vf2d, scale: &Vf2d, tint: &Pixel) {
  unsafe { cpp::DrawPartialRotatedDecal(pos, &decal.inner, angle, center, source_pos, source_size, scale, tint) }
}

/// Draws string decal with default colour and scale.
#[inline]
pub fn draw_string_decal(pos: &Vf2d, text: &str) -> Result<(), Error> {
  draw_string_decal_ext(pos, text, WHITE, &Vf2d::new(1.0, 1.0))
}

/// Draws string decal with colour and scale.
pub fn draw_string_decal_ext(pos: &Vf2d, text: &str, col: Pixel, scale: &Vf2d) -> Result<(), Error> {
  let ctext = CString::new(text)?;
  unsafe { cpp::DrawStringDecal(pos, ctext.as_ptr(), col, scale) }
  Ok(())
}

/// Draws string.
#[inline]
pub fn draw_string(x: i32, y: i32, text: &str, col: Pixel) -> Result<(), Error> {
  draw_string_with_scale(x, y, text, col, 1)
}

/// Draws string.
/// Allows to set scale.
pub fn draw_string_with_scale(x: i32, y: i32, text: &str, col: Pixel, scale: u32) -> Result<(), Error> {
  let ctext = CString::new(text)?;
  unsafe { cpp::DrawString(x, y, ctext.as_ptr(), col, scale) }
  Ok(())
}

/// Clears entire draw target to Pixel.
pub fn clear(p: Pixel) {
  unsafe { cpp::Clear(p) }
}

/// Clears the rendering back buffer.
pub fn clear_buffer(p: Pixel, depth: bool) {
  unsafe { cpp::ClearBuffer(p, depth) }
}

#[cfg(test)]
mod tests {
  use super::*;

  // Pixel tests

  #[test]
  fn test_pixel_display() {
    let p = Pixel::rgba(1, 2, 3, 4);
    assert_eq!(&format!("{}", p), "(R: 1 G: 2 B: 3 A: 4)");
  }

  // Vi2d tests

  #[test]
  fn test_vi2d_create() {
    let a = Vi2d { x: 1, y: 2 };
    let b: Vi2d = (1i32, 2i32).into();
    let c = Vi2d::new(1, 2);
    assert_eq!(a, b);
    assert_eq!(a, c);
  }

  #[test]
  fn test_vi2d_display() {
    let a = Vi2d::new(2, 3);
    assert_eq!(&format!("{}", a), "(2, 3)");
  }

  #[test]
  fn test_vi2d_add() {
    let mut res = Vi2d::new(1, 2) + Vi2d::new(3, 4);
    assert_eq!(res, Vi2d::new(4, 6));
    res += Vi2d::new(1, 1);
    assert_eq!(res, Vi2d::new(5, 7));
  }

  #[test]
  fn test_vi2d_sub() {
    let mut res = Vi2d::new(4, 5) - Vi2d::new(3, 4);
    assert_eq!(res, Vi2d::new(1, 1));
    res -= Vi2d::new(1, 1);
    assert_eq!(res, Vi2d::new(0, 0));
  }

  #[test]
  fn test_vi2d_mul() {
    let mut res = Vi2d::new(4, 5) * Vi2d::new(3, 4);
    assert_eq!(res, Vi2d::new(12, 20));
    res *= Vi2d::new(2, 3);
    assert_eq!(res, Vi2d::new(24, 60));
  }

  #[test]
  fn test_vi2d_div() {
    let mut res = Vi2d::new(8, 6) / Vi2d::new(2, 3);
    assert_eq!(res, Vi2d::new(4, 2));
    res /= Vi2d::new(2, 2);
    assert_eq!(res, Vi2d::new(2, 1));
  }

  // Vf2d tests

  #[test]
  fn test_vf2d_create() {
    let a = Vf2d { x: 1.0, y: 2.0 };
    let b: Vf2d = (1.0f32, 2.0f32).into();
    let c = Vf2d::new(1.0, 2.0);
    assert_eq!(a, b);
    assert_eq!(a, c);
  }

  #[test]
  fn test_vf2d_display() {
    let a = Vf2d::new(2.0, 3.0);
    assert_eq!(&format!("{}", a), "(2.0, 3.0)");
    let a = Vf2d::new(2.1, 3.1);
    assert_eq!(&format!("{}", a), "(2.1, 3.1)");
  }

  #[test]
  fn test_vf2d_add() {
    let mut res = Vf2d::new(1.0, 2.0) + Vf2d::new(3.0, 4.0);
    assert_eq!(res, Vf2d::new(4.0, 6.0));
    res += Vf2d::new(1.0, 1.0);
    assert_eq!(res, Vf2d::new(5.0, 7.0));
  }

  #[test]
  fn test_vf2d_sub() {
    let mut res = Vf2d::new(4.0, 5.0) - Vf2d::new(3.0, 4.0);
    assert_eq!(res, Vf2d::new(1.0, 1.0));
    res -= Vf2d::new(1.0, 1.0);
    assert_eq!(res, Vf2d::new(0.0, 0.0));
  }

  #[test]
  fn test_vf2d_mul() {
    let mut res = Vf2d::new(4.0, 5.0) * Vf2d::new(3.0, 4.0);
    assert_eq!(res, Vf2d::new(12.0, 20.0));
    res *= Vf2d::new(2.0, 3.0);
    assert_eq!(res, Vf2d::new(24.0, 60.0));
  }

  #[test]
  fn test_vf2d_div() {
    let mut res = Vf2d::new(8.0, 6.0) / Vf2d::new(2.0, 3.0);
    assert_eq!(res, Vf2d::new(4.0, 2.0));
    res /= Vf2d::new(2.0, 2.0);
    assert_eq!(res, Vf2d::new(2.0, 1.0));
  }
}
