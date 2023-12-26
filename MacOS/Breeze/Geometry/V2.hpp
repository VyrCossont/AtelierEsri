#pragma once

namespace Breeze {

/// 2D vector mixin.
template <typename Base, typename Element>
struct V2 {
  Element x;
  Element y;

  constexpr V2(Element x, Element y) : x(x), y(y) {}

  bool operator==(const Base& rhs) const { return x == rhs.x && y == rhs.y; }

  bool operator!=(const Base& rhs) const { return x != rhs.x || y != rhs.y; }

  Base operator+() const { return {+x, +y}; }

  Base operator-() const { return {-x, -y}; }

  Base operator+(const Base& rhs) const { return {x + rhs.x, y + rhs.y}; }

  Base& operator+=(const Base& rhs) {
    x += rhs.x;
    y += rhs.y;
    return *static_cast<Base*>(this);
  }

  Base operator-(const Base& rhs) const { return {x - rhs.x, y - rhs.y}; }

  Base& operator-=(const Base& rhs) {
    x -= rhs.x;
    y -= rhs.y;
    return *static_cast<Base*>(this);
  }

  Base operator*(const Base& rhs) const { return {x * rhs.x, y * rhs.y}; }

  Base operator*(const Element rhs) const { return {x * rhs, y * rhs}; }

  Base& operator*=(const Base& rhs) {
    x *= rhs.x;
    y *= rhs.y;
    return *static_cast<Base*>(this);
  }

  Base& operator*=(const Element rhs) {
    x *= rhs;
    y *= rhs;
    return *static_cast<Base*>(this);
  }

  Base operator/(const Base& rhs) const { return {x / rhs.x, y / rhs.y}; }

  Base operator/(const Element rhs) const { return {x / rhs, y / rhs}; }

  Base& operator/=(const Base& rhs) {
    x /= rhs.x;
    y /= rhs.y;
    return *static_cast<Base*>(this);
  }

  Base& operator/=(const Element rhs) {
    x /= rhs;
    y /= rhs;
    return *static_cast<Base*>(this);
  }

  Base operator%(const Base& rhs) const { return {x % rhs.x, y % rhs.y}; }

  Base operator%(const Element rhs) const { return {x % rhs, y % rhs}; }

  Base& operator%=(const Base& rhs) {
    x %= rhs.x;
    y %= rhs.y;
    return *static_cast<Base*>(this);
  }

  Base& operator%=(const Element rhs) {
    x %= rhs;
    y %= rhs;
    return *static_cast<Base*>(this);
  }

  friend Base operator*(const Element lhs, const Base& rhs) {
    return {lhs * rhs.x, lhs * rhs.y};
  }

  friend Base operator/(const Element lhs, const Base& rhs) {
    return {lhs / rhs.x, lhs / rhs.y};
  }

  friend Base operator%(const Element lhs, const Base& rhs) {
    return {lhs % rhs.x, lhs % rhs.y};
  }

  friend std::ostream& operator<<(std::ostream& os, const Base& rhs) {
    os << std::string("{") << rhs.x << ", " << rhs.y << "}";
    return os;
  }
};

}  // namespace Breeze
