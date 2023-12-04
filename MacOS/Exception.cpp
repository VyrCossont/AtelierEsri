#include "Exception.hpp"

#include "Strings.hpp"

namespace AtelierEsri {

Exception::Exception(const char *message, const OSErr osErr, const char *file,
                     const uint32_t line, const char *function)
    : message(message), osErr(osErr), file(file), line(line),
      function(function) {}

std::string Exception::Explanation() const {
  if (osErr) {
    return Strings::FormatShort("%s (OSErr %d)", message, osErr);
  } else {
    return {message};
  }
}

#ifdef _WIN32
static const char separator = '\\';
#else
static const char separator = '/';
#endif

std::string Exception::Location() const {
  char const *filename = file;
  if (file) {
    char const *c = file;
    while (*c) {
      // Take the last part of the path, assuming host system separators.
      // TODO: (Vyr) Can we extract the filename at compile time?
      if (*c == separator && *(c + 1)) {
        filename = c + 1;
      }
      c++;
    }
  }

  return Strings::FormatShort("%s:%lu (%s)", filename, line, function);
}

} // namespace AtelierEsri
