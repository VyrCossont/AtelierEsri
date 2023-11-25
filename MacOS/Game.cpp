#include "Game.hpp"

#include <ctgmath>

#include "Assets.h"
#include "Drawing.hpp"
#include "MaskedImage.hpp"

namespace AtelierEsri {

Result<Game> Game::Setup(Window &window) noexcept {
  GUARD_LET_TRY(MaskedImage, spriteSheetImage,
                MaskedImage::Get(assetSpriteSheet00ImagePictResourceId,
                                 assetSpriteSheet00MaskPictResourceId, window));
  GUARD_LET_TRY(SpriteSheet, spriteSheet,
                SpriteSheet::Get(std::move(spriteSheetImage),
                                 assetSpriteSheet00RgnResourceId));
  return Ok(Game(std::move(spriteSheet)));
}

void Game::Update() {}

Result<Unit> Game::Draw(GWorld &gWorld) noexcept {
  QD::Reset();

  GWorldActiveGuard activeGuard = gWorld.MakeActive();

  Rect rect = gWorld.Bounds();
  Pattern background = QD::Gray();
  FillRect(&rect, &background);

  Rect dstRect = {0, 0, 64, 64};
  TRY(spriteSheet.Draw(gWorld, assetSpriteSheet00AvatarEsriSpriteIndex,
                       dstRect));

  Ngon hex = Ngon({120, 120}, 32, 6, M_PI + M_PI_2);
  {
    ManagedPolygon polygon = hex.Polygon();
    OffsetPoly(polygon.get(), -2, -2);
    QD_CHECKED(ErasePoly(polygon.get()), "Couldn't clear hexagon");
    PenSize(4, 4);
    QD_CHECKED(FramePoly(polygon.get()), "Couldn't draw hexagon");
  }
  {
    Pattern pattern = QD::White();
    PenSize(2, 2);
    for (uint8_t i = 0; i < 6; i++) {
      V2I center = hex[i];
      Rect nodeRect;
      int16_t nodeR = 6;
      nodeRect.left = static_cast<int16_t>(center.x - nodeR);
      nodeRect.right = static_cast<int16_t>(center.x + nodeR);
      nodeRect.top = static_cast<int16_t>(center.y - nodeR);
      nodeRect.bottom = static_cast<int16_t>(center.y + nodeR);
      FillOval(&nodeRect, &pattern);
      FrameOval(&nodeRect);

      if (i < 3) {
        Rect pipRect;
        pipRect.left = static_cast<int16_t>(center.x - 4);
        pipRect.right = static_cast<int16_t>(center.x + 4);
        pipRect.top = static_cast<int16_t>(center.y - 4);
        pipRect.bottom = static_cast<int16_t>(center.y + 4);
        TRY(spriteSheet.Draw(gWorld, assetSpriteSheet00ElementFireSpriteIndex,
                             pipRect));
      }
    }
  }

  return Ok(Unit());
}

Game::Game(SpriteSheet &&spriteSheet) noexcept
    : spriteSheet(std::move(spriteSheet)) {}

} // namespace AtelierEsri
