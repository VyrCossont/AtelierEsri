// ReSharper disable CppDFAConstantParameter
#include <catch2/catch_test_macros.hpp>

#include "C2.hpp"
#include "R2.hpp"
#include "V2.hpp"

namespace Breeze {
struct V2I : V2<V2I, int> {
  using Element = int;

  constexpr V2I(const int x, const int y) : V2(x, y){};
};

struct R2I : R2<R2I, V2I> {
  constexpr R2I(const V2I origin, const V2I size) : R2(origin, size){};
};

void require_commutative_intersection(const R2I &a, const R2I &b) {
  REQUIRE(a.Intersects(b));
  REQUIRE(b.Intersects(a));
}

void require_commutative_nonintersection(const R2I &a, const R2I &b) {
  REQUIRE(!a.Intersects(b));
  REQUIRE(!b.Intersects(a));
}

TEST_CASE("intersecting squares, coincident") {
  constexpr R2I a{{0, 0}, {1, 1}};
  constexpr R2I b{{0, 0}, {1, 1}};
  require_commutative_intersection(a, b);
}

TEST_CASE("intersecting squares, mutual corner overlap") {
  constexpr R2I a{{0, 0}, {2, 2}};
  constexpr R2I b{{1, 1}, {2, 2}};
  require_commutative_intersection(a, b);
}

TEST_CASE("intersecting squares, corners touching") {
  constexpr R2I a{{0, 0}, {1, 1}};
  constexpr R2I b{{1, 1}, {1, 1}};
  require_commutative_intersection(a, b);
}

TEST_CASE("intersecting squares, sides touching") {
  constexpr R2I a{{0, 0}, {1, 1}};
  constexpr R2I b{{1, 0}, {1, 1}};
  require_commutative_intersection(a, b);
}

TEST_CASE("intersecting squares, full containment") {
  constexpr R2I a{{0, 0}, {3, 3}};
  constexpr R2I b{{1, 1}, {1, 1}};
  require_commutative_intersection(a, b);
}

TEST_CASE("intersecting rectangles, mutual center overlap") {
  constexpr R2I a{{1, 0}, {1, 3}};
  constexpr R2I b{{0, 1}, {3, 1}};
  require_commutative_intersection(a, b);
}

TEST_CASE("intersecting rectangles, 2 corners contained") {
  constexpr R2I a{{0, 0}, {2, 3}};
  constexpr R2I b{{1, 1}, {2, 1}};
  require_commutative_intersection(a, b);
}

TEST_CASE("non-intersecting squares") {
  constexpr R2I a{{0, 0}, {1, 1}};
  constexpr R2I b{{2, 2}, {1, 1}};
  require_commutative_nonintersection(a, b);
}

struct C2I : C2<C2I, V2I, R2I> {
  constexpr C2I(const V2I center, const int radius) : C2(center, radius){};
};

void require_commutative_intersection(const C2I &a, const C2I &b) {
  REQUIRE(a.Intersects(b));
  REQUIRE(b.Intersects(a));
}

void require_commutative_nonintersection(const C2I &a, const C2I &b) {
  REQUIRE(!a.Intersects(b));
  REQUIRE(!b.Intersects(a));
}

TEST_CASE("intersecting circles, coincident") {
  constexpr C2I a{{0, 0}, 1};
  constexpr C2I b{{0, 0}, 1};
  require_commutative_intersection(a, b);
}

TEST_CASE("intersecting circles, overlapping") {
  constexpr C2I a{{0, 0}, 1};
  constexpr C2I b{{1, 0}, 1};
  require_commutative_intersection(a, b);
}

TEST_CASE("intersecting circles, touching") {
  constexpr C2I a{{0, 0}, 1};
  constexpr C2I b{{2, 0}, 1};
  require_commutative_intersection(a, b);
}

TEST_CASE("non-intersecting circles, horizontal gap") {
  constexpr C2I a{{0, 0}, 1};
  constexpr C2I b{{3, 0}, 1};
  require_commutative_nonintersection(a, b);
}

TEST_CASE("non-intersecting circles, diagonal gap") {
  constexpr C2I a{{0, 0}, 1};
  constexpr C2I b{{2, 2}, 1};
  require_commutative_nonintersection(a, b);
}

}  // namespace Breeze
