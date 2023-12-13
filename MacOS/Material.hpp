#pragma once

#include "Breeze/Alchemy.hpp"

namespace AtelierEsri {

/// Adds name and icon to a Breeze material.
struct Material {
  Breeze::Material data;
  std::string name;
  std::string description;
  size_t spriteIndex;

  static std::vector<Material> Catalog();
};

}  // namespace AtelierEsri
