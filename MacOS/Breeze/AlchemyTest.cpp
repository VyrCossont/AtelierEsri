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
