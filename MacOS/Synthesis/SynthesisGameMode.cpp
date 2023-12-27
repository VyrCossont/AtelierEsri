#include "SynthesisGameMode.hpp"

#include <algorithm>

#include "Debug.hpp"

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
  state.onLog = [&](const char* fileName,
                    const uint32_t line,
                    const char* func,
                    const std::string& message) {
    Debug::Printfln(fileName, line, func, "%s", message.c_str());
  };

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
  const Breeze::SynthesisResult result = state.Result();
  DEBUG_LOG(
      "Synthesis result: %d x %s",
      result.quantity,
      game.Catalog()[result.item.material.id].name.c_str()
  );
  // TODO: display UI to pick traits up to trait cap
  result.ApplyToInventory(game.Inventory());
  DEBUG_LOG("%s", "Applied result to inventory.");

  atelierInteriorGameMode.CompleteSynthesis(result);
}

// Used by lambda above.
// ReSharper disable once CppDFAUnreachableFunctionCall
void SynthesisGameMode::CancelSynthesis() const {
  atelierInteriorGameMode.CancelSynthesis();
}

}  // namespace AtelierEsri
