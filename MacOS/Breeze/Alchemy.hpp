#pragma once

#include <better-enums/enum.h>

#include <cstdint>
#include <functional>
#include <optional>
#include <string>
#include <variant>
#include <vector>

#include "EnumSet.hpp"
#include "Geometry/V2.hpp"

namespace Breeze {

using Quality = int;
using ElementCount = int;
using EffectSlot = int;
constexpr int NumEffectSlots = 4;

struct RecipeGridPoint : V2<RecipeGridPoint, int> {};

// NOLINTBEGIN(*-explicit-constructor, *-no-recursion)

BETTER_ENUM(Element, uint8_t, Fire, Ice, Lightning, Wind)

// TODO: (Vyr) we'll probably need more categories
BETTER_ENUM(
    Category,
    uint32_t,
    Water,
    Plants,
    Uni,
    Flowers,
    Medicinal,
    Poisons,
    Elixirs,
    Sand,
    Stone,
    Ore,
    Gemstones,
    Gunpowder,
    Fuel,
    Edibles,
    Fruit,
    Beehives,
    Mushrooms,
    Neutralizers,
    GeneralGoods,
    Metal,
    Jewels,
    Spices,
    Seeds,
    Food,
    Medicine,
    Bombs,
    MagicTools,
    Ingots,
    Cloth,
    Weapons,
    Armor,
    Accessories
)

// NOLINTEND(*-explicit-constructor, *-no-recursion)

/// Consumed in synthesis.
struct EffectQuality {
  Quality bonus;
};

/// Consumed in synthesis.
struct EffectSynthQuantity {
  int bonus;
};

/// Consumed in synthesis.
struct EffectAddElement {
  Element element;
};

/// Consumed in synthesis.
struct EffectAddCategory {
  Category category;
};

struct EffectElementalDamage {
  Element element;
  int amount;
};

/// Does not stack with itself.
struct EffectSingleTarget {};

using Effect = std::variant<
    EffectQuality,
    EffectSynthQuantity,
    EffectAddElement,
    EffectAddCategory,
    EffectElementalDamage,
    EffectSingleTarget>;

/// An abstract material, such as red neutralizer.
struct Material {
  /// Material catalog ID, used to look up display info, etc.
  size_t id;
  EnumSet<Element> elements;
  ElementCount elementValue;
  EnumSet<Category> categories;
  std::vector<Effect> traits;

  /// Materials catalog for demos.
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
  /// Should be a superset of `material.traits`.
  /// Traits are transferrable.
  std::vector<Effect> traits;
  /// Effects are added during synthesis and aren't transferrable.
  std::vector<Effect> effects;
  /// Some effects apply to item effect slots.
  /// These can only be transferred through chain synthesis,
  /// and are replaced by other effects that target the same slot.
  std::array<std::optional<Effect>, NumEffectSlots> slotEffects;

  /// Combined list of all types of effect.
  [[nodiscard]] std::vector<std::reference_wrapper<const Effect>> AllEffects(
  ) const;
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
  std::optional<EffectSlot> slot;
  ElementCount count;
};

/// Node in an abstract recipe.
///
/// Nodes that do not have an elemental or quality unlock requirement are
/// considered mandatory by the recipe, and must have items placed in them to
/// finish the synthesis.
struct RecipeNode {
  RecipeGridPoint gridPos;
  Element element;
  RecipeNodeInput input;
  std::vector<RecipeNodeEffect> effects;
  std::optional<RecipeNodeElementalRequirement> elementalUnlockRequirement;
  std::optional<Quality> qualityUnlockRequirement;
  std::optional<std::reference_wrapper<const RecipeNode>> parent;
};

struct Recipe {
  std::vector<RecipeNode> nodes;
  Material &material;
};

/// Item placed at a node. Mostly useful as an undo stack entry.
struct SynthesisPlacement {
  const RecipeNode &node;
  const Item &item;
};

struct SynthesisResult {
  Item item;
  int quantity{};
};

using PlayerInventory = std::vector<Item>;

struct SynthesisState {
  Recipe &recipe;

  /// Determined by your alchemy level and skills.
  int maxPlacements;
  /// Determined by your alchemy level and skills.
  int maxQuality;

  /// Input: your inventory.
  PlayerInventory inventory;

  /// Items you've placed on nodes.
  std::vector<SynthesisPlacement> placements;

  /// Items allowed for a given node.
  /// Skips items you've already used.
  [[nodiscard]] std::vector<std::reference_wrapper<const Item>> AllowedItemsFor(
      const RecipeNode &node
  ) const;

  /// Add an item to a node.
  void Place(const RecipeNode &node, const Item &item);

  /// Can we finish the synthesis?
  [[nodiscard]] bool CanFinish() const;

  /// Can we add any more items?
  [[nodiscard]] bool CanPlace() const;

  /// Are this node's unlock requirements fulfilled?
  [[nodiscard]] bool Unlocked(const RecipeNode &node) const;

  /// Element value of items placed on a node matching the node's element.
  [[nodiscard]] ElementCount ElementValue(const RecipeNode &node) const;

  /// Element value of items placed on a node matching an arbitrary element.
  [[nodiscard]] ElementCount ElementValue(
      const RecipeNode &node, Element element
  ) const;

  /// Items placed in a given node.
  [[nodiscard]] std::vector<std::reference_wrapper<const Item>> ItemsPlacedIn(
      const RecipeNode &node
  ) const;

  /// Return the current state of the item being synthesized,
  /// including *all* available traits.
  [[nodiscard]] SynthesisResult Result() const;
};

}  // namespace Breeze
