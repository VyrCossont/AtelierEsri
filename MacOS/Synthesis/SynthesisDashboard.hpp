#pragma once

#include "Breeze/Alchemy.hpp"
#include "ElementValueDisplay.hpp"
#include "Game.hpp"
#include "SpriteSheet.hpp"

namespace AtelierEsri {

/// Component that displays synthesis state.
class SynthesisDashboard {
 public:
  SynthesisDashboard(
      const Breeze::SynthesisState& state,
      const std::vector<Material>& catalog,
      const SpriteSheet& spriteSheet
  );

  /// Update internal state from synthesis state.
  void Refresh();

  void Update() const;

  void Layout();
  // TODO: uniform component interface with bounds, invalidation, layout, etc.

  /// Window space.
  R2I bounds{{0, 0}, {0, 0}};

 private:
  const Breeze::SynthesisState& state;
  const std::vector<Material>& catalog;
  const SpriteSheet& spriteSheet;

  ElementValueDisplay elementValueDisplay;

  static constexpr int MaterialIconWidth = 32;

  [[nodiscard]] R2I IconRect() const;
};

}  // namespace AtelierEsri
