#include <MacTypes.h>
#include <MacWindows.h>

#include "Env.hpp"
#include "GWorld.hpp"
#include "Result.hpp"
#include "Window.hpp"

namespace AtelierEsri {

Window::Window(int16_t resourceID) : resourceID(resourceID) {}

Window::~Window() { Dismiss(); }

const WindowRef Window::allOtherWindows = (WindowRef)-1;

Result<std::monostate, Error> Window::Present(WindowRef inFrontOf) {
  GUARD_LET_TRY(bool, hasColorQuickDraw, Env::HasColorQuickDraw());

  if (hasColorQuickDraw) {
    windowRef = GetNewCWindow(resourceID, nullptr, inFrontOf);
  } else {
    windowRef = GetNewWindow(resourceID, nullptr, inFrontOf);
  }

  return Ok(std::monostate());
}

void Window::Dismiss() {
  if (!windowRef) {
    DisposeWindow(windowRef);
  }
}

Result<GWorld> Window::FastGWorld() {
  REQUIRE_NOT_NULL(windowRef);

  // Create a GWorld matching the window device, color table, alignment, etc.
  Rect portRect;
  GetWindowPortBounds(windowRef, &portRect);
  GWorldPtr gWorldPtr;
  OS_CHECKED(NewGWorld(&gWorldPtr, 0, &portRect, nullptr, nullptr, 0),
             "Couldn't create offscreen GWorld");
  if (!gWorldPtr) {
    BAIL("Couldn't create offscreen GWorld: gWorldPtr is null");
  }

  return Ok(GWorld(gWorldPtr));
}

Result<Rect> Window::PortBounds() {
  REQUIRE_NOT_NULL(windowRef);

  Rect bounds;
  GetWindowPortBounds(windowRef, &bounds);

  return Ok(bounds);
}

// TODO: what happens if this is a non-color window?
Result<CGrafPtr> Window::Port() {
  REQUIRE_NOT_NULL(windowRef);

  return Ok(GetWindowPort(windowRef));
}

} // namespace AtelierEsri
