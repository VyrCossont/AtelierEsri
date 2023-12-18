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
      scrollBar(inventoryVScrollBarCNTLResourceID, window),
      gWorld(ContentGWorld()) {
  window.onResize = [&](const Window& window, const V2I prevSize) {
    PositionScrollBar();
    const Rect windowBounds = window.PortBounds();
#if TARGET_API_MAC_CARBON
    InvalWindowRect(window.Unmanaged(), &windowBounds);
#else
    InvalRect(&windowBounds);
#endif
    gWorld = ContentGWorld();
  };
  window.onUpdate = [&](const Window& window) { scrollBar.Draw(); };

  window.onActivate = [&](const Window& window) { scrollBar.Show(); };
  window.onDeactivate = [&](const Window& window) { scrollBar.Hide(); };

  // These scroll increments don't change with window size.
  scrollBar.onScrollLineUp = [&](const ScrollBar& scrollBar) {
    scrollBar.ScrollBy(-InventoryCell::Size.y);
  };
  scrollBar.onScrollLineDown = [&](const ScrollBar& scrollBar) {
    scrollBar.ScrollBy(InventoryCell::Size.y);
  };

  // These do.
  ConfigureScroll();
}

void InventoryController::Update() const {
  {
    const GWorldActiveGuard activeGuard = gWorld.MakeActive();
    QD::Reset();

    const Rect gWorldRect = gWorld.Bounds();
    EraseRect(&gWorldRect);

    // Draw inventory cells into the content GWorld.
    const size_t itemsPerRow = ItemsPerRow();
    const size_t firstItemIndex =
        ItemsPerRow() * scrollBar.Value() / InventoryCell::Size.y;
    const size_t rowsPerPage = RowsPerPage();
    for (int rowIndex = 0; rowIndex < rowsPerPage; ++rowIndex) {
      for (int itemIndexWithinRow = 0; itemIndexWithinRow < itemsPerRow;
           ++itemIndexWithinRow) {
        const size_t itemIndex =
            firstItemIndex + (rowIndex * itemsPerRow) + itemIndexWithinRow;
        if (itemIndex >= inventory.size()) {
          goto noMoreItems;
        }

        const Breeze::Item& item = inventory[itemIndex];
        const Material& material = catalog[item.material.id];
        const V2I origin =
            V2I{itemIndexWithinRow, rowIndex} * InventoryCell::Size;
        const InventoryCell cell(item, material, spriteSheet, origin);
        cell.Draw(gWorld);
      }
    }
  }

noMoreItems:
  // Copy the content GWorld into the window.
  Rect windowRect = window.PortBounds();
  // Leave room for the scroll bar.
  windowRect.right -= 15;
  window.CopyFrom(gWorld, gWorld.Bounds(), windowRect);
  scrollBar.Draw();
}

GWorld InventoryController::ContentGWorld() const {
  const auto [top, left, bottom, right] = window.PortBounds();
  return window.FastGWorld(
      static_cast<int16_t>(right - left - 15),
      static_cast<int16_t>(bottom - top)
  );
}

size_t InventoryController::ItemsPerRow() const {
  const auto [top, left, bottom, right] = gWorld.Bounds();
  const auto contentWidth = static_cast<int16_t>(right - left);
  return contentWidth / InventoryCell::Size.x;
}

size_t InventoryController::NumRows() const {
  const size_t itemsPerRow = ItemsPerRow();
  const size_t inventorySize = inventory.size();
  size_t numRows = inventorySize / itemsPerRow;
  if (inventorySize % itemsPerRow) {
    numRows += 1;
  }
  return numRows;
}

int16_t InventoryController::ScrollHeight() const {
  return std::max(
      static_cast<int16_t>(0),
      static_cast<int16_t>((NumRows() - RowsPerPage()) * InventoryCell::Size.y)
  );
}

size_t InventoryController::RowsPerPage() const {
  const auto [top, left, bottom, right] = gWorld.Bounds();
  return (bottom - top) / InventoryCell::Size.y;
}

void InventoryController::ConfigureScroll() {
  scrollBar.SetMin(0);
  scrollBar.SetMax(ScrollHeight());
  scrollBar.SetValue(0);

  const auto pageHeight =
      static_cast<int16_t>(RowsPerPage() * InventoryCell::Size.y);
  scrollBar.onScrollPageUp = [&](const ScrollBar& scrollBar) {
    scrollBar.ScrollBy(pageHeight);
  };
  scrollBar.onScrollPageDown = [&](const ScrollBar& scrollBar) {
    scrollBar.ScrollBy(static_cast<int16_t>(-pageHeight));
  };
}

void InventoryController::PositionScrollBar() const {
  const R2I windowBounds = window.PortBounds();
  const R2I bounds = {
      {windowBounds.size.x - 15, -1},
      {16, windowBounds.size.y - 13},
  };
  scrollBar.Bounds(bounds);
}

}  // namespace AtelierEsri
