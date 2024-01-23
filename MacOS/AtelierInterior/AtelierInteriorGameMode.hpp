#pragma once

#include "Breeze/Alchemy.hpp"
#include "Control.hpp"
#include "Game.hpp"
#include "Synthesis/SynthesisGameMode.hpp"
#include "Window.hpp"

namespace AtelierEsri {

class SynthesisGameMode;

class AtelierInteriorGameMode final : public GameMode {
 public:
  explicit AtelierInteriorGameMode(Game& game);

  /// Called when a synthesis is completed successfully.
  /// At this point, the items should have already been added to the inventory.
  void CompleteSynthesis(Breeze::SynthesisResult result);

  /// Called when a synthesis is cancelled.
  void CancelSynthesis();

 private:
  /// Disable controls that shouldn't be used during synthesis.
  /// Start synthesis game mode.
  void Synthesize();

  /// Re-enable controls after synthesis.
  void EndSynthesis();

  /// Draw icons in cabinet area.
  void DrawCabinet() const;

  /// Invalidate cabinet area.
  void InvalidateCabinet() const;

  Window window;
  Button synthesizeButton;
  Button theaterButton;
  Button walkaroundButton;
  SynthesisGameMode* synthesisInProgress = nullptr;
  Picture atelierInterior;

  // TODO: these should come from asset pipeline as an `RGN#` resource
  /// Display slots on the cabinet.
  static const std::vector<R2I> CabinetSlots;
};

}  // namespace AtelierEsri
