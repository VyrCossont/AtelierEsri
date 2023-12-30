#include "MaskedImage.hpp"

#include <PictUtils.h>

namespace AtelierEsri {

Picture Picture::Get(const ResourceID resourceID) {
  PICTResource resource = PICTResource::Get(resourceID);
  return Picture(std::move(resource));
}

Picture::Picture(PICTResource &&resource) : resource(std::move(resource)) {}

Picture::Picture(Picture &&src) noexcept : resource(std::move(src.resource)) {}

Picture &Picture::operator=(Picture &&src) noexcept {
  this->resource = std::move(src.resource);
  return *this;
}

Rect Picture::Bounds() {
  PictInfo pictInfo;
  OS_CHECKED(
      GetPictInfo(resource.Unmanaged(), &pictInfo, 0, 0, 0, 0),
      "Couldn't get picture info"
  );
  return pictInfo.sourceRect;
}

void Picture::Draw(const Rect &rect) {
  QD_CHECKED(DrawPicture(resource.Unmanaged(), &rect), "Couldn't draw picture");
}

MaskedImage MaskedImage::Get(
    const ResourceID imageResourceID, const ResourceID maskResourceID
) {
  constexpr V2I origin = {0, 0};

  Picture imagePicture = Picture::Get(imageResourceID);
  const R2I imageRect = imagePicture.Bounds();
  if (imageRect.origin != origin) {
    BAIL("Image rect doesn't start at origin");
  }

  Picture maskPicture = Picture::Get(maskResourceID);
  const R2I maskRect = imagePicture.Bounds();
  if (imageRect.origin != origin) {
    BAIL("Mask rect doesn't start at origin");
  }

  if (imageRect != maskRect) {
    BAIL("Image dimensions don't match mask dimensions");
  }

  GWorld image(imageRect.size);
  DrawInto(imagePicture, imageRect, image);

  GWorld mask(maskRect.size);
  DrawInto(maskPicture, maskRect, mask);

  return MaskedImage(std::move(image), std::move(mask), imageRect);
}

MaskedImage::MaskedImage(MaskedImage &&src) noexcept
    : image(std::move(src.image)), mask(std::move(src.mask)), rect(src.rect) {}

MaskedImage &MaskedImage::operator=(MaskedImage &&src) noexcept {
  this->image = std::move(src.image);
  this->mask = std::move(src.mask);
  this->rect = src.rect;
  return *this;
}

Rect MaskedImage::Bounds() const { return rect; }

void MaskedImage::Draw(const Rect &srcRect, const Rect &dstRect) const {
  const GWorldLockPixelsGuard imageLockPixelsGuard = image.LockPixels();
  const GWorldLockPixelsGuard maskLockPixelsGuard = mask.LockPixels();

  QD_CHECKED(
      CopyMask(
          imageLockPixelsGuard.Bits(),
          maskLockPixelsGuard.Bits(),
          QD::CurrentPortBits(),
          &srcRect,
          &srcRect,
          &dstRect
      ),
      "CopyMask failed"
  );
}

MaskedImage::MaskedImage(GWorld &&image, GWorld &&mask, const Rect rect)
    : image(std::move(image)), mask(std::move(mask)), rect(rect) {}

void MaskedImage::DrawInto(
    Picture &picture, const Rect &rect, const GWorld &gWorld
) {
  GWorldLockPixelsGuard lockPixelsGuard = gWorld.LockPixels();
  GWorldActiveGuard activeGuard = gWorld.MakeActive();

  picture.Draw(rect);
}

}  // namespace AtelierEsri
