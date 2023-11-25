#pragma once

#include <vector>

#include <MacTypes.h>

#include "MaskedImage.hpp"
#include "Resource.hpp"
#include "Result.hpp"

namespace AtelierEsri {

class SpriteSheet {
public:
  static Result<SpriteSheet> Get(MaskedImage &&maskedImage,
                                 ResourceID rgnResourceID) noexcept;

  /// Copy a sprite into a `GWorld`.
  Result<Unit> Draw(GWorld &gWorld, size_t spriteIndex,
                    const Rect &dstRect) noexcept;

  SpriteSheet(SpriteSheet &&src) noexcept;
  SpriteSheet &operator=(SpriteSheet &&src) noexcept;
  SpriteSheet(const SpriteSheet &src) = delete;
  SpriteSheet &operator=(const SpriteSheet &src) = delete;

private:
  explicit SpriteSheet(MaskedImage &&maskedImage,
                       std::vector<Rect> &&regions) noexcept;
  static Result<std::vector<Rect>> ReadRGN(ResourceID rgnResourceID) noexcept;
  static Result<std::vector<Rect>> ReadRGN(size_t rgnLen,
                                           uint8_t *rgnPtr) noexcept;

  MaskedImage maskedImage;
  std::vector<Rect> regions;
};

} // namespace AtelierEsri
