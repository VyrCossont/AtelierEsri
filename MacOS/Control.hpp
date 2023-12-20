#pragma once

#include <Controls.h>

#include <functional>

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
  virtual ~Control() = default;

  void Draw() const;

  void Show() const;
  void Hide() const;

  [[nodiscard]] Rect Bounds() const;
  void Bounds(const Rect &bounds) const;

  void Hilite(ControlPartCode part) const;
  static constexpr ControlPartCode HiliteNone = 0;
  static constexpr ControlPartCode HiliteDisable = 255;

  [[nodiscard]] std::string Title() const;
  void Title(const std::string &title) const;

  /// Handle a mouse down on a given part.
  virtual void HandleMouseDown(Point point, ControlPartCode part) const = 0;

 protected:
  static ControlRef GetNewControl(ResourceID resourceID, const Window &owner);
  void SetRefConToThis();

  ControlRef ref;
};

class Button final : public Control {
 public:
  Button(ResourceID resourceID, const Window &owner);

  void HandleMouseDown(Point point, ControlPartCode part) const override;

  std::function<void(const Button &)> onClick;
};

/// Common ancestor of checkboxes and radio buttons.
class Toggle : public Control {
 public:
  Toggle(ResourceID resourceID, const Window &owner);

  void HandleMouseDown(Point point, ControlPartCode part) const override;

  [[nodiscard]] bool Checked() const;
  void SetChecked(bool checked) const;

 protected:
  virtual void HandleClick() const = 0;
};

class Checkbox final : public Toggle {
  Checkbox(ResourceID resourceID, const Window &owner);

  std::function<void(const Checkbox &)> onClick;

 protected:
  void HandleClick() const override;
};

class RadioButton final : public Toggle {
  RadioButton(ResourceID resourceID, const Window &owner);

  std::function<void(const RadioButton &)> onClick;

 protected:
  void HandleClick() const override;
};

class ScrollBar final : public Control {
 public:
  ScrollBar(ResourceID resourceID, const Window &owner);

  [[nodiscard]] int16_t Value() const;
  void SetValue(int16_t value) const;
  [[nodiscard]] int16_t Min() const;
  void SetMin(int16_t min) const;
  [[nodiscard]] int16_t Max() const;
  void SetMax(int16_t max) const;

  /// Scroll by some amount, within the bounds of the scroll bar.
  void ScrollBy(int amount) const;

  void HandleMouseDown(Point point, ControlPartCode part) const override;

  std::function<void(const ScrollBar &)> onScrollLineUp;
  std::function<void(const ScrollBar &)> onScrollLineDown;
  std::function<void(const ScrollBar &)> onScrollPageUp;
  std::function<void(const ScrollBar &)> onScrollPageDown;
  /// Argument is value of scroll bar at start of drag.
  std::function<void(const ScrollBar &, int16_t)> onScrollBoxDragged;

  /// Position a horizontal scrollbar according to the HIG:
  /// https://preterhuman.net/macstuff/insidemac/Toolbox/Toolbox-313.html
  void PositionHScrollBar(V2I windowSize, int preScrollAreaWidth = 0) const;

  /// Position a vertical scrollbar according to the HIG:
  /// https://preterhuman.net/macstuff/insidemac/Toolbox/Toolbox-313.html
  void PositionVScrollBar(V2I windowSize, int preScrollAreaHeight = 0) const;

 private:
  static pascal void ActionProc(ControlRef ref, ControlPartCode part);
  // We don't bother freeing this because there's only ever one.
  static ControlActionUPP ActionProcUPP;

  /// Height for horizontal scroll bar or width for vertical scroll bar.
  static constexpr int16_t MinorDimension = 16;
  static constexpr int16_t WindowOverlap = 1;
};

}  // namespace AtelierEsri
