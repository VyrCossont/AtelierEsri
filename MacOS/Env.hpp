#pragma once

#include <cstdint>

namespace AtelierEsri {

class Env {
public:
  /// Set up Toolbox stuff.
  static void Initialize();
  static uint64_t Microseconds();
};

} // namespace AtelierEsri
