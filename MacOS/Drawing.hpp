#pragma once

#include <MacTypes.h>
#include <OSUtils.h>
#include <Quickdraw.h>

#include <memory>
#include <optional>

#include "Breeze/Geometry/R2.hpp"
#include "Breeze/Geometry/V2.hpp"
#include "Resource.hpp"

namespace AtelierEsri {

/// 2D int vector interoperable with QuickDraw `Point`.
struct V2I : Breeze::V2<V2I, int> {
  using Element = int;

  V2I(int x, int y);

  // ReSharper disable once CppNonExplicitConvertingConstructor
  V2I(Point point);  // NOLINT(*-explicit-constructor)

  // ReSharper disable once CppNonExplicitConversionOperator
  operator Point() const;  // NOLINT(*-explicit-constructor)
};

/// 2D int rectangle interoperable with QuickDraw `Rect`.
struct R2I : Breeze::R2<V2I> {
  R2I(V2I origin, V2I size);
  R2I(int top, int left, int bottom, int right);

  // ReSharper disable once CppNonExplicitConvertingConstructor
  R2I(Rect rect);  // NOLINT(*-explicit-constructor)

  // ReSharper disable once CppNonExplicitConversionOperator
  operator Rect() const;  // NOLINT(*-explicit-constructor)
};

/// QuickDraw global state.
class QD {
 public:
  /// Return whether Color QuickDraw is available.
  static bool HasColor();

  /// Reset graphics port to defaults.
  static void Reset();

  /// Get the bounds of the entire desktop, aka the "gray region".
  static Rect DesktopBounds();

  static Pattern Black();
  static Pattern DarkGray();
  static Pattern Gray();
  static Pattern LightGray();
  static Pattern White();

  static void MoveTo(V2I point);
  static void LineTo(V2I point);

 private:
#if !TARGET_API_MAC_CARBON
  static std::optional<SysEnvRec> sysEnvRec;
#endif
};

using ManagedPolygon =
    std::unique_ptr<PolyPtr, ResourceReleaser<PolyHandle, KillPoly>>;

#if TARGET_API_MAC_CARBON
using ManagedRegion = std::
    unique_ptr<struct OpaqueRgnHandle, ResourceReleaser<RgnHandle, DisposeRgn>>;
#else
using ManagedRegion =
    std::unique_ptr<RgnPtr, ResourceReleaser<RgnHandle, DisposeRgn>>;
#endif

class Ngon {
 public:
  Ngon(V2I center, int r, int n, float theta = 0, bool reverse = false);

  V2I operator[](int i) const;

  [[nodiscard]] ManagedPolygon Polygon() const;

 private:
  V2I center;
  int r;
  int n;
  float theta;
  bool reverse;
};

}  // namespace AtelierEsri
