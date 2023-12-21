#pragma once

#include <MacTypes.h>
#include <MacWindows.h>

#include <functional>

#include "Drawing.hpp"
#include "GWorld.hpp"
#include "Resource.hpp"

namespace AtelierEsri {

/// A window created from a resource.
/// Doesn't use `Resource` because the getter function takes a second parameter.
class Window {
 public:
  explicit Window(
      ResourceID resourceID, WindowRef behind = InFrontOfAllOtherWindows
  );
  Window(Window &&src) noexcept;
  Window &operator=(Window &&src) noexcept;
  Window(const Window &src) = delete;
  Window &operator=(const Window &src) = delete;
  ~Window();

  /// Valid only for `Window(…, behind)`.
  static const WindowRef InFrontOfAllOtherWindows;
  /// Valid only for `Window(…, behind)`.
  static const WindowRef BehindAllOtherWindows;

  /// Get a GWorld optimized for copy to this window.
  /// If `w` or `h` are not zero, sets a custom size.
  [[nodiscard]] GWorld FastGWorld(int16_t w = 0, int16_t h = 0) const;

  /// Copy from a GWorld into this window.
  void CopyFrom(const GWorld &gWorld, Rect gWorldRect, Rect windowRect) const;

  /// Draw all controls attached to this window.
  void DrawControls() const;

  /// Draw visible controls attached to this window.
  void UpdateControls(RgnHandle updateRegion) const;

  [[nodiscard]] WindowRef Unmanaged() const;
  [[nodiscard]] Rect PortBounds() const;
  [[nodiscard]] CGrafPtr Port() const;

  /// Make this window the current QuickDraw graphics port.
  [[nodiscard]] GWorldActiveGuard MakeActivePort() const;

  /// Handle a mouse down event on this window.
  /// Assumes that the event is actually in this window
  /// and that the point is in the global coordinate system.
  void HandleMouseDown(Point point, WindowPartCode part) const;

  /// https://preterhuman.net/macstuff/insidemac/Toolbox/Toolbox-40.html#MARKER-9-214
  void HandleActivate() const;
  void HandleDeactivate() const;

  /// Draw a window.
  /// https://preterhuman.net/macstuff/insidemac/Toolbox/Toolbox-40.html#MARKER-2-175
  void HandleUpdate() const;

  // TODO: track mouse up so we can convert this to onContentClick
  /// Point is in window-local coordinates.
  std::function<void(const Window &, Point point)> onContentMouseDown;

  std::function<void(const Window &)> onClose;

  std::function<void(const Window &, V2I prevSize)> onResize;

  std::function<void(const Window &)> onActivate;
  std::function<void(const Window &)> onDeactivate;

  /// Called *before* we draw the window's controls.
  std::function<void(const Window &)> onUpdate;

  [[nodiscard]] bool GrowIcon() const;
  void GrowIcon(bool value);

 private:
  static WindowRef GetNewWindow(ResourceID resourceID, WindowRef behind);
  void SetRefConToThis();

  WindowRef ref;
  bool growIcon = false;
};

}  // namespace AtelierEsri
