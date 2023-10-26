#include <MacTypes.h>
#include <MacWindows.h>

namespace AtelierEsri {

class Window {
public:
  explicit Window(SInt16 resourceID);
  ~Window();
  static const WindowRef inFrontOfAllOtherWindows;
  void Present();
  void Dismiss();

private:
  SInt16 resourceID;
  WindowRef windowRef;
};

} // namespace AtelierEsri
