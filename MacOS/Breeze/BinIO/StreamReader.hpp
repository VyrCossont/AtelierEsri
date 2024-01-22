#pragma once

#include <cstddef>
#include <istream>
#include <stdexcept>
#include <string>

namespace Breeze {

class StreamReader {
 public:
  StreamReader(std::istream& stream, const std::size_t len)
      : stream(stream), len(len) {}

  template <typename T>
  void read_be(T& value) {
#if __BYTE_ORDER__ != __ORDER_BIG_ENDIAN__
    throw std::runtime_error("Endianness not supported");
#endif

    read_native(value);
  }

  template <typename T>
  void read_le(T& value) {
#if __BYTE_ORDER__ != __ORDER_LITTLE_ENDIAN__
    throw std::runtime_error("Endianness not supported");
#endif

    read_native(value);
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
    // This is safe only because we're reading a short Pascal string (\p).
    // Long Pascal strings (\P) would need an endianness.
    read_native(count);
    if (count > len) {
      throw std::runtime_error("Not enough data");
    }

    char data[count];
    stream.read(data, count);
    len -= count;
    value.assign(data, count);
  }

 private:
  std::istream& stream;
  std::size_t len;
};

}  // namespace Breeze
