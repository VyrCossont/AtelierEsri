#include "InventoryController.hpp"

#include "AppResources.h"
#include "Drawing.hpp"
#include "InventoryCell.hpp"
#include "Material.hpp"

namespace AtelierEsri {

InventoryController::InventoryController(
    const Breeze::PlayerInventory& inventory,
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
    PositionScrollBar();

    // TODO: can be more conservative
    const Rect windowBounds = window.PortBounds();
#if TARGET_API_MAC_CARBON
    InvalWindowRect(window.Unmanaged(), &windowBounds);
#else
    InvalRect(&windowBounds);
#endif
  };

  window.onActivate = [&]([[maybe_unused]] const Window& window) {
    scrollBar.Visible(true);
  };
  window.onDeactivate = [&]([[maybe_unused]] const Window& window) {
    scrollBar.Visible(false);
  };

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
  const size_t firstItemIndex =
      itemsPerRow * scrollBar.Value() / InventoryCell::Size.y;
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
      const InventoryCell cell(item, material, spriteSheet, origin);
      cell.Draw();
    }
  }
}

void InventoryController::CalculateLayout() {
  inventoryRect = window.PortBounds();
  inventoryRect.size.x -= scrollBarInset;
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

void InventoryController::PositionScrollBar() const {
  const R2I windowBounds = window.PortBounds();
  const R2I bounds = {
      {windowBounds.size.x - scrollBarInset, -1},
      {16, windowBounds.size.y - 13},
  };
  scrollBar.Bounds(bounds);
}

}  // namespace AtelierEsri
