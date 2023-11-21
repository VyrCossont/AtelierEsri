#pragma once

#include <cstdint>
#include <memory>
#include <optional>

#include <MacTypes.h>
#include <OSUtils.h>
#include <Quickdraw.h>

#include "Resource.hpp"
#include "Result.hpp"

namespace AtelierEsri {

/// QuickDraw global state.
class QD {
public:
  /// Return whether Color QuickDraw is available.
  static Result<bool> HasColor() noexcept;

  /// Reset graphics port to defaults.
  static void Reset() noexcept;

  static Pattern Black() noexcept;
  static Pattern DarkGray() noexcept;
  static Pattern Gray() noexcept;
  static Pattern LightGray() noexcept;
  static Pattern White() noexcept;

private:
#if !TARGET_API_MAC_CARBON
  static std::optional<SysEnvRec> sysEnvRec;
#endif
};

using ManagedPolygon =
    std::unique_ptr<PolyPtr, ResourceReleaser<PolyHandle, KillPoly>>;

#if TARGET_API_MAC_CARBON
using ManagedRegion = std::unique_ptr<struct OpaqueRgnHandle,
                                      ResourceReleaser<RgnHandle, DisposeRgn>>;
#else
using ManagedRegion =
    std::unique_ptr<RgnPtr, ResourceReleaser<RgnHandle, DisposeRgn>>;
#endif

struct V2I {
  int16_t x;
  int16_t y;
};

class Ngon {
public:
  Ngon(V2I center, int16_t r, uint8_t n, float theta) noexcept;

  V2I operator[](uint8_t i) const noexcept;

  ManagedPolygon Polygon() const noexcept;

private:
  V2I center;
  int16_t r;
  uint8_t n;
  float theta;
};

} // namespace AtelierEsri
