#pragma once

#include <Quickdraw.h>

#include "GWorld.hpp"
#include "Resource.hpp"
#include "Result.hpp"
#include "Window.hpp"

namespace AtelierEsri {

/// `PICT` resource with typed operations.
class Picture {
public:
  static Result<Picture> Get(ResourceID resourceID) noexcept;
  Result<Rect> Bounds() noexcept;
  Result<Unit> Draw(const Rect &rect) noexcept;

  Picture(Picture &&src) noexcept;
  Picture &operator=(Picture &&src) noexcept;
  Picture(const Picture &src) = delete;
  Picture &operator=(const Picture &src) = delete;

private:
  explicit Picture(PICTResource &&resource) noexcept;
  PICTResource resource;
};

class MaskedImage {
public:
  static Result<MaskedImage>
  Get(int16_t imageResourceID, int16_t maskResourceID, Window &window) noexcept;
  Rect Bounds() noexcept;
  /// Copy the masked image into a `GWorld`.
  Result<Unit> Draw(GWorld &gWorld, const Rect &srcRect,
                    const Rect &dstRect) noexcept;

  MaskedImage(MaskedImage &&src) noexcept;
  MaskedImage &operator=(MaskedImage &&src) noexcept;
  MaskedImage(const MaskedImage &src) = delete;
  MaskedImage &operator=(const MaskedImage &src) = delete;

private:
  explicit MaskedImage(GWorld &&image, GWorld &&mask, Rect rect) noexcept;
  /// Used to copy an image or mask picture into a `GWorld` during setup.
  static Result<Unit> DrawInto(Picture &picture, const Rect &rect,
                               GWorld &gWorld) noexcept;

  GWorld image;
  GWorld mask;
  Rect rect;
};

} // namespace AtelierEsri
