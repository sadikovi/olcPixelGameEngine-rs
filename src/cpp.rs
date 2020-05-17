//! Extern module that contains olcPixelGameEngine API

#[allow(non_camel_case_types)]
pub type c_char = i8;
#[allow(non_camel_case_types)]
pub type c_float = f32;
#[allow(non_camel_case_types)]
pub type c_void = std::ffi::c_void;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub enum RCode {
  CONSTRUCT_FAIL,
  CONSTRUCT_NO_FILE,
  START_FAIL,
  START_NO_FILE,
  OK
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(non_camel_case_types)]
/// Mirror of `olc::Pixel::Mode`.
pub enum PixelMode {
  /// `olc::Pixel::NORMAL` = No transparency.
  NORMAL,
  /// `olc::Pixel::MASK` = Transparent if alpha is < 255.
  MASK,
  /// `olc::Pixel::ALPHA` = Full transparency.
  ALPHA,
  /// `olc::Pixel::CUSTOM` = Custom transparency.
  CUSTOM
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
/// Mirror of `olc::Pixel`.
pub struct Pixel {
  pub r: u8,
  pub g: u8,
  pub b: u8,
  pub a: u8
}

impl Pixel {
  /// Creates a new pixel with RGBA value.
  pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Pixel {
    Pixel { r, g, b, a }
  }

  /// Creates a new pixel with RGB value.
  pub const fn rgb(r: u8, g: u8, b: u8) -> Pixel {
    Pixel { r, g, b, a: 0xFF }
  }
}

#[link(name="olcRustBindingApp", kind="static")]
extern "C" {
  /// Utility c++ rand function.
  pub fn c_rand() -> i32;

  /// Starts the main game loop.
  // Default values: full_screen = false and vsync = false
  pub fn start(name: *const c_char, binding: *mut c_void, screen_w: i32, screen_h: i32, pixel_w: i32, pixel_h: i32, full_screen: bool, vsync: bool) -> RCode;

  // olcPixelGameEngine API

  // Returns the width of the screen in "pixels"
  pub fn ScreenWidth() -> i32;
  // Returns the height of the screen in "pixels"
  pub fn ScreenHeight() -> i32;
  // Returns the width of the currently selected drawing target in "pixels"
  pub fn GetDrawTargetWidth() -> i32;
  // Returns the height of the currently selected drawing target in "pixels"
  pub fn GetDrawTargetHeight() -> i32;
  // Resize the primary screen sprite
  pub fn SetScreenSize(w: i32, h: i32);
  // Gets the current Frames Per Second
  pub fn GetFPS() -> u32;

  pub fn SetDrawTarget(layer: u8);
  pub fn EnableLayer(layer: u8, b: bool);
  pub fn SetLayerOffset(layer: u8, x: c_float, y: c_float);
  pub fn SetLayerScale(layer: u8, x: c_float, y: c_float);
  pub fn SetLayerTint(layer: u8, tint: Pixel);

  pub fn CreateLayer() -> u32;

  // Change the pixel mode for different optimisations
  // olc::Pixel::NORMAL = No transparency
  // olc::Pixel::MASK   = Transparent if alpha is < 255
  // olc::Pixel::ALPHA  = Full transparency
  pub fn SetPixelMode(m: PixelMode);
  pub fn GetPixelMode() -> PixelMode;
  // Change the blend factor form between 0.0f to 1.0f;
  pub fn SetPixelBlend(fBlend: c_float);

  pub fn Draw(x: i32, y: i32, p: Pixel) -> bool;
  // Draws a line from (x1, y1) to (x2, y2)
  pub fn DrawLine(x1: i32, y1: i32, x2: i32, y2: i32, p: Pixel, pattern: u32);
  // Draws a circle located at (x, y) with radius
  pub fn DrawCircle(x: i32, y: i32, radius: i32, p: Pixel, mask: u8);
  // Fills a circle located at (x, y) with radius
  pub fn FillCircle(x: i32, y: i32, radius: i32, p: Pixel);
  // Draws a rectangle at (x, y) to (x+w, y+h)
  pub fn DrawRect(x: i32, y: i32, w: i32, h: i32, p: Pixel);
  // Fills a rectangle at (x, y) to (x+w, y+h)
  pub fn FillRect(x: i32, y: i32, w: i32, h: i32, p: Pixel);
  // Draws a triangle between points (x1, y1), (x2, y2) and (x3, y3)
  pub fn DrawTriangle(x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, p: Pixel);
  // Flat fills a triangle between points (x1, y1), (x2, y2) and (x3, y3)
  pub fn FillTriangle(x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, p: Pixel);

  pub fn DrawString(x: i32, y: i32, sText: *const c_char, col: Pixel, scale: u32);
  // Clears entire draw target to Pixel
  pub fn Clear(p: Pixel);
  // Clears the rendering back buffer
  pub fn ClearBuffer(p: Pixel, bDepth: bool);
}
