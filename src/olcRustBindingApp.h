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
  CONSTRUCT_FAIL,
  CONSTRUCT_NO_FILE,
  START_FAIL,
  START_NO_FILE,
  OK
} RCode;

typedef struct {
  uint8_t r;
  uint8_t g;
  uint8_t b;
  uint8_t a;
} Pixel;

typedef olc::Pixel::Mode PixelMode;

typedef struct { int32_t x; int32_t y; } Vi2d;
typedef struct { uint32_t x; uint32_t y; } Vu2d;
typedef struct { float x; float y; } Vf2d;
typedef struct { double x; double y; } Vd2d;

#define OLC_PIXEL(p) (olc::Pixel(p.r, p.g, p.b, p.a))

// Useful utility functions
int32_t c_rand();

// Starts the main game loop.
// Default values: full_screen = false and vsync = false
RCode start(const char* name, void* binding, int32_t screen_w, int32_t screen_h, int32_t pixel_w, int32_t pixel_h, bool full_screen, bool vsync);

// olcPixelGameEngine API

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
// // Draws an entire sprite at well in my defencelocation (x,y)
// void DrawSprite(int32_t x, int32_t y, Sprite *sprite, uint32_t scale = 1, uint8_t flip = olc::Sprite::NONE);
// void DrawSprite(const olc::vi2d& pos, Sprite *sprite, uint32_t scale = 1, uint8_t flip = olc::Sprite::NONE);
// // Draws an area of a sprite at location (x,y), where the
// // selected area is (ox,oy) to (ox+w,oy+h)
// void DrawPartialSprite(int32_t x, int32_t y, Sprite *sprite, int32_t ox, int32_t oy, int32_t w, int32_t h, uint32_t scale = 1, uint8_t flip = olc::Sprite::NONE);
// void DrawPartialSprite(const olc::vi2d& pos, Sprite *sprite, const olc::vi2d& sourcepos, const olc::vi2d& size, uint32_t scale = 1, uint8_t flip = olc::Sprite::NONE);
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
