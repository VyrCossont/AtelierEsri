#include "Game.hpp"

#include <ctgmath>

#include "Assets.h"
#include "Drawing.hpp"
#include "MaskedImage.hpp"

namespace AtelierEsri {

Game Game::Setup(Window &window) {
  MaskedImage spriteSheetImage =
      MaskedImage::Get(assetSpriteSheet00ImagePictResourceId,
                       assetSpriteSheet00MaskPictResourceId, window);
  SpriteSheet spriteSheet = SpriteSheet::New(std::move(spriteSheetImage),
                                             assetSpriteSheet00RgnResourceId);
  return Game(std::move(spriteSheet));
}

void Game::Update() {}

void Game::Draw(GWorld &gWorld) {
  QD::Reset();

  GWorldActiveGuard activeGuard = gWorld.MakeActive();

  const Rect rect = gWorld.Bounds();
  const Pattern background = QD::Gray();
  FillRect(&rect, &background);

  constexpr Rect dstRect = {0, 0, 64, 64};
  spriteSheet.Draw(gWorld, assetSpriteSheet00AvatarEsriSpriteIndex, dstRect);

  const Ngon hex = Ngon({120, 120}, 32, 6, M_PI + M_PI_2);
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
        spriteSheet.Draw(gWorld, assetSpriteSheet00ElementFireSpriteIndex,
                         pipRect);
      }
    }
  }
}

Game::Game(SpriteSheet &&spriteSheet) : spriteSheet(std::move(spriteSheet)) {}

} // namespace AtelierEsri
