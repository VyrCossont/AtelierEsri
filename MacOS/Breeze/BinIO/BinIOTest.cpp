#include <catch2/catch_test_macros.hpp>
#include <sstream>

#include "MemReader.hpp"
#include "StreamReader.hpp"

namespace Breeze {

struct NamedPoint {
  std::string name;
  int16_t x{};
  int16_t y{};
  std::vector<uint8_t> colors;

  template <typename Reader>
  static NamedPoint read(Reader& reader) {
    NamedPoint value;

    reader.read_pstr(value.name);
    reader.template align<int16_t>();
    reader.read(value.x);
    reader.read(value.y);
    reader.template read_vec<uint8_t>(value.colors);

    return value;
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
    0x03,
    0x0a,
    0x0b,
    0x0c,
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
    0x03,
    0x0a,
    0x0b,
    0x0c,
};
#endif

TEST_CASE("memory reader, native order") {
#if __BYTE_ORDER__ == __ORDER_BIG_ENDIAN__
  MemReader<BE> reader(named_point_test_data, sizeof named_point_test_data);
#elif __BYTE_ORDER__ == __ORDER_LITTLE_ENDIAN__
  MemReader<LE> reader(named_point_test_data, sizeof named_point_test_data);
#else
  FAIL("Where did you find a PDP to run this on?");
#endif

  auto [name, x, y, colors] = NamedPoint::read(reader);
  REQUIRE(x == 16);
  REQUIRE(y == 24);
  REQUIRE(name == "name");
  REQUIRE(colors.size() == 3);
  REQUIRE(colors[0] == 10);
  REQUIRE(colors[1] == 11);
  REQUIRE(colors[2] == 12);
}

TEST_CASE("stream reader, native order") {
  std::stringstream stream(
      std::string(named_point_test_data, sizeof named_point_test_data)
  );
#if __BYTE_ORDER__ == __ORDER_BIG_ENDIAN__
  StreamReader<BE> reader(stream, sizeof named_point_test_data);
#elif __BYTE_ORDER__ == __ORDER_LITTLE_ENDIAN__
  StreamReader<LE> reader(stream, sizeof named_point_test_data);
#else
  FAIL("Where did you find a PDP to run this on?");
#endif

  auto [name, x, y, colors] = NamedPoint::read(reader);
  REQUIRE(x == 16);
  REQUIRE(y == 24);
  REQUIRE(name == "name");
  REQUIRE(colors.size() == 3);
  REQUIRE(colors[0] == 10);
  REQUIRE(colors[1] == 11);
  REQUIRE(colors[2] == 12);
}

}  // namespace Breeze
