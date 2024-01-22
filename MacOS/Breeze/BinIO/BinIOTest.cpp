#include <catch2/catch_test_macros.hpp>
#include <sstream>

#include "MemReader.hpp"
#include "StreamReader.hpp"

namespace Breeze {

struct NamedPoint {
  std::string name;
  std::int16_t x{};
  std::int16_t y{};

  template <typename Reader>
  static void read_be(Reader& reader, NamedPoint& value) {
    reader.read_pstr(value.name);
    reader.template align<std::int16_t>();
    reader.read_be(value.x);
    reader.read_be(value.y);
  }

  template <typename Reader>
  static void read_le(Reader& reader, NamedPoint& value) {
    reader.read_pstr(value.name);
    reader.template align<std::int16_t>();
    reader.read_le(value.x);
    reader.read_le(value.y);
  }
};

#if __BYTE_ORDER__ == __ORDER_BIG_ENDIAN__
constexpr char named_point_test_data[] = {
    0x04,
    'n',
    'a',
    'm',
    'e',
    0x00,
    0x00,
    0x10,
    0x00,
    0x18,
};
#elif __BYTE_ORDER__ == __ORDER_LITTLE_ENDIAN__
constexpr char named_point_test_data[] = {
    0x04,
    'n',
    'a',
    'm',
    'e',
    0x00,
    0x10,
    0x00,
    0x18,
    0x00,
};
#endif

TEST_CASE("memory reader, native order") {
  MemReader reader(named_point_test_data, sizeof named_point_test_data);
  NamedPoint named_point;

#if __BYTE_ORDER__ == __ORDER_BIG_ENDIAN__
  NamedPoint::read_be(reader, value);
#elif __BYTE_ORDER__ == __ORDER_LITTLE_ENDIAN__
  NamedPoint::read_le(reader, named_point);
#else
  FAIL("Where did you find a PDP to run this on?");
#endif

  REQUIRE(named_point.x == 16);
  REQUIRE(named_point.y == 24);
  REQUIRE(named_point.name == "name");
}

TEST_CASE("stream reader, native order") {
  std::stringstream stream(
      std::string(named_point_test_data, sizeof named_point_test_data)
  );
  StreamReader reader(stream, sizeof named_point_test_data);
  NamedPoint named_point;

#if __BYTE_ORDER__ == __ORDER_BIG_ENDIAN__
  NamedPoint::read_be(reader, value);
#elif __BYTE_ORDER__ == __ORDER_LITTLE_ENDIAN__
  NamedPoint::read_le(reader, named_point);
#else
  FAIL("Where did you find a PDP to run this on?");
#endif

  REQUIRE(named_point.x == 16);
  REQUIRE(named_point.y == 24);
  REQUIRE(named_point.name == "name");
}

}  // namespace Breeze
