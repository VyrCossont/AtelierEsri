#pragma once

#include <optional>

#include <MacTypes.h>
#include <OSUtils.h>

#include "Result.hpp"

namespace AtelierEsri {

class Env {
public:
  static Result<bool, OSErr> HasColorQuickDraw();

private:
#if !TARGET_API_MAC_CARBON
  static std::optional<SysEnvRec> sysEnvRec;
#endif
};

} // namespace AtelierEsri
