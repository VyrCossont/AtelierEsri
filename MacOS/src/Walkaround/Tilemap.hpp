#pragma once

#include <unordered_map>

#include "MaskedImage.hpp"
#include "TMXData.hpp"
#include "TSXData.hpp"

namespace AtelierEsri {

class TilemapTile;

class Tileset {
 public:
  explicit Tileset(const TSXAsset& asset);

  /// `dst` is in window space.
  void draw_tile(const TilemapTile& tile, const R2I& dst) const;

 private:
  MaskedImage image;
  const V2I tile_size;
  std::vector<V2I> tile_origins;
};

// TODO: Hack for loading TSX assets from code instead of resources,
//  until auto-derived BinIO resource deserializers are ready
class TSXResourceIDResolver {
 public:
  static std::shared_ptr<Tileset> get(ResourceID resource_id);

 private:
  static std::unordered_map<ResourceID, std::weak_ptr<Tileset>> cache;
};

class TilemapTile {
  friend Tileset;

 public:
  TilemapTile(const std::shared_ptr<Tileset>&, const TMXTile& tmx_tile);

  /// `dst` is in window space.
  void draw_tile(const R2I& dst) const;

 private:
  const bool flip_h;
  const bool flip_v;
  const bool flip_d;
  const std::shared_ptr<Tileset> tileset;
  const uint16_t tile_id;
};

class Tilemap;

class TilemapTileLayer {
 public:
  TilemapTileLayer(const Tilemap& tilemap, const TMXTileLayer& tmx_tile_layer);

  /// `pos` is a position in tiles.
  /// Returns `nullptr` if there is no tile at that position.
  [[nodiscard]] const TilemapTile* tile_at(const V2I& pos) const;

  const std::string name;

 private:
  const Tilemap& tilemap;
  /// In tiles.
  const V2I size;
  std::vector<std::optional<TilemapTile>> tiles;
};

class TilemapRegionGroup {
 public:
  TilemapRegionGroup(std::string name, const std::vector<Rect>& regions);

  const std::string name;
  const std::vector<Rect> regions;
};

class Tilemap {
  friend TilemapTileLayer;

 public:
  explicit Tilemap(const TMXAsset& asset);

  /// `src` is in map space.
  /// `dst` is in window space.
  void draw_layer(const R2I& src, const R2I& dst, size_t layer_index) const;

 private:
  /// In tiles.
  const V2I size;
  /// In pixels.
  const V2I tile_size;
  std::vector<std::shared_ptr<Tileset>> tilesets;
  std::vector<TilemapTileLayer> tile_layers;
  std::vector<TilemapRegionGroup> region_groups;
};

// TODO: Hack for loading TMX assets from code instead of resources,
//  until auto-derived BinIO resource deserializers are ready
class TMXResourceIDResolver {
 public:
  static Tilemap get(ResourceID resource_id);
};

}  // namespace AtelierEsri
