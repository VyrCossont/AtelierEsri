#include "Env.hpp"

namespace AtelierEsri {

#if !TARGET_API_MAC_CARBON
std::optional<SysEnvRec> Env::sysEnvRec = {};
#endif

Result<bool, OSErr> Env::HasColorQuickDraw() {
#if TARGET_API_MAC_CARBON
  return Ok(true);
#else
  if (!sysEnvRec.has_value()) {
    OSErr error;
    SysEnvRec newSysEnvRec;
    error = SysEnvirons(1, &newSysEnvRec);
    if (error != noErr) {
      return Err(error);
    }
    sysEnvRec = newSysEnvRec;
  }
  return Ok(sysEnvRec->hasColorQD != 0);
#endif
}

} // namespace AtelierEsri
