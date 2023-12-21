#pragma once

#include <better-enums/enum.h>

#include <cstdint>
#include <functional>
#include <optional>
#include <string>
#include <unordered_set>
#include <variant>
#include <vector>

#include "EnumSet.hpp"
#include "Geometry/V2.hpp"

namespace Breeze {
using Quality = int;

using ElementCount = int;

using EffectSlot = int;
constexpr int NumEffectSlots = 4;

struct RecipeGridPoint : V2<RecipeGridPoint, int> {
  RecipeGridPoint(const int x, const int y) : V2(x, y) {}
};

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

BETTER_ENUM(Stat, uint8_t, ATK, DEF, SPD)

BETTER_ENUM(
    EquipmentKind,
    uint8_t,
    None,
    Weapon,
    Armor,
    Accessory,
    Attack,
    Heal,
    Buff,
    Debuff,
    Tool
)

// NOLINTEND(*-explicit-constructor, *-no-recursion)

// TODO: figure out how to document these enums.
//  The below highlight in CLion but don't show up in hover docs.

/*! \enum EquipmentKind::None
 * Not equippable. Raw materials, synthesis materials, key items, etc.
 */

/*! \enum EquipmentKind::Tool
 * Includes harvesting tools used in the field: axes, fishing rods, etc.
 * as well as field utility items like hiking boots and backpacks.
 */

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

struct EffectEquipmentStat {
  Stat stat;
  int bonus;
};

/// Does not stack with itself.
struct EffectSingleTarget {};

using Effect = std::variant<
    EffectQuality,
    EffectSynthQuantity,
    EffectAddElement,
    EffectAddCategory,
    EffectElementalDamage,
    EffectEquipmentStat,
    EffectSingleTarget>;

struct Material;

/// Recipe nodes can take either a specific material or a category.
using RecipeNodeInput =
    std::variant<std::reference_wrapper<const Material>, Category>;

/// This is a requirement on the node's *parent*:
/// when the parent has an elemental value of this much, enable this node.
struct RecipeNodeElementalRequirement {
  Element element;
  ElementCount count;
};

struct RecipeNodeEffect {
  ElementCount count;
  Effect effect;
  std::optional<EffectSlot> slot;
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
  std::optional<std::reference_wrapper<const RecipeNode>> parent;
  std::optional<RecipeNodeElementalRequirement> elementalUnlockRequirement;
  std::optional<Quality> qualityUnlockRequirement;
};

/// Recipe for a material.
struct Recipe {
  std::vector<RecipeNode> nodes;
  // TODO: recipes will have plot, level, or recipe book requirements
};

/// An abstract material, such as red neutralizer.
struct Material {
  /// Material catalog ID, used to look up display info, etc.
  size_t id;

  /// Raw materials, key items, etc. don't have recipes.
  std::optional<Recipe> recipe;

  EquipmentKind equipmentKind = EquipmentKind::None;

  // Defaults for generating materials.
  // Syntheses or field gathering bonuses may give individual items more.
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

}  // namespace Breeze

// Specializations so we can use `std::unordered_set` with
// `std::reference_wrapper<const Breeze::Item>` elements,
// which can only meaningfully be compared by address equality.

template <>
struct std::hash<std::reference_wrapper<const Breeze::Item>> {
  size_t operator()(const Breeze::Item &item) const noexcept {
    return std::hash<const Breeze::Item *>()(&item);
  }
};

template <>
struct std::equal_to<std::reference_wrapper<const Breeze::Item>> {
  constexpr bool operator()(const Breeze::Item &lhs, const Breeze::Item &rhs)
      const {
    return &lhs == &rhs;
  }
};

namespace Breeze {

/// Item placed at a node. Mostly useful as an undo stack entry.
struct SynthesisPlacement {
  const RecipeNode &node;
  const Item &item;
};

using PlayerInventory = std::vector<Item>;

/// Output of a synthesis.
/// Depending on stage, may have more traits than legal for an inventory item.
struct SynthesisResult {
  Item item;
  int quantity = 1;
  /// References to inventory items that would be consumed.
  std::unordered_set<std::reference_wrapper<const Item>> usedItems;

  /// Consume the used items and add the synthesis results.
  /// Should be called at most once.
  void ApplyToInventory(PlayerInventory &inventory) const;
};

PlayerInventory DemoInventory(const std::vector<Material> &catalog);

class SynthesisState {
 public:
  SynthesisState(
      const Material &material,
      int maxPlacements,
      int maxQuality,
      const PlayerInventory &inventory
  );
  // This type has internal pointers and can't be trivially copied.
  SynthesisState(const SynthesisState &src) = delete;
  SynthesisState operator=(const SynthesisState &src) = delete;

  [[nodiscard]] const Material &Output() const;

  /// Items allowed for a given node.
  /// Skips items you've already used.
  [[nodiscard]] std::vector<std::reference_wrapper<const Item>> AllowedItemsFor(
      const RecipeNode &node
  ) const;

  /// Can we finish the synthesis?
  [[nodiscard]] bool CanFinish() const;

  /// Can we add any more items?
  [[nodiscard]] int PlacementsRemaining() const;

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

  /// Add an item to a node.
  void Place(const RecipeNode &node, const Item &item);

  /// Undo the last item placement.
  /// Returns true if there was something to undo,
  /// or false if the undo stack is empty.
  bool Undo();

 private:
  /// Material being synthesized. Must have a recipe.
  const Material &material;

  /// Determined by your alchemy level and skills.
  int maxPlacements;
  /// Determined by your alchemy level and skills.
  int maxQuality;

  /// Your inventory.
  const PlayerInventory &inventory;

  /// Items you've placed on nodes.
  std::vector<SynthesisPlacement> placements;
};

}  // namespace Breeze
