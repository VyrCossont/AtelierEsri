#include "Control.hpp"

#include <ControlDefinitions.h>

namespace AtelierEsri {

Control::Control(const ResourceID resourceID, const Window &owner) {
  // ReSharper disable once CppLocalVariableMayBeConst
  ControlRef ref = GetNewControl(resourceID, owner.Unmanaged());
  REQUIRE_NOT_NULL(ref);
  this->ref = ref;
  SetRefConToThis();
}

Control::Control(Control &&src) noexcept : ref(src.ref) {
  src.ref = nullptr;
  SetRefConToThis();
}

Control &Control::operator=(Control &&src) noexcept {
  ref = src.ref;
  src.ref = nullptr;
  SetRefConToThis();
  return *this;
}

void Control::SetRefConToThis() {
  SetControlReference(ref, reinterpret_cast<int32_t>(this));
}

Button::Button(const ResourceID resourceID, const Window &owner)
    : Control(resourceID, owner) {}

void Button::HandleMouseDown(const Point point, const ControlPartCode part) {
  if (part == kControlButtonPart) {
    if (const ControlPartCode mouseUpPart = TrackControl(ref, point, nullptr)) {
      if (mouseUpPart == kControlButtonPart) {
        if (onClick) {
          onClick(*this);
        }
      }
    }
  }
}

ScrollBar::ScrollBar(const ResourceID resourceID, const Window &owner)
    : Control(resourceID, owner) {}

int16_t ScrollBar::Value() const { return GetControlValue(ref); }

void ScrollBar::SetValue(const int16_t value) const {
  SetControlValue(ref, value);
}

int16_t ScrollBar::Min() const { return GetControlMinimum(ref); }

void ScrollBar::SetMin(const int16_t min) const { SetControlMinimum(ref, min); }

int16_t ScrollBar::Max() const { return GetControlMaximum(ref); }

void ScrollBar::SetMax(const int16_t max) const { SetControlMaximum(ref, max); }

// ReSharper disable once CppParameterMayBeConst
void ScrollBar::ActionProc(ControlRef ref, const ControlPartCode part) {
  if (!ref) {
    return;
  }

  const auto scrollBar =
      reinterpret_cast<ScrollBar *>(GetControlReference(ref));
  if (!scrollBar) {
    return;
  }

  switch (part) {
  case kControlUpButtonPart:
    if (scrollBar->onScrollLineUp) {
      scrollBar->onScrollLineUp(*scrollBar);
    }
    break;

  case kControlDownButtonPart:
    if (scrollBar->onScrollLineDown) {
      scrollBar->onScrollLineDown(*scrollBar);
    }
    break;

  case kControlPageUpPart:
    if (scrollBar->onScrollPageUp) {
      scrollBar->onScrollPageUp(*scrollBar);
    }
    break;

  case kControlPageDownPart:
    if (scrollBar->onScrollPageDown) {
      scrollBar->onScrollPageDown(*scrollBar);
    }
    break;

  default:
    break;
  }
}

ControlActionUPP ScrollBar::ActionProcUPP = NewControlActionUPP(ActionProc);

void ScrollBar::HandleMouseDown(const Point point, const ControlPartCode part) {
  // https://preterhuman.net/macstuff/insidemac/Toolbox/Toolbox-312.html#MARKER-9-271
  switch (part) {
  case kControlIndicatorPart: {
    const int16_t beginValue = Value();
    if (kControlIndicatorPart == TrackControl(ref, point, nullptr)) {
      if (onScrollBoxDragged) {
        onScrollBoxDragged(*this, beginValue);
      }
    }
  } break;

  case kControlUpButtonPart:
  case kControlDownButtonPart:
  case kControlPageUpPart:
  case kControlPageDownPart:
    TrackControl(ref, point, ActionProcUPP);
    break;

  default:
    break;
  }
}

} // namespace AtelierEsri