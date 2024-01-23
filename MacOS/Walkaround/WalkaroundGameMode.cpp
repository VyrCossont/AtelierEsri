#include "WalkaroundGameMode.hpp"

#include "AppResources.h"
#include "Assets.h"

namespace AtelierEsri {

WalkaroundGameMode::WalkaroundGameMode(
    Game& game, const ResourceID tmx_resource_id, const std::string& name
)
    : GameMode(game),
      // Yes, this is intentional. For now.
      window(atelierInteriorWINDResourceID),
      tilemap(TMXResourceIDResolver::get(tmx_resource_id)) {
  window.onUpdate = [&]([[maybe_unused]] const Window& window) { draw(); };
  window.Title(name);
}

// NOLINTBEGIN(*-convert-member-functions-to-static)
void WalkaroundGameMode::draw() const {
  // NOLINTEND(*-convert-member-functions-to-static)
  const GWorldActiveGuard activeGuard = window.MakeActivePort();
  QD::Reset();

  // TODO: support Tiled TMX map background color
  const Pattern pattern = QD::Black();
  const Rect windowRect = window.PortBounds();
  FillRect(&windowRect, &pattern);

  // TODO: hardcoded to village map
  // TODO: intersperse sprites
  const R2I src = windowRect;
  for (const size_t layer_index :
       {assetVillageGroundTileLayerIndex, assetVillageBuildingsTileLayerIndex
       }) {
    tilemap.draw_layer(src, windowRect, layer_index);
  }
}

// NOLINTBEGIN(*-convert-member-functions-to-static)
void WalkaroundGameMode::invalidate() const {
  // NOLINTEND(*-convert-member-functions-to-static)
  const GWorldActiveGuard activeGuard = window.MakeActivePort();
  const Rect windowRect = window.PortBounds();
#if TARGET_API_MAC_CARBON
  InvalWindowRect(window.Unmanaged(), &windowRect);
#else
  InvalRect(&windowRect);
#endif
}

}  // namespace AtelierEsri
