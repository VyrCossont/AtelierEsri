#include "SpriteSheet.hpp"

namespace AtelierEsri {

NinePatch::NinePatch(const Rect &frameRect, const Rect &centerRect)
    : patchNW(),
      patchNE(),
      patchSE(),
      patchSW(),
      patchN(),
      patchE(),
      patchS(),
      patchW(),
      patchCenter(),
      insets() {
  const R2I frameR = frameRect;
  R2I centerR = centerRect;
  centerR.origin += frameR.origin;
  patchCenter = centerR;

  const auto patchNWR = R2I::FromCorners(frameR.NW(), centerR.NW());
  patchNW = patchNWR;
  const auto patchSWR = R2I::FromCorners(frameR.SW(), centerR.SW());
  patchSW = patchSWR;
  const auto patchSER = R2I::FromCorners(frameR.SE(), centerR.SE());
  patchSE = patchSER;
  const auto patchNER = R2I::FromCorners(frameR.NE(), centerR.NE());
  patchNE = patchNER;

  patchN = R2I::FromCorners(patchNWR.NE(), patchNER.SW());
  patchE = R2I::FromCorners(patchNER.SE(), patchSER.NW());
  patchS = R2I::FromCorners(patchSWR.NE(), patchSER.SW());
  patchW = R2I::FromCorners(patchNWR.SE(), patchSWR.NW());

  insets = {
      centerRect.top,
      centerRect.left,
      static_cast<int16_t>(centerRect.bottom - frameR.Width()),
      static_cast<int16_t>(centerRect.right - frameR.Height())
  };
}

SpriteSheet::SpriteSheet(
    MaskedImage &&maskedImage,
    const ResourceID rgnResourceID,
    const ResourceID ninepatchResourceID
)
    : maskedImage(std::move(maskedImage)),
      regions(ReadRGN(rgnResourceID)),
      patches(Read9PC(ninepatchResourceID)) {}

SpriteSheet::SpriteSheet(SpriteSheet &&src) noexcept
    : maskedImage(std::move(src.maskedImage)),
      regions(std::move(src.regions)) {}

SpriteSheet &SpriteSheet::operator=(SpriteSheet &&src) noexcept {
  this->maskedImage = std::move(src.maskedImage);
  this->regions = std::move(src.regions);
  return *this;
}

std::vector<Rect> SpriteSheet::ReadRGN(const ResourceID rgnResourceID) {
  RGNResource rgnResource = RGNResource::Get(rgnResourceID);
  const size_t rgnLen = RES_CHECKED(
      GetMaxResourceSize(rgnResource.Unmanaged()),
      "Couldn't get RGN# resource size"
  );
  return ReadRGN(rgnLen, reinterpret_cast<uint8_t *>(*rgnResource.Unmanaged()));
}

std::vector<Rect> SpriteSheet::ReadRGN(const size_t rgnLen, uint8_t *rgnPtr) {
  if (rgnLen < sizeof(uint16_t)) {
    BAIL("RGN# resource too small");
  }

  uint16_t count = *reinterpret_cast<uint16_t *>(rgnPtr);
  rgnPtr += sizeof(uint16_t);

  std::vector<Rect> regions{};
  regions.reserve(count);

  const uint8_t *rgnBytesEnd = rgnPtr + rgnLen;
  while (count > 0) {
    if (rgnPtr >= rgnBytesEnd) {
      BAIL("Read past end of RGN#");
    }

    // Skip region name.
    const uint8_t pstrLen = *rgnPtr;
    rgnPtr += 1 + pstrLen;

    // Align to word boundary.
    if (reinterpret_cast<int32_t>(rgnPtr) & 1) {
      rgnPtr++;
    }

    // Read rect.
    if (rgnPtr + sizeof(Rect) > rgnBytesEnd) {
      BAIL("Read past end of RGN#");
    }
    regions.push_back(*reinterpret_cast<Rect *>(rgnPtr));
    rgnPtr += sizeof(Rect);

    count--;
  }

  return regions;
}

std::vector<NinePatch> SpriteSheet::Read9PC(const ResourceID ninepatchResourceID
) {
  NinePatchResource ninepatchResource =
      NinePatchResource::Get(ninepatchResourceID);
  const size_t ninepatchLen = RES_CHECKED(
      GetMaxResourceSize(ninepatchResource.Unmanaged()),
      "Couldn't get 9PC# resource size"
  );
  return Read9PC(
      ninepatchLen, reinterpret_cast<uint8_t *>(*ninepatchResource.Unmanaged())
  );
}

std::vector<NinePatch> SpriteSheet::Read9PC(
    const size_t ninepatchLen, uint8_t *ninepatchPtr
) {
  if (ninepatchLen < sizeof(uint16_t)) {
    BAIL("9PC# resource too small");
  }

  uint16_t count = *reinterpret_cast<uint16_t *>(ninepatchPtr);
  ninepatchPtr += sizeof(uint16_t);

  std::vector<NinePatch> patches{};
  patches.reserve(count);

  const uint8_t *ninepatchBytesEnd = ninepatchPtr + ninepatchLen;
  while (count > 0) {
    if (ninepatchPtr >= ninepatchBytesEnd) {
      BAIL("Read past end of 9PC#");
    }

    // Skip patch name.
    const uint8_t pstrLen = *ninepatchPtr;
    ninepatchPtr += 1 + pstrLen;

    // Align to word boundary.
    if (reinterpret_cast<int32_t>(ninepatchPtr) & 1) {
      ninepatchPtr++;
    }

    // Read frame rect.
    if (ninepatchPtr + sizeof(Rect) > ninepatchBytesEnd) {
      BAIL("Read past end of 9PC#");
    }
    const Rect frame = *reinterpret_cast<Rect *>(ninepatchPtr);
    ninepatchPtr += sizeof(Rect);

    // Read center rect.
    if (ninepatchPtr + sizeof(Rect) > ninepatchBytesEnd) {
      BAIL("Read past end of 9PC#");
    }
    const Rect center = *reinterpret_cast<Rect *>(ninepatchPtr);
    ninepatchPtr += sizeof(Rect);

    patches.emplace_back(frame, center);

    count--;
  }

  return patches;
}

void SpriteSheet::Draw(const size_t spriteIndex, const Rect &dstRect) const {
  if (spriteIndex >= regions.size()) {
    BAIL("Invalid sprite index");
  }
  const Rect srcRect = regions[spriteIndex];
  maskedImage.Draw(srcRect, dstRect);
}

void SpriteSheet::Draw9Patch(const size_t patchIndex, const Rect &dstRect)
    const {
  if (patchIndex >= patches.size()) {
    BAIL("Invalid patch index");
  }
  const auto
      [patchNW,
       patchNE,
       patchSE,
       patchSW,
       patchN,
       patchE,
       patchS,
       patchW,
       patchCenter,
       insets] = patches[patchIndex];

  // Calculate corresponding patches in destination rectangle.
  Rect dstRectCenter = dstRect;
  dstRectCenter.top = static_cast<int16_t>(insets.top);
  dstRectCenter.left = static_cast<int16_t>(insets.left);
  dstRectCenter.bottom =
      static_cast<int16_t>(dstRect.bottom - dstRect.top + insets.bottom);
  dstRectCenter.right =
      static_cast<int16_t>(dstRect.right - dstRect.left + insets.right);
  const auto
      [dstNW,
       dstNE,
       dstSE,
       dstSW,
       dstN,
       dstE,
       dstS,
       dstW,
       dstCenter,
       // ReSharper disable once CppDeclaratorNeverUsed
       dstInset] = NinePatch(dstRect, dstRectCenter);

  // Draw pieces of patch.

  maskedImage.Draw(patchNW, dstNW);
  maskedImage.Draw(patchNE, dstNE);
  maskedImage.Draw(patchSE, dstSE);
  maskedImage.Draw(patchSW, dstSW);
  maskedImage.Draw(patchN, dstN);
  maskedImage.Draw(patchE, dstE);
  maskedImage.Draw(patchS, dstS);
  maskedImage.Draw(patchW, dstW);
  maskedImage.Draw(patchCenter, dstCenter);
}

}  // namespace AtelierEsri
