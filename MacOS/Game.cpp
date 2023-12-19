#include "Game.hpp"

#include <ctgmath>

#include "AppResources.h"
#include "Assets.h"
#include "Breeze/Alchemy.hpp"
#include "Drawing.hpp"
#include "Env.hpp"
#include "MaskedImage.hpp"

namespace AtelierEsri {

void GameMode::Tick(const uint64_t currentTimestampUsec) {
  // Default implementation does nothing.
}

GameMode::GameMode(Game& game) : game(game) {}

GameModeTitleScreen::GameModeTitleScreen(Game& game)
    : GameMode(game),
      window(titleScreenWINDResourceID),
      titleScreen(MaskedImage::Get(
          assetSceneNewTitleScreenImagePictResourceId,
          assetSceneNewTitleScreenMaskPictResourceId
      )),
      dismissTimestampUsec(Env::Microseconds() + displayDurationUsec) {
  window.onUpdate = [&](const Window& window) {
    GWorldActiveGuard activeGuard = window.MakeActivePort();
    titleScreen.Draw(titleScreen.Bounds(), window.PortBounds());
  };

  window.onContentMouseDown = [&]([[maybe_unused]] const Window& window,
                                  [[maybe_unused]] const Point point) {
    EnterAtelier();
  };
}

void GameModeTitleScreen::Tick(const uint64_t currentTimestampUsec) {
  if (currentTimestampUsec > dismissTimestampUsec) {
    EnterAtelier();
  }
}

void GameModeTitleScreen::EnterAtelier() const {
  game.PopTo(this);
  game.Push(new GameModeAtelierInterior(game));
}

GameModeAtelierInterior::GameModeAtelierInterior(Game& game)
    : GameMode(game),
      window(atelierInteriorWINDResourceID),
      atelierInterior(MaskedImage::Get(
          assetSceneAtelierInteriorImagePictResourceId,
          assetSceneAtelierInteriorMaskPictResourceId
      )) {
  window.onUpdate = [&](const Window& window) {
    GWorldActiveGuard activeGuard = window.MakeActivePort();
    atelierInterior.Draw(atelierInterior.Bounds(), window.PortBounds());
  };
}

Game::Game()
    : modeStack{new GameModeTitleScreen(*this)},
      spriteSheet(
          MaskedImage::Get(
              assetSpriteSheet00ImagePictResourceId,
              assetSpriteSheet00MaskPictResourceId
          ),
          assetSpriteSheet00RgnResourceId
      ),
      breezeCatalog(Breeze::Material::Catalog()),
      catalog(Material::Catalog(breezeCatalog)),
      inventory(DemoInventory(breezeCatalog)) {}

void Game::Tick(const uint64_t currentTimestampUsec) const {
  for (const auto mode : modeStack) {
    mode->Tick(currentTimestampUsec);
  }
}

void Game::Push(GameMode* mode) { modeStack.push_back(mode); }

void Game::PopTo(const GameMode* mode) {
  while (!modeStack.empty()) {
    const GameMode* popped = modeStack.back();
    modeStack.pop_back();
    delete popped;
    if (popped == mode) {
      return;
    }
  }
}

void XXXDraw(const GWorld& gWorld) {
  QD::Reset();

  GWorldActiveGuard activeGuard = gWorld.MakeActive();

  const Rect rect = gWorld.Bounds();
  const Pattern background = QD::Gray();
  FillRect(&rect, &background);

  constexpr int nodeR = 32;
  constexpr int numPoints = 6;
  const auto hex = Ngon({120, 120}, nodeR, numPoints, M_PI + M_PI_2);
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
        // spriteSheet.Draw(assetSpriteSheet00ElementFireSpriteIndex, pipRect);
      }
    }
  }
}

}  // namespace AtelierEsri
