#pragma once

#include <MacTypes.h>
#include <MacWindows.h>

#include <functional>

#include "GWorld.hpp"
#include "Resource.hpp"

namespace AtelierEsri {

/// A window created from a resource.
/// Doesn't use `Resource` because the getter function takes a second parameter.
class Window {
 public:
  static Window Present(
      ResourceID resourceID, WindowRef inFrontOf = AllOtherWindows
  );
  Window(Window &&src) noexcept;
  Window &operator=(Window &&src) noexcept;
  Window(const Window &src) = delete;
  Window &operator=(const Window &src) = delete;
  ~Window();

  /// Valid only for `Present(inFrontOf)`.
  static const WindowRef AllOtherWindows;

  /// Get a GWorld optimized for copy to this window.
  /// If `w` or `h` are not zero, sets a custom size.
  [[nodiscard]] GWorld FastGWorld(int16_t w = 0, int16_t h = 0) const;

  /// Copy from a GWorld into this window.
  void CopyFrom(const GWorld &gWorld, Rect gWorldRect, Rect windowRect) const;

  [[nodiscard]] WindowRef Unmanaged() const;
  [[nodiscard]] Rect PortBounds() const;
  [[nodiscard]] CGrafPtr Port() const;

  /// Handle a mouse down event on this window.
  /// Assumes that the event is actually in this window
  /// and that the point is in the global coordinate system.
  void HandleMouseDown(Point point, WindowPartCode part) const;

  /// Make this window the current QuickDraw graphics port.
  [[nodiscard]] GWorldActiveGuard MakeActivePort() const;

  // TODO: track mouse up so we can convert this to onContentClick
  /// Point is in window-local coordinates.
  std::function<void(const Window &, Point point)> onContentMouseDown;

 private:
  explicit Window(WindowRef ref);
  void SetRefConToThis();

  WindowRef ref;
};

}  // namespace AtelierEsri
