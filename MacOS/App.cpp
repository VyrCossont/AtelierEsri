#include "App.hpp"

#include <Devices.h>
#include <Events.h>
#include <Menus.h>
#include <ToolUtils.h>

#include "AppResources.h"
#include "Debug.hpp"
#include "Env.hpp"
#include "GWorld.hpp"
#include "Game.hpp"
#include "Result.hpp"

namespace AtelierEsri {

Result<App> App::New() noexcept {
  TRY(SetupMenuBar());
  GUARD_LET_TRY(Window, gameWindow, Window::Present(gameWINDResourceID));
  GUARD_LET_TRY(GWorld, offscreenGWorld, gameWindow.FastGWorld());
  GUARD_LET_TRY(Game, game, Game::Setup(gameWindow));
  return Ok(
      App(std::move(gameWindow), std::move(offscreenGWorld), std::move(game)));
}

Result<Unit> App::SetupMenuBar() noexcept {
  MenuBarHandle menuBar = GetNewMBar(menuBarMBARResourceID);
  REQUIRE_NOT_NULL(menuBar);
  SetMenuBar(menuBar);

  // Attach system-managed Apple menu items.
  MenuHandle appleMenu = GetMenuHandle(appleMenuMENUResourceID);
  if (!appleMenu) {
    // TODO: if we do this a lot, add a managed handle type
    DisposeHandle(menuBar);
  }
  REQUIRE_NOT_NULL(appleMenu);
  AppendResMenu(appleMenu, 'DRVR');

  DrawMenuBar();

  // Menu bar not retained after this because it doesn't have to be:
  // https://preterhuman.net/macstuff/insidemac/Toolbox/Toolbox-98.html#MARKER-9-260
  DisposeHandle(menuBar);

  return Ok(Unit());
}

App::App(Window gameWindow, GWorld offscreenGWorld, Game game) noexcept
    : gameWindow(std::move(gameWindow)),
      offscreenGWorld(std::move(offscreenGWorld)), game(std::move(game)) {}

Result<Unit> Copy(GWorld &gWorld, Window &window) {
  GUARD_LET_TRY(GWorldLockPixelsGuard, lockPixelsGuard, gWorld.LockPixels());

  Rect gWorldRect = gWorld.Bounds();
  const BitMap *gWorldBits = lockPixelsGuard.Bits();

  Rect windowRect = window.PortBounds();
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

Result<Unit> App::EventLoop() noexcept {
  EventRecord event;
  /// Ticks (approx. 1/60th of a second)
  uint32_t sleepTimeTicks = 1;
  uint64_t frameDurationUsec = sleepTimeTicks * 1000 * 1000 / 60;

  uint64_t lastFrameTimestampUsec = Env::Microseconds();
  while (true) {
    // Don't set a mouse region so we don't get mouse-moved events for now.
    WaitNextEvent(everyEvent, &event, sleepTimeTicks, nullptr);

    switch (event.what) {
    case keyDown:
      if (event.modifiers & cmdKey) {
        // MenuKey() is an A-line trap and the declaration confuses CLion.
#pragma clang diagnostic push
#pragma ide diagnostic ignored "UnusedLocalVariable"
        auto key = static_cast<int16_t>(event.message & charCodeMask);
#pragma clang diagnostic pop
        bool quit = HandleMenuSelection(MenuKey(key));
        if (quit) {
          return Ok(Unit());
        }
      }
      break;

    case mouseDown: {
      WindowPtr windowPtr;
      switch (FindWindow(event.where, &windowPtr)) {
      case inMenuBar: {
        bool quit = HandleMenuSelection(MenuSelect(event.where));
        if (quit) {
          return Ok(Unit());
        }
      } break;

      case inSysWindow:
#if !TARGET_API_MAC_CARBON
        // Handle desk accessory interactions.
        SystemClick(&event, windowPtr);
#endif
        break;

      default:
        break;
      }
    } break;

    default:
      // Ignore most kinds of event, including disk formatting events.
      break;
    }

    uint64_t currentTimestampUsec = Env::Microseconds();
    if ((currentTimestampUsec - lastFrameTimestampUsec) >= frameDurationUsec) {
      // TODO: handle multiple elapsed frames
      lastFrameTimestampUsec = currentTimestampUsec;

      game.Update();

      game.Draw(offscreenGWorld);

      Copy(offscreenGWorld, gameWindow);
      SetPort((GrafPtr)gameWindow.Port().ok_value());
    }
  }
}

enum AppleMenuItems {
  about = 1,
};

enum FileMenuItems {
  open = 1,
  save = 2,
  // separator
  quit = 4,
};

bool App::HandleMenuSelection(int32_t menuSelection) noexcept {
  int16_t menuID = HiWord(menuSelection);
  int16_t menuItem = LoWord(menuSelection);
  Debug::Printfln("Menu selection: menuID = %d, menuItem = %d", menuID,
                  menuItem);
  switch (menuID) {
  case appleMenuMENUResourceID:
    switch (menuItem) {
    case AppleMenuItems::about:
      AboutBox();
      break;

    default: {
#if !TARGET_API_MAC_CARBON
      // https://preterhuman.net/macstuff/insidemac/Toolbox/Toolbox-104.html#HEADING104-8
      Str255 itemName;
      MenuHandle appleMenu = GetMenuHandle(appleMenuMENUResourceID);
      GetMenuItemText(appleMenu, menuItem, itemName);
      OpenDeskAcc(itemName);
      // TODO: implement suspend/resume events
      //  As is, this will open the Apple menu item only after the app quits.
      //  https://preterhuman.net/macstuff/insidemac/Toolbox/Toolbox-41.html#HEADING41-14
#endif
    } break;
    }

    break;

  case fileMenuMENUResourceID:
    switch (menuItem) {
    case FileMenuItems::open:
      // TODO
      break;

    case FileMenuItems::save:
      // TODO
      break;

    case FileMenuItems::quit:
      HiliteMenu(0);
      return true;

    default:
      break;
    }
    break;

  default:
    break;
  }

  HiliteMenu(0);
  return false;
}

void App::AboutBox() noexcept {
  // TODO
}

} // namespace AtelierEsri
