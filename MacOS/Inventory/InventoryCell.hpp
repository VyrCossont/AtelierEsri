#pragma once

#include "Breeze/Alchemy.hpp"
#include "Drawing.hpp"
#include "Material.hpp"
#include "SpriteSheet.hpp"

namespace AtelierEsri {

class InventoryCell {
 public:
  InventoryCell(
      const Breeze::Item& item,
      const Material& material,
      const SpriteSheet& spriteSheet,
      V2I origin
  );

  static constexpr V2I Size = {32, 32};

  void Draw(const GWorld& gWorld) const;

  [[nodiscard]] Rect Bounds() const;

 private:
  const Breeze::Item& item;
  const Material& material;
  const SpriteSheet& spriteSheet;
  V2I origin;
  bool hilite = false;
};

}  // namespace AtelierEsri
