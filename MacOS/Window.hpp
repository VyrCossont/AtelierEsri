#pragma once

#include <MacTypes.h>
#include <MacWindows.h>

#include "Exception.hpp"
#include "GWorld.hpp"
#include "Resource.hpp"

namespace AtelierEsri {

/// A window created from a resource.
/// Doesn't use `Resource` because the getter function takes a second parameter.
class Window {
public:
  static Window Present(ResourceID resourceID,
                        WindowRef inFrontOf = allOtherWindows);
  Window(Window &&src) noexcept;
  Window &operator=(Window &&src) noexcept;
  Window(const Window &src) = delete;
  Window &operator=(const Window &src) = delete;
  ~Window();

  /// Valid only for `Present(inFrontOf)`.
  static const WindowRef allOtherWindows;

  /// Get a GWorld optimized for copy to this window.
  /// If `w` or `h` are not zero, sets a custom size.
  GWorld FastGWorld(int16_t w = 0, int16_t h = 0);

  Rect PortBounds();
  CGrafPtr Port();

private:
  explicit Window(WindowRef windowRef);
  WindowRef windowRef;
};

} // namespace AtelierEsri
