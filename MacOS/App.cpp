#include <Dialogs.h>
#include <Events.h>
#include <Fonts.h>
#include <MacWindows.h>
#include <Menus.h>
#include <QuickDraw.h>
#include <TextEdit.h>

#include "AppResources.h"

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

void App::Run() {
  EventRecord event;
  /// Ticks (approx. 1/60th of a second)
  UInt32 sleepTime;
  int demoState = 0;

  // Sleep less than the text caret blinking interval so we can animate it
  // properly. (PSKM p. 165)
  // TODO: (Vyr) Once we have a game going, we're going to want to run at 15 hz
  //  (or 30 or 60).
  sleepTime = GetCaretTime() / 2;

  while (true) {
    // Don't set a mouse region so we don't get mouse-moved events for now.
    WaitNextEvent(everyEvent, &event, sleepTime, nil);
    switch (event.what) {
    case keyDown:
      switch (demoState) {
      case 0:
        helloAlert.Show();
        demoState++;
        break;

      case 1:
        gameWindow.Present();
        demoState++;
        break;

      default:
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
