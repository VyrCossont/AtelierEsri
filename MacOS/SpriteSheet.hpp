#pragma once

#include <MacTypes.h>

#include <vector>

#include "MaskedImage.hpp"
#include "Resource.hpp"

namespace AtelierEsri {

/// A 9-patch sprite. All rects are in sprite sheet coordinate space.
struct NinePatch {
  NinePatch(const Rect &frameRect, const Rect &centerRect);

  // Not scaled.
  Rect patchNW;
  Rect patchNE;
  Rect patchSE;
  Rect patchSW;

  // Scaled in one direction.
  Rect patchN;
  Rect patchE;
  Rect patchS;
  Rect patchW;

  // Scaled in both directions.
  Rect patchCenter;

  /// Not a true Rect. Used to convert the destination rectangle when drawing.
  Rect insets;
};

class SpriteSheet {
 public:
  explicit SpriteSheet(
      MaskedImage &&maskedImage,
      ResourceID rgnResourceID,
      ResourceID ninepatchResourceID
  );

  /// Copy a sprite into the current graphics port.
  void Draw(size_t spriteIndex, const Rect &dstRect) const;

  /// Draw a 9-patch into the current graphics port.
  void Draw9Patch(size_t patchIndex, const Rect &dstRect) const;

  SpriteSheet(SpriteSheet &&src) noexcept;
  SpriteSheet &operator=(SpriteSheet &&src) noexcept;
  SpriteSheet(const SpriteSheet &src) = delete;
  SpriteSheet &operator=(const SpriteSheet &src) = delete;

 private:
  static std::vector<Rect> ReadRGN(ResourceID rgnResourceID);
  static std::vector<Rect> ReadRGN(size_t rgnLen, uint8_t *rgnPtr);

  static std::vector<NinePatch> Read9PC(ResourceID ninepatchResourceID);
  static std::vector<NinePatch> Read9PC(
      size_t ninepatchLen, uint8_t *ninepatchPtr
  );

  MaskedImage maskedImage;
  std::vector<Rect> regions;
  std::vector<NinePatch> patches;
};

}  // namespace AtelierEsri
