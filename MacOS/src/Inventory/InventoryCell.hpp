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
      V2I origin,
      bool selected
  );

  static constexpr V2I Size = {32, 32};

  /// Draw the inventory cell into the current graphics port.
  void Draw() const;

  [[nodiscard]] Rect Bounds() const;

 private:
  const Breeze::Item& item;
  const Material& material;
  const SpriteSheet& spriteSheet;
  V2I origin;
  bool selected;
};

}  // namespace AtelierEsri
