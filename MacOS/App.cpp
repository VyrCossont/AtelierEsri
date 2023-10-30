#include <optional>

#include <Dialogs.h>
#include <Events.h>
#include <Fonts.h>
#include <MacWindows.h>
#include <Menus.h>
#include <QuickDraw.h>
#include <TextEdit.h>

#include "AppResources.h"
#include "Debug.hpp"
#include "GWorld.hpp"
#include "Result.hpp"

#include "App.hpp"

namespace AtelierEsri {

App::App() : helloAlert(helloALRTResourceID), gameWindow(gameWINDResourceID) {
  Initialize();
}

void App::Initialize() {
#if !TARGET_API_MAC_CARBON
  InitGraf(&qd.thePort);
  InitFonts();
  InitWindows();
  InitMenus();
  TEInit();
  InitDialogs(nil);
#endif
  InitCursor();
  // Flush any low-level events left over from other apps. (PSKM p. 167)
  FlushEvents(everyEvent, 0);
}

void Draw(GWorld &gWorld) {
  Result<GWorldLockPixelsGuard, OSErr> lockResult = gWorld.LockPixels();
  if (lockResult.is_err()) {
    SysError(lockResult.err_value());
    ExitToShell();
  }
  GWorldLockPixelsGuard lockPixelsGuard = lockResult.ok_value();
  GWorldActiveGuard activeGuard = gWorld.MakeActive();

  Rect rect = gWorld.Bounds();
  EraseRect(&rect);
  InsetRect(&rect, 40, 40);

  Pattern pattern;
#if TARGET_API_MAC_CARBON
  GetQDGlobalsBlack(&pattern);
#else
  pattern = qd.black;
#endif

  FillRect(&rect, &pattern);
}

void Copy(GWorld &gWorld, Window &window) {
  Result<GWorldLockPixelsGuard, OSErr> lockResult = gWorld.LockPixels();
  if (lockResult.is_err()) {
    SysError(__LINE__);
    ExitToShell();
  }
  GWorldLockPixelsGuard lockPixelsGuard = lockResult.take_ok_value();

  Rect gWorldRect = gWorld.Bounds();
  const BitMap *gWorldBits = lockPixelsGuard.Bits();

  Result<Rect, OSErr> windowRectResult = window.PortBounds();
  if (windowRectResult.is_err()) {
    SysError(__LINE__);
    ExitToShell();
  }
  Rect windowRect = windowRectResult.ok_value();

  Result<CGrafPtr, OSErr> windowPortResult = window.Port();
  if (windowPortResult.is_err()) {
    SysError(__LINE__);
    ExitToShell();
  }
  CGrafPtr windowPort = windowPortResult.ok_value();

#if TARGET_API_MAC_CARBON
  const BitMap *windowBits = GetPortBitMapForCopyBits(windowPort);
#else
  const BitMap *windowBits = &((GrafPtr)windowPort)->portBits;
#endif

  CopyBits(gWorldBits, windowBits, &gWorldRect, &windowRect, srcCopy, nil);
  OSErr qdError = QDError();
  if (qdError != noErr) {
    SysError(__LINE__);
    ExitToShell();
  }
}

void App::Run() {
  EventRecord event;
  /// Ticks (approx. 1/60th of a second)
  UInt32 sleepTime;
  int demoState = 0;
  std::optional<GWorld> offscreenGWorld;

  // Sleep less than the text caret blinking interval so we can animate it
  // properly. (PSKM p. 165)
  // TODO: (Vyr) Once we have a game going, we're going to want to run at 15 hz
  //  (or 30 or 60).
  sleepTime = GetCaretTime() / 2;

  Debug::Printfln("Starting event loop.");
  while (true) {
    // Don't set a mouse region so we don't get mouse-moved events for now.
    WaitNextEvent(everyEvent, &event, sleepTime, nil);
    switch (event.what) {
    case keyDown:
      Debug::Printfln("demoState = %#08x", demoState);

      switch (demoState) {
      case 0:
        helloAlert.Show();
        demoState++;
        break;

      case 1:
        gameWindow.Present();
        demoState++;
        break;

      case 2: {
        Result<GWorld, OSErr> gWorldResult = gameWindow.FastGWorld();
        Debug::Printfln("Past FastGWorld call");
        if (gWorldResult.is_err()) {
          SysError(gWorldResult.take_err_value());
          ExitToShell();
        }
        Debug::Printfln("Past gWorldResult error handling");
        offscreenGWorld = gWorldResult.take_ok_value();
        Debug::Printfln("Past gWorldResult take_ok_value call");
      }
        demoState++;
        break;

      case 3:
        Draw(offscreenGWorld.value());
        demoState++;
        break;

      case 4:
        Copy(offscreenGWorld.value(), gameWindow);
        demoState++;
        break;

      default:
        Debug::Printfln("Finished.");
        return;
      }
      break;

    default:
      // Ignore most kinds of event, including disk formatting events.
      break;
    }
  }
}

} // namespace AtelierEsri
