#pragma once

#include "Breeze/Alchemy.hpp"
#include "Design.hpp"
#include "Game.hpp"
#include "SynthesisCell.hpp"
#include "SynthesisDashboard.hpp"

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
  void SetupCompleteButton();
  void SetupCancelButton();
  void SetupUndoButton();

  /// Draw the controller's window contents.
  void Draw() const;

  /// Invalidate the controller's window contents.
  void InvalidateRecipeArea() const;

  /// Move scroll bars and adjust max values given window size and recipe size.
  void LayoutAndConfigureScrollBars() const;

  /// Move buttons given window size.
  void LayoutButtons() const;

  /// Show/hide and enable/disable buttons given synthesis state.
  void ConfigureButtons() const;

  /// Add this to a window space point to get a recipe space point.
  [[nodiscard]] V2I RecipeSpaceTranslation() const;

  /// Handle a content area click. Point is in window space.
  void Click(V2I point);

  // TODO: trigger on Undo menu item and ⌘Z as well
  /// Called when the undo button is clicked.
  void Undo() const;

  /// Called when an ingredient is added or removed.
  void SynthesisStateChanged() const;

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

  /// Area in window space into which we draw the recipe grid.
  R2I RecipeArea() const;

  Window window;
  ScrollBar hScrollBar;
  ScrollBar vScrollBar;

  SynthesisDashboard dashboard;

  Button completeButton;
  Button cancelButton;
  Button undoButton;

  /// Height of dashboard and button area above recipe grid.
  static constexpr int DashboardHeight = 100;

  /// Width of area reserved for buttons to right of dashboard.
  static constexpr int DashboardButtonWidth =
      DashboardHeight - 2 * Design::MinorSpacing;
};

}  // namespace AtelierEsri
