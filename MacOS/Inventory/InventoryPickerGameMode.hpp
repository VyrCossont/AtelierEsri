#pragma once

#include "Game.hpp"
#include "InventoryController.hpp"

namespace AtelierEsri {

class InventoryPickerGameMode final : public GameMode {
 public:
  explicit InventoryPickerGameMode(Game& game);

 private:
  Window window;
};

}  // namespace AtelierEsri
