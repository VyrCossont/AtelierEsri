#pragma once

#include <cstddef>

#include <MacTypes.h>

namespace AtelierEsri {

class Debug {
public:
  static OSErr Printfln(const char *fmt, ...);

private:
  /// Append to a log file in the same directory.
  static OSErr FilePrint(size_t num_bytes, const char *buffer);
  /// Log to the modem port at 9600 8-N-1.
  static OSErr SerialPrint(size_t num_bytes, const char *buffer);
};

} // namespace AtelierEsri
