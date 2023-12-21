#include "Window.hpp"

#include <MacTypes.h>
#include <ToolUtils.h>

#include "Control.hpp"
#include "Drawing.hpp"
#include "Exception.hpp"
#include "GWorld.hpp"

namespace AtelierEsri {

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

bool Window::GrowIcon() const { return growIcon; }

void Window::GrowIcon(const bool value) { growIcon = value; }

WindowRef Window::GetNewWindow(
    const ResourceID resourceID, const WindowRef behind
) {
  WindowRef ref;
  // Not actually identical: these are A-line traps and Clang can't parse them.
  if (QD::HasColor()) {  // NOLINT(*-branch-clone)
    ref = GetNewCWindow(resourceID, nullptr, behind);
  } else {
    ref = ::GetNewWindow(resourceID, nullptr, behind);
  }
  REQUIRE_NOT_NULL(ref);
  return ref;
}

void Window::SetRefConToThis() {
  SetWRefCon(ref, reinterpret_cast<int32_t>(this));
}

const WindowRef Window::InFrontOfAllOtherWindows =
    reinterpret_cast<WindowRef>(-1);
const WindowRef Window::BehindAllOtherWindows = nullptr;

Window::Window(const ResourceID resourceID, const WindowRef behind)
    : ref(GetNewWindow(resourceID, behind)) {
  SetRefConToThis();
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

  QD_CHECKED(
      CopyBits(
          gWorldBits,
          QD::CurrentPortBits(),
          &gWorldRect,
          &windowRect,
          srcCopy,
          nullptr
      ),
      "Couldn't copy from offscreen GWorld"
  );
}

void Window::DrawControls() const { ::DrawControls(ref); }

// ReSharper disable once CppParameterMayBeConst
void Window::UpdateControls(RgnHandle updateRegion) const {
  ::UpdateControls(ref, updateRegion);
}

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
      break;

    case inDrag: {
      const Rect desktop = QD::DesktopBounds();
      DragWindow(ref, point, &desktop);
    } break;

    case inGrow: {
      // TODO: Arbitrary limits; windows should be able to customize this.
      // This is not a real Rect, but min and max limits for each dimension:
      // https://preterhuman.net/macstuff/insidemac/Toolbox/Toolbox-250.html
      // Note that 32767 is the actual max; 65535 is wrong and causes the
      // window to vanish when resized.
      //
      // `limits` is used but `GrowWindow` is an A-line trap;
      // you know the deal.
      // ReSharper disable once CppDFAUnreadVariable
      // ReSharper disable once CppDFAUnusedValue
      constexpr Rect limits{
          .top = 64,
          .left = 64,
          .bottom = 32767,
          .right = 32767,
      };
      const R2I prevBounds = PortBounds();
      if (const int32_t newDimensions = GrowWindow(ref, point, &limits)) {
        const int16_t width = LoWord(newDimensions);
        const int16_t height = HiWord(newDimensions);
        SizeWindow(ref, width, height, true);
        if (onResize) {
          onResize(*this, prevBounds.size);
        }
      }
    } break;

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

void Window::HandleActivate() const {
  if (onActivate) {
    onActivate(*this);
  }
}

void Window::HandleDeactivate() const {
  if (onDeactivate) {
    onDeactivate(*this);
  }
}

void Window::HandleUpdate() const {
  GWorldActiveGuard activeGuard = MakeActivePort();

  BeginUpdate(ref);

  if (onUpdate) {
    onUpdate(*this);
  }

  RgnHandle visibleRegion;
#if TARGET_API_MAC_CARBON
  GetPortVisibleRegion(Port(), visibleRegion);
#else
  visibleRegion = Port()->visRgn;
#endif
  UpdateControls(visibleRegion);

  if (growIcon) {
    // Also draws disabled scroll bar tracks in document windows:
    // https://preterhuman.net/macstuff/insidemac/Toolbox/Toolbox-233.html
    DrawGrowIcon(ref);
  }

  EndUpdate(ref);
}

GWorldActiveGuard Window::MakeActivePort() const {
  return GWorldActiveGuard(Port());
}

}  // namespace AtelierEsri
