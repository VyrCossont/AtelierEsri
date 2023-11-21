#include "Drawing.hpp"

namespace AtelierEsri {

#if !TARGET_API_MAC_CARBON
std::optional<SysEnvRec> QD::sysEnvRec = {};
#endif

Result<bool> QD::HasColor() noexcept {
#if TARGET_API_MAC_CARBON
  return Ok(true);
#else
  if (!sysEnvRec.has_value()) {
    SysEnvRec newSysEnvRec;
    OS_CHECKED(SysEnvirons(1, &newSysEnvRec),
               "Couldn't check QuickDraw capabilities");
    sysEnvRec = newSysEnvRec;
  }
  return Ok(sysEnvRec->hasColorQD != 0);
#endif
}

void QD::Reset() noexcept {
  PenNormal();

  Pattern defaultBackground = QD::White();
  BackPat(&defaultBackground);
}

Pattern QD::Black() noexcept {
#if TARGET_API_MAC_CARBON
  Pattern pattern;
  GetQDGlobalsBlack(&pattern);
  return pattern;
#else
  return qd.black;
#endif
}

Pattern QD::DarkGray() noexcept {
#if TARGET_API_MAC_CARBON
  Pattern pattern;
  GetQDGlobalsDarkGray(&pattern);
  return pattern;
#else
  return qd.dkGray;
#endif
}

Pattern QD::Gray() noexcept {
#if TARGET_API_MAC_CARBON
  Pattern pattern;
  GetQDGlobalsGray(&pattern);
  return pattern;
#else
  return qd.gray;
#endif
}

Pattern QD::LightGray() noexcept {
#if TARGET_API_MAC_CARBON
  Pattern pattern;
  GetQDGlobalsLightGray(&pattern);
  return pattern;
#else
  return qd.ltGray;
#endif
}

Pattern QD::White() noexcept {
#if TARGET_API_MAC_CARBON
  Pattern pattern;
  GetQDGlobalsWhite(&pattern);
  return pattern;
#else
  return qd.white;
#endif
}

Ngon::Ngon(V2I center, int16_t r, uint8_t n, float theta) noexcept
    : center(center), r(r), n(n), theta(theta) {}

V2I Ngon::operator[](uint8_t i) const noexcept {
  float thetaI = theta + (static_cast<float>(M_TWOPI) * static_cast<float>(i)) /
                             static_cast<float>(n);
  return {
      static_cast<int16_t>(
          center.x + static_cast<int16_t>(static_cast<float>(r) * cos(thetaI))),
      static_cast<int16_t>(
          center.y +
          static_cast<int16_t>(static_cast<float>(r) * sin(thetaI)))};
}

ManagedPolygon Ngon::Polygon() const noexcept {
  PolyHandle polygon = OpenPoly();
  V2I point = operator[](0);
  MoveTo(point.x, point.y);
  for (uint8_t i = 1; i <= n; i++) {
    point = operator[](i);
    LineTo(point.x, point.y);
  }
  ClosePoly();
  return ManagedPolygon(polygon);
}

} // namespace AtelierEsri
