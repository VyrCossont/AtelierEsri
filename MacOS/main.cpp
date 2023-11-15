#include <LowMem.h>
#include <MacTypes.h>

#include "Alert.hpp"
#include "App.hpp"
#include "AppResources.h"
#include "Debug.hpp"
#include "Env.hpp"
#include "Result.hpp"
#include "Strings.hpp"

namespace AtelierEsri {

/// Error code for an app-specific abnormal exit.
/// Should be one not used in `MacErrors.h`.
static OSErr appError = static_cast<OSErr>(666);

/// Display a fatal error.
void FatalError(const Error &error) noexcept {
  Debug::Printfln("Fatal error: %s\n%s", error.Explanation().c_str(),
                  error.Location().c_str());

  Str255 explanation, location;
  Strings::ToPascal(error.Explanation(), explanation);
  Strings::ToPascal(error.Location(), location);
  ParamText(explanation, location, nullptr, nullptr);

  AtelierEsri::Alert alert(errorALRTResourceID, AtelierEsri::AlertType::stop);
  // Ignore the result.
  alert.Show();
}

/// Create and run the app.
Result<Unit> Run() {
  Env::Initialize();

  GUARD_LET_TRY(App, app, App::New());
  return app.EventLoop();
}

} // namespace AtelierEsri

int main() {
  using namespace AtelierEsri;

  OSErr osErr = noErr;
  Result<Unit> result = Run();
  if (result.is_err()) {
    FatalError(result.err_value());
    osErr = result.err_value().osErr;
    osErr = osErr ? osErr : appError;
  }

#if !TARGET_API_MAC_CARBON
  if (osErr) {
    // Set the low-memory global indicating why the app crashed:
    // https://preterhuman.net/macstuff/insidemac/Processes/Processes-27.html
    LMSetDSErrCode(osErr);
  }
#endif

  return 0;
}
