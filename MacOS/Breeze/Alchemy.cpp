#include "Alchemy.hpp"

#include <cassert>

namespace Breeze {

std::vector<std::reference_wrapper<const Item>> SynthesisState::AllowedItemsFor(
    const RecipeNode &node
) const {
  std::vector<std::reference_wrapper<const Item>> items{};

  if (const auto materialInput =
          std::get_if<std::reference_wrapper<Material>>(&node.input)) {
    const Material &material = *materialInput;
    for (const Item &item : inventory) {
      if (&item.material == &material) {
        items.emplace_back(item);
      }
    }

  } else if (const auto categoryInput = std::get_if<Category>(&node.input)) {
    const Category category = *categoryInput;
    for (const Item &item : inventory) {
      if (item.categories.test(category)) {
        items.emplace_back(item);
      }
    }

  } else {
    assert(false, "Unknown material input type");
  }

  return items;
}

void SynthesisState::Place(const RecipeNode &node, const Item &item) {
  placements.push_back({node, item});
}

bool SynthesisState::CanFinish() const {
  // ReSharper disable once CppUseStructuredBinding
  for (const auto &node : recipe.nodes) {  // NOLINT(*-use-anyofallof)
    if (node.elementalUnlockRequirement.has_value() ||
        node.qualityUnlockRequirement.has_value()) {
      // This node is not mandatory. Ignore it.
      continue;
    }

    // This node is mandatory and needs at least one item.
    if (ItemsPlacedIn(node).empty()) {
      return false;
    }
  }

  return true;
}

bool SynthesisState::CanPlace() const {
  return maxPlacements > placements.size();
}

bool SynthesisState::Unlocked(const RecipeNode &node) const {
  if (node.elementalUnlockRequirement) {
    assert(
        node.parent,
        "Malformed recipe: "
        "node has elemental unlock requirement but no parent"
    );
    const Element element = node.elementalUnlockRequirement->element;
    const ElementCount requiredValue = node.elementalUnlockRequirement->count;
    const RecipeNode &parent = *node.parent;
    if (ElementValue(parent, element) < requiredValue) {
      return false;
    }
  }

  if (node.qualityUnlockRequirement) {
    const Quality requiredQuality = *node.qualityUnlockRequirement;
    if (Result().item.quality < requiredQuality) {
      return false;
    }
  }

  return true;
}

ElementCount SynthesisState::ElementValue(const RecipeNode &node) const {
  return ElementValue(node, node.element);
}

ElementCount SynthesisState::ElementValue(
    const RecipeNode &node, const Element element
) const {
  ElementCount elementValue = 0;
  // ReSharper disable once CppRangeBasedForIncompatibleReference
  for (const Item &item : ItemsPlacedIn(node)) {
    if (item.elements.test(element)) {
      elementValue += item.elementValue;
    }
  }
  return elementValue;
}

SynthesisResult SynthesisState::Result() const {
  SynthesisResult result = {
      .item =
          Item{
              .material = recipe.material,
              .elements = recipe.material.elements,
              .elementValue = recipe.material.elementValue,
              .categories = recipe.material.categories,
              .traits = recipe.material.traits,
              .effects = {},
              .slotEffects = {},
          },
      .quantity = 1
  };

  // Start quality at the average of the ingredient qualities.
  for (const auto [node, placementItem] : placements) {
    result.item.quality += placementItem.quality;
  }
  result.item.quality /= static_cast<int>(placements.size());

  // For each node, apply any unlocked effects.
  for (auto &node : recipe.nodes) {
    ElementCount elementValue = 0;
    for (const auto &[placementNode, item] : placements) {
      if (&node == &placementNode && item.elements[node.element]) {
        elementValue += result.item.elementValue;
      }
    }

    for (auto &[effect, slot, count] : node.effects) {
      if (count < elementValue) {
        break;
      }
      elementValue -= count;

      // Some effects are consumed during synthesis.
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
        if (slot) {
          result.item.slotEffects[*slot] = effect;
        } else {
          // TODO: implement stacking/merging
          result.item.effects.push_back(effect);
        }
      }
      // ReSharper restore CppDeclarationHidesLocal
    }
  }

  // Quality cap.
  result.item.quality = std::min(maxQuality, result.item.quality);

  return result;
}

// TODO: add lookup structure for this
std::vector<std::reference_wrapper<const Item>> SynthesisState::ItemsPlacedIn(
    const RecipeNode &node
) const {
  std::vector<std::reference_wrapper<const Item>> items{};
  for (const auto &[placementNode, item] : placements) {
    if (&placementNode == &node) {
      items.emplace_back(item);
    }
  }
  return items;
}

std::vector<Material> Material::Catalog() {
  std::vector<Material> catalog{};

  // Duplicate copper ore a bunch of times.
  for (size_t id = 0; id < 42; ++id) {
    Material material{
        .id = id,
        .elements = 1 << Element::Fire,
        .elementValue = 1,
        .categories = 1 << Category::Ore
    };
    catalog.push_back(material);
  }

  // Copper ingot.
  const Material material{
      .id = 42,
      .elements = 1 << Element::Fire,
      .elementValue = 1,
      .categories = 1 << Category::Ore
  };
  catalog.push_back(material);

  return catalog;
}

std::vector<std::reference_wrapper<const Effect>> Item::AllEffects() const {
  std::vector<std::reference_wrapper<const Effect>> allEffects{};
  allEffects.insert(allEffects.end(), traits.begin(), traits.end());
  allEffects.insert(allEffects.end(), effects.begin(), effects.end());
  for (const auto &slotEffect : slotEffects) {
    if (slotEffect) {
      allEffects.emplace_back(*slotEffect);
    }
  }
  return allEffects;
}

}  // namespace Breeze
