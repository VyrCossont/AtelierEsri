#include "Strings.hpp"

#include <cstdarg>
#include <cstdio>
#include <cstring>

namespace AtelierEsri {

std::string Strings::FormatShort(const char *fmt, ...) {
  std::va_list args;
  va_start(args, fmt);
  char buffer[256];
  int len = vsnprintf(buffer, sizeof buffer, fmt, args);
  if (len < 0) {
    return {"AtelierEsri::Strings::FormatShort: vsnprintf failed!"};
  }
  va_end(args);
  return {buffer};
}

void Strings::ToPascal(const std::string &str, Str255 &pStr) {
  unsigned char len =
      std::min(str.size(), static_cast<std::string::size_type>(255));
  pStr[0] = len;
  memcpy(&pStr[1], str.data(), len);
}

} // namespace AtelierEsri
