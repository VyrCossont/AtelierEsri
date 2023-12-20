#pragma once

#include "Game.hpp"
#include "SynthesisController.hpp"

namespace AtelierEsri {

class SynthesisGameMode final : public GameMode {
 public:
  explicit SynthesisGameMode(Game& game);

 private:
  SynthesisController synthesisController;
};

}  // namespace AtelierEsri
