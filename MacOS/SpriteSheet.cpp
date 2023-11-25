#include "SpriteSheet.hpp"

namespace AtelierEsri {

Result<SpriteSheet> SpriteSheet::Get(MaskedImage &&maskedImage,
                                     ResourceID rgnResourceID) noexcept {
  GUARD_LET_TRY(std::vector<Rect>, regions, ReadRGN(rgnResourceID));
  return Ok(SpriteSheet(std::move(maskedImage), std::move(regions)));
}

SpriteSheet::SpriteSheet(MaskedImage &&maskedImage,
                         std::vector<Rect> &&regions) noexcept
    : maskedImage(std::move(maskedImage)), regions(std::move(regions)) {}

SpriteSheet::SpriteSheet(SpriteSheet &&src) noexcept
    : maskedImage(std::move(src.maskedImage)), regions(std::move(src.regions)) {
}

SpriteSheet &SpriteSheet::operator=(SpriteSheet &&src) noexcept {
  this->maskedImage = std::move(src.maskedImage);
  this->regions = std::move(src.regions);
  return *this;
}

Result<std::vector<Rect>>
SpriteSheet::ReadRGN(ResourceID rgnResourceID) noexcept {
  GUARD_LET_TRY(RGNResource, rgnResource, RGNResource::Get(rgnResourceID));
  size_t rgnLen = RES_CHECKED(GetMaxResourceSize(rgnResource.Unmanaged()),
                              "Couldn't get RGN# resource size");
  return ReadRGN(rgnLen, reinterpret_cast<uint8_t *>(*rgnResource.Unmanaged()));
}

Result<std::vector<Rect>> SpriteSheet::ReadRGN(size_t rgnLen,
                                               uint8_t *rgnPtr) noexcept {
  if (rgnLen < sizeof(uint16_t)) {
    BAIL("RGN# resource too small");
  }

  std::vector<Rect> regions{};
  uint16_t count = *reinterpret_cast<uint16_t *>(rgnPtr);
  regions.reserve(count);

  uint8_t *rgnBytesEnd = rgnPtr + rgnLen;
  rgnPtr += sizeof(uint16_t);
  while (count > 0) {
    if (rgnPtr >= rgnBytesEnd) {
      BAIL("Read past end of RGN#");
    }

    // Skip region name.
    uint8_t pstrLen = *rgnPtr;
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

  return Ok(regions);
}

Result<Unit> SpriteSheet::Draw(GWorld &gWorld, size_t spriteIndex,
                               const Rect &dstRect) noexcept {
  if (spriteIndex >= regions.size()) {
    BAIL("Invalid sprite index");
  }
  Rect srcRect = regions[spriteIndex];
  return maskedImage.Draw(gWorld, srcRect, dstRect);
}

} // namespace AtelierEsri
