#include "Alchemy.hpp"

#include <iostream>
#include <sstream>
#include <stdexcept>

namespace Breeze {

void SynthesisResult::ApplyToInventory(PlayerInventory &inventory) const {
  // TODO: Item contains a const reference and can't be move-assigned.
  //  Consider replacing const refs and ref wrappers with pointers to consts.

  PlayerInventory temp;
  for (const Item &item : inventory) {
    if (!usedItems.count(item)) {
      temp.push_back(item);
    }
  }
  for (int i = 0; i < quantity; ++i) {
    temp.push_back(item);
  }

  inventory.clear();
  for (const Item &item : temp) {
    inventory.push_back(item);
  }
}

PlayerInventory DemoInventory(const std::vector<Material> &catalog) {
  PlayerInventory inventory{};

  // Give five of every raw material.
  for (size_t materialIndex = 0; materialIndex < catalog.size() - 1;
       ++materialIndex) {
    const Material &material = catalog[materialIndex];
    Item item = {
        .material = material,
        .elements = material.elements,
        .elementValue = material.elementValue,
        .quality = 50,
        .categories = material.categories,
        .traits = material.traits
    };
    for (size_t itemIndex = 0; itemIndex < 5; ++itemIndex) {
      inventory.push_back(item);
    }
  }

  return inventory;
}

SynthesisState::SynthesisState(
    const Material &material,
    const int maxPlacements,
    const int maxQuality,
    const PlayerInventory &inventory
)
    : material(material),
      maxPlacements(maxPlacements),
      maxQuality(maxQuality),
      inventory(inventory) {
  if (!material.recipe) {
    std::stringstream message;
    message << "Can't synthesize material with ID " << material.id
            << ": material doesn't have a recipe";
    throw std::invalid_argument(message.str());
  }
}

const Material &SynthesisState::Output() const { return material; }

std::vector<std::reference_wrapper<const Item>> SynthesisState::AllowedItemsFor(
    const RecipeNode &node
) const {
  std::vector<std::reference_wrapper<const Item>> items{};

  /// Set of all items that have already been placed.
  std::unordered_set<std::reference_wrapper<const Item>> itemsInUse{};
  for (const RecipeNode &recipeNode : material.recipe->nodes) {
    for (std::reference_wrapper<const Item> placedItem :
         ItemsPlacedIn(recipeNode)) {
      itemsInUse.insert(placedItem);
    }
  }

  if (const auto materialInput =
          std::get_if<std::reference_wrapper<const Material>>(&node.input)) {
    const Material &material = *materialInput;
    for (const Item &item : inventory) {
      if (&item.material == &material && !itemsInUse.count(item)) {
        items.emplace_back(item);
      }
    }

  } else if (const auto categoryInput = std::get_if<Category>(&node.input)) {
    const Category category = *categoryInput;
    for (const Item &item : inventory) {
      if (item.categories.test(category) && !itemsInUse.count(item)) {
        items.emplace_back(item);
      }
    }

  } else {
    throw std::logic_error("Unknown recipe input type.");
  }

  return items;
}

void SynthesisState::Place(const RecipeNode &node, const Item &item) {
  bool itemAcceptable = false;
  // ReSharper disable once CppRangeBasedForIncompatibleReference
  for (const Item &allowedItem : AllowedItemsFor(node)) {
    if (&item == &allowedItem) {
      itemAcceptable = true;
      break;
    }
  }
  if (!itemAcceptable) {
    std::stringstream message;
    message << "Node @ " << node.gridPos
            << " can't accept item with material ID " << item.material.id
            << " and categories {";
    bool first = true;
    for (auto category : Category::_values()) {
      if (!item.categories.test(category)) {
        continue;
      }

      if (first) {
        first = false;
      } else {
        message << ", ";
      }
      message << category._to_string();
    }
    message << "}";
    throw std::invalid_argument(message.str());
  }

  placements.push_back({node, item});
}

bool SynthesisState::CanUndo() const { return !placements.empty(); }

bool SynthesisState::Undo() {
  if (placements.empty()) {
    return false;
  }
  placements.pop_back();
  return true;
}

bool SynthesisState::CanFinish() const {
  // ReSharper disable once CppUseStructuredBinding
  for (const auto &node : material.recipe->nodes) {  // NOLINT(*-use-anyofallof)
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

int SynthesisState::PlacementsRemaining() const {
  return maxPlacements - static_cast<int>(placements.size());
}

bool SynthesisState::CanPlace() const { return PlacementsRemaining() > 0; }

bool SynthesisState::Unlocked(const RecipeNode &node) const {
  if (node.elementalUnlockRequirement) {
    if (!node.parent) {
      std::stringstream message;
      message << "Malformed recipe: node @ " << node.gridPos
              << " has elemental unlock requirement but no parent";
      throw std::invalid_argument(message.str());
    }

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
              .material = material,
              .elements = material.elements,
              .elementValue = material.elementValue,
              .categories = material.categories,
              .traits = material.traits,
              .effects = {},
              .slotEffects = {},
          },
  };

  // Start quality at the average of the ingredient qualities.
  for (const auto [node, placementItem] : placements) {
    result.item.quality += placementItem.quality;
  }
  result.item.quality /= static_cast<int>(placements.size());

  // For each node, apply any unlocked effects.
  for (auto &node : material.recipe->nodes) {
    ElementCount elementValue = 0;
    for (const auto &[placementNode, item] : placements) {
      if (&node == &placementNode && item.elements[node.element]) {
        elementValue += result.item.elementValue;
      }
    }

    for (auto &[count, effect, slot] : node.effects) {
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
        // Effects that aren't consumed.
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

  // Apply quality cap.
  result.item.quality = std::min(maxQuality, result.item.quality);

  // Collect items that would be consumed by this synthesis.
  for (const auto [node, placementItem] : placements) {
    result.usedItems.insert(placementItem);
  }

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

  // Override defaults for a few items:

  // Elerium.
  catalog[5].elementValue = 3;
  catalog[5].elements = 1 << Element::Wind;
  catalog[5].categories = 1 << Category::Gemstones | 1 << Category::Poisons;

  // Lump.
  catalog[16].elements = 1 << Element::Wind;
  catalog[16].categories = 1 << Category::Fuel;

  // Page.
  catalog[24].categories = 1 << Category::Fuel;

  // Water.
  catalog[39].elements = 1 << Element::Ice;
  catalog[39].categories = 1 << Category::Water;

  // Wood.
  catalog[40].elementValue = 2;
  catalog[40].categories = 1 << Category::Fuel;

  // Copper ingot: our first synthesizable item.
  Recipe copperIngotRecipe;
  copperIngotRecipe.nodes.push_back(  // NOLINT(*-use-emplace)
      {
          .gridPos = {0, 0},
          .element = Element::Fire,
          .input = catalog[19],  // copper ore
          .effects =
              {
                  {.count = 1, .effect = EffectEquipmentStat{Stat::DEF, 1}},
                  {.count = 2, .effect = EffectEquipmentStat{Stat::DEF, 2}},
                  {.count = 3, .effect = EffectEquipmentStat{Stat::DEF, 3}},
              },
      }
  );
  copperIngotRecipe.nodes.push_back(  // NOLINT(*-use-emplace)
      {
          .gridPos = {1, 0},
          .element = Element::Fire,
          .input = Category::Fuel,
          .effects =
              {
                  {.count = 1, .effect = EffectEquipmentStat{Stat::ATK, 1}},
                  {.count = 2, .effect = EffectEquipmentStat{Stat::ATK, 2}},
                  {.count = 3, .effect = EffectEquipmentStat{Stat::ATK, 3}},
              },
          .parent = copperIngotRecipe.nodes[0],
      }
  );
  const Material copperIngot{
      .id = 42,
      .recipe = copperIngotRecipe,
      .elements = 1 << Element::Fire,
      .elementValue = 1,
      .categories = 1 << Category::Ingots,
  };
  catalog.push_back(copperIngot);

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
