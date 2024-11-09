#include "SynthesisController.hpp"

#include "AppResources.h"

namespace AtelierEsri {
SynthesisController::SynthesisController(
    Breeze::SynthesisState& state,
    const std::vector<Material>& catalog,
    const SpriteSheet& spriteSheet,
    const WindowRef behind
)
    : state(state),
      catalog(catalog),
      spriteSheet(spriteSheet),
      cells(std::move(CreateCells(state, catalog, spriteSheet))),
      recipeBounds(CalculateRecipeBounds(cells)),
      window(synthesisWINDResourceID, behind),
      hScrollBar(synthesisHScrollBarCNTLResourceID, window),
      vScrollBar(synthesisVScrollBarCNTLResourceID, window),
      dashboard(state, catalog, spriteSheet),
      completeButton(synthesisCompleteButtonCNTLResourceID, window),
      cancelButton(synthesisCancelButtonCNTLResourceID, window),
      undoButton(synthesisUndoButtonCNTLResourceID, window) {
  SetupWindow();
  SetupHScrollBar();
  SetupVScrollBar();
  SetupCompleteButton();
  SetupCancelButton();
  SetupUndoButton();

  LayoutAndConfigureScrollBars();
  ConfigureButtons();
}

void SynthesisController::SetupWindow() {
  window.GrowIcon(true);

  window.onUpdate = [&]([[maybe_unused]] const Window& window) { Draw(); };

  window.onResize = [&]([[maybe_unused]] const Window& window,
                        [[maybe_unused]] const V2I prevSize) {
    LayoutAndConfigureScrollBars();
    LayoutButtons();

    // TODO: can be more conservative, see Apple example using regions
    InvalidateRecipeArea();
  };

  window.onActivate = [&]([[maybe_unused]] const Window& window) {
    hScrollBar.Visible(true);
    vScrollBar.Visible(true);
  };
  window.onDeactivate = [&]([[maybe_unused]] const Window& window) {
    hScrollBar.Visible(false);
    vScrollBar.Visible(false);
  };

  window.onClose = [&]([[maybe_unused]] const Window& window) {
    CancelSynthesis();
  };

  window.onContentMouseDown = [&]([[maybe_unused]] const Window& window,
                                  const Point point) { Click(point); };
}

void SynthesisController::SetupHScrollBar() {
  // TODO: `InvalidateRecipeArea` is *probably* overkill
  // TODO: should be using `ScrollRect` for scroll-triggered updates: see
  //  https://preterhuman.net/macstuff/insidemac/QuickDraw/QuickDraw-20.html#MARKER-9-78

  hScrollBar.onScrollPageUp = [&](const ScrollBar& scrollBar) {
    scrollBar.ScrollBy(static_cast<int16_t>(-SynthesisCell::XHalfSpace));
    InvalidateRecipeArea();
  };
  hScrollBar.onScrollPageDown = [&](const ScrollBar& scrollBar) {
    scrollBar.ScrollBy(static_cast<int16_t>(SynthesisCell::XHalfSpace));
    InvalidateRecipeArea();
  };
  hScrollBar.onScrollLineUp = [&](const ScrollBar& scrollBar) {
    scrollBar.ScrollBy(static_cast<int16_t>(-3 * SynthesisCell::XHalfSpace));
    InvalidateRecipeArea();
  };
  hScrollBar.onScrollLineDown = [&](const ScrollBar& scrollBar) {
    scrollBar.ScrollBy(static_cast<int16_t>(3 * SynthesisCell::XHalfSpace));
    InvalidateRecipeArea();
  };
  hScrollBar.onScrollBoxDragged =
      [&]([[maybe_unused]] const ScrollBar& scrollBar,
          [[maybe_unused]] const int16_t startValue) {
        InvalidateRecipeArea();
      };
}

void SynthesisController::SetupVScrollBar() {
  vScrollBar.onScrollPageUp = [&](const ScrollBar& scrollBar) {
    scrollBar.ScrollBy(-SynthesisCell::YHalfSpace);
    InvalidateRecipeArea();
  };
  vScrollBar.onScrollPageDown = [&](const ScrollBar& scrollBar) {
    scrollBar.ScrollBy(SynthesisCell::YHalfSpace);
    InvalidateRecipeArea();
  };
  vScrollBar.onScrollLineUp = [&](const ScrollBar& scrollBar) {
    scrollBar.ScrollBy(-3 * SynthesisCell::YHalfSpace);
    InvalidateRecipeArea();
  };
  vScrollBar.onScrollLineDown = [&](const ScrollBar& scrollBar) {
    scrollBar.ScrollBy(3 * SynthesisCell::YHalfSpace);
    InvalidateRecipeArea();
  };
  vScrollBar.onScrollBoxDragged =
      [&]([[maybe_unused]] const ScrollBar& scrollBar,
          [[maybe_unused]] const int16_t startValue) {
        InvalidateRecipeArea();
      };
}

void SynthesisController::SetupCompleteButton() {
  completeButton.onClick = [&]([[maybe_unused]] const Button& button) {
    CompleteSynthesis();
  };
}

void SynthesisController::SetupCancelButton() {
  cancelButton.onClick = [&]([[maybe_unused]] const Button& button) {
    CancelSynthesis();
  };
}

void SynthesisController::SetupUndoButton() {
  undoButton.onClick = [&]([[maybe_unused]] const Button& button) { Undo(); };
}

void SynthesisController::Draw() const {
  const GWorldActiveGuard activeGuard = window.MakeActivePort();
  QD::Reset();

  QD::Erase(window.PortBounds());

  dashboard.Draw();

  // Draw the cells.
  {
    // Origin transform is applied to clipping region, so we apply the inverse,
    // to get a clipping region that is in the right part of window space:
    // https://preterhuman.net/macstuff/insidemac/QuickDraw/QuickDraw-40.html
    R2I clipRect = RecipeArea();
    clipRect.origin += RecipeSpaceTranslation();
    const ChangeClip changeClip(clipRect);
    const ChangeOrigin changeOrigin(RecipeSpaceTranslation());
    for (const SynthesisCell& cell : cells) {
      cell.Draw();
    }
  }
}

// NOLINTBEGIN(*-convert-member-functions-to-static)
void SynthesisController::InvalidateRecipeArea() const {
  // NOLINTEND(*-convert-member-functions-to-static)
  GWorldActiveGuard activeGuard = window.MakeActivePort();
  const Rect recipeAreaRect = RecipeArea();
#if TARGET_API_MAC_CARBON
  InvalWindowRect(window.Unmanaged(), &recipeAreaRect);
#else
  InvalRect(&recipeAreaRect);
#endif
}

void SynthesisController::LayoutAndConfigureScrollBars() const {
  const V2I windowSize = R2I{window.PortBounds()}.size;
  hScrollBar.PositionHScrollBar(windowSize);
  vScrollBar.PositionVScrollBar(windowSize, DashboardHeight);

  // Reduce window size to account for scroll bars and dashboard.
  const R2I recipeArea = RecipeArea();
  const V2I recipeSize = recipeBounds.size;

  if (recipeSize.x <= recipeArea.Width()) {
    // Disable this scroll bar.
    hScrollBar.SetValue(0);
    hScrollBar.SetMax(0);
  } else {
    // Adjust the scroll bar, preserving the scroll position if possible.
    const int max = recipeSize.x - recipeArea.Width();
    const int prevValue = hScrollBar.Value();
    const int value = std::min(prevValue, max);
    hScrollBar.SetValue(static_cast<int16_t>(value));
    hScrollBar.SetMax(static_cast<int16_t>(max));
  }

  if (recipeSize.y <= recipeArea.Height()) {
    // Disable this scroll bar.
    vScrollBar.SetValue(0);
    vScrollBar.SetMax(0);
  } else {
    // Adjust the scroll bar, preserving the scroll position if possible.
    const int max = recipeSize.y - recipeArea.Height();
    const int prevValue = vScrollBar.Value();
    const int value = std::min(prevValue, max);
    vScrollBar.SetValue(static_cast<int16_t>(value));
    vScrollBar.SetMax(static_cast<int16_t>(max));
  }
}

void SynthesisController::LayoutButtons() const {
  completeButton.Visible(false);
  cancelButton.Visible(false);
  undoButton.Visible(false);

  const V2I windowSize = R2I{window.PortBounds()}.size;
  constexpr V2I buttonSize = {Design::ButtonWidth, Design::ButtonHeight};
  const R2I completeButtonRect = {
      {windowSize.x - buttonSize.x - Design::MinorSpacing, Design::MinorSpacing
      },
      buttonSize
  };
  // Cancel and undo buttons share the same position.
  const R2I cancelUndoButtonRect = {
      {completeButtonRect.Left(),
       completeButtonRect.Bottom() + Design::MinorSpacing},
      buttonSize
  };

  completeButton.Bounds(completeButtonRect);
  cancelButton.Bounds(cancelUndoButtonRect);
  undoButton.Bounds(cancelUndoButtonRect);

  ConfigureButtons();
}

void SynthesisController::ConfigureButtons() const {
  completeButton.Enabled(state.CanFinish());
  completeButton.Visible(true);

  const bool canUndo = state.CanUndo();

  cancelButton.Enabled(!canUndo);
  cancelButton.Visible(!canUndo);

  undoButton.Enabled(canUndo);
  undoButton.Visible(canUndo);
}

V2I SynthesisController::RecipeSpaceTranslation() const {
  const V2I scrollOffset{hScrollBar.Value(), vScrollBar.Value()};
  return recipeBounds.origin + scrollOffset - V2I{0, DashboardHeight};
}

// This member function cannot in fact be const.
// ReSharper disable once CppMemberFunctionMayBeConst
void SynthesisController::Click(const V2I point) {
  const V2I recipePoint = point + RecipeSpaceTranslation();
  bool cellSelectionChanged = false;
  const SynthesisCell* newSelectedCell = nullptr;

  // If a cell was clicked, toggle its selection.
  for (auto& cell : cells) {
    if (cell.Bounds().Contains(recipePoint)) {
      cellSelectionChanged = true;
      const bool selected = !cell.selected;
      cell.selected = selected;
      if (selected) {
        newSelectedCell = &cell;
      }
      break;
    }
  }

  // If a cell was selected, deselect the other cells.
  if (newSelectedCell) {
    for (auto& cell : cells) {
      if (&cell != newSelectedCell) {
        cell.selected = false;
      }
    }
  }
  // TODO: display the effect level list

  if (cellSelectionChanged) {
    // Trigger a redraw.
    InvalidateRecipeArea();

    // Close the existing ingredient picker, if there is one.
    if (ingredientPicker) {
      ingredientPicker.reset();
    }

    if (newSelectedCell) {
      const Breeze::RecipeNode& node = newSelectedCell->node;
      ingredientPicker.emplace(
          state.AllowedItemsFor(node), catalog, spriteSheet
      );
      ingredientPicker->onItemAction =
          [&]([[maybe_unused]] const InventoryController& inventoryController,
              const Breeze::Item& item) {
            state.Place(node, item);
            // TODO: update ingredient picker item list without losing state
            ingredientPicker.reset();
            InvalidateRecipeArea();
            ConfigureButtons();
          };
      ingredientPicker->onClose =
          [&]([[maybe_unused]] const InventoryController& inventoryController) {
            ingredientPicker.reset();
          };
    }
  }
}

void SynthesisController::Undo() const {
  // TODO: select previous cell to which something was added, if there is one
  state.Undo();
  SynthesisStateChanged();
}

void SynthesisController::SynthesisStateChanged() const {
  ConfigureButtons();
  InvalidateRecipeArea();
}

void SynthesisController::CompleteSynthesis() const {
  if (onCompleteSynthesis) {
    onCompleteSynthesis(*this);
  }
}

void SynthesisController::CancelSynthesis() const {
  // TODO: dialog box to confirm the user wants to stop synthesizing
  if (onCancelSynthesis) {
    onCancelSynthesis(*this);
  }
}

std::vector<SynthesisCell> SynthesisController::CreateCells(
    const Breeze::SynthesisState& state,
    const std::vector<Material>& catalog,
    const SpriteSheet& spriteSheet
) {
  const std::vector<Breeze::RecipeNode>& nodes = state.Output().recipe->nodes;
  std::vector<SynthesisCell> cells;
  cells.reserve(nodes.size());
  for (const Breeze::RecipeNode& node : nodes) {
    cells.emplace_back(state, node, catalog, spriteSheet);
  }
  return cells;
}

R2I SynthesisController::CalculateRecipeBounds(
    const std::vector<SynthesisCell>& cells
) {
  R2I bounds{{0, 0}, {0, 0}};
  for (const SynthesisCell& cell : cells) {
    bounds |= cell.Bounds();
  }
  return bounds;
}

R2I SynthesisController::RecipeArea() const {
  return {
      {0, DashboardHeight},
      R2I{window.PortBounds()}.size - V2I{0, DashboardHeight} -
          V2I{ScrollBar::MinorDimension, ScrollBar::MinorDimension} +
          V2I{ScrollBar::WindowOverlap, ScrollBar::WindowOverlap},
  };
}

}  // namespace AtelierEsri