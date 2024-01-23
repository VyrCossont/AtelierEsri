#pragma once

#include <MacTypes.h>

#include <cstddef>
#include <cstdint>
#include <sstream>

// ReSharper disable once CppUnusedIncludeDirective
#include "Breeze/Strings.hpp"

namespace AtelierEsri {
class Debug {
 public:
  static OSErr Printfln(
      const char *fileName,
      uint32_t line,
      const char *func,
      const char *fmt,
      ...
  );

 private:
  /// Append to a log file in the same directory.
  static OSErr FilePrint(size_t num_bytes, const char *buffer);
  /// Log to the modem port at 9600 8-N-1.
  static OSErr SerialPrint(size_t num_bytes, const char *buffer);
};

#define DEBUG_LOG(fmt, ...)                  \
  ::AtelierEsri::Debug::Printfln(            \
      ::Breeze::Strings::FileName(__FILE__), \
      __LINE__,                              \
      __func__,                              \
      (fmt),                                 \
      __VA_ARGS__                            \
  )

#define DEBUG_INSPECT(expr)                     \
  do {                                          \
    ::std::stringstream ss;                     \
    ss << ::std::string(#expr " = ") << (expr); \
    DEBUG_LOG("%s", ss.str().c_str());          \
  } while (false)

}  // namespace AtelierEsri
