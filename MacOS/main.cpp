#include <LowMem.h>
#include <MacTypes.h>

#include "App.hpp"

int main() {
  AtelierEsri::App app;

  OSErr osErr = app.Run();

#if !TARGET_API_MAC_CARBON
  if (osErr) {
    // Set the low-memory global indicating why the app crashed:
    // https://preterhuman.net/macstuff/insidemac/Processes/Processes-27.html
    LMSetDSErrCode(osErr);
  }
#endif

  return 0;
}
