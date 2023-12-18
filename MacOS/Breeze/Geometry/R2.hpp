#pragma once

namespace Breeze {
/// Rectangle mixin.
template <typename Vector>
struct R2 {
  using Element = typename Vector::Element;

  Vector origin;
  Vector size;

  R2(const Vector origin, const Vector size) : origin(origin), size(size) {}

  R2(const Element left,
     const Element top,
     const Element right,
     const Element bottom)
      : origin(left, top), size(right - left, bottom - top) {}

  Element Left() const { return origin.x; }
  Element Right() const { return origin.x + size.x; }
  Element Top() const { return origin.y; }
  Element Bottom() const { return origin.y + size.y; }

  bool Contains(Vector point) const {
    return point.x >= origin.x && point.y >= origin.y &&
           point.x <= origin.x + size.x && point.y <= origin.y + size.y;
  }
};

}  // namespace Breeze
