#pragma once

#include "AtelierInterior/AtelierInteriorGameMode.hpp"
#include "Game.hpp"
#include "SynthesisController.hpp"

namespace AtelierEsri {

class AtelierInteriorGameMode;

class SynthesisGameMode final : public GameMode {
 public:
  explicit SynthesisGameMode(
      Game& game, AtelierInteriorGameMode& atelierInteriorGameMode
  );

 private:
  // TODO: we could also update alchemist level here
  /// Complete the synthesis by removing consumed items from the inventory,
  /// adding synthesized items to the inventory, and passing the synthesis
  /// result to the atelier interior game mode for display.
  void CompleteSynthesis() const;

  /// Cancel synthesis without doing anything.
  void CancelSynthesis() const;

  AtelierInteriorGameMode& atelierInteriorGameMode;
  Breeze::SynthesisState state;
  SynthesisController synthesisController;
};

}  // namespace AtelierEsri
