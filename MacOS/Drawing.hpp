#pragma once

#include <MacTypes.h>
#include <OSUtils.h>
#include <QDOffscreen.h>
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

  constexpr V2I(const int x, const int y) : V2(x, y){};

  // ReSharper disable once CppNonExplicitConvertingConstructor
  V2I(Point point);  // NOLINT(*-explicit-constructor)

  // ReSharper disable once CppNonExplicitConversionOperator
  operator Point() const;  // NOLINT(*-explicit-constructor)
};

/// 2D int rectangle interoperable with QuickDraw `Rect`.
struct R2I : Breeze::R2<R2I, V2I> {
  constexpr R2I(const V2I origin, const V2I size) : R2(origin, size){};

  R2I(Element left, Element top, Element right, Element bottom);

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

  // TODO: rename this to something more specific
  /// Reset graphics port to default color and pattern.
  /// Doesn't affect the clipping region or origin translation.
  static void Reset();

  /// Get the bounds of the entire desktop, aka the "gray region".
  static Rect DesktopBounds();

  /// Get the port's bits. Should be used *only* with `CopyBits`, etc.
  static const BitMap* CurrentPortBits();

  static Pattern Black();
  static Pattern DarkGray();
  static Pattern Gray();
  static Pattern LightGray();
  static Pattern White();

  static void MoveTo(const V2I& point);
  static void LineTo(const V2I& point);

  static void Erase(const R2I& rect);

  static int TextWidth(const std::string& text);
  static void DrawText(const std::string& text);

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

/// RAII wrapper for changing the origin coordinates of the current port.
class ChangeOrigin {
 public:
  explicit ChangeOrigin(const V2I& translation);
  ChangeOrigin(const ChangeOrigin& other) = delete;
  ChangeOrigin& operator=(const ChangeOrigin& other) = delete;
  ~ChangeOrigin();
};

}  // namespace AtelierEsri
