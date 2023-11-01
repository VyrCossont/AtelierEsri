#pragma once

#include <memory>

#include <QDOffscreen.h>

#include "Result.hpp"

namespace AtelierEsri {

class GWorldLockPixelsGuard;
class GWorldActiveGuard;

/// An offscreen GWorld.
class GWorld {
public:
  explicit GWorld(GWorldPtr ptr) noexcept;
  GWorld(GWorld &&src) noexcept;
  GWorld &operator=(GWorld &&src) noexcept;
  GWorld(const GWorld &src) = delete;
  GWorld &operator=(const GWorld &src) = delete;
  ~GWorld() noexcept;
  Result<GWorldLockPixelsGuard> LockPixels() noexcept;
  GWorldActiveGuard MakeActive() noexcept;
  Rect Bounds() noexcept;

private:
  GWorldPtr ptr;
};

/// Guard object that locks the GWorld's pixels into memory.
class GWorldLockPixelsGuard {
public:
  static Result<GWorldLockPixelsGuard> Construct(GWorldPtr ptr) noexcept;
  ~GWorldLockPixelsGuard() noexcept;
  /// Get the GWorld's bits. Should be used *only* with `CopyBits`, etc.
  const BitMap *Bits() noexcept;

private:
  GWorldLockPixelsGuard(GWorldPtr ptr, PixMapHandle hdl) noexcept;
  GWorldPtr ptr;
  PixMapHandle hdl;
};

/// Guard object that makes the GWorld active for drawing operations.
class GWorldActiveGuard {
public:
  explicit GWorldActiveGuard(GWorldPtr ptr) noexcept;
  ~GWorldActiveGuard() noexcept;

private:
  CGrafPtr prevPort = nil;
  GDHandle prevDevice = nil;
};

} // namespace AtelierEsri
