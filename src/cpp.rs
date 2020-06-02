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
  FAIL,
  NO_FILE,
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

/// Generic 2D vector type. See [`Vf2d`](Vf2d) and [`Vi2d`](Vi2d) for more information.
///
/// Don't use this generic struct directly.
/// Prefer using [`Vf2d`](Vf2d) (alias for `V2d<f32>`) and [`Vi2d`](Vi2d) (alias for `V2d<i32>`)
/// instead.
/// This struct is only exported to show available functions and implemented traits.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct V2d<T> {
  pub x: T,
  pub y: T
}

/// Mirror of `olc::vi2d`. A 2D integer vector type.
/// Implements `std::ops::Add`, `std::ops::Sub`, `std::ops::Mul`, and `std::ops::Div` as well as
/// all their assignment equivalents.
///
/// Example usage:
/// ```
/// # extern crate olc_pixel_game_engine;
/// # use crate::olc_pixel_game_engine::Vi2d;
/// let mut a = Vi2d::new(1, 2) + Vi2d { x: 3, y: 4 };
/// a *= (2i32, 2i32).into();
/// assert_eq!(a, Vi2d::new(8, 12));
/// ```
pub type Vi2d = V2d<i32>;
/// Mirror of `olc::vf2d`. A 2D float vector type.
/// Implements `std::ops::Add`, `std::ops::Sub`, `std::ops::Mul`, and `std::ops::Div` as well as
/// all their assignment equivalents.
///
/// Example usage:
/// ```
/// # extern crate olc_pixel_game_engine;
/// # use crate::olc_pixel_game_engine::Vf2d;
/// let mut a = Vf2d::new(10.0, 20.0) - Vf2d { x: 5.0, y: 5.0 };
/// a /= (5f32, 5f32).into();
/// assert_eq!(a, Vf2d::new(1.0, 3.0));
/// ```
pub type Vf2d = V2d<c_float>;

/// Mirror of `olc::Pixel`. Represents a 32-bit RGBA value.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Pixel {
  pub r: u8,
  pub g: u8,
  pub b: u8,
  pub a: u8
}

/// Mirror of `olc::HWButton`. Represents the button state, either keyboard or mouse.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HWButton {
  /// Set once during the frame the event occurs.
  pub pressed: bool,
  /// Set once during the frame the event occurs.
  pub released: bool,
  /// Set true for all frames between pressed and released events.
  pub held: bool
}

/// Mirror of `olc::Key`. Represents a key on a keyboard.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum Key {
  NONE,
  A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
  K0, K1, K2, K3, K4, K5, K6, K7, K8, K9,
  F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
  UP, DOWN, LEFT, RIGHT,
  SPACE, TAB, SHIFT, CTRL, INS, DEL, HOME, END, PGUP, PGDN,
  BACK, ESCAPE, RETURN, ENTER, PAUSE, SCROLL,
  NP0, NP1, NP2, NP3, NP4, NP5, NP6, NP7, NP8, NP9,
  NP_MUL, NP_DIV, NP_ADD, NP_SUB, NP_DECIMAL, PERIOD
}

// Internal sprite datastructure.
// Does not support Clone and Copy due to Drop freeing the underlying olc sprite.
#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct Sprite {
  olc_sprite: *const c_void
}

/// Mirror of `olc::Sprite::Mode`.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum SpriteMode {
  NORMAL,
  PERIODIC
}

/// Mirror of `olc::Sprite::Flip`.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum SpriteFlip {
  NONE,
  HORIZ,
  VERT
}

// Internal Decal datastructure.
// Does not support Clone and Copy due to Drop freeing the underlying olc decal.
#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct Decal {
  olc_decal: *const c_void
}

/// Mirror of the `olc::LayerDesc`. Contains layer description, must be treated as read-only since
/// no modifications to the object are propagated back to the engine.
/// Does not support Clone and Copy, used as a container for layer information.
#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct LayerDesc {
  /// Layer id.
  pub id: u8,
  /// Layer offset.
  pub offset: Vf2d,
  /// Layer scale.
  pub scale: Vf2d,
  /// Layer tint.
  pub tint: Pixel,
  /// Whether or not this layer is enabled to be rendered.
  pub shown: bool,
  pub sprite: Sprite // layer backing sprite
}

