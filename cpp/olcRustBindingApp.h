#ifdef __cplusplus
extern "C" {
#endif

// Called once on user create.
bool onUserCreate(void* binding);
// Called for every frame.
bool onUserUpdate(void* binding, float elapsed_time);
// Called once on user destroy.
bool onUserDestroy(void* binding);

typedef enum {
  FAIL,
  NO_FILE,
  OK
} RCode;

typedef struct {
  uint8_t r;
  uint8_t g;
  uint8_t b;
  uint8_t a;
} Pixel;

typedef olc::Pixel::Mode PixelMode;

typedef struct {
  // Set once during the frame the event occurs
  bool pressed;
  // Set once during the frame the event occurs
  bool released;
  // Set true for all frames between pressed and released events
  bool held;
} HWButton;

typedef olc::Key Key;

typedef struct {
  olc::Sprite* olc_sprite;
} Sprite;

typedef olc::Sprite::Mode SpriteMode;
typedef olc::Sprite::Flip SpriteFlip;

#define TO_RCODE(code) (toRCode(code))
#define TO_PIXEL(p) (toPixel(p))
#define TO_OLC_PIXEL(p) (olc::Pixel(p.r, p.g, p.b, p.a))
#define TO_HWBUTTON(b) (toHWButton(b))
#define TO_SPRITE(s) (toSprite(s))
#define TO_OLC_SPRITE(s) (s->olc_sprite)

static inline RCode toRCode(olc::rcode code) {
  switch (code) {
    case olc::rcode::OK: return RCode::OK;
    case olc::rcode::NO_FILE: return RCode::NO_FILE;
    case olc::rcode::FAIL:
    default:
      return RCode::FAIL;
  }
}

static inline Pixel toPixel(olc::Pixel p) {
  Pixel res;
  res.r = p.r;
  res.g = p.g;
  res.b = p.b;
  res.a = p.a;
  return res;
}

static inline HWButton toHWButton(olc::HWButton b) {
  HWButton res;
  res.pressed = b.bPressed;
  res.released = b.bReleased;
  res.held = b.bHeld;
  return res;
}

static inline Sprite toSprite(olc::Sprite* ptr) {
  Sprite s;
  s.olc_sprite = ptr;
  return s;
}

// Useful utility functions
int32_t c_rand();

// Starts the main game loop.
// Default values: full_screen = false and vsync = false
RCode start(const char* name, void* binding, int32_t screen_w, int32_t screen_h, int32_t pixel_w, int32_t pixel_h, bool full_screen, bool vsync);

// olcPixelGameEngine API

// Creates a new empty sprite
Sprite SpriteNullConstructor();
// Creates a new sprite with dimensions
Sprite SpriteConstructor(int32_t w, int32_t h);
// Calls destructor on the underlying olc sprite
void SpriteDestructor(Sprite* s);
// Loads image into the sprite
RCode SpriteLoadFromFile(Sprite* s, const char* image_file);
// Returns sprite width
int32_t SpriteWidth(Sprite* s);
// Returns sprite height
int32_t SpriteHeight(Sprite* s);
// Returns true if data pointer is not null
bool SpriteHasData(Sprite* s);
// Sets sample mode for the sprite
void SpriteSetSampleMode(Sprite* s, SpriteMode mode);
// Returns sample mode of the sprite
SpriteMode SpriteGetSampleMode(Sprite* s);
// Returns sprite pixel at (x, y)
Pixel SpriteGetPixel(Sprite* s, int32_t x, int32_t y);
// Sets sprite pixel at (x, y)
bool  SpriteSetPixel(Sprite* s, int32_t x, int32_t y, Pixel p);
// Sprite sample for (x, y)
Pixel SpriteSample(Sprite* s, float x, float y);
// Sprite sample BL for (u, v)
Pixel SpriteSampleBL(Sprite* s, float u, float v);

// Returns true if window is currently in focus
bool IsFocused();
// Get the state of a specific keyboard button
HWButton GetKey(Key k);
// Get the state of a specific mouse button
HWButton GetMouse(uint32_t b);
// Get Mouse X coordinate in "pixel" space
int32_t GetMouseX();
// Get Mouse Y coordinate in "pixel" space
int32_t GetMouseY();
// Get Mouse Wheel Delta
int32_t GetMouseWheel();

// Returns the width of the screen in "pixels"
int32_t ScreenWidth();
// Returns the height of the screen in "pixels"
int32_t ScreenHeight();
// Returns the width of the currently selected drawing target in "pixels"
int32_t GetDrawTargetWidth();
// Returns the height of the currently selected drawing target in "pixels"
int32_t GetDrawTargetHeight();
// Returns the currently active draw target
// TODO olc::Sprite* GetDrawTarget();
// Resize the primary screen sprite
void SetScreenSize(int w, int h);
// Specify which Sprite should be the target of drawing functions, use nullptr
// to specify the primary screen
// TODO void SetDrawTarget(Sprite *target);
// Gets the current Frames Per Second
uint32_t GetFPS();

void SetDrawTarget(uint8_t layer);
void EnableLayer(uint8_t layer, bool b);
void SetLayerOffset(uint8_t layer, float x, float y);
void SetLayerScale(uint8_t layer, float x, float y);
void SetLayerTint(uint8_t layer, const Pixel& tint);

// TODO: std::vector<LayerDesc>& GetLayers();
uint32_t CreateLayer();

// Change the pixel mode for different optimisations
// olc::Pixel::NORMAL = No transparency
// olc::Pixel::MASK   = Transparent if alpha is < 255
// olc::Pixel::ALPHA  = Full transparency
void SetPixelMode(PixelMode m);
PixelMode GetPixelMode();
// Change the blend factor form between 0.0f to 1.0f;
void SetPixelBlend(float fBlend);

bool Draw(int32_t x, int32_t y, Pixel p);
// Draws a line from (x1,y1) to (x2,y2)
void DrawLine(int32_t x1, int32_t y1, int32_t x2, int32_t y2, Pixel p, uint32_t pattern);
// Draws a circle located at (x,y) with radius
void DrawCircle(int32_t x, int32_t y, int32_t radius, Pixel p, uint8_t mask);
// Fills a circle located at (x,y) with radius
void FillCircle(int32_t x, int32_t y, int32_t radius, Pixel p);
// Draws a rectangle at (x,y) to (x+w,y+h)
void DrawRect(int32_t x, int32_t y, int32_t w, int32_t h, Pixel p);
// Fills a rectangle at (x,y) to (x+w,y+h)
void FillRect(int32_t x, int32_t y, int32_t w, int32_t h, Pixel p);
// Draws a triangle between points (x1,y1), (x2,y2) and (x3,y3)
void DrawTriangle(int32_t x1, int32_t y1, int32_t x2, int32_t y2, int32_t x3, int32_t y3, Pixel p);
// Flat fills a triangle between points (x1,y1), (x2,y2) and (x3,y3)
void FillTriangle(int32_t x1, int32_t y1, int32_t x2, int32_t y2, int32_t x3, int32_t y3, Pixel p);
// Draws an entire sprite at well in my defencelocation (x,y)
void DrawSprite(int32_t x, int32_t y, Sprite *sprite, uint32_t scale, SpriteFlip flip);
// Draws an area of a sprite at location (x,y), where the
// selected area is (ox,oy) to (ox+w,oy+h)
void DrawPartialSprite(int32_t x, int32_t y, Sprite *sprite, int32_t ox, int32_t oy, int32_t w, int32_t h, uint32_t scale, SpriteFlip flip);
// // Draws a whole decal, with optional scale and tinting
// void DrawDecal(const olc::vf2d& pos, olc::Decal *decal, const olc::vf2d& scale = { 1.0f,1.0f }, const olc::Pixel& tint = olc::WHITE);
// // Draws a region of a decal, with optional scale and tinting
// void DrawPartialDecal(const olc::vf2d& pos, olc::Decal* decal, const olc::vf2d& source_pos, const olc::vf2d& source_size, const olc::vf2d& scale = { 1.0f,1.0f }, const olc::Pixel& tint = olc::WHITE);
//
// void DrawWarpedDecal(olc::Decal* decal, const olc::vf2d(&pos)[4], const olc::Pixel& tint = olc::WHITE);
// void DrawWarpedDecal(olc::Decal* decal, const olc::vf2d* pos, const olc::Pixel& tint = olc::WHITE);
// void DrawWarpedDecal(olc::Decal* decal, const std::array<olc::vf2d, 4>& pos, const olc::Pixel& tint = olc::WHITE);
//
// void DrawRotatedDecal(const olc::vf2d& pos, olc::Decal* decal, const float fAngle, const olc::vf2d& center = { 0.0f, 0.0f }, const olc::vf2d& scale = { 1.0f,1.0f }, const olc::Pixel& tint = olc::WHITE);
//
// void DrawStringDecal(const olc::vf2d& pos, const std::string& sText, const Pixel col = olc::WHITE, const olc::vf2d& scale = { 1.0f, 1.0f });
// void DrawPartialRotatedDecal(const olc::vf2d& pos, olc::Decal* decal, const float fAngle, const olc::vf2d& center, const olc::vf2d& source_pos, const olc::vf2d& source_size, const olc::vf2d& scale = { 1.0f, 1.0f }, const olc::Pixel& tint = olc::WHITE);
//
// void DrawPartialWarpedDecal(olc::Decal* decal, const olc::vf2d(&pos)[4], const olc::vf2d& source_pos, const olc::vf2d& source_size, const olc::Pixel& tint = olc::WHITE);
// void DrawPartialWarpedDecal(olc::Decal* decal, const olc::vf2d* pos, const olc::vf2d& source_pos, const olc::vf2d& source_size, const olc::Pixel& tint = olc::WHITE);
// void DrawPartialWarpedDecal(olc::Decal* decal, const std::array<olc::vf2d, 4>& pos, const olc::vf2d& source_pos, const olc::vf2d& source_size, const olc::Pixel& tint = olc::WHITE);
//
void DrawString(int32_t x, int32_t y, const char* sText, Pixel col, uint32_t scale);
// Clears entire draw target to Pixel
void Clear(Pixel p);
// Clears the rendering back buffer
void ClearBuffer(Pixel p, bool bDepth);

#ifdef __cplusplus
}
#endif