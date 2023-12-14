#include "SpriteSheet.hpp"

namespace AtelierEsri {

SpriteSheet::SpriteSheet(
    MaskedImage &&maskedImage, const ResourceID rgnResourceID
)
    : maskedImage(std::move(maskedImage)), regions(ReadRGN(rgnResourceID)) {}

SpriteSheet::SpriteSheet(SpriteSheet &&src) noexcept
    : maskedImage(std::move(src.maskedImage)),
      regions(std::move(src.regions)) {}

SpriteSheet &SpriteSheet::operator=(SpriteSheet &&src) noexcept {
  this->maskedImage = std::move(src.maskedImage);
  this->regions = std::move(src.regions);
  return *this;
}

std::vector<Rect> SpriteSheet::ReadRGN(ResourceID rgnResourceID) {
  RGNResource rgnResource = RGNResource::Get(rgnResourceID);
  const size_t rgnLen = RES_CHECKED(
      GetMaxResourceSize(rgnResource.Unmanaged()),
      "Couldn't get RGN# resource size"
  );
  return ReadRGN(rgnLen, reinterpret_cast<uint8_t *>(*rgnResource.Unmanaged()));
}

std::vector<Rect> SpriteSheet::ReadRGN(size_t rgnLen, uint8_t *rgnPtr) {
  if (rgnLen < sizeof(uint16_t)) {
    BAIL("RGN# resource too small");
  }

  std::vector<Rect> regions{};
  uint16_t count = *reinterpret_cast<uint16_t *>(rgnPtr);
  regions.reserve(count);

  const uint8_t *rgnBytesEnd = rgnPtr + rgnLen;
  rgnPtr += sizeof(uint16_t);
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

void SpriteSheet::Draw(
    const GWorld &gWorld, const size_t spriteIndex, const Rect &dstRect
) const {
  if (spriteIndex >= regions.size()) {
    BAIL("Invalid sprite index");
  }
  const Rect srcRect = regions[spriteIndex];
  return maskedImage.Draw(gWorld, srcRect, dstRect);
}

}  // namespace AtelierEsri
