#include "InventoryCell.hpp"

#include "Design.hpp"
#include "Drawing.hpp"

namespace AtelierEsri {

InventoryCell::InventoryCell(
    const Breeze::Item& item,
    const Material& material,
    const SpriteSheet& spriteSheet,
    const V2I origin,
    const bool selected
)
    : item(item),
      material(material),
      spriteSheet(spriteSheet),
      origin(origin),
      selected(selected) {}

void InventoryCell::Draw() const {
  if (selected) {
    // Draw selection round rect.
    // TODO: we need an R2::Inset() method
    Rect rect = Bounds();
    rect.top += Design::SpacingMini;
    rect.left += Design::SpacingMini;
    rect.bottom -= Design::SpacingMini;
    rect.right -= Design::SpacingMini;
    const Pattern pattern = QD::LightGray();
    FillRoundRect(
        &rect,
        2 * Design::CornerRadiusMini,
        2 * Design::CornerRadiusMini,
        &pattern
    );
  }

  // Draw item icon.
  const R2I iconRect = {origin + V2I{8, 8}, {16, 16}};
  spriteSheet.Draw(material.spriteIndex, iconRect);

  // Draw separator lines on bottom and right edges.
  constexpr int16_t lineWidth = 1;

  const V2I bottomLeft = origin + V2I{0, Size.y - lineWidth};
  const V2I bottomRight = origin + V2I{Size.x - lineWidth, Size.y - lineWidth};
  const V2I topRight = origin + V2I{Size.x - lineWidth, 0};
  QD::MoveTo(bottomLeft);
  QD::LineTo(bottomRight);
  QD::LineTo(topRight);
}

Rect InventoryCell::Bounds() const { return R2I{origin, Size}; }

}  // namespace AtelierEsri