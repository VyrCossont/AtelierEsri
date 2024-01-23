#include <algorithm>
#include <catch2/catch_test_macros.hpp>

#include "Alchemy.hpp"

using namespace Breeze;

TEST_CASE("alchemy category properties") {
  constexpr Category bomb = Category::Bombs;
  REQUIRE(bomb._to_string() == std::string("Bombs"));
  REQUIRE(bomb._to_integral() == 25);
}

TEST_CASE("alchemy bitset properties") {
  constexpr EnumSet<Category> beeBomb =
      1 << Category::Bombs | 1 << Category::Beehives;
  REQUIRE(beeBomb.test(Category::Bombs));
  REQUIRE(beeBomb.test(Category::Beehives));
  REQUIRE(!beeBomb.test(Category::Uni));
}

TEST_CASE("alchemy materials catalog") {
  REQUIRE(!Material::Catalog().empty());
}

TEST_CASE("happy path copper ore synthesis") {
  const auto catalog = Material::Catalog();
  const auto& copperIngot = catalog[42];
  constexpr int maxPlacements = 5;
  constexpr int maxQuality = 120;
  auto inventory = DemoInventory(catalog);
  const int initialInventorySize = inventory.size();
  SynthesisState state{copperIngot, maxPlacements, maxQuality, inventory};

  REQUIRE(!state.CanFinish());

  const auto& oreNode = copperIngot.recipe->nodes[0];
  REQUIRE(state.ItemsPlacedIn(oreNode).empty());

  const auto oreItems = state.AllowedItemsFor(oreNode);
  REQUIRE(!oreItems.empty());

  REQUIRE(state.CanPlace());
  state.Place(oreNode, oreItems[0]);
  REQUIRE(!state.ItemsPlacedIn(oreNode).empty());
  REQUIRE(!state.CanFinish());

  const auto& fuelNode = copperIngot.recipe->nodes[1];
  REQUIRE(state.ItemsPlacedIn(fuelNode).empty());

  const auto fuelItems = state.AllowedItemsFor(fuelNode);
  REQUIRE(!fuelItems.empty());

  REQUIRE(state.CanPlace());
  state.Place(fuelNode, fuelItems[0]);
  REQUIRE(!state.ItemsPlacedIn(fuelNode).empty());
  REQUIRE(state.CanFinish());

  REQUIRE(state.PlacementsRemaining() == 3);

  const auto result = state.Result();
  REQUIRE(result.quantity == 1);
  REQUIRE(&result.item.material == &copperIngot);
  REQUIRE(result.usedItems.size() == 2);

  result.ApplyToInventory(inventory);
  REQUIRE(
      inventory.size() ==
      initialInventorySize - result.usedItems.size() + result.quantity
  );
  REQUIRE(
      std::find_if(
          inventory.begin(),
          inventory.end(),
          [&](const auto& item) -> bool {
            return &item.material == &copperIngot;
          }
      ) != inventory.end()
  );
}