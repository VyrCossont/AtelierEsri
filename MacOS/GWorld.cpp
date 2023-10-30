#include "GWorld.hpp"

#include "Quickdraw.h"

#include "Debug.hpp"

namespace AtelierEsri {

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

Result<GWorldLockPixelsGuard, OSErr> GWorld::LockPixels() {
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

Result<GWorldLockPixelsGuard, OSErr>
GWorldLockPixelsGuard::Construct(GWorldPtr ptr) {
  Debug::Printfln("GWorld ptr = %#08x", ptr);

  PixMapHandle hdl = GetGWorldPixMap(ptr);

  Debug::Printfln("GWorld hdl = %#08x", hdl);
  if (hdl == nil) {
    return Err((OSErr)__LINE__);
  }
  bool locked = LockPixels(hdl);
  if (!locked) {
    return Err((OSErr)__LINE__);
  }
  return Ok(GWorldLockPixelsGuard(ptr, hdl));
}

GWorldLockPixelsGuard::GWorldLockPixelsGuard(GWorldPtr ptr, PixMapHandle hdl) {
  this->ptr = ptr;
  this->hdl = hdl;
}

GWorldLockPixelsGuard::~GWorldLockPixelsGuard() { UnlockPixels(hdl); }

const BitMap *GWorldLockPixelsGuard::Bits() {
#if TARGET_API_MAC_CARBON
  return GetPortBitMapForCopyBits(ptr);
#else
  return &((GrafPtr)ptr)->portBits;
#endif
}

GWorldActiveGuard::GWorldActiveGuard(GWorldPtr ptr) {
  GetGWorld(&this->prevPort, &this->prevDevice);
  GDHandle device = GetGWorldDevice(ptr);
  SetGWorld(ptr, device);
}

GWorldActiveGuard::~GWorldActiveGuard() { SetGWorld(prevPort, prevDevice); }

} // namespace AtelierEsri
