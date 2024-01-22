#pragma once

#include <cstddef>
#include <stdexcept>
#include <string>

namespace Breeze {

class MemReader {
 public:
  MemReader(const char* data, const std::size_t len) : data(data), len(len) {}

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
    if (reinterpret_cast<std::uintptr_t>(data) % alignof(T)) {
      throw std::runtime_error("Unaligned access");
    }
    if (sizeof(T) > len) {
      throw std::runtime_error("Not enough data");
    }

    value = *reinterpret_cast<T*>(const_cast<char*>(data));
    data += sizeof(T);
    len -= sizeof(T);
  }

  template <typename T>
  void align() {
    if (const size_t advance =
            alignof(T) - reinterpret_cast<std::uintptr_t>(data) % alignof(T)) {
      if (advance > len) {
        throw std::runtime_error("Not enough data");
      }
      data += advance;
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

    value.assign(data, count);
    data += count;
    len -= count;
  }

 private:
  const char* data;
  std::size_t len;
};

}  // namespace Breeze
