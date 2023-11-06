#include "Game.hpp"

#include "Assets.h"
#include "Env.hpp"

namespace AtelierEsri {

Result<Game> Game::Setup(Window &window) noexcept {
  GUARD_LET_TRY(MaskedImage, avatar,
                MaskedImage::Get(assetAvatarEsriImagePictResourceId,
                                 assetAvatarEsriMaskPictResourceId, window));
  return Ok(Game(std::move(avatar)));
}

void Game::Update() {}

Result<Unit> Game::Draw(GWorld &gWorld) noexcept {
  GWorldActiveGuard activeGuard = gWorld.MakeActive();

  Rect rect = gWorld.Bounds();
  Pattern background = Env::Gray();
  FillRect(&rect, &background);

  TRY(avatar.Draw(gWorld, avatar.Bounds()));

  return Ok(Unit());
}

Game::Game(MaskedImage &&avatar) noexcept : avatar(std::move(avatar)) {}

} // namespace AtelierEsri
