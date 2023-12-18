#include "Drawing.hpp"

#include <MacWindows.h>

#include <ctgmath>

namespace AtelierEsri {

#if !TARGET_API_MAC_CARBON
std::optional<SysEnvRec> QD::sysEnvRec = {};
#endif

V2I::V2I(const Point point) : V2(point.h, point.v) {}

V2I::operator Point() const {
  return {
      .v = static_cast<int16_t>(y),
      .h = static_cast<int16_t>(x),
  };
}

R2I R2I::Around(const V2I center, const Element halfWidth) {
  return Around(center, halfWidth, halfWidth);
}

R2I R2I::Around(
    const V2I center, const Element halfWidth, const Element halfHeight
) {
  return {
      center - V2I{halfWidth, halfHeight},
      {2 * halfWidth, 2 * halfHeight},
  };
}

R2I::R2I(const Rect rect) : R2(rect.left, rect.top, rect.right, rect.bottom) {}

R2I::operator Rect() const {
  return {
      .top = static_cast<int16_t>(Top()),
      .left = static_cast<int16_t>(Left()),
      .bottom = static_cast<int16_t>(Bottom()),
      .right = static_cast<int16_t>(Right()),
  };
}

bool QD::HasColor() {
#if TARGET_API_MAC_CARBON
  return true;
#else
  if (!sysEnvRec.has_value()) {
    SysEnvRec newSysEnvRec;
    OS_CHECKED(
        SysEnvirons(1, &newSysEnvRec), "Couldn't check QuickDraw capabilities"
    );
    sysEnvRec = newSysEnvRec;
  }
  return sysEnvRec->hasColorQD != 0;
#endif
}

void QD::Reset() {
  PenNormal();

  const Pattern defaultBackground = White();
  BackPat(&defaultBackground);
}

Rect QD::DesktopBounds() {
  RgnHandle grayRegion = GetGrayRgn();
#if TARGET_API_MAC_CARBON
  Rect bounds;
  GetRegionBounds(grayRegion, &bounds);
  return bounds;
#else
  return grayRegion[0]->rgnBBox;
#endif
}

const BitMap* QD::CurrentPortBits() {
  GWorldPtr port;
  GDHandle device;
  GetGWorld(&port, &device);
#if TARGET_API_MAC_CARBON
  return GetPortBitMapForCopyBits(port);
#else
  return &reinterpret_cast<GrafPtr>(port)->portBits;
#endif
}

Pattern QD::Black() {
#if TARGET_API_MAC_CARBON
  Pattern pattern;
  GetQDGlobalsBlack(&pattern);
  return pattern;
#else
  return qd.black;
#endif
}

Pattern QD::DarkGray() {
#if TARGET_API_MAC_CARBON
  Pattern pattern;
  GetQDGlobalsDarkGray(&pattern);
  return pattern;
#else
  return qd.dkGray;
#endif
}

Pattern QD::Gray() {
#if TARGET_API_MAC_CARBON
  Pattern pattern;
  GetQDGlobalsGray(&pattern);
  return pattern;
#else
  return qd.gray;
#endif
}

Pattern QD::LightGray() {
#if TARGET_API_MAC_CARBON
  Pattern pattern;
  GetQDGlobalsLightGray(&pattern);
  return pattern;
#else
  return qd.ltGray;
#endif
}

Pattern QD::White() {
#if TARGET_API_MAC_CARBON
  Pattern pattern;
  GetQDGlobalsWhite(&pattern);
  return pattern;
#else
  return qd.white;
#endif
}

void QD::MoveTo(const V2I& point) {
  ::MoveTo(static_cast<int16_t>(point.x), static_cast<int16_t>(point.y));
}

void QD::LineTo(const V2I& point) {
  ::LineTo(static_cast<int16_t>(point.x), static_cast<int16_t>(point.y));
}

void QD::Erase(const R2I& rect) {
  const Rect qdRect(rect);
  EraseRect(&qdRect);
}

Ngon::Ngon(
    const V2I center,
    const int r,
    const int n,
    const float theta,
    const bool reverse
)
    : center(center), r(r), n(n), theta(theta), reverse(reverse) {}

V2I Ngon::operator[](const int i) const {
  const float thetaI =
      theta + (reverse ? -1.0f : 1.0f) *
                  (static_cast<float>(M_TWOPI) * static_cast<float>(i)) /
                  static_cast<float>(n);
  return center + V2I{
                      static_cast<int>(static_cast<float>(r) * cos(thetaI)),
                      static_cast<int>(static_cast<float>(r) * sin(thetaI)),
                  };
}

ManagedPolygon Ngon::Polygon() const {
  // ReSharper disable once CppLocalVariableMayBeConst
  PolyHandle polygon = OpenPoly();
  V2I point = operator[](0);
  QD::MoveTo(point);
  for (uint8_t i = 1; i <= n; i++) {
    point = operator[](i);
    QD::LineTo(point);
  }
  ClosePoly();
  return ManagedPolygon(polygon);
}

}  // namespace AtelierEsri
