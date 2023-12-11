#pragma once

#include <MacTypes.h>

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
  [[nodiscard]] GWorld FastGWorld(int16_t w = 0, int16_t h = 0) const;

  [[nodiscard]] WindowRef Unmanaged() const;
  [[nodiscard]] Rect PortBounds() const;
  [[nodiscard]] CGrafPtr Port() const;

  /// Handle a mouse down event on this window.
  /// Assumes that the event is actually in this window
  /// and that the point is in the global coordinate system.
  void HandleMouseDown(Point point);

  /// Make this window the current QuickDraw graphics port.
  [[nodiscard]] GWorldActiveGuard MakeActive() const;

private:
  explicit Window(WindowRef windowRef);
  void SetRefConToThis();

  WindowRef windowRef;
};

} // namespace AtelierEsri
