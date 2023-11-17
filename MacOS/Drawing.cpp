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

ManagedPolygon Ngon(int16_t x, int16_t y, int16_t r, uint8_t n,
                    float theta) noexcept {
  PolyHandle polygon = OpenPoly();
  MoveTo(static_cast<int16_t>(
             x + static_cast<int16_t>(static_cast<float>(r) * cos(theta))),
         static_cast<int16_t>(
             y + static_cast<int16_t>(static_cast<float>(r) * sin(theta))));
  for (uint8_t i = 1; i <= n; i++) {
    float thetaI =
        theta + (static_cast<float>(M_TWOPI) * static_cast<float>(i)) /
                    static_cast<float>(n);
    LineTo(static_cast<int16_t>(
               x + static_cast<int16_t>(static_cast<float>(r) * cos(thetaI))),
           static_cast<int16_t>(
               y + static_cast<int16_t>(static_cast<float>(r) * sin(thetaI))));
  }
  ClosePoly();
  return ManagedPolygon(polygon);
}

} // namespace AtelierEsri
