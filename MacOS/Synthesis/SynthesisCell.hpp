#pragma once

#include <cmath>

#include "Breeze/Alchemy.hpp"
#include "Drawing.hpp"
#include "Game.hpp"
#include "SpriteSheet.hpp"

namespace AtelierEsri {

class SynthesisCell {
 public:
  explicit SynthesisCell(
      const Breeze::RecipeNode& node,
      const std::vector<Material>& catalog,
      const SpriteSheet& spriteSheet
  );

  /// Inter-center vertical spacing.
  static constexpr int YHalfSpace = 40;
  /// Inter-center horizontal spacing.
  static constexpr int XHalfSpace = static_cast<int>(YHalfSpace * 2 / sqrt(3));
  static constexpr V2I HalfSpace = {XHalfSpace, YHalfSpace};

  [[nodiscard]] const R2I& Bounds() const;

  void Update() const;

 private:
  // TODO: calc this in constructor, it doesn't change

  [[nodiscard]] static V2I CalculateCenter(const Breeze::RecipeNode& node);
  [[nodiscard]] static R2I CalculateBounds(const V2I& center);

  const Breeze::RecipeNode& node;

  /// Used to find icons for items on a node,
  /// as well as specific-material inputs.
  const std::vector<Material>& catalog;

  /// Used to draw icons for items on a node,
  /// specific-material inputs,
  /// and element pips.
  const SpriteSheet& spriteSheet;

  V2I center;
  R2I bounds;
};

}  // namespace AtelierEsri
