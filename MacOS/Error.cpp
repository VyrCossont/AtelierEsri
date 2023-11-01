#include "Error.hpp"

#include "Strings.hpp"

namespace AtelierEsri {

Error::Error(const char *message, OSErr osErr, const char *file, uint32_t line,
             const char *function)
    : message(message), osErr(osErr), file(file), line(line),
      function(function) {}

std::string Error::Explanation() const {
  if (osErr) {
    return Strings::FormatShort("%s (OSErr %d)", message, osErr);
  } else {
    return {message};
  }
}

std::string Error::Location() const {
  char const *filename = file;
  if (file) {
    char const *c = file;
    while (*c) {
      // Take the last part of the path, assuming host system separators.
      // TODO: (Vyr) Can we do this at compile time?
#ifdef _WIN32
      const char separator = '\\';
#else
      const char separator = '/';
#endif
      if (*c == separator && *(c + 1)) {
        filename = c + 1;
      }
      c++;
    }
  }

  return Strings::FormatShort("%s:%lu (%s)", filename, line, function);
}

} // namespace AtelierEsri
