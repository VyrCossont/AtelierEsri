#include <optional>

#include <Dialogs.h>
#include <Events.h>
#include <Fonts.h>
#include <MacWindows.h>
#include <Menus.h>
#include <TextEdit.h>

#include "AppResources.h"
#include "Debug.hpp"
#include "Env.hpp"
#include "GWorld.hpp"
#include "Game.hpp"
#include "Result.hpp"
#include "Strings.hpp"

#include "App.hpp"

namespace AtelierEsri {

App::App() : gameWindow(gameWINDResourceID) { Initialize(); }

void App::Initialize() {
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

Result<Unit> Draw(GWorld &gWorld) {
  GUARD_LET_TRY(GWorldLockPixelsGuard, lockPixelsGuard, gWorld.LockPixels());
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

  return Ok(Unit());
}

Result<Unit> Copy(GWorld &gWorld, Window &window) {
  GUARD_LET_TRY(GWorldLockPixelsGuard, lockPixelsGuard, gWorld.LockPixels());

  Rect gWorldRect = gWorld.Bounds();
  const BitMap *gWorldBits = lockPixelsGuard.Bits();

  GUARD_LET_TRY(Rect, windowRect, window.PortBounds());
  GUARD_LET_TRY(CGrafPtr, windowPort, window.Port());

#if TARGET_API_MAC_CARBON
  const BitMap *windowBits = GetPortBitMapForCopyBits(windowPort);
#else
  const BitMap *windowBits = &((GrafPtr)windowPort)->portBits;
#endif

  QD_CHECKED(CopyBits(gWorldBits, windowBits, &gWorldRect, &windowRect, srcCopy,
                      nullptr),
             "Couldn't copy from offscreen GWorld");

  return Ok(Unit());
}

OSErr App::Run() {
  auto result = EventLoop();
  if (result.is_ok()) {
    return noErr;
  }

  FatalError(result.err_value());

  OSErr osErr = result.err_value().osErr;
  return osErr ? osErr : appError;
}

Result<std::monostate, Error> App::EventLoop() {
  EventRecord event;
  /// Ticks (approx. 1/60th of a second)
  uint32_t sleepTimeTicks = 1;
  uint64_t frameDurationUsec = sleepTimeTicks * 1000 * 1000 / 60;
  uint16_t demoState = 0;
  std::optional<GWorld> offscreenGWorld;
  Game game;

  uint64_t lastFrameTimestampUsec = Env::Microseconds();
  while (true) {
    // Don't set a mouse region so we don't get mouse-moved events for now.
    WaitNextEvent(everyEvent, &event, sleepTimeTicks, nullptr);

    switch (event.what) {
    case keyDown:
      switch (demoState) {
      case 0:
        gameWindow.Present();
        break;

      case 1: {
        Result<GWorld> gWorldResult = gameWindow.FastGWorld();
        if (gWorldResult.is_err()) {
          return Err(gWorldResult.take_err_value());
        }
        offscreenGWorld = gWorldResult.take_ok_value();
      } break;

      case 2:
        Draw(offscreenGWorld.value());
        break;

      case 3:
        Copy(offscreenGWorld.value(), gameWindow);
        break;

      default:
        return Ok(std::monostate());
      }
      demoState++;
      break;

    default:
      // Ignore most kinds of event, including disk formatting events.
      break;
    }

    uint64_t currentTimestampUsec = Env::Microseconds();
    if ((currentTimestampUsec - lastFrameTimestampUsec) >= frameDurationUsec) {
      // TODO: handle multiple elapsed frames
      lastFrameTimestampUsec = currentTimestampUsec;

      game.Update();

      if (offscreenGWorld.has_value()) {
        Copy(offscreenGWorld.value(), gameWindow);
        SetPort((GrafPtr)gameWindow.Port().ok_value());
        game.Draw();
      }
    }
  }
}

// Not used in `MacErrors.h`.
OSErr App::appError = 666;

void App::FatalError(const Error &error) {
  Str255 explanation, location;
  Strings::ToPascal(error.Explanation(), explanation);
  Strings::ToPascal(error.Location(), location);
  ParamText(explanation, location, nullptr, nullptr);

  Alert alert(errorALRTResourceID, AlertType::stop);
  // Ignore the result.
  alert.Show();
}

} // namespace AtelierEsri
