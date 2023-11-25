#pragma once

#include <Quickdraw.h>

#include "Exception.hpp"
#include "GWorld.hpp"
#include "Resource.hpp"
#include "Window.hpp"

namespace AtelierEsri {

/// `PICT` resource with typed operations.
class Picture {
public:
  static Picture Get(ResourceID resourceID);
  Rect Bounds();
  void Draw(const Rect &rect);

  Picture(Picture &&src) noexcept;
  Picture &operator=(Picture &&src) noexcept;
  Picture(const Picture &src) = delete;
  Picture &operator=(const Picture &src) = delete;

private:
  explicit Picture(PICTResource &&resource);
  PICTResource resource;
};

class MaskedImage {
public:
  static MaskedImage Get(int16_t imageResourceID, int16_t maskResourceID,
                         Window &window);
  Rect Bounds();
  /// Copy the masked image into a `GWorld`.
  void Draw(GWorld &gWorld, const Rect &srcRect, const Rect &dstRect);

  MaskedImage(MaskedImage &&src) noexcept;
  MaskedImage &operator=(MaskedImage &&src) noexcept;
  MaskedImage(const MaskedImage &src) = delete;
  MaskedImage &operator=(const MaskedImage &src) = delete;

private:
  explicit MaskedImage(GWorld &&image, GWorld &&mask, Rect rect);
  /// Used to copy an image or mask picture into a `GWorld` during setup.
  static void DrawInto(Picture &picture, const Rect &rect, GWorld &gWorld);

  GWorld image;
  GWorld mask;
  Rect rect;
};

} // namespace AtelierEsri
