#pragma once

#include <cstdint>
#include <string>

#include <MacTypes.h>

namespace AtelierEsri {

class Exception : std::exception {
public:
  /// `osErr` may be 0/`noErr` for non-OS errors.
  /// FUTURE: If we need other error domains, we can make this a variant.
  Exception(const char *message, OSErr osErr, const char *file, uint32_t line,
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

/// Construct an exception with a message and source location.
#define EXCEPTION(message)                                                     \
  ::AtelierEsri::Exception((message), noErr, __FILE__, __LINE__, __func__)

/// Construct an exception with a message, OS error code, and source location.
#define OS_EXCEPTION(message, osErr)                                           \
  ::AtelierEsri::Exception((message), (osErr), __FILE__, __LINE__, __func__)

#pragma endregion

#pragma region Flow control macros

/// Flow control statement:
/// Return an error result with a message and source location.
#define BAIL(message) throw ::AtelierEsri::Exception(EXCEPTION((message)))

/// Flow control statement:
/// If a pointer type is null,
/// return an error result with a message and source location.
#define REQUIRE_NOT_NULL(valueName)                                            \
  do {                                                                         \
    if (valueName == nullptr) {                                                \
      BAIL("Requirement failed: " #valueName " is null");                      \
    }                                                                          \
  } while (false)

/// Flow control statement:
/// Return an error result with a message, OS error code, and source location.
#define OS_BAIL(message, osErr) throw OS_EXCEPTION((message), (osErr))

/// Flow control statement:
/// Evaluate an expression that returns an OS error code.
/// If there is no error, pass the result through.
/// Otherwise, return an error result with a message, OS error code, and source
/// location.
#define OS_CHECKED(expr, message)                                              \
  do {                                                                         \
    OSErr osErr = (expr);                                                      \
    if (osErr) {                                                               \
      OS_BAIL((message), osErr);                                               \
    }                                                                          \
  } while (false)

/// Flow control statement:
/// Evaluate an expression that can set an OS error code returned out of band.
/// If there is no error, pass the result through.
/// Otherwise, return an error result with a message, OS error code, and source
/// location.
#define OOB_CHECKED(expr, message, errorExpr)                                  \
  (expr);                                                                      \
  do {                                                                         \
    OSErr osErr = (errorExpr);                                                 \
    if (osErr) {                                                               \
      OS_BAIL((message), osErr);                                               \
    }                                                                          \
  } while (false)

// TODO: (Vyr) OOB_CHECKED operations should know the error return value of the
//  function they're calling: -1, nil, etc., and only call `errorExpr` then.

/// Checked Color QuickDraw operation.
/// https://preterhuman.net/macstuff/insidemac/QuickDraw/QuickDraw-255.html
/// Note that Basic QuickDraw machines will never return an error code this way.
#define QD_CHECKED(expr, message) OOB_CHECKED((expr), (message), QDError())

/// Checked Resource Manager operation.
/// https://preterhuman.net/macstuff/insidemac/MoreToolbox/MoreToolbox-35.html
#define RES_CHECKED(expr, message) OOB_CHECKED((expr), (message), ResError())

#pragma endregion
