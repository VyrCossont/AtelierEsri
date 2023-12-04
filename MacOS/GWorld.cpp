#include "GWorld.hpp"

#include "Quickdraw.h"

namespace AtelierEsri {

GWorld::GWorld(const GWorldPtr ptr) : ptr(ptr) {}

GWorld::GWorld(GWorld &&src) noexcept {
  ptr = src.ptr;
  src.ptr = nullptr;
}

GWorld &GWorld::operator=(GWorld &&src) noexcept {
  ptr = src.ptr;
  src.ptr = nullptr;
  return *this;
}

GWorld::~GWorld() {
  if (ptr) {
    DisposeGWorld(ptr);
  }
}

GWorldLockPixelsGuard GWorld::LockPixels() {
  return GWorldLockPixelsGuard::Construct(ptr);
}

GWorldActiveGuard GWorld::MakeActive() { return GWorldActiveGuard(ptr); }

Rect GWorld::Bounds() {
#if TARGET_API_MAC_CARBON
  Rect bounds;
  GetPortBounds(ptr, &bounds);
  return bounds;
#else
  return ptr->portRect;
#endif
}

GWorldLockPixelsGuard GWorldLockPixelsGuard::Construct(GWorldPtr ptr) {
  PixMapHandle hdl = GetGWorldPixMap(ptr);
  REQUIRE_NOT_NULL(hdl);

  if (!LockPixels(hdl)) {
    BAIL("Couldn't lock pixels for offscreen GWorld");
  }

  return {ptr, hdl};
}

GWorldLockPixelsGuard::GWorldLockPixelsGuard(const GWorldPtr ptr,
                                             const PixMapHandle hdl)
    : ptr(ptr), hdl(hdl) {}

GWorldLockPixelsGuard::~GWorldLockPixelsGuard() { UnlockPixels(hdl); }

const BitMap *GWorldLockPixelsGuard::Bits() const {
#if TARGET_API_MAC_CARBON
  return GetPortBitMapForCopyBits(ptr);
#else
  return &reinterpret_cast<GrafPtr>(ptr)->portBits;
#endif
}

GWorldActiveGuard::GWorldActiveGuard(GWorldPtr ptr) {
  GetGWorld(&this->prevPort, &this->prevDevice);
  // ReSharper disable once CppLocalVariableMayBeConst
  GDHandle device = GetGWorldDevice(ptr);
  SetGWorld(ptr, device);
}

GWorldActiveGuard::~GWorldActiveGuard() { SetGWorld(prevPort, prevDevice); }

} // namespace AtelierEsri
