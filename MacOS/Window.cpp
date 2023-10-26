#include <MacTypes.h>
#include <MacWindows.h>

#include "Window.hpp"

namespace AtelierEsri {

Window::Window(SInt16 resourceID) {
  this->resourceID = resourceID;
  this->windowRef = nil;
}

Window::~Window() { Dismiss(); }

const WindowRef Window::inFrontOfAllOtherWindows = (WindowRef)-1;

void Window::Present() {
  windowRef = GetNewCWindow(resourceID, nil, inFrontOfAllOtherWindows);
}

void Window::Dismiss() {
  if (windowRef != nil) {
    DisposeWindow(windowRef);
  }
}

} // namespace AtelierEsri
