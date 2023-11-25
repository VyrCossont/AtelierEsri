#include "Drawing.hpp"

namespace AtelierEsri {

#if !TARGET_API_MAC_CARBON
std::optional<SysEnvRec> QD::sysEnvRec = {};
#endif

bool QD::HasColor() {
#if TARGET_API_MAC_CARBON
  return true;
#else
  if (!sysEnvRec.has_value()) {
    SysEnvRec newSysEnvRec;
    OS_CHECKED(SysEnvirons(1, &newSysEnvRec),
               "Couldn't check QuickDraw capabilities");
    sysEnvRec = newSysEnvRec;
  }
  return sysEnvRec->hasColorQD != 0;
#endif
}

void QD::Reset() {
  PenNormal();

  Pattern defaultBackground = QD::White();
  BackPat(&defaultBackground);
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

Ngon::Ngon(Point center, int16_t r, uint8_t n, float theta)
    : center(center), r(r), n(n), theta(theta) {}

Point Ngon::operator[](uint8_t i) const {
  float thetaI = theta + (static_cast<float>(M_TWOPI) * static_cast<float>(i)) /
                             static_cast<float>(n);
  return {
      static_cast<int16_t>(
          center.h + static_cast<int16_t>(static_cast<float>(r) * cos(thetaI))),
      static_cast<int16_t>(
          center.v +
          static_cast<int16_t>(static_cast<float>(r) * sin(thetaI)))};
}

ManagedPolygon Ngon::Polygon() const {
  PolyHandle polygon = OpenPoly();
  Point point = operator[](0);
  MoveTo(point.h, point.v);
  for (uint8_t i = 1; i <= n; i++) {
    point = operator[](i);
    LineTo(point.h, point.v);
  }
  ClosePoly();
  return ManagedPolygon(polygon);
}

} // namespace AtelierEsri
