#include "GWorld.hpp"

#include "Quickdraw.h"

namespace AtelierEsri {

GWorld::GWorld(GWorldPtr ptr) noexcept : ptr(ptr) {}

GWorld::GWorld(GWorld &&src) noexcept {
  ptr = src.ptr;
  src.ptr = nullptr;
}

GWorld &GWorld::operator=(GWorld &&src) noexcept {
  ptr = src.ptr;
  src.ptr = nullptr;
  return *this;
}

GWorld::~GWorld() noexcept {
  if (ptr) {
    DisposeGWorld(ptr);
  }
}

Result<GWorldLockPixelsGuard> GWorld::LockPixels() noexcept {
  return GWorldLockPixelsGuard::Construct(ptr);
}

GWorldActiveGuard GWorld::MakeActive() noexcept {
  return GWorldActiveGuard(ptr);
}

Rect GWorld::Bounds() noexcept {
#if TARGET_API_MAC_CARBON
  Rect bounds;
  GetPortBounds(ptr, &bounds);
  return bounds;
#else
  return ptr->portRect;
#endif
}

Result<GWorldLockPixelsGuard>
GWorldLockPixelsGuard::Construct(GWorldPtr ptr) noexcept {
  PixMapHandle hdl = GetGWorldPixMap(ptr);
  REQUIRE_NOT_NULL(hdl);

  bool locked = LockPixels(hdl);
  if (!locked) {
    BAIL("Couldn't lock pixels for offscreen GWorld");
  }

  return Ok(GWorldLockPixelsGuard(ptr, hdl));
}

GWorldLockPixelsGuard::GWorldLockPixelsGuard(GWorldPtr ptr,
                                             PixMapHandle hdl) noexcept
    : ptr(ptr), hdl(hdl) {}

GWorldLockPixelsGuard::~GWorldLockPixelsGuard() noexcept { UnlockPixels(hdl); }

const BitMap *GWorldLockPixelsGuard::Bits() noexcept {
#if TARGET_API_MAC_CARBON
  return GetPortBitMapForCopyBits(ptr);
#else
  return &((GrafPtr)ptr)->portBits;
#endif
}

GWorldActiveGuard::GWorldActiveGuard(GWorldPtr ptr) noexcept {
  GetGWorld(&this->prevPort, &this->prevDevice);
  GDHandle device = GetGWorldDevice(ptr);
  SetGWorld(ptr, device);
}

GWorldActiveGuard::~GWorldActiveGuard() noexcept {
  SetGWorld(prevPort, prevDevice);
}

} // namespace AtelierEsri
