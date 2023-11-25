#include <catch2/catch_test_macros.hpp>

#include "Alchemy.hpp"

using namespace Breeze;

TEST_CASE("alchemy library says hello") {
  Alchemy alchemy{};
  REQUIRE(alchemy.Text() == std::string("hello world"));
}
