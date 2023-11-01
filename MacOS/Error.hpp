#pragma once

#include <cstdint>
#include <string>

#include <MacTypes.h>

#include "Result.hpp"

namespace AtelierEsri {

class Error {
public:
  /// `osErr` may be 0/`noErr` for non-OS errors.
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

#define ERROR(message)                                                         \
  ::AtelierEsri::Error((message), noErr, __FILE__, __LINE__, __func__)

#define OS_ERROR(message, osErr)                                               \
  ::AtelierEsri::Exception((message), (osErr), __FILE__, __LINE__, __func__)

#define BAIL(message) return ::AtelierEsri::Err(ERROR(message))

#define OS_BAIL(message, osErr)                                                \
  return ::AtelierEsri::Err(OS_ERROR(message, osErr))

#define OS_CHECKED(expr, message)                                              \
  {                                                                            \
    OSErr osErr = (expr);                                                      \
    if (osErr) {                                                               \
      OS_BAIL(message, osErr);                                                 \
    }                                                                          \
  }

} // namespace AtelierEsri
