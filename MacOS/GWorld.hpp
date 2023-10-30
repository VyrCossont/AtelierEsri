#pragma once

#include <memory>

#include <QDOffscreen.h>

#include "Result.hpp"

namespace AtelierEsri {

class GWorldLockPixelsGuard;
class GWorldActiveGuard;

class GWorld {
public:
  explicit GWorld(GWorldPtr ptr) : ptr(ptr){};
  GWorld(GWorld &&src) noexcept;
  GWorld &operator=(GWorld &&src) noexcept;
  GWorld(const GWorld &src) = delete;
  GWorld &operator=(const GWorld &src) = delete;
  ~GWorld();
  Result<GWorldLockPixelsGuard, OSErr> LockPixels();
  GWorldActiveGuard MakeActive();
  Rect Bounds();

private:
  GWorldPtr ptr;
};

class GWorldLockPixelsGuard {
public:
  static Result<GWorldLockPixelsGuard, OSErr> Construct(GWorldPtr ptr);
  ~GWorldLockPixelsGuard();
  const BitMap *Bits();

private:
  explicit GWorldLockPixelsGuard(GWorldPtr ptr, PixMapHandle hdl);
  GWorldPtr ptr;
  PixMapHandle hdl;
};

class GWorldActiveGuard {
public:
  explicit GWorldActiveGuard(GWorldPtr ptr);
  ~GWorldActiveGuard();

private:
  CGrafPtr prevPort = nil;
  GDHandle prevDevice = nil;
};

} // namespace AtelierEsri
