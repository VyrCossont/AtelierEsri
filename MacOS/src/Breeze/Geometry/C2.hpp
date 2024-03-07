#pragma once

namespace Breeze {

/// Circle mixin.
template <typename Base, typename Vector, typename Rect>
struct C2 {
  using Element = typename Vector::Element;

  Vector center;
  Element radius;

  constexpr C2(const Vector center, const Element radius)
      : center(center), radius(radius) {}

  Rect Bounds() const { return Rect::Around(center, radius); }

  [[nodiscard]] bool Empty() const { return radius == 0; }

  [[nodiscard]] bool Contains(const Vector& point) const {
    // Float implementations should use std::hypot to avoid overflow:
    // https://walkingrandomly.com/?p=6633
    // https://www.johndcook.com/blog/2010/06/02/whats-so-hard-about-finding-a-hypotenuse/
    // Integer implementations should use larger temporary variables.
    const Vector d = point - center;
    const Element radius_squared = radius * radius;
    const Element hypot_squared = d.x * d.x + d.y * d.y;
    return hypot_squared <= radius_squared;
  }

  [[nodiscard]] bool Intersects(const Base& other) const {
    const Element sum_of_radii = radius + other.radius;
    const Element sum_of_radii_squared = sum_of_radii * sum_of_radii;
    const Element dx = center.x - other.center.x;
    const Element dy = center.y - other.center.y;
    const Element d_squared = dx * dx + dy * dy;
    return d_squared <= sum_of_radii_squared;
  }

  [[nodiscard]] bool Intersects(const Rect& other) const {
    // TODO: a whole bunch of cases
    return Bounds().Intersects(other);
  }

  friend std::ostream& operator<<(std::ostream& os, const Base& rhs) {
    os << std::string("{") << rhs.origin << ", " << rhs.size << "}";
    return os;
  }
};

}  // namespace Breeze
