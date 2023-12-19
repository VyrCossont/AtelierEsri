#pragma once

#include "Breeze/Alchemy.hpp"
#include "GWorld.hpp"
#include "Inventory/InventoryController.hpp"
#include "Material.hpp"
#include "SpriteSheet.hpp"

namespace AtelierEsri {

class Game {
 public:
  explicit Game(const Window &window);

  /// Called at approximately 60 Hz.
  void Update(int16_t scrollBarPosition);
  void Draw(const GWorld &gWorld) const;

 private:
  SpriteSheet spriteSheet;
  std::vector<Breeze::Material> breezeCatalog;
  std::vector<Material> catalog;
  Breeze::PlayerInventory inventory;
  InventoryController inventoryController;
  int16_t yOffset = 0;
};

}  // namespace AtelierEsri