#[link(name="olcRustBindingApp", kind="static")]
extern "C" {
  /// Utility C++ srand function.
  pub fn c_srand(seed: u32);

  /// Utility C++ rand function.
  pub fn c_rand() -> i32;

  /// Starts the main game loop.
  // Default values: full_screen = false and vsync = false
  pub fn start(name: *const c_char, binding: *mut c_void, screen_w: i32, screen_h: i32, pixel_w: i32, pixel_h: i32, full_screen: bool, vsync: bool) -> RCode;

  // Sprite API

  pub fn SpriteNullConstructor() -> Sprite;
  pub fn SpriteConstructor(w: i32, h: i32) -> Sprite;
  pub fn SpriteDestructor(s: &Sprite);
  pub fn SpriteLoadFromFile(s: &Sprite, image_file: *const c_char) -> RCode;
  pub fn SpriteWidth(s: &Sprite) -> i32;
  pub fn SpriteHeight(s: &Sprite) -> i32;
  pub fn SpriteHasData(s: &Sprite) -> bool;
  pub fn SpriteSetSampleMode(s: &Sprite, mode: SpriteMode);
  pub fn SpriteGetSampleMode(s: &Sprite) -> SpriteMode;
  pub fn SpriteGetPixel(s: &Sprite, x: i32, y: i32) -> Pixel;
  pub fn SpriteSetPixel(s: &Sprite, x: i32, y: i32, p: Pixel) -> bool;
  pub fn SpriteSample(s: &Sprite, x: c_float, y: c_float) -> Pixel;
  pub fn SpriteSampleBL(s: &Sprite, u: c_float, v: c_float) -> Pixel;

  pub fn DecalConstructor(s: &Sprite) -> Decal;
  pub fn DecalId(d: &Decal) -> i32;
  pub fn DecalScale(d: &Decal) -> Vf2d;
  pub fn DecalDestructor(d: &Decal);

  // olcPixelGameEngine API

  pub fn IsFocused() -> bool;
  // Get the state of a specific keyboard button
  pub fn GetKey(k: Key) -> HWButton;
  // Get the state of a specific mouse button
  pub fn GetMouse(b: u32) -> HWButton;
  // Get Mouse X coordinate in "pixel" space
  pub fn GetMouseX() -> i32;
  // Get Mouse Y coordinate in "pixel" space
  pub fn GetMouseY() -> i32;
  // Get Mouse Wheel Delta
  pub fn GetMouseWheel() -> i32;

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

  pub fn CreateLayer() -> u32;
  pub fn SetPrimaryDrawTarget();
  pub fn SetDrawTarget(layer: u8);
  pub fn GetDrawTarget(layer: u8) -> LayerDesc;
  pub fn GetPrimaryDrawTarget() -> LayerDesc;
  pub fn EnableLayer(layer: u8, b: bool);
  pub fn SetLayerOffset(layer: u8, x: c_float, y: c_float);
  pub fn SetLayerScale(layer: u8, x: c_float, y: c_float);
  pub fn SetLayerTint(layer: u8, tint: Pixel);

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

  // Draws an entire sprite at well in my defencelocation (x,y)
  pub fn DrawSprite(x: i32, y: i32, sprite: &Sprite, scale: u32, flip: SpriteFlip);
  // Draws an area of a sprite at location (x,y), where the
  // selected area is (ox,oy) to (ox+w,oy+h)
  pub fn DrawPartialSprite(x: i32, y: i32, sprite: &Sprite, ox: i32, oy: i32, w: i32, h: i32, scale: u32, flip: SpriteFlip);

  // Draws a whole decal, with optional scale and tinting
  pub fn DrawDecal(pos: &Vf2d, decal: &Decal, scale: &Vf2d, tint: &Pixel);
  // Draws a region of a decal, with optional scale and tinting
  pub fn DrawPartialDecal(pos: &Vf2d, decal: &Decal, source_pos: &Vf2d, source_size: &Vf2d, scale: &Vf2d, tint: &Pixel);

  // Draws warped decal
  pub fn DrawWarpedDecal(decal: &Decal, pos: *const Vf2d, tint: &Pixel);
  // Draws partial warped decal
  pub fn DrawPartialWarpedDecal(decal: &Decal, pos: *const Vf2d, source_pos: &Vf2d, source_size: &Vf2d, tint: &Pixel);

  // Draws rotated decal
  pub fn DrawRotatedDecal(pos: &Vf2d, decal: &Decal, angle: c_float, center: &Vf2d, scale: &Vf2d, tint: &Pixel);
  // Draws partially rotated decal
  pub fn DrawPartialRotatedDecal(pos: &Vf2d, decal: &Decal, angle: c_float, center: &Vf2d, source_pos: &Vf2d, source_size: &Vf2d, scale: &Vf2d, tint: &Pixel);

  pub fn DrawStringDecal(pos: &Vf2d, sText: *const c_char, col: Pixel, scale: &Vf2d);
  pub fn DrawString(x: i32, y: i32, sText: *const c_char, col: Pixel, scale: u32);

  // Clears entire draw target to Pixel
  pub fn Clear(p: Pixel);
  // Clears the rendering back buffer
  pub fn ClearBuffer(p: Pixel, bDepth: bool);
}
