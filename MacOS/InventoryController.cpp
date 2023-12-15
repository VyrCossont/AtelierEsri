#include "InventoryController.hpp"

#include "AppResources.h"
#include "Drawing.hpp"
#include "Material.hpp"

namespace AtelierEsri {

InventoryCell::InventoryCell(
    const Breeze::Item& item,
    const Material& material,
    const SpriteSheet& spriteSheet,
    const Point point
)
    : item(item), material(material), spriteSheet(spriteSheet), point(point) {}

void InventoryCell::Draw(const GWorld& gWorld) const {
  const GWorldActiveGuard activeGuard = gWorld.MakeActive();
  QD::Reset();

  // Draw item icon.
  Rect iconRect;
  iconRect.left = static_cast<int16_t>(point.h + 8);
  iconRect.right = static_cast<int16_t>(iconRect.left + 16);
  iconRect.top = static_cast<int16_t>(point.v + 8);
  iconRect.bottom = static_cast<int16_t>(iconRect.top + 16);
  spriteSheet.Draw(gWorld, material.spriteIndex, iconRect);

  // Draw separator lines on bottom and right edges.
  constexpr int16_t lineWidth = 1;
  const Point bottomLeft = {
      .v = static_cast<int16_t>(point.v + Height - lineWidth),
      .h = point.h,
  };
  const Point bottomRight = {
      .v = static_cast<int16_t>(point.v + Height - lineWidth),
      .h = static_cast<int16_t>(point.h + Width - lineWidth),
  };
  const Point topRight = {
      .v = point.v,
      .h = static_cast<int16_t>(point.h + Width - lineWidth),
  };
  MoveTo(bottomLeft.h, bottomLeft.v);
  LineTo(bottomRight.h, bottomRight.v);
  LineTo(topRight.h, topRight.v);
}

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
  window.onActivate = [&](const Window& window) { scrollBar.Show(); };
  window.onDeactivate = [&](const Window& window) { scrollBar.Hide(); };

  // These scroll increments don't change with window size.
  scrollBar.onScrollLineUp = [&](const ScrollBar& scrollBar) {
    scrollBar.ScrollBy(-InventoryCell::Height);
  };
  scrollBar.onScrollLineDown = [&](const ScrollBar& scrollBar) {
    scrollBar.ScrollBy(InventoryCell::Height);
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
        ItemsPerRow() * scrollBar.Value() / InventoryCell::Height;
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
        const Point point{
            .v = static_cast<int16_t>(rowIndex * InventoryCell::Height),
            .h =
                static_cast<int16_t>(itemIndexWithinRow * InventoryCell::Width),
        };
        const InventoryCell cell(item, material, spriteSheet, point);
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

  // Draw the window's controls.
  window.DrawControls();
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
  return contentWidth / InventoryCell::Width;
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
      static_cast<int16_t>((NumRows() - RowsPerPage()) * InventoryCell::Height)
  );
}

size_t InventoryController::RowsPerPage() const {
  const auto [top, left, bottom, right] = gWorld.Bounds();
  return (bottom - top) / InventoryCell::Height;
}

void InventoryController::ConfigureScroll() {
  scrollBar.SetMin(0);
  scrollBar.SetMax(ScrollHeight());
  scrollBar.SetValue(0);

  const auto pageHeight =
      static_cast<int16_t>(RowsPerPage() * InventoryCell::Height);
  scrollBar.onScrollPageUp = [&](const ScrollBar& scrollBar) {
    scrollBar.ScrollBy(static_cast<int16_t>(-pageHeight));
  };
  scrollBar.onScrollPageDown = [&](const ScrollBar& scrollBar) {
    scrollBar.ScrollBy(pageHeight);
  };
}

}  // namespace AtelierEsri
