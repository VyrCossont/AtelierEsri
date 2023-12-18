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
    const int16_t imageResourceID,
    const int16_t maskResourceID,
    const Window &window
) {
  Picture imagePicture = Picture::Get(imageResourceID);
  const Rect imageRect = imagePicture.Bounds();
  if (imageRect.left != 0 || imageRect.top != 0) {
    BAIL("Image rect doesn't start at origin");
  }

  Picture maskPicture = Picture::Get(maskResourceID);
  const Rect maskRect = imagePicture.Bounds();
  if (maskRect.left != 0 || maskRect.top != 0) {
    BAIL("Mask rect doesn't start at origin");
  }

  if (imageRect.right != maskRect.right ||
      imageRect.bottom != maskRect.bottom) {
    BAIL("Image dimensions don't match mask dimensions");
  }

  GWorld image = window.FastGWorld(imageRect.right, imageRect.bottom);
  DrawInto(imagePicture, imageRect, image);

  GWorld mask = window.FastGWorld(imageRect.right, imageRect.bottom);
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

void MaskedImage::DrawInto(Picture &picture, const Rect &rect, GWorld &gWorld) {
  GWorldLockPixelsGuard lockPixelsGuard = gWorld.LockPixels();
  GWorldActiveGuard activeGuard = gWorld.MakeActive();

  picture.Draw(rect);
}

}  // namespace AtelierEsri
