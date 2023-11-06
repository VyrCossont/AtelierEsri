#pragma once

#include "GWorld.hpp"
#include "MaskedImage.hpp"

namespace AtelierEsri {

class Game {
public:
  static Result<Game> Setup(Window &window) noexcept;
  /// Called at approximately 60 Hz.
  void Update();
  Result<Unit> Draw(GWorld &gWorld) noexcept;

private:
  explicit Game(MaskedImage &&avatar) noexcept;
  MaskedImage avatar;
};

} // namespace AtelierEsri
