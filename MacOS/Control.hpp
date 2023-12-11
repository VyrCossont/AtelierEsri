#pragma once

#include <functional>

#include <Controls.h>

#include "Resource.hpp"
#include "Window.hpp"

namespace AtelierEsri {

/// A control within a window.
class Control {
public:
  Control(ResourceID resourceID, const Window &owner);
  Control(Control &&src) noexcept;
  Control &operator=(Control &&src) noexcept;
  Control(const Control &src) = delete;
  Control &operator=(const Control &src) = delete;

  void Draw() const;

  void Show() const;
  void Hide() const;

  /// Move control's origin point to this point.
  void Move(Point point) const;
  /// Change control's size to this size (stored as a point).
  void Size(Point size) const;

  void Hilite(ControlPartCode part) const;
  static constexpr ControlPartCode HiliteNone = 0;
  static constexpr ControlPartCode HiliteDisable = 255;

  [[nodiscard]] std::string Title() const;
  void SetTitle(const std::string &title) const;

  /// Handle a mouse down on a given part.
  virtual void HandleMouseDown(Point point, ControlPartCode part) = 0;

protected:
  ~Control() = default;
  void SetRefConToThis();

  ControlRef ref;
};

class Button final : Control {
public:
  Button(ResourceID resourceID, const Window &owner);

  void HandleMouseDown(Point point, ControlPartCode part) override;

  std::function<void(const Button &)> onClick;
};

class ScrollBar final : Control {
public:
  ScrollBar(ResourceID resourceID, const Window &owner);

  [[nodiscard]] int16_t Value() const;
  void SetValue(int16_t value) const;
  [[nodiscard]] int16_t Min() const;
  void SetMin(int16_t min) const;
  [[nodiscard]] int16_t Max() const;
  void SetMax(int16_t max) const;

  /// Scroll by some amount, within the bounds of the scroll bar.
  void ScrollBy(int16_t amount) const;

  void HandleMouseDown(Point point, ControlPartCode part) override;

  std::function<void(const ScrollBar &)> onScrollLineUp;
  std::function<void(const ScrollBar &)> onScrollLineDown;
  std::function<void(const ScrollBar &)> onScrollPageUp;
  std::function<void(const ScrollBar &)> onScrollPageDown;
  /// Argument is value of scroll bar at start of drag.
  std::function<void(const ScrollBar &, int16_t)> onScrollBoxDragged;

private:
  static pascal void ActionProc(ControlRef ref, ControlPartCode part);
  // We don't bother freeing this because there's only ever one.
  static ControlActionUPP ActionProcUPP;
};

} // namespace AtelierEsri
