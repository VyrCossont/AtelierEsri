#include "Game.hpp"

#include <ctgmath>

#include "Assets.h"
#include "Drawing.hpp"

namespace AtelierEsri {

Result<Game> Game::Setup(Window &window) noexcept {
  GUARD_LET_TRY(MaskedImage, avatar,
                MaskedImage::Get(assetAvatarEsriImagePictResourceId,
                                 assetAvatarEsriMaskPictResourceId, window));
  return Ok(Game(std::move(avatar)));
}

void Game::Update() {}

Result<Unit> Game::Draw(GWorld &gWorld) noexcept {
  QD::Reset();

  GWorldActiveGuard activeGuard = gWorld.MakeActive();

  Rect rect = gWorld.Bounds();
  Pattern background = QD::Gray();
  FillRect(&rect, &background);

  TRY(avatar.Draw(gWorld, avatar.Bounds()));

  ManagedPolygon hex = Ngon(120, 120, 32, 6, M_PI_2);
  QD_CHECKED(ErasePoly(hex.get()), "Couldn't clear hexagon");
  PenSize(3, 3);
  QD_CHECKED(FramePoly(hex.get()), "Couldn't draw hexagon");

  return Ok(Unit());
}

Game::Game(MaskedImage &&avatar) noexcept : avatar(std::move(avatar)) {}

} // namespace AtelierEsri
