#include <MacTypes.h>
#include <MacWindows.h>

#include "Env.hpp"
#include "GWorld.hpp"
#include "Result.hpp"
#include "Window.hpp"

namespace AtelierEsri {

Window::Window(WindowRef windowRef) noexcept : windowRef(windowRef) {}

Window::Window(Window &&src) noexcept {
  windowRef = src.windowRef;
  src.windowRef = nullptr;
}

Window &Window::operator=(Window &&src) noexcept {
  windowRef = src.windowRef;
  src.windowRef = nullptr;
  return *this;
}

Window::~Window() noexcept {
  if (windowRef) {
    DisposeWindow(windowRef);
  }
}

const WindowRef Window::allOtherWindows = reinterpret_cast<WindowRef>(-1);

Result<Window> Window::Present(ResourceID resourceID,
                               WindowRef inFrontOf) noexcept {
  GUARD_LET_TRY(bool, hasColorQuickDraw, Env::HasColorQuickDraw());

  WindowRef windowRef;
  if (hasColorQuickDraw) {
    windowRef = GetNewCWindow(resourceID, nullptr, inFrontOf);
  } else {
    windowRef = GetNewWindow(resourceID, nullptr, inFrontOf);
  }
  REQUIRE_NOT_NULL(windowRef);

  return Ok(Window(windowRef));
}

Result<GWorld> Window::FastGWorld(int16_t w, int16_t h) noexcept {
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

  return Ok(GWorld(gWorldPtr));
}

Rect Window::PortBounds() noexcept {
  Rect bounds;
  GetWindowPortBounds(windowRef, &bounds);
  return bounds;
}

// TODO: what happens if this is a non-color window?
Result<CGrafPtr> Window::Port() noexcept {
  CGrafPtr port = GetWindowPort(windowRef);
  REQUIRE_NOT_NULL(port);
  return Ok(port);
}

} // namespace AtelierEsri
