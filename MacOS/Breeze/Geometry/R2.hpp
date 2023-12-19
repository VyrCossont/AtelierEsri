#pragma once

namespace Breeze {
/// Rectangle mixin.
template <typename Base, typename Vector>
struct R2 {
  using Element = typename Vector::Element;

  Vector origin;
  Vector size;

  constexpr R2(const Vector origin, const Vector size)
      : origin(origin), size(size) {}

  R2(const Element left,
     const Element top,
     const Element right,
     const Element bottom)
      : origin(left, top), size(right - left, bottom - top) {}

  [[nodiscard]] Element Left() const { return origin.x; }
  [[nodiscard]] Element Right() const { return origin.x + size.x; }
  [[nodiscard]] Element Top() const { return origin.y; }
  [[nodiscard]] Element Bottom() const { return origin.y + size.y; }
  [[nodiscard]] Element Width() const { return size.x; }
  [[nodiscard]] Element Height() const { return size.y; }

  [[nodiscard]] bool Contains(const Vector point) const {
    return point.x >= Left() && point.y >= Top() && point.x <= Right() &&
           point.y <= Bottom();
  }

  bool operator==(const Base& rhs) const {
    return size == rhs.size && origin == rhs.origin;
  }

  bool operator!=(const Base& rhs) const {
    return size != rhs.size || origin != rhs.origin;
  }
};

}  // namespace Breeze
