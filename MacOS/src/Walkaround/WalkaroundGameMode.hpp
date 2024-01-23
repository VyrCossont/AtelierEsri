#pragma once

#include "Game.hpp"
#include "Tilemap.hpp"
#include "Window.hpp"

namespace AtelierEsri {

class WalkaroundGameMode final : public GameMode {
 public:
  explicit WalkaroundGameMode(
      Game& game, ResourceID tmx_resource_id, const std::string& name
  );

  /// Called when we exit a map, returning to the previous location.
  void exit();

  // TODO: switch to a new mode if we entered an interesting building

 private:
  void draw() const;
  void invalidate() const;

  Window window;
  const Tilemap tilemap;
};

}  // namespace AtelierEsri
