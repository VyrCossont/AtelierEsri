#pragma once

#include "Breeze/Alchemy.hpp"

namespace AtelierEsri {

/// Adds name and icon to a Breeze material.
/// Look these up by Breeze material ID.
struct Material {
  std::string name;
  std::string description;
  size_t spriteIndex;

  /// Provide metadata for Breeze demo catalog.
  static std::vector<Material> Catalog(
      const std::vector<Breeze::Material>& breezeCatalog
  );
};

}  // namespace AtelierEsri
