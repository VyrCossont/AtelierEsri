#include "Alchemy.hpp"

#include <cassert>

namespace Breeze {

SynthesisResult SynthesisState::Result() const {
  SynthesisResult result = {
      .item =
          Item{
              .material = recipe.material,
              .elements = recipe.material.elements,
              .elementValue = recipe.material.elementValue,
              .categories = recipe.material.categories,
              .traits = recipe.material.traits,
          },
      .quantity = 1};

  // Start quality at the average of the ingredient qualities.
  for (const auto [node, placementItem] : placements) {
    result.item.quality += placementItem.quality;
  }
  result.item.quality /= static_cast<int>(placements.size());

  // For each node, apply any unlocked effects.
  for (auto &recipeNode : recipe.nodes) {
    ElementCount elementValue = 0;
    for (const auto &[node, item] : placements) {
      if (&recipeNode == &node && item.elements[recipeNode.element]) {
        elementValue += result.item.elementValue;
      }
    }

    for (auto &[effect, count] : recipeNode.effects) {
      if (count < elementValue) {
        break;
      }
      elementValue -= count;

      // ReSharper disable CppDeclarationHidesLocal
      if (const auto e = std::get_if<EffectQuality>(&effect)) {
        result.item.quality += e->bonus;
      } else if (const auto e = std::get_if<EffectSynthQuantity>(&effect)) {
        result.quantity += e->bonus;
      } else if (const auto e = std::get_if<EffectAddElement>(&effect)) {
        result.item.elements[e->element] = true;
      } else if (const auto e = std::get_if<EffectAddCategory>(&effect)) {
        result.item.categories[e->category] = true;
      } else {
        // TODO: many effects can be passed through to the trait list
        assert("Unknown variant case for Breeze::Effect");
      }
      // ReSharper restore CppDeclarationHidesLocal
    }
  }

  return result;
}

} // namespace Breeze
