#pragma once

namespace Breeze {
/// Rectangle mixin.
template <typename Base, typename Vector>
struct R2 {
  using Element = typename Vector::Element;

  Vector origin;
  Vector size;

  constexpr R2(const Vector origin, const Vector size)
      : origin(origin), size(size) {
    if (size.x < 0 || size.y < 0) {
      throw std::invalid_argument("Size must be positive");
    }
  }

  R2(const Element left,
     const Element top,
     const Element right,
     const Element bottom)
      : R2({left, top}, {right - left, bottom - top}) {}

  /// Generate square of a given size around a point.
  [[nodiscard]] static Base Around(
      const Vector center, const Element halfWidth
  ) {
    return Around(center, {halfWidth, halfWidth});
  }

  /// Generate rectangle of given dimensions around a point.
  [[nodiscard]] static Base Around(const Vector center, const Vector halfSize) {
    return {center - halfSize, 2 * halfSize};
  }

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

  [[nodiscard]] bool Contains(const Base rect) const {
    return rect.Left() >= Left() && rect.Top() >= Top() &&
           rect.Right() <= Right() && rect.Bottom() <= Bottom();
  }

  [[nodiscard]] bool Empty() const { return size.x == 0 || size.y == 0; }

  bool operator==(const Base& rhs) const {
    return size == rhs.size && origin == rhs.origin;
  }

  bool operator!=(const Base& rhs) const {
    return size != rhs.size || origin != rhs.origin;
  }

  /// Union: return rectangle covering both rectangles.
  Base operator|(const Base& rhs) const {
    auto [minLeft, minTop, maxRight, maxBottom] = UnionCore(rhs);
    return {minLeft, minTop, maxRight, maxBottom};
  }

  /// Union: extend this rectangle to cover other rectangle.
  void operator|=(const Base& rhs) {
    auto [minLeft, minTop, maxRight, maxBottom] = UnionCore(rhs);
    origin.x = minLeft;
    origin.y = minTop;
    size.x = maxRight - minLeft;
    size.y = maxBottom - minTop;
  }

  friend std::ostream& operator<<(std::ostream& os, const Base& rhs) {
    os << std::string("{") << rhs.origin << ", " << rhs.size << "}";
    return os;
  }

 private:
  struct UnionCoreResult {
    Element minLeft, minTop, maxRight, maxBottom;
  };

  /// Shared by union operator implementations.
  UnionCoreResult UnionCore(const Base& rhs) {
    return {
        std::min(Left(), rhs.Left()),
        std::min(Top(), rhs.Top()),
        std::max(Right(), rhs.Right()),
        std::max(Bottom(), rhs.Bottom()),
    };
  }
};

}  // namespace Breeze
