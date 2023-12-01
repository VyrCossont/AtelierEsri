//! Code from https://aantron.github.io/better-enums/demo/BitSets.html
//! believed to be under the BSD license like the rest of the library.

#pragma once

#include <bitset>

#include <better-enums/enum.h>

namespace Breeze {

template <typename Enum>
constexpr Enum max_loop(Enum accumulator, size_t index) {
  return index >= Enum::_size() ? accumulator
         : Enum::_values()[index] > accumulator
             ? max_loop<Enum>(Enum::_values()[index], index + 1)
             : max_loop<Enum>(accumulator, index + 1);
}

template <typename Enum> constexpr Enum max() {
  return max_loop<Enum>(Enum::_values()[0], 1);
}

template <typename Enum>
using EnumSet = std::bitset<max<Enum>()._to_integral() + 1>;

} // namespace Breeze
