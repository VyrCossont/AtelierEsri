#pragma once

#include <cstdint>
#include <memory>
#include <optional>

#include <MacTypes.h>
#include <OSUtils.h>
#include <Quickdraw.h>

#include "Resource.hpp"

namespace AtelierEsri {

/// QuickDraw global state.
class QD {
public:
  /// Return whether Color QuickDraw is available.
  static bool HasColor();

  /// Reset graphics port to defaults.
  static void Reset();

  static Pattern Black();
  static Pattern DarkGray();
  static Pattern Gray();
  static Pattern LightGray();
  static Pattern White();

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

class Ngon {
public:
  Ngon(Point center, int16_t r, uint8_t n, float theta);

  Point operator[](uint8_t i) const;

  [[nodiscard]] ManagedPolygon Polygon() const;

private:
  Point center;
  int16_t r;
  uint8_t n;
  float theta;
};

} // namespace AtelierEsri
