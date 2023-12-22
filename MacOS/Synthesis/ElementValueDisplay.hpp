#pragma once

#include "Breeze/Alchemy.hpp"
#include "SpriteSheet.hpp"

namespace AtelierEsri {

/// Display a pip or empty slot for each element, followed by a numeric value.
class ElementValueDisplay {
 public:
  explicit ElementValueDisplay(const SpriteSheet& spriteSheet);

  Breeze::EnumSet<Breeze::Element> elements;
  Breeze::ElementCount elementValue = 0;

  V2I origin = {0, 0};

  [[nodiscard]] R2I Bounds() const;

  void Update() const;

 private:
  const SpriteSheet& spriteSheet;

  static size_t Icon(Breeze::Element element);

  static constexpr int PipWidth = 8;
};

}  // namespace AtelierEsri
