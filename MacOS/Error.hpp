#pragma once

#include <cstdint>
#include <string>

#include <MacTypes.h>

namespace AtelierEsri {

class Error {
public:
  /// `osErr` may be 0/`noErr` for non-OS errors.
  /// FUTURE: If we need other error domains, we can make this a variant.
  Error(const char *message, OSErr osErr, const char *file, uint32_t line,
        const char *function);

  [[nodiscard]] std::string Explanation() const;
  [[nodiscard]] std::string Location() const;

  const OSErr osErr;

private:
  const char *message;
  const char *file;
  const char *function;
  uint32_t line;
};

} // namespace AtelierEsri

#pragma region Constructor macros

/// Construct an error with a message and source location.
#define ERROR(message)                                                         \
  ::AtelierEsri::Error((message), noErr, __FILE__, __LINE__, __func__)

/// Construct an error with a message, OS error code, and source location.
#define OS_ERROR(message, osErr)                                               \
  ::AtelierEsri::Error((message), (osErr), __FILE__, __LINE__, __func__)

#pragma endregion
