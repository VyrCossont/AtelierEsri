#include "SynthesisCell.hpp"

#include "Assets.h"

namespace AtelierEsri {

SynthesisCell::SynthesisCell(
    const Breeze::SynthesisState& state,
    const Breeze::RecipeNode& node,
    const std::vector<Material>& catalog,
    const SpriteSheet& spriteSheet
)
    : node(node),
      state(state),
      catalog(catalog),
      spriteSheet(spriteSheet),
      center(CalculateCenter(node)),
      bounds(CalculateBounds(center)) {}

const R2I& SynthesisCell::Bounds() const { return bounds; }

void SynthesisCell::Draw() const {
  QD::Reset();
  QD::Erase(bounds);

  if (selected) {
    constexpr int haloR = 40;
    const Pattern pattern = QD::LightGray();
    const Rect halo = R2I::Around(center, haloR);
    FillOval(&halo, &pattern);
  }

  // TODO: polygon can probably be cached too
  //  Heck, we might be able to make a `Picture` or `GWorld` of this whole deal.

  constexpr int nodeR = 32;
  constexpr int numPoints = 6;
  const auto ngon = Ngon(center, nodeR, numPoints, 0, true);
  {
    // Draw polygon, adjusted to center it in its own frame
    // (rectangles, ovals, etc. don't need this).
    constexpr int halfPenWidth = 2;
    const ManagedPolygon polygon = ngon.Polygon();
    OffsetPoly(polygon.get(), -halfPenWidth, -halfPenWidth);
    QD_CHECKED(ErasePoly(polygon.get()), "Couldn't clear hexagon");
    PenSize(2 * halfPenWidth, 2 * halfPenWidth);
    QD_CHECKED(FramePoly(polygon.get()), "Couldn't draw hexagon");
  }
  {
    const Pattern pattern = QD::White();
    PenSize(2, 2);
    for (auto i = 0; i < numPoints; i++) {
      const V2I center = ngon[i];
      constexpr int pipSlotHalfWidth = 6;
      const Rect pipSlotRect = R2I::Around(center, pipSlotHalfWidth);
      FillOval(&pipSlotRect, &pattern);
      FrameOval(&pipSlotRect);

      if (i < 3) {
        constexpr int pipHalfWidth = 4;
        const auto pipRect = R2I::Around(center, pipHalfWidth);
        spriteSheet.Draw(assetSpriteSheet00ElementFireSpriteIndex, pipRect);
      }
    }
  }

  const std::vector<std::reference_wrapper<const Breeze::Item>> items =
      state.ItemsPlacedIn(node);
  if (!items.empty()) {
    // TODO: draw more than one item
    const Breeze::Item& item = items[0];
    // ReSharper disable once CppUseStructuredBinding
    const Material& material = catalog[item.material.id];
    constexpr int MaterialIconHalfWidth = 8;
    // TODO: we're drawing sprite 0 (Allie) and crashing when an item is added
    //  Obviously this is wrong, if funny.
    //  Something's probably null and should't be.
    spriteSheet.Draw(
        material.spriteIndex, R2I::Around(center, MaterialIconHalfWidth)
    );
  }
}

R2I SynthesisCell::CalculateBounds(const V2I& center) {
  return R2I::Around(center, HalfSpace);
}

V2I SynthesisCell::CalculateCenter(const Breeze::RecipeNode& node) {
  V2I center = V2I{node.gridPos.x, node.gridPos.y} * 2 * HalfSpace;
  if (node.gridPos.x % 2) {
    // Stagger odd columns downward.
    center.y += YHalfSpace;
  }
  return center;
}

}  // namespace AtelierEsri
