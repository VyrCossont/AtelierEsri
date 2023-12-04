#pragma once

#include <memory>

#include <QDOffscreen.h>

#include "Exception.hpp"

namespace AtelierEsri {

class GWorldLockPixelsGuard;
class GWorldActiveGuard;

/// An offscreen GWorld.
class GWorld {
public:
  explicit GWorld(GWorldPtr ptr);
  GWorld(GWorld &&src) noexcept;
  GWorld &operator=(GWorld &&src) noexcept;
  GWorld(const GWorld &src) = delete;
  GWorld &operator=(const GWorld &src) = delete;
  ~GWorld();
  GWorldLockPixelsGuard LockPixels();
  GWorldActiveGuard MakeActive();
  Rect Bounds();

private:
  GWorldPtr ptr;
};

/// Guard object that locks the GWorld's pixels into memory.
class GWorldLockPixelsGuard {
public:
  static GWorldLockPixelsGuard Construct(GWorldPtr ptr);
  ~GWorldLockPixelsGuard();
  /// Get the GWorld's bits. Should be used *only* with `CopyBits`, etc.
  [[nodiscard]] const BitMap *Bits() const;

private:
  GWorldLockPixelsGuard(GWorldPtr ptr, PixMapHandle hdl);
  GWorldPtr ptr;
  PixMapHandle hdl;
};

/// Guard object that makes the GWorld active for drawing operations.
class GWorldActiveGuard {
public:
  explicit GWorldActiveGuard(GWorldPtr ptr);
  ~GWorldActiveGuard();

private:
  CGrafPtr prevPort = nullptr;
  GDHandle prevDevice = nullptr;
};

} // namespace AtelierEsri
