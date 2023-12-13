#pragma once

#include "Breeze/Alchemy.hpp"
#include "GWorld.hpp"
#include "InventoryController.hpp"
#include "Material.hpp"
#include "SpriteSheet.hpp"

namespace AtelierEsri {

class Game {
 public:
  static Game Setup(Window &window);
  /// Called at approximately 60 Hz.
  void Update(int16_t scrollBarPosition);
  void Draw(GWorld &gWorld);

 private:
  explicit Game(SpriteSheet &&spriteSheet);

  /// Generate a fresh inventory for demo purposes.
  static Breeze::PlayerInventory DemoInventory();

  SpriteSheet spriteSheet;
  std::vector<Material> catalog;
  Breeze::PlayerInventory inventory;
  InventoryController inventoryController;
  int16_t yOffset = 0;
};

}  // namespace AtelierEsri
