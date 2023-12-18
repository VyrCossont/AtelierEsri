#include "Game.hpp"

#include <ctgmath>

#include "Assets.h"
#include "Breeze/Alchemy.hpp"
#include "Drawing.hpp"
#include "MaskedImage.hpp"

namespace AtelierEsri {

Game::Game(const Window &window)
    : spriteSheet(
          MaskedImage::Get(
              assetSpriteSheet00ImagePictResourceId,
              assetSpriteSheet00MaskPictResourceId,
              window
          ),
          assetSpriteSheet00RgnResourceId
      ),
      catalog(Material::Catalog()),
      inventory(DemoInventory()),
      inventoryController(inventory, catalog, spriteSheet) {}

void Game::Update(const int16_t scrollBarPosition) {
  yOffset = static_cast<int16_t>(-(scrollBarPosition - 50));

  // TODO: separate this thing's Update() from its Draw()?
  inventoryController.Update();
}

void Game::Draw(const GWorld &gWorld) const {
  QD::Reset();

  GWorldActiveGuard activeGuard = gWorld.MakeActive();

  const Rect rect = gWorld.Bounds();
  const Pattern background = QD::Gray();
  FillRect(&rect, &background);

  const Rect dstRect = {yOffset, 0, static_cast<int16_t>(64 + yOffset), 64};
  spriteSheet.Draw(assetSpriteSheet00AvatarEsriSpriteIndex, dstRect);

  constexpr int nodeR = 32;
  constexpr int numPoints = 6;
  const auto hex = Ngon(
      {120, static_cast<int16_t>(120 + yOffset)},
      nodeR,
      numPoints,
      M_PI + M_PI_2
  );
  {
    // Draw polygon, adjusted to center it in its own frame
    // (rectangles, ovals, etc. don't need this).
    constexpr int halfPenWidth = 2;
    const ManagedPolygon polygon = hex.Polygon();
    OffsetPoly(polygon.get(), -halfPenWidth, -halfPenWidth);
    QD_CHECKED(ErasePoly(polygon.get()), "Couldn't clear hexagon");
    PenSize(2 * halfPenWidth, 2 * halfPenWidth);
    QD_CHECKED(FramePoly(polygon.get()), "Couldn't draw hexagon");
  }
  {
    const Pattern pattern = QD::White();
    PenSize(2, 2);
    for (auto i = 0; i < numPoints; i++) {
      const V2I center = hex[i];
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
}

Breeze::PlayerInventory Game::DemoInventory() {
  Breeze::PlayerInventory inventory{};

  // Give two of every raw material.
  const std::vector<Material> catalog = Material::Catalog();
  for (size_t materialIndex = 0; materialIndex < catalog.size() - 1;
       ++materialIndex) {
    const Breeze::Material &material = catalog[materialIndex].data;
    Breeze::Item item = {
        .material = material,
        .elements = material.elements,
        .elementValue = material.elementValue,
        .quality = 50,
        .categories = material.categories,
        .traits = material.traits
    };
    for (size_t itemIndex = 0; itemIndex < 2; ++itemIndex) {
      inventory.push_back(item);
    }
  }

  return inventory;
}

}  // namespace AtelierEsri
