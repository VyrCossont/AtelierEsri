#pragma once

#include <cstdint>
#include <functional>
#include <optional>
#include <string>
#include <variant>
#include <vector>

#include <better-enums/enum.h>

#include "EnumSet.hpp"

namespace Breeze {

using Quality = int;
using ElementCount = int;

struct RecipeGridPoint {
  int x;
  int y;
};

// NOLINTBEGIN(*-explicit-constructor, *-no-recursion)

BETTER_ENUM(Element, uint8_t, Fire, Ice, Lightning, Wind)

// TODO: (Vyr) we'll probably need more categories
BETTER_ENUM(Category, uint32_t, Water, Plants, Uni, Flowers, Medicinal, Poisons,
            Elixirs, Sand, Stone, Ore, Gemstones, Gunpowder, Fuel, Edibles,
            Fruit, Beehives, Mushrooms, Neutralizers, GeneralGoods, Metal,
            Jewels, Spices, Seeds, Food, Medicine, Bombs, MagicTools, Ingots,
            Cloth, Weapons, Armor, Accessories)

// NOLINTEND(*-explicit-constructor, *-no-recursion)

struct EffectQuality {
  Quality bonus;
};

struct EffectSynthQuantity {
  int bonus;
};

struct EffectAddElement {
  Element element;
};

struct EffectAddCategory {
  Category category;
};

struct EffectElementalDamage {
  Element element;
  int amount;
};

using Effect =
    std::variant<EffectQuality, EffectSynthQuantity, EffectAddElement,
                 EffectAddCategory, EffectElementalDamage>;

/// An abstract material, such as red neutralizer.
struct Material {
  EnumSet<Element> elements;
  ElementCount elementValue;
  EnumSet<Category> categories;
  std::vector<Effect> traits;

  /// Materials catalog for demos,
  static std::vector<Material> Catalog();
};

/// An actual instance of a material, such as quality 200 red neutralizer with
/// the Inferno trait.
struct Item {
  const Material &material;
  /// Should be a superset of `material.elements`.
  EnumSet<Element> elements;
  /// Should be ≥ `material.elementValue`.
  ElementCount elementValue;
  Quality quality;
  /// Should be a superset of `material.categories`.
  EnumSet<Category> categories;
  // TODO: effects (1, 2, 3, 4) and traits (from materials) should be separated.
  /// Should be a superset of `material.traits`.
  std::vector<Effect> traits;
};

/// Recipe nodes can take either a specific material or a category.
using RecipeNodeInput =
    std::variant<std::reference_wrapper<Material>, Category>;

/// This is a requirement on the node's *parent*:
/// when the parent has an elemental value of this much, enable this node.
struct RecipeNodeElementalRequirement {
  Element element;
  ElementCount count;
};

struct RecipeNodeEffect {
  Effect effect;
  ElementCount count;
};

/// Node in an abstract recipe.
struct RecipeNode {
  RecipeGridPoint gridPos;
  Element element;
  RecipeNodeInput input;
  std::vector<RecipeNodeEffect> effects;
  std::optional<RecipeNodeElementalRequirement> elementalRequirement;
  std::optional<Quality> qualityRequirement;
  /// All required nodes must have at least one item to finish synthesis.
  bool required;
  std::optional<std::reference_wrapper<RecipeNode>> parent;
};

struct Recipe {
  std::vector<RecipeNode> nodes;
  Material &material;
};

/// Item placed at a node. Mostly useful as an undo stack entry.
struct SynthesisPlacement {
  RecipeNode &node;
  Item &item;
};

struct SynthesisResult {
  Item item;
  int quantity{};
};

struct SynthesisState {
  Recipe &recipe;
  /// Determined by your alchemy level and skills.
  int maxPlacements;
  std::vector<SynthesisPlacement> placements;

  /// Can we finish the synthesis?
  [[nodiscard]] bool CanFinish() const;

  /// Can we add any more items?
  [[nodiscard]] bool CanPlace() const;

  /// Return the current state of the item being synthesized,
  /// including *all* available traits.
  [[nodiscard]] SynthesisResult Result() const;
};

using PlayerInventory = std::vector<Item>;

} // namespace Breeze
