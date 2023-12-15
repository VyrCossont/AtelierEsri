#pragma once

namespace Breeze {

/// 2D vector mixin.
template <typename Base, typename Element>
struct V2 {
  Element x;
  Element y;

  V2(Element x, Element y) : x(x), y(y) {}

  Base operator+(const Base& rhs) const { return {x + rhs.x, y + rhs.y}; }

  Base operator+=(const Base& rhs) {
    x += rhs.x;
    y += rhs.y;
    return *this;
  }

  Base operator-(const Base& rhs) const { return {x - rhs.x, y - rhs.y}; }

  Base operator-=(const Base& rhs) {
    x -= rhs.x;
    y -= rhs.y;
    return *this;
  }

  Base operator*(const Base& rhs) const { return {x * rhs.x, y * rhs.y}; }

  Base operator*(const Element rhs) const { return {x * rhs, y * rhs}; }

  Base operator*=(const Base& rhs) {
    x *= rhs.x;
    y *= rhs.y;
    return *this;
  }

  Base operator*=(const Element rhs) {
    x *= rhs;
    y *= rhs;
    return *this;
  }

  Base operator/(const Base& rhs) const { return {x / rhs.x, y / rhs.y}; }

  Base operator/(const Element rhs) const { return {x / rhs, y / rhs}; }

  Base operator/=(const Base& rhs) {
    x /= rhs.x;
    y /= rhs.y;
    return *this;
  }

  Base operator/=(const Element rhs) {
    x /= rhs;
    y /= rhs;
    return *this;
  }

  Base operator%(const Base& rhs) const { return {x % rhs.x, y % rhs.y}; }

  Base operator%(const Element rhs) const { return {x % rhs, y % rhs}; }

  Base operator%=(const Base& rhs) {
    x %= rhs.x;
    y %= rhs.y;
    return *this;
  }

  Base operator%=(const Element rhs) {
    x %= rhs;
    y %= rhs;
    return *this;
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
};

}  // namespace Breeze
