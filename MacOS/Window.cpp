#include "Window.hpp"

#include <MacTypes.h>

#include "Control.hpp"
#include "Drawing.hpp"
#include "Exception.hpp"
#include "GWorld.hpp"

namespace AtelierEsri {

Window::Window(const WindowRef ref) : ref(ref) { SetRefConToThis(); }

Window::Window(Window &&src) noexcept {
  ref = src.ref;
  src.ref = nullptr;
  SetRefConToThis();
}

Window &Window::operator=(Window &&src) noexcept {
  ref = src.ref;
  src.ref = nullptr;
  SetRefConToThis();
  return *this;
}

Window::~Window() {
  if (ref) {
    DisposeWindow(ref);
  }
}

void Window::SetRefConToThis() {
  SetWRefCon(ref, reinterpret_cast<long>(this));
}

const WindowRef Window::AllOtherWindows = reinterpret_cast<WindowRef>(-1);

Window Window::Present(const ResourceID resourceID, const WindowRef inFrontOf) {
  const bool hasColorQuickDraw = QD::HasColor();

  WindowRef ref;
  // Not actually identical: these are A-line traps and Clang can't parse them.
  if (hasColorQuickDraw) {  // NOLINT(*-branch-clone)
    ref = GetNewCWindow(resourceID, nullptr, inFrontOf);
  } else {
    ref = GetNewWindow(resourceID, nullptr, inFrontOf);
  }
  REQUIRE_NOT_NULL(ref);

  return Window(ref);
}

GWorld Window::FastGWorld(const int16_t w, const int16_t h) const {
  Rect rect;
  GetWindowPortBounds(ref, &rect);
  if (w > 0 && h > 0) {
    // Change GWorld dimensions to something other than the window's dimensions.
    rect.right = static_cast<int16_t>(rect.left + w);
    rect.bottom = static_cast<int16_t>(rect.top + h);
  }

  // Create a GWorld matching the window device, color table, etc.
  GWorldPtr gWorldPtr;
  OS_CHECKED(
      NewGWorld(&gWorldPtr, 0, &rect, nullptr, nullptr, 0),
      "Couldn't create offscreen GWorld"
  );
  REQUIRE_NOT_NULL(gWorldPtr);

  return GWorld(gWorldPtr);
}

void Window::CopyFrom(
    const GWorld &gWorld, const Rect gWorldRect, const Rect windowRect
) const {
  const GWorldLockPixelsGuard lockPixelsGuard = gWorld.LockPixels();
  const BitMap *gWorldBits = lockPixelsGuard.Bits();
  CGrafPtr windowPort = Port();

#if TARGET_API_MAC_CARBON
  const BitMap *windowBits = GetPortBitMapForCopyBits(windowPort);
#else
  const BitMap *windowBits = &reinterpret_cast<GrafPtr>(windowPort)->portBits;
#endif

  QD_CHECKED(
      CopyBits(
          gWorldBits, windowBits, &gWorldRect, &windowRect, srcCopy, nullptr
      ),
      "Couldn't copy from offscreen GWorld"
  );
}

void Window::DrawControls() const { ::DrawControls(ref); }

WindowRef Window::Unmanaged() const { return ref; }

Rect Window::PortBounds() const {
  Rect bounds;
  GetWindowPortBounds(ref, &bounds);
  return bounds;
}

// TODO: what happens if this is a non-color window?
CGrafPtr Window::Port() const {
  // ReSharper disable once CppLocalVariableMayBeConst
  CGrafPtr port = GetWindowPort(ref);
  REQUIRE_NOT_NULL(port);
  return port;
}

void Window::HandleMouseDown(Point point, const WindowPartCode part) const {
  switch (part) {
    case inContent:
      if (ref == FrontWindow()) {
        // Make this window the active QD port so we can convert coordinates.
        {
          GWorldActiveGuard activeGuard = MakeActivePort();
          GlobalToLocal(&point);
        }

        // If we clicked on a control, handle that.
        ControlRef eventControlRef;
        if (const ControlPartCode controlPart =
                FindControl(point, ref, &eventControlRef)) {
          if (eventControlRef) {
            if (const auto control = reinterpret_cast<Control *>(
                    GetControlReference(eventControlRef)
                )) {
              control->HandleMouseDown(point, controlPart);
            }
          }
        } else {
          // Handle non-control clicks.
          if (onContentMouseDown) {
            onContentMouseDown(*this, point);
          }
        }
      } else {
        // If we're not the active window, become the active window.
        SelectWindow(ref);
      }

    case inDrag: {
      const Rect desktop = QD::DesktopBounds();
      DragWindow(ref, point, &desktop);
    } break;

    case inGrow:
      // TODO
      break;

    case inGoAway:
      if (TrackGoAway(ref, point)) {
        if (onClose) {
          onClose(*this);
        }
      }
      break;

    case inZoomIn:
    case inZoomOut:
      // TODO
      break;

    default:
      break;
  }
}

GWorldActiveGuard Window::MakeActivePort() const {
  return GWorldActiveGuard(Port());
}

}  // namespace AtelierEsri
