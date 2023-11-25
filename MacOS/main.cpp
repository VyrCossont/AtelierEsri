#include <LowMem.h>
#include <MacTypes.h>

#include "Alert.hpp"
#include "App.hpp"
#include "AppResources.h"
#include "Debug.hpp"
#include "Env.hpp"
#include "Exception.hpp"
#include "Strings.hpp"

namespace AtelierEsri {

/// Error code for an app-specific abnormal exit.
/// Should be one not used in `MacErrors.h`.
static OSErr appError = static_cast<OSErr>(666);

/// Display a fatal error.
void FatalError(const std::string &explanation, const std::string &location) {
  Debug::Printfln("Fatal error: %s\n%s", explanation.c_str(), location.c_str());

  Str255 pascalExplanation, pascalLocation;
  Strings::ToPascal(explanation, pascalExplanation);
  Strings::ToPascal(location, pascalLocation);
  ParamText(pascalExplanation, pascalLocation, nullptr, nullptr);

  AtelierEsri::Alert alert(errorALRTResourceID, AtelierEsri::AlertType::stop);
  alert.Show();
}

/// Create and run the app.
void Run() {
  Env::Initialize();

  App app = App::New();
  return app.EventLoop();
}

} // namespace AtelierEsri

int main() {
  using namespace AtelierEsri;

  OSErr osErr = noErr;
  try {
    Run();
  } catch (const Exception &e) {
    FatalError(e.Explanation(), e.Location());
    osErr = e.osErr;
    osErr = osErr ? osErr : appError;
  } catch (const std::exception &e) {
    std::string explanation(e.what());
    std::string location("<unknown location>");
    osErr = appError;
  } catch (...) {
    std::string explanation("<unknown exception>");
    std::string location("<unknown location>");
    osErr = appError;
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
