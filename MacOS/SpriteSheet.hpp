#pragma once

#include <vector>

#include <MacTypes.h>

#include "MaskedImage.hpp"
#include "Resource.hpp"

namespace AtelierEsri {

class SpriteSheet {
public:
  static SpriteSheet New(MaskedImage &&maskedImage, ResourceID rgnResourceID);

  /// Copy a sprite into a `GWorld`.
  void Draw(GWorld &gWorld, size_t spriteIndex, const Rect &dstRect);

  SpriteSheet(SpriteSheet &&src) noexcept;
  SpriteSheet &operator=(SpriteSheet &&src) noexcept;
  SpriteSheet(const SpriteSheet &src) = delete;
  SpriteSheet &operator=(const SpriteSheet &src) = delete;

private:
  explicit SpriteSheet(MaskedImage &&maskedImage, std::vector<Rect> &&regions);
  static std::vector<Rect> ReadRGN(ResourceID rgnResourceID);
  static std::vector<Rect> ReadRGN(size_t rgnLen, uint8_t *rgnPtr);

  MaskedImage maskedImage;
  std::vector<Rect> regions;
};

} // namespace AtelierEsri
