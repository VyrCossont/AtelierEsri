#include "SynthesisGameMode.hpp"

#include <algorithm>

namespace AtelierEsri {

SynthesisGameMode::SynthesisGameMode(
    Game& game, AtelierInteriorGameMode& atelierInteriorGameMode
)
    : GameMode(game),
      atelierInteriorGameMode(atelierInteriorGameMode),
      state(
          game.BreezeCatalog().back(),  // TODO: Copper ore for demo
          game.PlayerMaxPlacements(),
          game.PlayerMaxQuality(),
          game.Inventory()
      ),
      synthesisController(state, game.Catalog(), game.MainSpriteSheet()) {
  synthesisController.onCompleteSynthesis =
      [&]([[maybe_unused]] const SynthesisController& synthesisController) {
        CompleteSynthesis();
      };
  synthesisController.onCancelSynthesis =
      [&]([[maybe_unused]] const SynthesisController& synthesisController) {
        CancelSynthesis();
      };
}

// Used by lambda above.
// ReSharper disable once CppDFAUnreachableFunctionCall
void SynthesisGameMode::CompleteSynthesis() const {
  EndSynthesis();

  const Breeze::SynthesisResult result = state.Result();
  result.ApplyToInventory(game.Inventory());

  atelierInteriorGameMode.CompleteSynthesis(result);
}

// Used by lambda above.
// ReSharper disable once CppDFAUnreachableFunctionCall
void SynthesisGameMode::CancelSynthesis() const {
  EndSynthesis();
  atelierInteriorGameMode.CancelSynthesis();
}

// Used by two methods above.
// ReSharper disable once CppDFAUnreachableFunctionCall
void SynthesisGameMode::EndSynthesis() const { game.PopTo(this); }

}  // namespace AtelierEsri
