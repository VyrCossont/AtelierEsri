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
      cells(std::move(
          CreateCells(state.Output().recipe->nodes, catalog, spriteSheet)
      )),
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
}

void SynthesisController::SetupWindow() {
  window.GrowIcon(true);

  window.onUpdate = [&]([[maybe_unused]] const Window& window) { Update(); };

  window.onResize = [&]([[maybe_unused]] const Window& window,
                        [[maybe_unused]] const V2I prevSize) {
    LayoutAndConfigureScrollBars();
    LayoutButtons();

    // TODO: can be more conservative, see Apple example using regions
    InvalidateEverything();
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
  // TODO: `InvalidateEverything` is *probably* overkill
  // TODO: should be using `ScrollRect` for scroll-triggered updates: see
  //  https://preterhuman.net/macstuff/insidemac/QuickDraw/QuickDraw-20.html#MARKER-9-78

  hScrollBar.onScrollPageUp = [&](const ScrollBar& scrollBar) {
    scrollBar.ScrollBy(static_cast<int16_t>(-SynthesisCell::XHalfSpace));
    InvalidateEverything();
  };
  hScrollBar.onScrollPageDown = [&](const ScrollBar& scrollBar) {
    scrollBar.ScrollBy(static_cast<int16_t>(SynthesisCell::XHalfSpace));
    InvalidateEverything();
  };
  hScrollBar.onScrollLineUp = [&](const ScrollBar& scrollBar) {
    scrollBar.ScrollBy(static_cast<int16_t>(-3 * SynthesisCell::XHalfSpace));
    InvalidateEverything();
  };
  hScrollBar.onScrollLineDown = [&](const ScrollBar& scrollBar) {
    scrollBar.ScrollBy(static_cast<int16_t>(3 * SynthesisCell::XHalfSpace));
    InvalidateEverything();
  };
  hScrollBar.onScrollBoxDragged =
      [&]([[maybe_unused]] const ScrollBar& scrollBar,
          [[maybe_unused]] const int16_t startValue) {
        InvalidateEverything();
      };
}

void SynthesisController::SetupVScrollBar() {
  vScrollBar.onScrollPageUp = [&](const ScrollBar& scrollBar) {
    scrollBar.ScrollBy(-SynthesisCell::YHalfSpace);
    InvalidateEverything();
  };
  vScrollBar.onScrollPageDown = [&](const ScrollBar& scrollBar) {
    scrollBar.ScrollBy(SynthesisCell::YHalfSpace);
    InvalidateEverything();
  };
  vScrollBar.onScrollLineUp = [&](const ScrollBar& scrollBar) {
    scrollBar.ScrollBy(-3 * SynthesisCell::YHalfSpace);
    InvalidateEverything();
  };
  vScrollBar.onScrollLineDown = [&](const ScrollBar& scrollBar) {
    scrollBar.ScrollBy(3 * SynthesisCell::YHalfSpace);
    InvalidateEverything();
  };
  vScrollBar.onScrollBoxDragged =
      [&]([[maybe_unused]] const ScrollBar& scrollBar,
          [[maybe_unused]] const int16_t startValue) {
        InvalidateEverything();
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

void SynthesisController::Update() const {
  const GWorldActiveGuard activeGuard = window.MakeActivePort();
  QD::Reset();

  QD::Erase(window.PortBounds());

  dashboard.Update();

  // Draw the cells.
  {
    const ChangeOrigin changeOrigin(RecipeSpaceTranslation());
    for (const SynthesisCell& cell : cells) {
      cell.Update();
    }
  }
}

// TODO: if this ends up being useful, move it up to `Window`.
void SynthesisController::InvalidateEverything() const {
  const Rect windowBounds = window.PortBounds();
#if TARGET_API_MAC_CARBON
  InvalWindowRect(window.Unmanaged(), &windowBounds);
#else
  InvalRect(&windowBounds);
#endif
}

void SynthesisController::LayoutAndConfigureScrollBars() const {
  const V2I windowSize = R2I{window.PortBounds()}.size;
  hScrollBar.PositionHScrollBar(windowSize);
  vScrollBar.PositionVScrollBar(windowSize, DashboardHeight);

  const V2I recipeSize = recipeBounds.size;

  if (recipeSize.x <= windowSize.x) {
    // Disable this scroll bar.
    hScrollBar.SetValue(0);
    hScrollBar.SetMax(0);
  } else {
    // Adjust the scroll bar, preserving the scroll position if possible.
    const int max = recipeSize.x - windowSize.x;
    const int prevValue = hScrollBar.Value();
    const int value = std::min(prevValue, max);
    hScrollBar.SetValue(static_cast<int16_t>(value));
    hScrollBar.SetMax(static_cast<int16_t>(max));
  }

  if (recipeSize.y <= windowSize.y) {
    // Disable this scroll bar.
    vScrollBar.SetValue(0);
    vScrollBar.SetMax(0);
  } else {
    // Adjust the scroll bar, preserving the scroll position if possible.
    const int max = recipeSize.y - windowSize.y;
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
  return recipeBounds.origin - scrollOffset - V2I{0, DashboardHeight};
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
  // TODO: open an inventory picker

  // If the cell selection changed, trigger a redraw.
  if (cellSelectionChanged) {
    // TODO: InvalidateEverything() doesn't actually trigger a redraw,
    //  but calling Update() directly wipes out the scroll bars.
    //  We're doing something wrong with invalid regions and update events
    Update();
  }
}

void SynthesisController::Undo() {
  // TODO: select previous cell to which something was added, if there is one
  state.Undo();
  SynthesisStateChanged();
}

void SynthesisController::SynthesisStateChanged() {
  ConfigureButtons();
  InvalidateEverything();
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
    const std::vector<Breeze::RecipeNode>& nodes,
    const std::vector<Material>& catalog,
    const SpriteSheet& spriteSheet
) {
  std::vector<SynthesisCell> cells;
  cells.reserve(nodes.size());
  for (const Breeze::RecipeNode& node : nodes) {
    cells.emplace_back(node, catalog, spriteSheet);
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

}  // namespace AtelierEsri