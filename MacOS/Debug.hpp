#pragma once

#include <cstddef>

#include <MacTypes.h>

namespace AtelierEsri {

class Debug {
public:
  static OSErr Printfln(const char *fmt, ...);

private:
  static OSErr FilePrint(size_t num_bytes, const char *buffer);
  static OSErr SerialPrint(size_t num_bytes, const char *buffer);
};

} // namespace AtelierEsri
