#pragma once

#include "Breeze/Alchemy.hpp"
#include "Game.hpp"
#include "SynthesisCell.hpp"

namespace AtelierEsri {

class SynthesisController {
 public:
  SynthesisController(
      const Breeze::Material& breezeMaterial,
      const std::vector<Material>& catalog,
      const SpriteSheet& spriteSheet,
      WindowRef behind = Window::InFrontOfAllOtherWindows
  );
  SynthesisController(const SynthesisController& src) = delete;
  SynthesisController& operator=(const SynthesisController& src) = delete;

 private:
  /// Draw the controller's window contents.
  void Update() const;

  /// Invalidate the controller's window contents.
  void InvalidateEverything() const;

  /// Move scroll bars and adjust max values given window size and recipe size.
  void ConfigureScrollBars() const;

  /// Material being synthesized.
  const Breeze::Material& breezeMaterial;
  /// Metadata for all materials.
  const std::vector<Material>& catalog;
  /// Icons for all materials.
  const SpriteSheet& spriteSheet;

  /// The cells that draw each recipe node.
  std::vector<SynthesisCell> cells;

  static std::vector<SynthesisCell> CreateCells(
      const Breeze::Recipe& recipe,
      const std::vector<Material>& catalog,
      const SpriteSheet& spriteSheet
  );

  /// Bounds of recipe grid in recipe space.
  /// We translate this to window space by adding the scroll bar values.
  const R2I recipeBounds;

  static R2I CalculateRecipeBounds(const std::vector<SynthesisCell>& cells);

  Window window;
  ScrollBar hScrollBar;
  ScrollBar vScrollBar;
};

}  // namespace AtelierEsri
