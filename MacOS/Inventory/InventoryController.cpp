#include "InventoryController.hpp"

#include <Events.h>

#include "AppResources.h"
#include "Drawing.hpp"
#include "Exception.hpp"
#include "InventoryCell.hpp"
#include "Material.hpp"

namespace AtelierEsri {

InventoryController::InventoryController(
    const std::vector<std::reference_wrapper<const Breeze::Item>>& inventory,
    const std::vector<Material>& catalog,
    const SpriteSheet& spriteSheet,
    const WindowRef behind
)
    : inventory(inventory),
      catalog(catalog),
      spriteSheet(spriteSheet),
      window(inventoryWINDResourceID, behind),
      scrollBar(inventoryVScrollBarCNTLResourceID, window) {
  window.GrowIcon(true);

  window.onUpdate = [&]([[maybe_unused]] const Window& window) { Draw(); };

  window.onResize = [&](const Window& window,
                        [[maybe_unused]] const V2I prevSize) {
    CalculateLayout();
    // TODO: add item details area
    scrollBar.PositionVScrollBar(R2I{window.PortBounds()}.size);

    InvalidateInventoryArea();
  };

  window.onActivate = [&]([[maybe_unused]] const Window& window) {
    scrollBar.Visible(true);
  };
  window.onDeactivate = [&]([[maybe_unused]] const Window& window) {
    scrollBar.Visible(false);
  };

  window.onContentMouseDown = [&]([[maybe_unused]] const Window& window,
                                  const Point point) { Click(point); };

  scrollBar.onScrollPageUp = [&](const ScrollBar& scrollBar) {
    scrollBar.ScrollBy(static_cast<int16_t>(-pageHeight));
  };
  scrollBar.onScrollPageDown = [&](const ScrollBar& scrollBar) {
    scrollBar.ScrollBy(static_cast<int16_t>(pageHeight));
  };
  scrollBar.onScrollLineUp = [&](const ScrollBar& scrollBar) {
    scrollBar.ScrollBy(-InventoryCell::Size.y);
  };
  scrollBar.onScrollLineDown = [&](const ScrollBar& scrollBar) {
    scrollBar.ScrollBy(InventoryCell::Size.y);
  };

  CalculateLayout();
  ConfigureScrollBar();
}

void InventoryController::Draw() const {
  const GWorldActiveGuard activeGuard = window.MakeActivePort();
  QD::Reset();

  QD::Erase(inventoryRect);

  // Draw inventory cells into the content GWorld.
  const size_t firstItemIndex = FirstItemIndex();
  for (int rowIndex = 0; rowIndex < rowsPerPage; ++rowIndex) {
    for (int itemIndexWithinRow = 0; itemIndexWithinRow < itemsPerRow;
         ++itemIndexWithinRow) {
      const size_t itemIndex =
          firstItemIndex + (rowIndex * itemsPerRow) + itemIndexWithinRow;
      if (itemIndex >= inventory.size()) {
        return;
      }

      const Breeze::Item& item = inventory[itemIndex];
      const Material& material = catalog[item.material.id];
      const V2I origin =
          V2I{itemIndexWithinRow, rowIndex} * InventoryCell::Size;
      const bool selected = (selectedItemIndex.has_value())
                                ? itemIndex == *selectedItemIndex
                                : false;
      const InventoryCell cell(item, material, spriteSheet, origin, selected);
      cell.Draw();
    }
  }
}

// NOLINTBEGIN(*-convert-member-functions-to-static)
void InventoryController::InvalidateInventoryArea() const {
  // NOLINTEND(*-convert-member-functions-to-static)
  // TODO: can be more conservative
  const Rect windowBounds = window.PortBounds();
#if TARGET_API_MAC_CARBON
  InvalWindowRect(window.Unmanaged(), &windowBounds);
#else
  InvalRect(&windowBounds);
#endif
}

void InventoryController::CalculateLayout() {
  inventoryRect = window.PortBounds();
  inventoryRect.size.x -= ScrollBar::MinorDimension - ScrollBar::WindowOverlap;
  itemsPerRow = inventoryRect.Width() / InventoryCell::Size.x;

  const auto inventorySize = static_cast<int>(inventory.size());
  numRows = inventorySize / itemsPerRow;
  if (inventorySize % itemsPerRow) {
    numRows += 1;
  }

  rowsPerPage = inventoryRect.Height() / InventoryCell::Size.y;

  scrollHeight = std::max(0, (numRows - rowsPerPage) * InventoryCell::Size.y);

  pageHeight = rowsPerPage * InventoryCell::Size.y;
}

void InventoryController::ConfigureScrollBar() const {
  scrollBar.SetMax(static_cast<int16_t>(scrollHeight));
  // TODO: currently loses position when window is resized
  scrollBar.SetValue(0);
}

void InventoryController::Click(V2I point) {
  if (!inventoryRect.Contains(point)) {
    // Click is outside inventory area entirely.
    // Ignore it.
    return;
  }
  point -= inventoryRect.origin;

  if (point.x >= itemsPerRow * InventoryCell::Size.x) {
    // Click is in right margin area, past the last cell.
    // Deselect any currently selected item.
    selectedItemIndex.reset();
    return;
  }

  const size_t itemIndexWithinRow = point.x / InventoryCell::Size.y;
  const size_t rowIndex = point.y / InventoryCell::Size.y;
  const size_t clickedItemIndex =
      FirstItemIndex() + (rowIndex * itemsPerRow) + itemIndexWithinRow;
  const uint32_t clickedItemTimeTicks = TickCount();

  if (selectedItemIndex.has_value()) {
    if (clickedItemIndex == *selectedItemIndex) {
      // Potential double click.
      if (clickedItemTimeTicks - selectedItemTimeTicks <= GetDblTime()) {
        // Double click.
        ItemAction();
      }
      // Otherwise, not a double click and the selection doesn't change.
    } else {
      // Not a click on the same item.
      selectedItemIndex = clickedItemIndex;
      ItemSelected();
    }
  } else {
    // No previously selected item.
    selectedItemIndex = clickedItemIndex;
    ItemSelected();
  }
  // Update the last-clicked time.
  selectedItemTimeTicks = clickedItemTimeTicks;
}

void InventoryController::ItemSelected() const {
  // Selection changed so we need to redraw.
  InvalidateInventoryArea();
}

void InventoryController::ItemAction() const {
  if (!selectedItemIndex.has_value()) {
    BAIL("ItemAction() shouldn't be called with nothing selected");
  }

  if (onItemAction) {
    onItemAction(*this, inventory[*selectedItemIndex]);
  }
}

size_t InventoryController::FirstItemIndex() const {
  return itemsPerRow * scrollBar.Value() / InventoryCell::Size.y;
}

}  // namespace AtelierEsri
