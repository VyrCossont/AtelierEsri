#include "GWorld.hpp"

#include "Quickdraw.h"

namespace AtelierEsri {

GWorld::GWorld(const V2I size) : ptr(nullptr) {
  const Rect rect = R2I({0, 0}, size);
  constexpr int16_t pixelDepth = 16;
  constexpr uint32_t flags = 0;
  OS_CHECKED(
      NewGWorld(&ptr, pixelDepth, &rect, nullptr, nullptr, flags),
      "Couldn't create GWorld"
  );
  REQUIRE_NOT_NULL(ptr);
}

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

GWorldLockPixelsGuard GWorld::LockPixels() const {
  return GWorldLockPixelsGuard::Construct(ptr);
}

GWorldActiveGuard GWorld::MakeActive() const { return GWorldActiveGuard(ptr); }

Rect GWorld::Bounds() const {
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

// ReSharper disable CppParameterMayBeConst
GWorldLockPixelsGuard::GWorldLockPixelsGuard(
    const GWorldPtr ptr, PixMapHandle hdl
)
    : ptr(ptr), hdl(hdl) {}
// ReSharper restore CppParameterMayBeConst

GWorldLockPixelsGuard::~GWorldLockPixelsGuard() { UnlockPixels(hdl); }

const BitMap *GWorldLockPixelsGuard::Bits() const {
#if TARGET_API_MAC_CARBON
  return GetPortBitMapForCopyBits(ptr);
#else
  return &reinterpret_cast<GrafPtr>(ptr)->portBits;
#endif
}

GWorldActiveGuard::GWorldActiveGuard(const GWorldPtr ptr) {
  GetGWorld(&this->prevPort, &this->prevDevice);
  // ReSharper disable once CppLocalVariableMayBeConst
  GDHandle device = GetGWorldDevice(ptr);
  SetGWorld(ptr, device);
}

GWorldActiveGuard::~GWorldActiveGuard() { SetGWorld(prevPort, prevDevice); }

}  // namespace AtelierEsri
