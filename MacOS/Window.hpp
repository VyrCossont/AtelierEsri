#pragma once

#include <MacTypes.h>
#include <MacWindows.h>

#include "Error.hpp"
#include "GWorld.hpp"
#include "Resource.hpp"
#include "Result.hpp"

namespace AtelierEsri {

/// A window created from a resource.
/// Doesn't use `Resource` because the getter function takes a second parameter.
class Window {
public:
  static Result<Window> Present(ResourceID resourceID,
                                WindowRef inFrontOf = allOtherWindows) noexcept;
  Window(Window &&src) noexcept;
  Window &operator=(Window &&src) noexcept;
  Window(const Window &src) = delete;
  Window &operator=(const Window &src) = delete;
  ~Window() noexcept;

  /// Valid only for `Present(inFrontOf)`.
  static const WindowRef allOtherWindows;

  /// Get a GWorld optimized for copy to this window.
  /// If `w` or `h` are not zero, sets a custom size.
  Result<GWorld> FastGWorld(int16_t w = 0, int16_t h = 0) noexcept;

  Rect PortBounds() noexcept;
  Result<CGrafPtr> Port() noexcept;

private:
  explicit Window(WindowRef windowRef) noexcept;
  WindowRef windowRef;
};

} // namespace AtelierEsri
