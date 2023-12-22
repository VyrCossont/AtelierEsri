#pragma once

#include "Breeze/Alchemy.hpp"
#include "Game.hpp"
#include "SynthesisCell.hpp"

namespace AtelierEsri {

class SynthesisController {
 public:
  SynthesisController(
      Breeze::SynthesisState& state,
      const std::vector<Material>& catalog,
      const SpriteSheet& spriteSheet,
      WindowRef behind = Window::InFrontOfAllOtherWindows
  );
  SynthesisController(const SynthesisController& src) = delete;
  SynthesisController& operator=(const SynthesisController& src) = delete;

  std::function<void(const SynthesisController&)> onCompleteSynthesis;

  std::function<void(const SynthesisController&)> onCancelSynthesis;

 private:
  // One-time setup methods to keep the constructor readable.
  void SetupWindow();
  void SetupHScrollBar();
  void SetupVScrollBar();

  /// Draw the controller's window contents.
  void Update() const;

  /// Invalidate the controller's window contents.
  void InvalidateEverything() const;

  /// Move scroll bars and adjust max values given window size and recipe size.
  void ConfigureScrollBars() const;

  /// Add this to a window space point to get a recipe space point.
  [[nodiscard]] V2I RecipeSpaceTranslation() const;

  /// Handle a content area click. Point is in window space.
  void Click(V2I point);

  /// Called to complete the synthesis.
  void CompleteSynthesis() const;

  /// Called to cancel the synthesis.
  void CancelSynthesis() const;

  /// Synthesis model.
  Breeze::SynthesisState& state;
  /// Metadata for all materials.
  const std::vector<Material>& catalog;
  /// Icons for all materials.
  const SpriteSheet& spriteSheet;

  /// The cells that draw each recipe node.
  std::vector<SynthesisCell> cells;

  static std::vector<SynthesisCell> CreateCells(
      const std::vector<Breeze::RecipeNode>& nodes,
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
