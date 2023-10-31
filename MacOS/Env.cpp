#include "Env.hpp"

#include <MacTypes.h>
#include <Timer.h>

namespace AtelierEsri {

/// Questionably safe tool for transmuting Mac 64-bit int structs to actual
/// ints. `bit_cast` doesn't exist in C++17.
union U64 {
  UnsignedWide as_struct;
  uint64_t as_int;
};

uint64_t Env::Microseconds() noexcept {
  U64 u64 = {0};
  ::Microseconds(&u64.as_struct);
  return u64.as_int;
}

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
