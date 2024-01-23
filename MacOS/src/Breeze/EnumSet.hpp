#pragma once

#include <bitset>

namespace Breeze {

// Code from https://aantron.github.io/better-enums/demo/BitSets.html
// believed to be under the BSD license like the rest of the library.

template <typename Enum>
constexpr Enum max_loop(Enum accumulator, size_t index) {
  return index >= Enum::_size() ? accumulator
         : Enum::_values()[index] > accumulator
             ? max_loop<Enum>(Enum::_values()[index], index + 1)
             : max_loop<Enum>(accumulator, index + 1);
}

template <typename Enum>
constexpr Enum max() {
  return max_loop<Enum>(Enum::_values()[0], 1);
}

template <typename Enum>
using EnumSet = std::bitset<max<Enum>()._to_integral() + 1>;

// Code below specific to this project.

template <typename Enum>
class BitsetFormatter {
 public:
  explicit BitsetFormatter(const EnumSet<Enum>& enumSet) : enumSet(enumSet) {}

  friend std::ostream& operator<<(
      std::ostream& os, const BitsetFormatter& rhs
  ) {
    os << std::string("{");

    bool first = true;
    for (auto value : Enum::_values()) {
      if (!rhs.enumSet.test(value)) {
        continue;
      }

      if (first) {
        first = false;
      } else {
        os << std::string(", ");
      }
      os << value._to_string();
    }

    os << std::string("}");
    return os;
  }

 private:
  const EnumSet<Enum>& enumSet;
};

}  // namespace Breeze
