#pragma once

#include <MacTypes.h>

#include <vector>

#include "MaskedImage.hpp"
#include "Resource.hpp"

namespace AtelierEsri {

class SpriteSheet {
 public:
  explicit SpriteSheet(MaskedImage &&maskedImage, ResourceID rgnResourceID);

  /// Copy a sprite into the current graphics port.
  void Draw(size_t spriteIndex, const Rect &dstRect) const;

  SpriteSheet(SpriteSheet &&src) noexcept;
  SpriteSheet &operator=(SpriteSheet &&src) noexcept;
  SpriteSheet(const SpriteSheet &src) = delete;
  SpriteSheet &operator=(const SpriteSheet &src) = delete;

 private:
  static std::vector<Rect> ReadRGN(ResourceID rgnResourceID);
  static std::vector<Rect> ReadRGN(size_t rgnLen, uint8_t *rgnPtr);

  MaskedImage maskedImage;
  std::vector<Rect> regions;
};

}  // namespace AtelierEsri
