#pragma once

#include <cstddef>
#include <stdexcept>
#include <string>

#include "BinIO.hpp"

namespace Breeze {

template <typename Endianness>
class MemReader {
 public:
  MemReader(const char* data, const size_t len) : data(data), len(len) {}

  template <typename Readable>
  std::enable_if_t<std::is_aggregate_v<Readable>, void> read(Readable& value) {
    value = Readable::read(*this);
  }

  template <typename Value, typename E = Endianness>
  std::enable_if_t<std::is_same_v<E, BE>, void> read(Value& value) {
#if __BYTE_ORDER__ == __ORDER_BIG_ENDIAN__
    read_native(value);
#else
    throw std::runtime_error("Endianness not supported");
#endif
  }

  template <typename Value, typename E = Endianness>
  std::enable_if_t<std::is_same_v<E, LE>, void> read(Value& value) {
#if __BYTE_ORDER__ == __ORDER_LITTLE_ENDIAN__
    read_native(value);
#else
    throw std::runtime_error("Endianness not supported");
#endif
  }

  /// Read a primitive type in the target's native byte order.
  template <typename Value>
  void read_native(Value& value) {
    if (reinterpret_cast<uintptr_t>(data) % alignof(Value)) {
      throw std::runtime_error("Unaligned access");
    }
    if (sizeof(Value) > len) {
      throw std::runtime_error("Not enough data");
    }

    value = *reinterpret_cast<Value*>(const_cast<char*>(data));
    data += sizeof(Value);
    len -= sizeof(Value);
  }

  /// Advance the cursor to the next position that's aligned for this type.
  template <typename Value>
  void align() {
    if (const size_t advance = alignof(Value) - reinterpret_cast<uintptr_t>(data
                                                ) % alignof(Value)) {
      if (advance > len) {
        throw std::runtime_error("Not enough data");
      }
      data += advance;
      len -= advance;
    }
  }

  void read_pstr(std::string& value) {
    std::uint8_t count;
    read(count);
    if (count > len) {
      throw std::runtime_error("Not enough data");
    }

    value.assign(data, count);
    data += count;
    len -= count;
  }

  // TODO: figure out a way to not duplicate this
  template <typename Count, typename Element>
  void read_vec(std::vector<Element>& values) {
    Count count;
    read(count);
    if (count * sizeof(Element) > len) {
      throw std::runtime_error("Not enough data");
    }

    values.clear();
    values.reserve(count);
    for (Count i = 0; i < count; ++i) {
      Element element;
      read(element);
      values.push_back(element);
    }
  }

 private:
  const char* data;
  size_t len;
};

}  // namespace Breeze
