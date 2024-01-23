#include "Strings.hpp"

#include <cstdarg>
#include <cstdio>
#include <cstring>

namespace AtelierEsri {

std::string Strings::FormatShort(const char *fmt, ...) {
  std::va_list args;
  va_start(args, fmt);
  char buffer[256];
  if (const int len = vsnprintf(buffer, sizeof buffer, fmt, args); len < 0) {
    return {"AtelierEsri::Strings::FormatShort: vsnprintf failed!"};
  }
  va_end(args);
  // ReSharper disable once CppDFALocalValueEscapesFunction
  return {buffer};
}

void Strings::ToPascal(const std::string &str, Str255 &pStr) {
  const unsigned char len =
      std::min(str.size(), static_cast<std::string::size_type>(255));
  pStr[0] = len;
  memcpy(&pStr[1], str.data(), len);
}

std::string Strings::FromPascal(const Str255 &pStr) {
  const size_t len = pStr[0];
  return {reinterpret_cast<const char *>(&pStr[1]), len};
}

size_t Strings::ReadPascal(const uint8_t *data, std::string &out) {
  const size_t len = *data++;
  out.assign(reinterpret_cast<const char *>(data), len);
  return 1 + len;
}

} // namespace AtelierEsri
