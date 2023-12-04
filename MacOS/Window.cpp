#include <MacTypes.h>
#include <MacWindows.h>

#include "Drawing.hpp"
#include "Exception.hpp"
#include "GWorld.hpp"
#include "Window.hpp"

namespace AtelierEsri {

Window::Window(WindowRef windowRef) : windowRef(windowRef) {}

Window::Window(Window &&src) noexcept {
  windowRef = src.windowRef;
  src.windowRef = nullptr;
}

Window &Window::operator=(Window &&src) noexcept {
  windowRef = src.windowRef;
  src.windowRef = nullptr;
  return *this;
}

Window::~Window() {
  if (windowRef) {
    DisposeWindow(windowRef);
  }
}

const WindowRef Window::allOtherWindows = reinterpret_cast<WindowRef>(-1);

Window Window::Present(ResourceID resourceID, WindowRef inFrontOf) {
  bool hasColorQuickDraw = QD::HasColor();

  WindowRef windowRef;
  if (hasColorQuickDraw) {
    windowRef = GetNewCWindow(resourceID, nullptr, inFrontOf);
  } else {
    windowRef = GetNewWindow(resourceID, nullptr, inFrontOf);
  }
  REQUIRE_NOT_NULL(windowRef);

  return Window(windowRef);
}

GWorld Window::FastGWorld(int16_t w, int16_t h) {
  Rect rect;
  GetWindowPortBounds(windowRef, &rect);
  if (w > 0 && h > 0) {
    // Change GWorld dimensions to something other than the window's dimensions.
    rect.right = static_cast<int16_t>(rect.left + w);
    rect.bottom = static_cast<int16_t>(rect.top + h);
  }

  // Create a GWorld matching the window device, color table, etc.
  GWorldPtr gWorldPtr;
  OS_CHECKED(NewGWorld(&gWorldPtr, 0, &rect, nullptr, nullptr, 0),
             "Couldn't create offscreen GWorld");
  REQUIRE_NOT_NULL(gWorldPtr);

  return GWorld(gWorldPtr);
}

Rect Window::PortBounds() {
  Rect bounds;
  GetWindowPortBounds(windowRef, &bounds);
  return bounds;
}

// TODO: what happens if this is a non-color window?
CGrafPtr Window::Port() {
  CGrafPtr port = GetWindowPort(windowRef);
  REQUIRE_NOT_NULL(port);
  return port;
}

} // namespace AtelierEsri
