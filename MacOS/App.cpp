#include "App.hpp"

#include <Devices.h>
#include <Events.h>
#include <Menus.h>
#include <ToolUtils.h>

#include "AppResources.h"
#include "Debug.hpp"
#include "Env.hpp"
#include "Exception.hpp"
#include "GWorld.hpp"
#include "Game.hpp"

namespace AtelierEsri {

App App::New() {
  SetupMenuBar();
  Window gameWindow = Window::Present(gameWINDResourceID);
  ControlHandle gameVScrollBar =
      gameWindow.AddControl(gameVScrollBarCNTLResourceID);
  GWorld offscreenGWorld = gameWindow.FastGWorld();
  Game game = Game::Setup(gameWindow);
  return {std::move(gameWindow), gameVScrollBar, std::move(offscreenGWorld),
          std::move(game)};
}

void App::SetupMenuBar() {
  const MenuBarHandle menuBar = GetNewMBar(menuBarMBARResourceID);
  REQUIRE_NOT_NULL(menuBar);
  SetMenuBar(menuBar);

  // Attach system-managed Apple menu items.
  // ReSharper disable once CppLocalVariableMayBeConst
  MenuHandle appleMenu = GetMenuHandle(appleMenuMENUResourceID);
  if (!appleMenu) {
    // TODO: if we do this a lot, add a managed handle type
    DisposeHandle(menuBar);
  }
  REQUIRE_NOT_NULL(appleMenu);
  // ReSharper disable once CppMultiCharacterLiteral
  AppendResMenu(appleMenu, 'DRVR');

  DrawMenuBar();

  // Menu bar not retained after this because it doesn't have to be:
  // https://preterhuman.net/macstuff/insidemac/Toolbox/Toolbox-98.html#MARKER-9-260
  DisposeHandle(menuBar);
}

App::App(Window gameWindow, const ControlHandle gameVScrollBar,
         GWorld offscreenGWorld, Game game)
    : gameWindow(std::move(gameWindow)), gameVScrollBar(gameVScrollBar),
      offscreenGWorld(std::move(offscreenGWorld)), game(std::move(game)) {}

void Copy(GWorld &gWorld, const Window &window) {
  const GWorldLockPixelsGuard lockPixelsGuard = gWorld.LockPixels();

  Rect gWorldRect = gWorld.Bounds();
  // Don't draw on top of the vertical scroll bar.
  gWorldRect.right -= 15;
  const BitMap *gWorldBits = lockPixelsGuard.Bits();

  Rect windowRect = window.PortBounds();
  windowRect.right -= 15;
  CGrafPtr windowPort = window.Port();

#if TARGET_API_MAC_CARBON
  const BitMap *windowBits = GetPortBitMapForCopyBits(windowPort);
#else
  const BitMap *windowBits = &reinterpret_cast<GrafPtr>(windowPort)->portBits;
#endif

  QD_CHECKED(CopyBits(gWorldBits, windowBits, &gWorldRect, &windowRect, srcCopy,
                      nullptr),
             "Couldn't copy from offscreen GWorld");
}

void App::EventLoop() {
  EventRecord event;
  /// Ticks (approx. 1/60th of a second)
  constexpr uint32_t sleepTimeTicks = 1;
  constexpr uint64_t frameDurationUsec = sleepTimeTicks * 1000 * 1000 / 60;

  uint64_t lastFrameTimestampUsec = Env::Microseconds();
  while (true) {
    // Don't set a mouse region so we don't get mouse-moved events for now.
    WaitNextEvent(everyEvent, &event, sleepTimeTicks, nullptr);

    switch (event.what) {
    case keyDown:
      if (event.modifiers & cmdKey) {
        // MenuKey() is an A-line trap and the declaration confuses CLion.
        // ReSharper disable once CppDFAUnreadVariable
        // ReSharper disable once CppDFAUnusedValue
        const auto key = static_cast<int16_t>(event.message & charCodeMask);
        // ReSharper disable once CppTooWideScope
        const bool quit = HandleMenuSelection(MenuKey(key));
        if (quit) {
          return;
        }
      }
      break;

    case mouseDown: {
      WindowPtr windowPtr;
      switch (FindWindow(event.where, &windowPtr)) {
      case inMenuBar: {
        // ReSharper disable once CppTooWideScope
        const bool quit = HandleMenuSelection(MenuSelect(event.where));
        if (quit) {
          return;
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

    if (const uint64_t currentTimestampUsec = Env::Microseconds();
        (currentTimestampUsec - lastFrameTimestampUsec) >= frameDurationUsec) {
      // TODO: handle multiple elapsed frames
      lastFrameTimestampUsec = currentTimestampUsec;

      game.Update();

      game.Draw(offscreenGWorld);

      Copy(offscreenGWorld, gameWindow);
      SetPort(reinterpret_cast<GrafPtr>(gameWindow.Port()));
    }
  }
}

enum class AppleMenuItems : int16_t {
  about = 1,
};

enum class FileMenuItems : int16_t {
  open = 1,
  save = 2,
  // separator
  quit = 4,
};

bool App::HandleMenuSelection(const int32_t menuSelection) {
  const int16_t menuID = HiWord(menuSelection);
  const int16_t menuItem = LoWord(menuSelection);
  Debug::Printfln("Menu selection: menuID = %d, menuItem = %d", menuID,
                  menuItem);
  switch (menuID) {
  case appleMenuMENUResourceID:
    switch (static_cast<AppleMenuItems>(menuItem)) {
    case AppleMenuItems::about:
      AboutBox();
      break;

    default: {
#if !TARGET_API_MAC_CARBON
      // https://preterhuman.net/macstuff/insidemac/Toolbox/Toolbox-104.html#HEADING104-8
      Str255 itemName;
      // ReSharper disable once CppLocalVariableMayBeConst
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
    switch (static_cast<FileMenuItems>(menuItem)) {
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

void App::AboutBox() {
  // TODO
}

} // namespace AtelierEsri
