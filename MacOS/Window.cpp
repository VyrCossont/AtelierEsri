#include <MacTypes.h>
#include <MacWindows.h>

#include "Env.hpp"
#include "GWorld.hpp"
#include "Result.hpp"
#include "Window.hpp"

namespace AtelierEsri {

Window::Window(SInt16 resourceID) {
  this->resourceID = resourceID;
  this->windowRef = nil;
}

Window::~Window() { Dismiss(); }

const WindowRef Window::inFrontOfAllOtherWindows = (WindowRef)-1;

Result<std::monostate, OSErr> Window::Present() {
  Result<bool, OSErr> result = Env::HasColorQuickDraw();
  if (result.is_err()) {
    return Err(result.err_value());
  }
  bool hasColorQuickDraw = result.ok_value();

  if (hasColorQuickDraw) {
    windowRef = GetNewCWindow(resourceID, nil, inFrontOfAllOtherWindows);
  } else {
    windowRef = GetNewWindow(resourceID, nil, inFrontOfAllOtherWindows);
  }

  return Ok(std::monostate());
}

void Window::Dismiss() {
  if (windowRef != nil) {
    DisposeWindow(windowRef);
  }
}

Result<GWorld, OSErr> Window::FastGWorld() {
  if (windowRef == nil) {
    return Err((OSErr)__LINE__);
  }

  Rect portRect;
  GWorldPtr gWorldPtr;
  // Creare a GWorld matching the window device, color table, alignment, etc.
  GetWindowPortBounds(windowRef, &portRect);
  OSErr error = NewGWorld(&gWorldPtr, 0, &portRect, nil, nil, 0);
  if (error != noErr) {
    return Err(error);
  }
  if (gWorldPtr == nil) {
    return Err((OSErr)__LINE__);
  }
  return Ok(GWorld(gWorldPtr));
}

Result<Rect, OSErr> Window::PortBounds() {
  if (windowRef == nil) {
    return Err((OSErr)__LINE__);
  }

  Rect bounds;
  GetWindowPortBounds(windowRef, &bounds);
  return Ok(bounds);
}

// TODO: what happens if this is a non-color window?
Result<CGrafPtr, OSErr> Window::Port() {
  if (windowRef == nil) {
    return Err((OSErr)__LINE__);
  }

  return Ok(GetWindowPort(windowRef));
}

} // namespace AtelierEsri
