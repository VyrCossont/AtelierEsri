#include "SynthesisCell.hpp"

#include "Assets.h"
#include "Exception.hpp"

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

  const std::vector<std::reference_wrapper<const Breeze::Item>> items =
      state.ItemsPlacedIn(node);

  // Find the sprite for this element.
  SpriteSheet::SpriteIndex pipSpriteIndex;
  switch (node.element) {
    case Breeze::Element::Fire:
      pipSpriteIndex = assetSpriteSheet00ElementFireSpriteIndex;
      break;
    case Breeze::Element::Ice:
      pipSpriteIndex = assetSpriteSheet00ElementIceSpriteIndex;
      break;
    case Breeze::Element::Lightning:
      pipSpriteIndex = assetSpriteSheet00ElementLightningSpriteIndex;
      break;
    case Breeze::Element::Wind:
      pipSpriteIndex = assetSpriteSheet00ElementWindSpriteIndex;
      break;
    default:
      BAIL("Unknown Breeze::Element in switch statement");
      break;
  }

  // Count up matching elemental pips.
  int numPips = 0;
  // ReSharper disable once CppRangeBasedForIncompatibleReference
  for (const Breeze::Item& item : items) {
    if (item.elements.test(node.element)) {
      numPips += item.elementValue;
    }
  }

  constexpr int nodeR = 32;
  constexpr int numPoints = 6;
  const auto ngon = Ngon(center, nodeR, numPoints, -5 * M_PI / 6);

  // Draw polygon, adjusted to center it in its own frame
  // (rectangles, ovals, etc. don't need this).
  constexpr int halfPenWidth = 2;
  const ManagedPolygon polygon = ngon.Polygon();
  OffsetPoly(polygon.get(), -halfPenWidth, -halfPenWidth);
  QD_CHECKED(ErasePoly(polygon.get()), "Couldn't clear hexagon");
  PenSize(2 * halfPenWidth, 2 * halfPenWidth);
  QD_CHECKED(FramePoly(polygon.get()), "Couldn't draw hexagon");

  // Draw pip slots and filled pips.
  const Pattern pattern = QD::White();
  PenSize(2, 2);
  for (auto i = 0; i < numPoints; i++) {
    const V2I center = ngon[i];
    constexpr int pipSlotHalfWidth = 6;
    const Rect pipSlotRect = R2I::Around(center, pipSlotHalfWidth);
    FillOval(&pipSlotRect, &pattern);
    FrameOval(&pipSlotRect);

    if (i < numPips) {
      constexpr int pipHalfWidth = 4;
      const auto pipRect = R2I::Around(center, pipHalfWidth);
      spriteSheet.Draw(pipSpriteIndex, pipRect);
    }
  }

  if (!items.empty()) {
    // TODO: draw more than one item
    const Breeze::Item& item = items[0];
    // ReSharper disable once CppUseStructuredBinding
    const Material& material = catalog[item.material.id];
    constexpr int MaterialIconHalfWidth = 8;
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
