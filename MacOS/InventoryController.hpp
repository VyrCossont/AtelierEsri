#pragma once

#include "Breeze/Alchemy.hpp"
#include "Control.hpp"
#include "Material.hpp"
#include "SpriteSheet.hpp"
#include "Window.hpp"

namespace AtelierEsri {

class InventoryCell {
 public:
  InventoryCell(
      const Breeze::Item& item,
      const Material& material,
      const SpriteSheet& spriteSheet,
      Point point
  );

  static constexpr int16_t Width = 32;
  static constexpr int16_t Height = 32;

  void Draw(const GWorld& gWorld) const;

 private:
  const Breeze::Item& item;
  const Material& material;
  const SpriteSheet& spriteSheet;
  Point point;
  bool hilite = false;
};

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
  /// Create a GWorld the size of the content area, excluding scroll bars.
  [[nodiscard]] GWorld ContentGWorld() const;

  /// Items per row given window width.
  [[nodiscard]] size_t ItemsPerRow() const;

  /// Number of rows given inventory size and items per row.
  [[nodiscard]] size_t NumRows() const;

  /// Vertical size of all rows of inventory minus one page
  /// (so we can't scroll past the end).
  [[nodiscard]] int16_t ScrollHeight() const;

  /// Whole number of rows that can fit in the content area at once.
  [[nodiscard]] size_t RowsPerPage() const;

  /// Set scroll bar increments given window size.
  void ConfigureScroll();

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
  GWorld gWorld;
};

}  // namespace AtelierEsri
