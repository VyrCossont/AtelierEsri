#pragma once

#include <string>

#include <MacTypes.h>

namespace AtelierEsri {

class Strings {
public:
  /// Use `printf` formatting to build a Pascal-length string.
  /// If there's a format error, return an error string.
  static std::string FormatShort(const char *fmt, ...);

  /// Convert the string to a Pascal string.
  /// If it's too long, it gets truncated.
  static void ToPascal(const std::string &str, Str255 &pstr);
};

} // namespace AtelierEsri
