#pragma once

#include <MacTypes.h>
#include <MacWindows.h>

#include "Error.hpp"
#include "GWorld.hpp"
#include "Result.hpp"

namespace AtelierEsri {

class Window {
public:
  explicit Window(int16_t resourceID);
  ~Window();
  /// Valid only for Present(inFrontOf).
  static const WindowRef allOtherWindows;
  Result<std::monostate> Present(WindowRef inFrontOf = allOtherWindows);
  void Dismiss();
  /// Get a GWorld optimized for copy to this window.
  /// If `w` or `h` are not zero, sets a custom size.
  Result<GWorld> FastGWorld(int16_t w = 0, int16_t h = 0);
  Result<Rect> PortBounds();
  Result<CGrafPtr> Port();

private:
  int16_t resourceID;
  WindowRef windowRef = nullptr;
};

} // namespace AtelierEsri
