#pragma once

#include <MacTypes.h>

#include <string>

namespace AtelierEsri {

class Strings {
 public:
  /// Use `printf` formatting to build a Pascal-length string.
  /// If there's a format error, return an error string.
  static std::string FormatShort(const char *fmt, ...);

  /// Convert the string to a Pascal string.
  /// If it's too long, it gets truncated.
  static void ToPascal(const std::string &str, Str255 &pStr);

  /// Create a C++ string from a Pascal string.
  static std::string FromPascal(const Str255 &pStr);

  /// Read a Pascal string from bytes.
  /// Returns number of bytes read.
  static size_t ReadPascal(const uint8_t *data, std::string &out);
};

}  // namespace AtelierEsri
