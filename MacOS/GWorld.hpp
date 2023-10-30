#pragma once

#include <memory>

#include <QDOffscreen.h>

#include "Result.hpp"

namespace AtelierEsri {

class GWorldLockPixelsGuard;
class GWorldActiveGuard;

class GWorld {
public:
  explicit GWorld(GWorldPtr ptr);
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
