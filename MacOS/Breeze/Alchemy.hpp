#pragma once

#include <cstdint>
#include <optional>
#include <string>
#include <variant>

#include <better-enums/enum.h>

#include "EnumSet.hpp"

namespace Breeze {

using Quality = uint16_t;
using ElementCount = uint8_t;

struct RecipeGridPoint {
  int8_t x;
  int8_t y;
};

// NOLINTBEGIN(*-explicit-constructor, *-no-recursion)

BETTER_ENUM(Element, uint8_t, Fire, Ice, Lightning, Wind)

BETTER_ENUM(Category, uint32_t, Water, Plants, Uni, Flowers, Medicinal, Poisons,
            Elixirs, Sand, Stone, Ore, Gemstones, Gunpowder, Fuel, Edibles,
            Fruit, Beehives, Mushrooms, Neutralizers, GeneralGoods, Metal,
            Jewels, Spices, Seeds, Food, Medicine, Bombs, MagicTools, Ingots,
            Cloth, Weapons, Armor, Accessories)

// NOLINTEND(*-explicit-constructor, *-no-recursion)

struct CategoryData {
  std::string name;
  std::string abbrev;
};

/// An abstract material, such as red neutralizer.
struct Material {
  std::string name;
};

/// An actual instance of a material, such as quality 200 red neutralizer with
/// the Inferno trait.
struct Item {
  Material &material;
  EnumSet<Element> elements{};
  ElementCount elementValue{};
  Quality quality{};
  EnumSet<Category> categories{};
};

struct EffectQuality {
  Quality bonus;
};

struct EffectSynthQuantity {
  uint8_t bonus;
};

struct EffectElementalDamage {
  Element element;
  uint16_t amount{};
};

using Effect =
    std::variant<EffectQuality, EffectSynthQuantity, EffectElementalDamage>;

} // namespace Breeze
