#include "App.hpp"

#include <Devices.h>
#include <DiskInit.h>
#include <Events.h>
#include <MacWindows.h>
#include <Menus.h>
#include <ToolUtils.h>

#include "AppResources.h"
#include "Debug.hpp"
#include "Env.hpp"
#include "Exception.hpp"
#include "GWorld.hpp"
#include "Game.hpp"

namespace AtelierEsri {

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

App::App()
    : gameWindow(gameWINDResourceID),
      gameVScrollBar(gameVScrollBarCNTLResourceID, gameWindow),
      offscreenGWorld(gameWindow.FastGWorld()),
      game(gameWindow) {
  SetupMenuBar();
}

void App::EventLoop() {
  EventRecord event;
  /// Ticks (approx. 1/60th of a second)
  constexpr uint32_t sleepTimeTicks = 1;
  constexpr uint64_t frameDurationUsec = sleepTimeTicks * 1000 * 1000 / 60;

  gameVScrollBar.SetMin(0);
  gameVScrollBar.SetMax(100);
  gameVScrollBar.SetValue(50);
  gameVScrollBar.onScrollLineUp = [&](const ScrollBar &scrollBar) {
    scrollBar.ScrollBy(-1);
  };
  gameVScrollBar.onScrollLineDown = [&](const ScrollBar &scrollBar) {
    scrollBar.ScrollBy(1);
  };
  gameVScrollBar.onScrollPageUp = [&](const ScrollBar &scrollBar) {
    scrollBar.ScrollBy(-10);
  };
  gameVScrollBar.onScrollPageDown = [&](const ScrollBar &scrollBar) {
    scrollBar.ScrollBy(10);
  };
  // Don't need to do anything to handle drags.

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
        WindowRef eventWindowRef;
        switch (const WindowPartCode part =
                    FindWindow(event.where, &eventWindowRef)) {
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
            SystemClick(&event, eventWindowRef);
#endif
            break;

          case inDesk:
            // Don't need to do anything.
            break;

          default:
            if (eventWindowRef) {
              if (const auto window =
                      reinterpret_cast<Window *>(GetWRefCon(eventWindowRef))) {
                window->HandleMouseDown(event.where, part);
              }
            }
            break;
        }
      } break;

      case updateEvt:
        if (const auto window = EventWindow(event)) {
          window->HandleUpdate();
        }
        break;

      case activateEvt:
        if (const auto window = EventWindow(event)) {
          if (event.modifiers & activeFlag) {
            window->HandleActivate();
          } else {
            window->HandleDeactivate();
          }
        }
        break;

      case diskEvt:
        DiskInserted(event);
        break;

      default:
        // Ignore other events.
        break;
    }

    if (const uint64_t currentTimestampUsec = Env::Microseconds();
        (currentTimestampUsec - lastFrameTimestampUsec) >= frameDurationUsec) {
      // TODO: handle multiple elapsed frames
      lastFrameTimestampUsec = currentTimestampUsec;

      game.Update(gameVScrollBar.Value());

      game.Draw(offscreenGWorld);

      // Exclude scroll bar from copy area.
      // ReSharper disable CppUseStructuredBinding
      Rect gWorldRect = offscreenGWorld.Bounds();
      gWorldRect.right -= 15;

      Rect windowRect = gameWindow.PortBounds();
      windowRect.right -= 15;
      // ReSharper restore CppUseStructuredBinding

      gameWindow.CopyFrom(offscreenGWorld, gWorldRect, windowRect);
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
  Debug::Printfln(
      "Menu selection: menuID = %d, menuItem = %d", menuID, menuItem
  );
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
          //  As is, this will open the Apple menu item only after the app
          //  quits.
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

void App::DiskInserted(const EventRecord &event) {
#if !TARGET_API_MAC_CARBON
  // https://preterhuman.net/macstuff/insidemac/Files/Files-373.html#MARKER-9-49
  DILoad();
  if (HiWord(event.message)) {
    // Disk mount failed. Offer to initialize it.
    // First param is the top left of the disk dialog, but System 7 always
    // ignores that and centers it on the screen for us.
    DIBadMount({120, 120}, event.message);
    // This returns an `OSErr`, but failure will also notify the user and
    // eject the disk, so we don't have to do anything with that error
    // code.
  }
  DIUnload();
#endif
}

Window *App::EventWindow(const EventRecord &event) {
  if (const auto eventWindowRef = reinterpret_cast<WindowRef>(event.message)) {
    if (const auto window =
            reinterpret_cast<Window *>(GetWRefCon(eventWindowRef))) {
      return window;
    }
  }
  return nullptr;
}

}  // namespace AtelierEsri
