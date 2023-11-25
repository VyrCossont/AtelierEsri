#pragma once

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
};

} // namespace AtelierEsri
