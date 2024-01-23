#include "Exception.hpp"

#include "Strings.hpp"

namespace AtelierEsri {

Exception::Exception(
    const char *message,
    const OSErr osErr,
    const char *fileName,
    const uint32_t line,
    const char *func
)
    : message(message),
      osErr(osErr),
      fileName(fileName),
      line(line),
      func(func) {}

std::string Exception::Explanation() const {
  if (osErr) {
    return Strings::FormatShort("%s (OSErr %d)", message, osErr);
  }
  return {message};
}

std::string Exception::Location() const {
  return Strings::FormatShort("%s:%lu (%s)", fileName, line, func);
}

}  // namespace AtelierEsri
