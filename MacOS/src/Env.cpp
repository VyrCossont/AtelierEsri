#include "Env.hpp"

#include <Dialogs.h>
#include <Events.h>
#include <Fonts.h>
#include <MacTypes.h>
#include <MacWindows.h>
#include <Menus.h>
#include <TextEdit.h>
#include <Timer.h>

namespace AtelierEsri {

void Env::Initialize() {
#if !TARGET_API_MAC_CARBON
  InitGraf(&qd.thePort);
  InitFonts();
  InitWindows();
  InitMenus();
  TEInit();
  InitDialogs(nullptr);
#endif
  InitCursor();
  // Flush any low-level events left over from other apps. (PSKM p. 167)
  FlushEvents(everyEvent, 0);
}

/// Questionably safe tool for transmuting Mac 64-bit int structs to actual
/// ints. `bit_cast` doesn't exist in C++17.
union U64 {
  UnsignedWide as_struct;
  uint64_t as_uint;
};

uint64_t Env::Microseconds() {
  U64 u64 = {.as_uint = 0};
  ::Microseconds(&u64.as_struct);
  return u64.as_uint;
}

}  // namespace AtelierEsri
