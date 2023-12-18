#pragma once

#include "Breeze/Alchemy.hpp"
#include "Control.hpp"
#include "Material.hpp"
#include "SpriteSheet.hpp"
#include "Window.hpp"

namespace AtelierEsri {

/// Note: this type must be re-created if any of the things it references move.
class InventoryController {
 public:
  InventoryController(
      const Breeze::PlayerInventory& inventory,
      const std::vector<Material>& catalog,
      const SpriteSheet& spriteSheet,
      WindowRef behind = Window::InFrontOfAllOtherWindows
  );

  /// Draw the inventory controller's window.
  void Update() const;

 private:
  /// Set layout counts and dimensions given window size.
  void CalculateLayout();

  /// Set scroll bar increments given window size.
  void ConfigureScroll() const;

  /// Move scroll bar to appropriate location for window size.
  void PositionScrollBar() const;

  const Breeze::PlayerInventory& inventory;
  const std::vector<Material>& catalog;
  const SpriteSheet& spriteSheet;

  // TODO: track which items are in use and should not be shown
  /// List of pointers to items in `inventory`.
  std::vector<Breeze::Item*> itemsInUse{};

  Window window;
  ScrollBar scrollBar;

  // Layout counts and dimensions.

  /// The scroll bar is actually 16 pixels wide, but 1 pixel is outside the
  /// window port proper, overlapping with the window decorations.
  static constexpr int scrollBarInset = 15;
  /// Area within window occupied by inventory cells and background.
  R2I inventoryRect = {{0, 0}, {0, 0}};
  /// Items per row given window width.
  int itemsPerRow = 0;
  /// Number of rows given inventory size and items per row.
  int numRows = 0;
  /// Vertical size of all rows of inventory minus one page
  /// (so we can't scroll past the end).
  int scrollHeight = 0;
  /// Whole number of rows that can fit in the content area at once.
  int rowsPerPage = 0;
  /// Height of one page of whole cells.
  int pageHeight = 0;
};

}  // namespace AtelierEsri
