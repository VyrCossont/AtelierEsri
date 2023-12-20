#include "SynthesisController.hpp"

#include "AppResources.h"

namespace AtelierEsri {
SynthesisController::SynthesisController(
    const Breeze::Material& breezeMaterial,
    const std::vector<Material>& catalog,
    const SpriteSheet& spriteSheet,
    const WindowRef behind
)
    : breezeMaterial(breezeMaterial),
      catalog(catalog),
      spriteSheet(spriteSheet),
      cells(std::move(CreateCells(*breezeMaterial.recipe, catalog, spriteSheet))
      ),
      recipeBounds(CalculateRecipeBounds(cells)),
      window(synthesisWINDResourceID, behind),
      hScrollBar(synthesisHScrollBarCNTLResourceID, window),
      vScrollBar(synthesisVScrollBarCNTLResourceID, window) {
  window.onUpdate = [&]([[maybe_unused]] const Window& window) { Update(); };

  window.onResize = [&]([[maybe_unused]] const Window& window,
                        [[maybe_unused]] const V2I prevSize) {
    ConfigureScrollBars();

    // TODO: can be more conservative, see Apple example using regions
    InvalidateEverything();
  };

  window.onActivate = [&]([[maybe_unused]] const Window& window) {
    hScrollBar.Show();
    vScrollBar.Show();
  };
  window.onDeactivate = [&]([[maybe_unused]] const Window& window) {
    hScrollBar.Hide();
    vScrollBar.Hide();
  };

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

  ConfigureScrollBars();
}

void SynthesisController::Update() const {
  const GWorldActiveGuard activeGuard = window.MakeActivePort();
  QD::Reset();

  QD::Erase(window.PortBounds());

  const V2I scrollOffset{hScrollBar.Value(), vScrollBar.Value()};
  ChangeOrigin changeOrigin(-(scrollOffset - recipeBounds.origin));
  for (const SynthesisCell& cell : cells) {
    cell.Update();
  }
}

void SynthesisController::InvalidateEverything() const {
  const Rect windowBounds = window.PortBounds();
#if TARGET_API_MAC_CARBON
  InvalWindowRect(window.Unmanaged(), &windowBounds);
#else
  InvalRect(&windowBounds);
#endif
}

void SynthesisController::ConfigureScrollBars() const {
  const V2I windowSize = R2I{window.PortBounds()}.size;
  hScrollBar.PositionHScrollBar(windowSize);
  vScrollBar.PositionVScrollBar(windowSize);

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

std::vector<SynthesisCell> SynthesisController::CreateCells(
    const Breeze::Recipe& recipe,
    const std::vector<Material>& catalog,
    const SpriteSheet& spriteSheet
) {
  std::vector<SynthesisCell> cells;
  cells.reserve(recipe.nodes.size());
  for (const Breeze::RecipeNode& node : recipe.nodes) {
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