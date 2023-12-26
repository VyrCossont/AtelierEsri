#pragma once

#include <functional>

#include "Breeze/Alchemy.hpp"
#include "Control.hpp"
#include "Material.hpp"
#include "SpriteSheet.hpp"
#include "Window.hpp"

namespace AtelierEsri {

/// Vertically scrolling resizable list of available alchemy materials.
///
/// Note: this type must be re-created if any of the things it references move.
class InventoryController {
 public:
  InventoryController(
      const std::vector<std::reference_wrapper<const Breeze::Item>>& inventory,
      const std::vector<Material>& catalog,
      const SpriteSheet& spriteSheet,
      WindowRef behind = Window::InFrontOfAllOtherWindows
  );
  InventoryController(const InventoryController& src) = delete;
  InventoryController& operator=(const InventoryController& src) = delete;

  std::function<void(const InventoryController&, const Breeze::Item&)>
      onItemAction;

 private:
  /// Draw the controller's window contents.
  void Draw() const;

  void InvalidateInventoryArea() const;

  /// Set layout counts and dimensions given window size.
  void CalculateLayout();

  /// Set scroll bar increments given window size.
  void ConfigureScrollBar() const;

  void Click(V2I point);

  /// Item was single-clicked to select it, navigated to with arrows, etc.
  void ItemSelected() const;

  /// Item was double-clicked, add button was clicked, etc.
  void ItemAction() const;

  /// Index of first currently displayed item.
  [[nodiscard]] size_t FirstItemIndex() const;

  const std::vector<std::reference_wrapper<const Breeze::Item>>& inventory;
  const std::vector<Material>& catalog;
  const SpriteSheet& spriteSheet;

  Window window;
  /// Vertical scroll bar.
  ScrollBar scrollBar;

  /// Index of selected item, if there is one.
  std::optional<size_t> selectedItemIndex;
  // TODO: replace with gesture recognizer
  /// Time the item was selected.
  /// Value only meaningful if `selectedItemIndex` has a value.
  uint64_t selectedItemTimeTicks = 0;

  // Layout counts and dimensions.

  /// The scroll bar is actually 16 pixels wide, but 1 pixel is outside the
  /// window port proper, overlapping with the window decorations.
  static constexpr int scrollBarInset = 15;
  /// Area within window occupied by inventory cells and background.
  R2I inventoryRect = {{0, 0}, {0, 0}};
  /// Items per row given window width.
  int itemsPerRow = 0;
  /// Number of rows given inventory size and items per row.
  /// Includes the partial row if items don't divide evenly.
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
