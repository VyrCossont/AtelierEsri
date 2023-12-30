#include "Game.hpp"

#include "AppResources.h"
#include "Assets.h"
#include "AtelierInterior/AtelierInteriorGameMode.hpp"
#include "Breeze/Alchemy.hpp"
#include "Debug.hpp"
#include "Env.hpp"
#include "MaskedImage.hpp"

namespace AtelierEsri {

void GameMode::Tick(const uint64_t currentTimestampUsec) {
  // Default implementation does nothing.
}

GameMode::GameMode(Game& game) : game(game) {}

TitleScreenGameMode::TitleScreenGameMode(Game& game)
    : GameMode(game),
      window(titleScreenWINDResourceID),
      titleScreen(assetSceneNewTitleScreenImagePictResourceId),
      dismissTimestampUsec(Env::Microseconds() + displayDurationUsec) {
  window.onUpdate = [&](const Window& window) {
    GWorldActiveGuard activeGuard = window.MakeActivePort();
    titleScreen.Draw(window.PortBounds());
  };

  window.onContentMouseDown = [&]([[maybe_unused]] const Window& window,
                                  [[maybe_unused]] const Point point) {
    EnterAtelier();
  };
}

void TitleScreenGameMode::Tick(const uint64_t currentTimestampUsec) {
  if (currentTimestampUsec > dismissTimestampUsec) {
    EnterAtelier();
  }
}

void TitleScreenGameMode::EnterAtelier() const {
  game.PopTo(this);
  game.Push(new AtelierInteriorGameMode(game));
}

Game::Game()
    : modeStack{new TitleScreenGameMode(*this)},
      spriteSheet(
          MaskedImage::Get(
              assetSpriteSheet00ImagePictResourceId,
              assetSpriteSheet00MaskPictResourceId
          ),
          assetSpriteSheet00RgnResourceId,
          assetSpriteSheet009PcResourceId
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

const SpriteSheet& Game::MainSpriteSheet() const { return spriteSheet; }

const std::vector<Breeze::Material>& Game::BreezeCatalog() const {
  return breezeCatalog;
}

const std::vector<Material>& Game::Catalog() const { return catalog; }

Breeze::PlayerInventory& Game::Inventory() { return inventory; }

Breeze::Quality Game::PlayerMaxQuality() { return 120; }

int Game::PlayerMaxPlacements() { return 5; }

}  // namespace AtelierEsri
