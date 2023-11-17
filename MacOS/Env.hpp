#pragma once

#include <cstdint>

namespace AtelierEsri {

class Env {
public:
  /// Set up Toolbox stuff.
  static void Initialize() noexcept;
  static uint64_t Microseconds() noexcept;
};

} // namespace AtelierEsri
