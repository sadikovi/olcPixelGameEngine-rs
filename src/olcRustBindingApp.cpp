#define OLC_PGE_APPLICATION
#include "../olcPixelGameEngine.h"
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
void EnableLayer(uint8_t layer, bool b) { app.EnableLayer(layer, b); }
void SetLayerOffset(uint8_t layer, float x, float y) { app.SetLayerOffset(layer, x, y); }
void SetLayerScale(uint8_t layer, float x, float y) { app.SetLayerScale(layer, x, y); }
void SetLayerTint(uint8_t layer, const Pixel& tint) { app.SetLayerTint(layer, TO_OLC_PIXEL(tint)); }

uint32_t CreateLayer() { return app.CreateLayer(); }

void SetPixelMode(PixelMode m) { app.SetPixelMode(m); }
PixelMode GetPixelMode() { return app.GetPixelMode(); }
void SetPixelBlend(float fBlend) { app.SetPixelBlend(fBlend); }

bool Draw(int32_t x, int32_t y, Pixel p) { return app.Draw(x, y, TO_OLC_PIXEL(p)); }
void DrawLine(int32_t x1, int32_t y1, int32_t x2, int32_t y2, Pixel p, uint32_t pattern) { app.DrawLine(x1, y1, x2, y2, TO_OLC_PIXEL(p), pattern); }
void DrawCircle(int32_t x, int32_t y, int32_t radius, Pixel p, uint8_t mask) { app.DrawCircle(x, y, radius, TO_OLC_PIXEL(p), mask); }
void FillCircle(int32_t x, int32_t y, int32_t radius, Pixel p) { app.FillCircle(x, y, radius, TO_OLC_PIXEL(p)); }
void DrawRect(int32_t x, int32_t y, int32_t w, int32_t h, Pixel p) { app.DrawRect(x, y, w, h, TO_OLC_PIXEL(p)); }
void FillRect(int32_t x, int32_t y, int32_t w, int32_t h, Pixel p) { app.FillRect(x, y, w, h, TO_OLC_PIXEL(p)); }
void DrawTriangle(int32_t x1, int32_t y1, int32_t x2, int32_t y2, int32_t x3, int32_t y3, Pixel p) { app.DrawTriangle(x1, y1, x2, y2, x3, y3, TO_OLC_PIXEL(p)); }
void FillTriangle(int32_t x1, int32_t y1, int32_t x2, int32_t y2, int32_t x3, int32_t y3, Pixel p) { app.FillTriangle(x1, y1, x2, y2, x3, y3, TO_OLC_PIXEL(p)); }

void DrawString(int32_t x, int32_t y, const char* sText, Pixel col, uint32_t scale) { app.DrawString(x, y, sText, TO_OLC_PIXEL(col), scale); }
void Clear(Pixel p) { app.Clear(TO_OLC_PIXEL(p)); }
void ClearBuffer(Pixel p, bool bDepth) { app.ClearBuffer(TO_OLC_PIXEL(p), bDepth); }

#ifdef __cplusplus
}
#endif
