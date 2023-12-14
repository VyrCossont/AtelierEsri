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
  spriteSheet.Draw(gWorld, assetSpriteSheet00AvatarEsriSpriteIndex, dstRect);

  const auto hex =
      Ngon({120, static_cast<int16_t>(120 + yOffset)}, 32, 6, M_PI + M_PI_2);
  {
    const ManagedPolygon polygon = hex.Polygon();
    OffsetPoly(polygon.get(), -2, -2);
    QD_CHECKED(ErasePoly(polygon.get()), "Couldn't clear hexagon");
    PenSize(4, 4);
    QD_CHECKED(FramePoly(polygon.get()), "Couldn't draw hexagon");
  }
  {
    const Pattern pattern = QD::White();
    PenSize(2, 2);
    for (uint8_t i = 0; i < 6; i++) {
      // ReSharper disable once CppUseStructuredBinding
      const Point center = hex[i];
      Rect nodeRect;
      constexpr int16_t nodeR = 6;
      nodeRect.left = static_cast<int16_t>(center.h - nodeR);
      nodeRect.right = static_cast<int16_t>(center.h + nodeR);
      nodeRect.top = static_cast<int16_t>(center.v - nodeR);
      nodeRect.bottom = static_cast<int16_t>(center.v + nodeR);
      FillOval(&nodeRect, &pattern);
      FrameOval(&nodeRect);

      if (i < 3) {
        Rect pipRect;
        pipRect.left = static_cast<int16_t>(center.h - 4);
        pipRect.right = static_cast<int16_t>(center.h + 4);
        pipRect.top = static_cast<int16_t>(center.v - 4);
        pipRect.bottom = static_cast<int16_t>(center.v + 4);
        spriteSheet.Draw(
            gWorld, assetSpriteSheet00ElementFireSpriteIndex, pipRect
        );
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
