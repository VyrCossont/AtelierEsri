#include "ElementValueDisplay.hpp"

#include <Quickdraw.h>

#include <iostream>
#include <sstream>

#include "Assets.h"
#include "Design.hpp"

namespace AtelierEsri {

ElementValueDisplay::ElementValueDisplay(const SpriteSheet& spriteSheet)
    : spriteSheet(spriteSheet) {}

R2I ElementValueDisplay::Bounds() const {
  constexpr int pipCount = Breeze::Element::_size();
  // Leave space for one more "pip" for the numeric value text.
  return {
      origin,
      {
          PipWidth * (pipCount + 1) + Design::SpacingMini * pipCount +
              2 * Design::CornerRadiusMini,
          PipWidth + 2 * Design::CornerRadiusMini,
      }
  };
}

void ElementValueDisplay::Update() const {
  QD::Reset();

  const R2I bounds = Bounds();
  QD::Erase(bounds);

  const Rect boundsRect = bounds;
  Pattern pattern = QD::Gray();
  FillRoundRect(
      &boundsRect,
      2 * Design::CornerRadiusMini,
      2 * Design::CornerRadiusMini,
      &pattern
  );

  pattern = QD::White();
  R2I pip{
      bounds.origin + V2I{Design::CornerRadiusMini, Design::CornerRadiusMini},
      {PipWidth, PipWidth}
  };
  for (auto element : Breeze::Element::_values()) {
    const Rect pipRect = pip;
    FillOval(&pipRect, &pattern);
    if (elements.test(element)) {
      spriteSheet.Draw(Icon(element), pip);
    }
    pip.origin.x += PipWidth + Design::SpacingMini;
  }

  // At this point, we've drawn all the elements and have one "pip" for the
  // numeric value.
  const Rect pipRect = pip;
  FillOval(&pipRect, &pattern);

  // Center and draw the numeric value.
  std::stringstream ss;
  ss << elementValue;
  const std::string text = ss.str();
  // TODO: this call is deprecated, replace with GetFNum()
  TextFont(kFontIDGeneva);
  TextSize(9);
  QD::MoveTo(pip.origin + V2I{(PipWidth - QD::TextWidth(text) / 2), 0});
  QD::DrawText(text);
}

size_t ElementValueDisplay::Icon(const Breeze::Element element) {
  switch (element) {
    case Breeze::Element::Fire:
      return assetSpriteSheet00ElementFireSpriteIndex;
    case Breeze::Element::Ice:
      return assetSpriteSheet00ElementIceSpriteIndex;
    case Breeze::Element::Lightning:
      return assetSpriteSheet00ElementLightningSpriteIndex;
    case Breeze::Element::Wind:
      return assetSpriteSheet00ElementWindSpriteIndex;
    default:
      BAIL("Unknown Breeze::Element case");
  }
}

}  // namespace AtelierEsri
