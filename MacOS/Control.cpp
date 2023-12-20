#include "Control.hpp"

#include <ControlDefinitions.h>

#include <algorithm>

#include "Strings.hpp"

namespace AtelierEsri {

Control::Control(const ResourceID resourceID, const Window &owner)
    : ref(GetNewControl(resourceID, owner)) {
  SetRefConToThis();
}

ControlRef Control::GetNewControl(
    const ResourceID resourceID, const Window &owner
) {
  // ReSharper disable once CppLocalVariableMayBeConst
  ControlRef ref = ::GetNewControl(resourceID, owner.Unmanaged());
  REQUIRE_NOT_NULL(ref);
  return ref;
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

void Control::Draw() const { Draw1Control(ref); }

void Control::Show() const { ShowControl(ref); }

void Control::Hide() const { HideControl(ref); }

Rect Control::Bounds() const {
#if TARGET_API_MAC_CARBON
  Rect bounds;
  GetControlBounds(ref, &bounds);
  return bounds;
#else
  return ref[0]->contrlRect;
#endif
}

void Control::Bounds(const Rect &bounds) const {
#if TARGET_API_MAC_CARBON
  SetControlBounds(ref, &bounds);
#else
  ref[0]->contrlRect = bounds;
#endif
}

void Control::Hilite(const ControlPartCode part) const {
  HiliteControl(ref, part);
}

std::string Control::Title() const {
  Str255 pStr;
  GetControlTitle(ref, pStr);
  return Strings::FromPascal(pStr);
}

void Control::Title(const std::string &title) const {
  Str255 pStr;
  Strings::ToPascal(title, pStr);
  SetControlTitle(ref, pStr);
}

void Control::SetRefConToThis() {
  SetControlReference(ref, reinterpret_cast<int32_t>(this));
}

Button::Button(const ResourceID resourceID, const Window &owner)
    : Control(resourceID, owner) {}

void Button::HandleMouseDown(const Point point, const ControlPartCode part)
    const {
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

Toggle::Toggle(const ResourceID resourceID, const Window &owner)
    : Control(resourceID, owner) {}

void Toggle::HandleMouseDown(const Point point, const ControlPartCode part)
    const {
  if (part == kControlCheckBoxPart) {
    if (const ControlPartCode mouseUpPart = TrackControl(ref, point, nullptr)) {
      if (mouseUpPart == kControlCheckBoxPart) {
        HandleClick();
      }
    }
  }
}

bool Toggle::Checked() const { return GetControlValue(ref) != 0; }

void Toggle::SetChecked(const bool checked) const {
  SetControlValue(ref, checked ? 1 : 0);
}

Checkbox::Checkbox(const ResourceID resourceID, const Window &owner)
    : Toggle(resourceID, owner) {}

void Checkbox::HandleClick() const {
  if (onClick) {
    onClick(*this);
  }
}

RadioButton::RadioButton(const ResourceID resourceID, const Window &owner)
    : Toggle(resourceID, owner) {}

void RadioButton::HandleClick() const {
  if (onClick) {
    onClick(*this);
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

void ScrollBar::ScrollBy(const int amount) const {
  const int min = Min();
  const int max = Max();
  const int value = std::max(min, std::min(Value() + amount, max));
  SetValue(static_cast<int16_t>(value));
}

void ScrollBar::PositionHScrollBar(
    const V2I windowSize, const int preScrollAreaWidth
) const {
  const R2I bounds = {
      {
          preScrollAreaWidth - WindowOverlap,
          windowSize.y - (MinorDimension - WindowOverlap),
      },
      {
          windowSize.x - (MinorDimension - 3 * WindowOverlap) -
              preScrollAreaWidth,
          MinorDimension,
      },
  };
  Bounds(bounds);
}

void ScrollBar::PositionVScrollBar(
    const V2I windowSize, const int preScrollAreaHeight
) const {
  const R2I bounds = {
      {
          windowSize.x - (MinorDimension - WindowOverlap),
          preScrollAreaHeight - WindowOverlap,
      },
      {
          MinorDimension,
          windowSize.y - (MinorDimension - 3 * WindowOverlap) -
              preScrollAreaHeight,
      },
  };
  Bounds(bounds);
}

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

void ScrollBar::HandleMouseDown(const Point point, const ControlPartCode part)
    const {
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

}  // namespace AtelierEsri