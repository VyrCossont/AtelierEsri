#include "AtelierInteriorGameMode.hpp"

#include "AppResources.h"
#include "Assets.h"
#include "Cinematic/AlchemySlightlyExplained.hpp"
#include "Cinematic/CinematicGameMode.hpp"
#include "Debug.hpp"

namespace AtelierEsri {

AtelierInteriorGameMode::AtelierInteriorGameMode(Game& game)
    : GameMode(game),
      window(atelierInteriorWINDResourceID),
      synthesizeButton(atelierInteriorSynthesizeButtonCNTLResourceID, window),
      theaterButton(atelierInteriorTheaterButtonCNTLResourceID, window),
      atelierInterior(assetSceneAtelierInteriorImagePictResourceId) {
  window.onUpdate = [&](const Window& window) {
    GWorldActiveGuard activeGuard = window.MakeActivePort();
    atelierInterior.Draw(window.PortBounds());
    DrawCabinet();
  };

  window.onActivate = [&]([[maybe_unused]] const Window& window) {
    synthesizeButton.Enabled(!synthesisInProgress);
  };
  window.onDeactivate = [&]([[maybe_unused]] const Window& window) {
    synthesizeButton.Enabled(false);
  };

  synthesizeButton.onClick = [&]([[maybe_unused]] const Button& button) {
    Synthesize();
  };

  theaterButton.onClick = [&]([[maybe_unused]] const Button& button) {
    game.Push(new CinematicGameMode(
        game, AlchemySlightlyExplained, "alchemy, slightly explained"
    ));
  };
}

void AtelierInteriorGameMode::CompleteSynthesis(
    const Breeze::SynthesisResult result
) {
  EndSynthesis();
  DEBUG_LOG("%s", "Ended synthesis");

  // TODO: display a summary modal

  InvalidateCabinet();
}

void AtelierInteriorGameMode::CancelSynthesis() { EndSynthesis(); }

void AtelierInteriorGameMode::EndSynthesis() {
  if (!synthesisInProgress) {
    BAIL("Can't end a synthesis without starting one");
  }

  game.PopTo(synthesisInProgress);
  synthesisInProgress = nullptr;

  synthesizeButton.Enabled(true);
}

void AtelierInteriorGameMode::DrawCabinet() const {
  // Display first N synthesized items (not raw materials).
  std::vector<std::reference_wrapper<const Breeze::Item>> synthesizedItems;
  for (const Breeze::Item& item : game.Inventory()) {
    if (item.material.recipe.has_value()) {
      synthesizedItems.emplace_back(item);
    }
    if (synthesizedItems.size() == CabinetSlots.size()) {
      break;
    }
  }

  for (int i = 0; i < synthesizedItems.size(); ++i) {
    const Breeze::Item& item = synthesizedItems[i];
    // ReSharper disable once CppUseStructuredBinding
    const Material& material = game.Catalog()[item.material.id];
    const R2I& cabinetSlot = CabinetSlots[i];
    game.MainSpriteSheet().Draw(material.spriteIndex, cabinetSlot);
  }
}

void AtelierInteriorGameMode::InvalidateCabinet() const {
  if (CabinetSlots.empty()) {
    return;
  }

  R2I cabinetArea = CabinetSlots[0];
  for (int i = 1; i < CabinetSlots.size(); ++i) {
    cabinetArea |= CabinetSlots[i];
  }

  // TODO: extract this to `Window::Invalidate(const R2I& rect)`
  GWorldActiveGuard activeGuard = window.MakeActivePort();
  const Rect cabinetAreaRect = cabinetArea;
#if TARGET_API_MAC_CARBON
  InvalWindowRect(window.Unmanaged(), &cabinetAreaRect);
#else
  InvalRect(&cabinetAreaRect);
#endif
}

// Reachable from lambda above.
// ReSharper disable once CppDFAUnreachableFunctionCall
void AtelierInteriorGameMode::Synthesize() {
  if (synthesisInProgress) {
    BAIL("Can't start two syntheses at once");
  }

  synthesizeButton.Enabled(false);

  synthesisInProgress = new SynthesisGameMode(game, *this);
  game.Push(synthesisInProgress);
}

const std::vector<R2I> AtelierInteriorGameMode::CabinetSlots{
    {{281, 55}, {16, 16}},
    {{306, 57}, {16, 16}},
    {{335, 59}, {16, 16}},
    {{281, 84}, {16, 16}},
    {{308, 88}, {16, 16}},
    {{335, 92}, {16, 16}},
    {{281, 111}, {16, 16}},
    {{307, 117}, {16, 16}},
    {{332, 123}, {16, 16}},
    {{280, 144}, {16, 16}},
    {{306, 153}, {16, 16}},
    {{331, 160}, {16, 16}},
};

}  // namespace AtelierEsri