#pragma once

#include <Alchemy.hpp>

#include "GWorld.hpp"
#include "SpriteSheet.hpp"

namespace AtelierEsri {

class Game {
public:
  static Game Setup(Window &window);
  /// Called at approximately 60 Hz.
  void Update();
  void Draw(GWorld &gWorld);

private:
  explicit Game(SpriteSheet &&spriteSheet);
  SpriteSheet spriteSheet;
  Breeze::PlayerInventory inventory;
};

/// Adds name and icon to a Breeze material.
struct Material {
  Breeze::Material data;
  std::string name;
  size_t spriteIndex;

  static std::vector<Material> Catalog();
};

} // namespace AtelierEsri
