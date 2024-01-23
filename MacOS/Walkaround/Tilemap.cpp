#include "Tilemap.hpp"

#include <utility>

#include "Assets.h"
#include "Drawing.hpp"
#include "SpriteSheet.hpp"

namespace AtelierEsri {

// TODO: can't handle unmasked images yet
Tileset::Tileset(const TSXAsset& asset)
    : image(MaskedImage::Get(
          asset.image_pict_resource_id, asset.mask_pict_resource_id.value()
      )),
      tile_size{asset.tile_width, asset.tile_height} {
  const V2I image_size{asset.image_width, asset.image_height};
  const int tiles_per_row = image_size.x / tile_size.x;
  const int tiles_per_col = image_size.y / tile_size.y;
  const int num_valid_tiles = tiles_per_row * tiles_per_col;

  tile_origins.reserve(num_valid_tiles);
  // NOLINTBEGIN(*-too-small-loop-variable)
  for (uint16_t tile_id = 0; tile_id < num_valid_tiles; ++tile_id) {
    // NOLINTEND(*-too-small-loop-variable)
    const auto [quot, rem] = std::div(tile_id, tiles_per_row);
    const int row = quot;
    const int col = rem;
    tile_origins.emplace_back(col * tile_size.x, row * tile_size.y);
  }
}

void Tileset::draw_tile(const TilemapTile& tile, const R2I& dst) const {
  if (tile.tile_id >= tile_origins.size()) {
    BAIL("tile ID too big for this tileset");
  }
  // TODO: support flip flags
  if (tile.flip_h || tile.flip_v || tile.flip_d) {
    BAIL("tile flip flags aren't supported yet");
  }
  image.Draw(R2I{tile_origins[tile.tile_id], tile_size}, dst);
}

std::shared_ptr<Tileset> TSXResourceIDResolver::get(const ResourceID resource_id
) {
  if (const auto existing = cache.find(resource_id); existing != cache.end()) {
    return existing->second.lock();
  }

  if (resource_id == assetKenneyMonochromeRpgTsxResourceId) {
    std::shared_ptr<Tileset> loaded{new Tileset{assetKenneyMonochromeRpgTsx}};
    cache[resource_id] = loaded;
    return loaded;
  }

  BAIL("unknown 'TSX ' resource ID");
}

std::unordered_map<ResourceID, std::weak_ptr<Tileset>>
    TSXResourceIDResolver::cache = {};

TilemapTile::TilemapTile(
    const std::shared_ptr<Tileset>& tileset, const TMXTile& tmx_tile
)
    : flip_h(tmx_tile.flip_h),
      flip_v(tmx_tile.flip_v),
      flip_d(tmx_tile.flip_d),
      tileset(tileset),
      tile_id(tmx_tile.tile_id) {}

void TilemapTile::draw_tile(const R2I& dst) const {
  tileset->draw_tile(*this, dst);
}

TilemapTileLayer::TilemapTileLayer(
    const Tilemap& tilemap, const TMXTileLayer& tmx_tile_layer
)
    : name(tmx_tile_layer.name),
      tilemap(tilemap),
      size(tmx_tile_layer.width, tmx_tile_layer.height) {
  tiles.reserve(tmx_tile_layer.tiles.size());
  for (const auto& tmx_tile : tmx_tile_layer.tiles) {
    if (tmx_tile.tileset_ordinal) {
      tiles.emplace_back(
          TilemapTile(tilemap.tilesets[tmx_tile.tileset_ordinal - 1], tmx_tile)
      );
    } else {
      tiles.emplace_back();
    }
  }
}

const TilemapTile* TilemapTileLayer::tile_at(const V2I& pos) const {
  if (pos.x < 0 || pos.y < 0) {
    return nullptr;
  }
  const size_t tile_index = pos.x + size.x * pos.y;
  if (tile_index >= tiles.size()) {
    return nullptr;
  }
  auto& maybe_tile = tiles[tile_index];
  return maybe_tile ? maybe_tile.operator->() : nullptr;
}

TilemapRegionGroup::TilemapRegionGroup(
    std::string name, const std::vector<Rect>& regions
)
    : name(std::move(name)), regions(regions) {}

Tilemap::Tilemap(const TMXAsset& asset)
    : size(asset.width, asset.height),
      tile_size(asset.tile_width, asset.tile_height) {
  tilesets.reserve(asset.tileset_resource_ids.size());
  for (const auto resource_id : asset.tileset_resource_ids) {
    tilesets.push_back(TSXResourceIDResolver::get(resource_id));
  }

  tile_layers.reserve(asset.tile_layers.size());
  for (const auto& tmx_tile_layer : asset.tile_layers) {
    tile_layers.emplace_back(*this, tmx_tile_layer);
  }

  tilesets.reserve(asset.region_groups.size());
  for (const auto& [name, rgn_resource_id] : asset.region_groups) {
    region_groups.emplace_back(name, SpriteSheet::ReadRGN(rgn_resource_id));
  }
}

void Tilemap::draw_layer(
    const R2I& src, const R2I& dst, const size_t layer_index
) const {
  if (layer_index >= tile_layers.size()) {
    BAIL("layer index too large");
  }
  const TilemapTileLayer& tile_layer = tile_layers[layer_index];

  // Clip our drawing so we don't need to worry about partial tiles.
  const ChangeClip change_clip(dst);

  // Find NW and SE corners of tile position range that fully contains `src`.
  const V2I tile_nw = src.NW() / tile_size;
  const V2I tile_se = src.SE() / tile_size;

  R2I tile_dst = {dst.origin, tile_size};
  for (int tile_y = tile_nw.y; tile_y <= tile_se.y; ++tile_y) {
    for (int tile_x = tile_nw.x; tile_x <= tile_se.x; ++tile_x) {
      if (const TilemapTile* tile = tile_layer.tile_at({tile_x, tile_y})) {
        tile->draw_tile(tile_dst);
      }
      tile_dst.origin.x += tile_size.x;
    }
    tile_dst.origin.x = dst.origin.x;
    tile_dst.origin.y += tile_size.y;
  }
}

Tilemap TMXResourceIDResolver::get(const ResourceID resource_id) {
  if (resource_id == assetVillageTmxResourceId) {
    return Tilemap(assetVillageTmx);
  }

  BAIL("unknown 'TMX ' resource ID");
}

}  // namespace AtelierEsri