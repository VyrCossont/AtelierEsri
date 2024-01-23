#pragma once

#include <cstddef>
#include <istream>
#include <stdexcept>
#include <string>

#include "BinIO.hpp"

namespace Breeze {

template <typename Endianness>
class StreamReader {
 public:
  StreamReader(std::istream& stream, const std::size_t len)
      : stream(stream), len(len) {}

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

  template <typename T>
  void read_native(T& value) {
    if (sizeof(T) > len) {
      throw std::runtime_error("Not enough data");
    }
    stream.read(reinterpret_cast<char*>(&value), sizeof(T));
  }

  template <typename T>
  void align() {
    if (const std::size_t advance = alignof(T) - stream.tellg() % alignof(T)) {
      if (advance > len) {
        throw std::runtime_error("Not enough data");
      }
      stream.seekg(advance, std::ios_base::cur);
      len -= advance;
    }
  }

  void read_pstr(std::string& value) {
    std::uint8_t count;
    read(count);
    if (count > len) {
      throw std::runtime_error("Not enough data");
    }

    char data[count];
    stream.read(data, count);
    len -= count;
    value.assign(data, count);
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
  std::istream& stream;
  std::size_t len;
};

}  // namespace Breeze
