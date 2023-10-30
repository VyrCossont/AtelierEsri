#include <MacTypes.h>
#include <MacWindows.h>

#include "GWorld.hpp"
#include "Result.hpp"

namespace AtelierEsri {

class Window {
public:
  explicit Window(SInt16 resourceID);
  ~Window();
  static const WindowRef inFrontOfAllOtherWindows;
  Result<std::monostate, OSErr> Present();
  void Dismiss();
  /// Get a GWorld optimized for copy to this window.
  Result<GWorld, OSErr> FastGWorld();
  Result<Rect, OSErr> PortBounds();
  Result<CGrafPtr, OSErr> Port();

private:
  SInt16 resourceID;
  WindowRef windowRef;
};

} // namespace AtelierEsri
