#pragma once

#include <optional>

#include <MacTypes.h>
#include <OSUtils.h>

#include "Error.hpp"
#include "Result.hpp"

namespace AtelierEsri {

class Env {
public:
  static uint64_t Microseconds() noexcept;
  static Result<bool> HasColorQuickDraw();

private:
#if !TARGET_API_MAC_CARBON
  static std::optional<SysEnvRec> sysEnvRec;
#endif
};

} // namespace AtelierEsri
