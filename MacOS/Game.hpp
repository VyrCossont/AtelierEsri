#pragma once

#include <Alchemy.hpp>

#include "GWorld.hpp"
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
  SpriteSheet spriteSheet;
  Breeze::PlayerInventory inventory;
  int16_t yOffset = 0;
};

/// Adds name and icon to a Breeze material.
struct Material {
  Breeze::Material data;
  std::string name;
  std::string description;
  size_t spriteIndex;

  static std::vector<Material> Catalog();
};

} // namespace AtelierEsri
