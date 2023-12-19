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
  const auto& copperOre = catalog[42];
  constexpr int maxPlacements = 5;
  constexpr int maxQuality = 120;
  const auto inventory = DemoInventory(catalog);
  SynthesisState state{copperOre, maxPlacements, maxQuality, inventory};

  REQUIRE(!state.CanFinish());

  const auto& oreNode = copperOre.recipe->nodes[0];
  REQUIRE(state.ItemsPlacedIn(oreNode).empty());

  const auto oreItems = state.AllowedItemsFor(oreNode);
  REQUIRE(!oreItems.empty());

  REQUIRE(state.CanPlace());
  state.Place(oreNode, oreItems[0]);
  REQUIRE(!state.ItemsPlacedIn(oreNode).empty());
  REQUIRE(!state.CanFinish());

  const auto& fuelNode = copperOre.recipe->nodes[1];
  REQUIRE(state.ItemsPlacedIn(fuelNode).empty());

  const auto fuelItems = state.AllowedItemsFor(fuelNode);
  REQUIRE(!fuelItems.empty());

  REQUIRE(state.CanPlace());
  state.Place(fuelNode, fuelItems[0]);
  REQUIRE(!state.ItemsPlacedIn(fuelNode).empty());
  REQUIRE(state.CanFinish());

  REQUIRE(state.PlacementsRemaining() == 3);

  const auto [item, quantity] = state.Result();
  REQUIRE(quantity == 1);
  REQUIRE(&item.material == &copperOre);
}