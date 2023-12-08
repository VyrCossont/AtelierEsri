#pragma once

#include <Controls.h>
#include <MacTypes.h>
#include <MacWindows.h>

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

  [[nodiscard]] Rect PortBounds() const;
  [[nodiscard]] CGrafPtr Port() const;

  /// Load a control resource and attach it to this window.
  /// Controls are disposed when their window is disposed,
  /// so we don't need a managed handle type for them.
  [[nodiscard]] ControlHandle AddControl(ResourceID resourceID) const;

private:
  explicit Window(WindowRef windowRef);
  WindowRef windowRef;
};

} // namespace AtelierEsri
