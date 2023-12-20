#include "SynthesisGameMode.hpp"

namespace AtelierEsri {

SynthesisGameMode::SynthesisGameMode(Game& game)
    : GameMode(game),
      synthesisController(
          game.BreezeCatalog().back(), game.Catalog(), game.MainSpriteSheet()
      ) {}

}  // namespace AtelierEsri