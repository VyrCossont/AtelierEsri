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
static OSErr appError = 666;

/// Display a fatal error.
void FatalError(
    const char *fileName,
    const uint32_t line,
    const char *func,
    const std::string &explanation,
    const std::string &location
) {
  Debug::Printfln(fileName, line, func, "%s", explanation.c_str());

  Str255 pascalExplanation, pascalLocation;
  Strings::ToPascal(explanation, pascalExplanation);
  Strings::ToPascal(location, pascalLocation);
  ParamText(pascalExplanation, pascalLocation, nullptr, nullptr);

  const Alert alert(errorALRTResourceID, stop);
  // ReSharper disable once CppExpressionWithoutSideEffects
  alert.Show();
}

/// Create and run the app.
void Run() {
  Env::Initialize();

  App app{};
  return app.EventLoop();
}

}  // namespace AtelierEsri

int main() {
  using namespace AtelierEsri;

  OSErr osErr = noErr;
  try {
    Run();
  } catch (const Exception &e) {
    FatalError(e.fileName, e.line, e.func, e.Explanation(), e.Location());
    osErr = e.osErr;
    // This code should be reachable. `e.osErr` can be initialized to any OSErr.
    // ReSharper disable once CppDFAUnreachableCode
    osErr = osErr ? osErr : appError;
  } catch (const std::exception &e) {
    const std::string explanation(e.what());
    const std::string location("<unknown location>");
    FatalError("???", 0, "???", explanation, location);
    osErr = appError;
  } catch (...) {
    const std::string explanation("<unknown exception>");
    const std::string location("<unknown location>");
    FatalError("???", 0, "???", explanation, location);
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
