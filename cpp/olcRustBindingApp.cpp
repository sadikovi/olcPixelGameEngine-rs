#define OLC_PGE_APPLICATION
#include "olcPixelGameEngine.h"
#include "olcRustBindingApp.h"

class RustBindingApp : public olc::PixelGameEngine
{
public:
  void* binding;

public:
  RustBindingApp()
  {
    sAppName = "Rust Binding App";
  }
  ~RustBindingApp()
  {}

public:
  bool inline OnUserCreate() override
  {
    return onUserCreate(this->binding);
  }

  bool inline OnUserUpdate(float fElapsedTime) override
  {
    return onUserUpdate(this->binding, fElapsedTime);
  }

  bool inline OnUserDestroy() override
  {
    return onUserDestroy(this->binding);
  }
};

#ifdef __cplusplus
extern "C" {
#endif

// C++ srand utility function
void c_srand(uint32_t seed) {
  srand(seed);
}

// C++ rand utility function
int32_t c_rand() {
  return rand();
}

// Should be available for the duration of the application.
RustBindingApp app;

RCode start(const char* name, void* binding, int32_t screen_w, int32_t screen_h, int32_t pixel_w, int32_t pixel_h, bool full_screen, bool vsync) {
  app.sAppName = name;
  app.binding = binding;

  olc::rcode res;

  res = app.Construct(screen_w, screen_h, pixel_w, pixel_h, full_screen, vsync);
  if (res) {
    res = app.Start();
  }
  return TO_RCODE(res);
}

Sprite SpriteNullConstructor() { return TO_SPRITE(new olc::Sprite()); }
Sprite SpriteConstructor(int32_t w, int32_t h) { return TO_SPRITE(new olc::Sprite(w, h)); }
void SpriteDestructor(Sprite* s) { delete s->olc_sprite; }
RCode SpriteLoadFromFile(Sprite* s, const char* image_file) { return TO_RCODE(TO_OLC_SPRITE(s)->LoadFromFile(image_file)); }
int32_t SpriteWidth(Sprite* s) { return TO_OLC_SPRITE(s)->width; }
int32_t SpriteHeight(Sprite* s) { return TO_OLC_SPRITE(s)->height; }
bool SpriteHasData(Sprite* s) { return TO_OLC_SPRITE(s)->pColData != nullptr; }
void SpriteSetSampleMode(Sprite* s, SpriteMode mode) { TO_OLC_SPRITE(s)->SetSampleMode(mode); }
SpriteMode SpriteGetSampleMode(Sprite* s) { return TO_OLC_SPRITE(s)->modeSample; }
Pixel SpriteGetPixel(Sprite* s, int32_t x, int32_t y) { return TO_PIXEL(TO_OLC_SPRITE(s)->GetPixel(x, y)); }
bool  SpriteSetPixel(Sprite* s, int32_t x, int32_t y, Pixel p) { return TO_OLC_SPRITE(s)->SetPixel(x, y, TO_OLC_PIXEL(p)); }
Pixel SpriteSample(Sprite* s, float x, float y) { return TO_PIXEL(TO_OLC_SPRITE(s)->Sample(x, y)); }
Pixel SpriteSampleBL(Sprite* s, float u, float v) { return TO_PIXEL(TO_OLC_SPRITE(s)->SampleBL(u, v)); }

Decal DecalConstructor(Sprite* s) { return TO_DECAL(new olc::Decal(TO_OLC_SPRITE(s))); }
int32_t DecalId(Decal* d) { return TO_OLC_DECAL(d)->id; }
Vf2d DecalScale(Decal* d) { return TO_VF2D(TO_OLC_DECAL(d)->vUVScale); }
void DecalDestructor(Decal* d) { delete d->olc_decal; }

bool IsFocused() { return app.IsFocused(); }
HWButton GetKey(Key k) { return TO_HWBUTTON(app.GetKey(k)); }
HWButton GetMouse(uint32_t b) { return TO_HWBUTTON(app.GetMouse(b)); }
int32_t GetMouseX() { return app.GetMouseX(); }
int32_t GetMouseY() { return app.GetMouseY(); }
int32_t GetMouseWheel() { return app.GetMouseWheel(); }

int32_t ScreenWidth() { return app.ScreenWidth(); }
int32_t ScreenHeight() { return app.ScreenHeight(); }
int32_t GetDrawTargetWidth() { return app.GetDrawTargetWidth(); }
int32_t GetDrawTargetHeight() { return app.GetDrawTargetHeight(); }
void SetScreenSize(int w, int h) { app.SetScreenSize(w, h); }
uint32_t GetFPS() { return app.GetFPS(); }

void SetDrawTarget(uint8_t layer) { app.SetDrawTarget(layer); }
void SetPrimaryDrawTarget() { app.SetDrawTarget(nullptr); }
LayerDesc GetDrawTarget(uint8_t layer) {
  if (layer < app.GetLayers().size()) {
    return TO_LAYER_DESC(layer, &app.GetLayers()[layer]);
  } else {
    LayerDesc l;
    return l;
  }
}
LayerDesc GetPrimaryDrawTarget() { return GetDrawTarget(0); }
void EnableLayer(uint8_t layer, bool b) { app.EnableLayer(layer, b); }
void SetLayerOffset(uint8_t layer, float x, float y) { app.SetLayerOffset(layer, x, y); }
void SetLayerScale(uint8_t layer, float x, float y) { app.SetLayerScale(layer, x, y); }
void SetLayerTint(uint8_t layer, const Pixel& tint) { app.SetLayerTint(layer, TO_OLC_PIXEL(tint)); }
uint32_t CreateLayer() { return app.CreateLayer(); }

void SetPixelMode(PixelMode m) { app.SetPixelMode(m); }
PixelMode GetPixelMode() { return app.GetPixelMode(); }
void SetPixelBlend(float fBlend) { app.SetPixelBlend(fBlend); }

// Drawing methods

bool Draw(int32_t x, int32_t y, Pixel p) {
  return app.Draw(x, y, TO_OLC_PIXEL(p));
}

void DrawLine(int32_t x1, int32_t y1, int32_t x2, int32_t y2, Pixel p, uint32_t pattern) {
  app.DrawLine(x1, y1, x2, y2, TO_OLC_PIXEL(p), pattern);
}

void DrawCircle(int32_t x, int32_t y, int32_t radius, Pixel p, uint8_t mask) {
  app.DrawCircle(x, y, radius, TO_OLC_PIXEL(p), mask);
}

void FillCircle(int32_t x, int32_t y, int32_t radius, Pixel p) {
  app.FillCircle(x, y, radius, TO_OLC_PIXEL(p));
}

void DrawRect(int32_t x, int32_t y, int32_t w, int32_t h, Pixel p) {
  app.DrawRect(x, y, w, h, TO_OLC_PIXEL(p));
}

void FillRect(int32_t x, int32_t y, int32_t w, int32_t h, Pixel p) {
  app.FillRect(x, y, w, h, TO_OLC_PIXEL(p));
}

void DrawTriangle(int32_t x1, int32_t y1, int32_t x2, int32_t y2, int32_t x3, int32_t y3, Pixel p) {
  app.DrawTriangle(x1, y1, x2, y2, x3, y3, TO_OLC_PIXEL(p));
}

void FillTriangle(int32_t x1, int32_t y1, int32_t x2, int32_t y2, int32_t x3, int32_t y3, Pixel p) {
  app.FillTriangle(x1, y1, x2, y2, x3, y3, TO_OLC_PIXEL(p));
}

void DrawSprite(int32_t x, int32_t y, Sprite *sprite, uint32_t scale, SpriteFlip flip) {
  app.DrawSprite(x, y, TO_OLC_SPRITE(sprite), scale, flip);
}

void DrawPartialSprite(int32_t x, int32_t y, Sprite *sprite, int32_t ox, int32_t oy, int32_t w, int32_t h, uint32_t scale, SpriteFlip flip) {
  app.DrawPartialSprite(x, y, TO_OLC_SPRITE(sprite), ox, oy, w, h, scale, flip);
}

void DrawDecal(const Vf2d& pos, Decal *decal, const Vf2d& scale, const Pixel& tint) {
  app.DrawDecal(TO_OLC_VF2D(pos), TO_OLC_DECAL(decal), TO_OLC_VF2D(scale), TO_OLC_PIXEL(tint));
}

void DrawPartialDecal(const Vf2d& pos, Decal* decal, const Vf2d& source_pos, const Vf2d& source_size, const Vf2d& scale, const Pixel& tint) {
  app.DrawPartialDecal(TO_OLC_VF2D(pos), TO_OLC_DECAL(decal), TO_OLC_VF2D(source_pos), TO_OLC_VF2D(source_size), TO_OLC_VF2D(scale), TO_OLC_PIXEL(tint));
}

void DrawWarpedDecal(Decal* decal, const Vf2d* pos, const Pixel& tint) {
  olc::vf2d pos0[4];
  pos0[0] = TO_OLC_VF2D(pos[0]);
  pos0[1] = TO_OLC_VF2D(pos[1]);
  pos0[2] = TO_OLC_VF2D(pos[2]);
  pos0[3] = TO_OLC_VF2D(pos[3]);
  app.DrawWarpedDecal(TO_OLC_DECAL(decal), pos0, TO_OLC_PIXEL(tint));
}

void DrawPartialWarpedDecal(Decal* decal, const Vf2d* pos, const Vf2d& source_pos, const Vf2d& source_size, const Pixel& tint) {
  olc::vf2d pos0[4];
  pos0[0] = TO_OLC_VF2D(pos[0]);
  pos0[1] = TO_OLC_VF2D(pos[1]);
  pos0[2] = TO_OLC_VF2D(pos[2]);
  pos0[3] = TO_OLC_VF2D(pos[3]);
  app.DrawPartialWarpedDecal(TO_OLC_DECAL(decal), pos0, TO_OLC_VF2D(source_pos), TO_OLC_VF2D(source_size), TO_OLC_PIXEL(tint));
}

void DrawRotatedDecal(const Vf2d& pos, Decal* decal, const float angle, const Vf2d& center, const Vf2d& scale, const Pixel& tint) {
  app.DrawRotatedDecal(TO_OLC_VF2D(pos), TO_OLC_DECAL(decal), angle, TO_OLC_VF2D(center), TO_OLC_VF2D(scale), TO_OLC_PIXEL(tint));
}

void DrawPartialRotatedDecal(const Vf2d& pos, Decal* decal, const float angle, const Vf2d& center, const Vf2d& source_pos, const Vf2d& source_size, const Vf2d& scale, const Pixel& tint) {
  app.DrawPartialRotatedDecal(TO_OLC_VF2D(pos), TO_OLC_DECAL(decal), angle, TO_OLC_VF2D(center), TO_OLC_VF2D(source_pos), TO_OLC_VF2D(source_size), TO_OLC_VF2D(scale), TO_OLC_PIXEL(tint));
}

void DrawStringDecal(const Vf2d& pos, const char* sText, const Pixel col, const Vf2d& scale) {
  app.DrawStringDecal(TO_OLC_VF2D(pos), sText, TO_OLC_PIXEL(col), TO_OLC_VF2D(scale));
}

void DrawString(int32_t x, int32_t y, const char* sText, Pixel col, uint32_t scale) {
  app.DrawString(x, y, sText, TO_OLC_PIXEL(col), scale);
}

void Clear(Pixel p) { app.Clear(TO_OLC_PIXEL(p)); }
void ClearBuffer(Pixel p, bool bDepth) { app.ClearBuffer(TO_OLC_PIXEL(p), bDepth); }

#ifdef __cplusplus
}
#endif
