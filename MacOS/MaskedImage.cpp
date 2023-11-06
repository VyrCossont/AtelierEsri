#include "MaskedImage.hpp"

#include <limits>

#include <PictUtils.h>
#include <Resources.h>

namespace AtelierEsri {

Result<Picture> Picture::Get(ResourceID resourceID) noexcept {
  GUARD_LET_TRY(PICTResource, resource, PICTResource::Get(resourceID));
  return Ok(Picture(std::move(resource)));
}

Picture::Picture(PICTResource &&resource) noexcept
    : resource(std::move(resource)) {}

Result<Rect> Picture::Bounds() noexcept {
  PictInfo pictInfo;
  OS_CHECKED(GetPictInfo(resource.Unmanaged(), &pictInfo, 0, 0, 0, 0),
             "Couldn't get picture info");
  return Ok(pictInfo.sourceRect);
}

Result<Unit> Picture::Draw(const Rect &rect) noexcept {
  QD_CHECKED(DrawPicture(resource.Unmanaged(), &rect), "Couldn't draw picture");
  return Ok(Unit());
}

Result<MaskedImage> MaskedImage::Get(int16_t imageResourceID,
                                     int16_t maskResourceID,
                                     Window &window) noexcept {
  GUARD_LET_TRY(Picture, imagePicture, Picture::Get(imageResourceID));
  GUARD_LET_TRY(Rect, imageRect, imagePicture.Bounds());
  if (imageRect.left != 0 || imageRect.top != 0) {
    BAIL("Image rect doesn't start at origin");
  }

  GUARD_LET_TRY(Picture, maskPicture, Picture::Get(maskResourceID));
  GUARD_LET_TRY(Rect, maskRect, imagePicture.Bounds());
  if (maskRect.left != 0 || maskRect.top != 0) {
    BAIL("Mask rect doesn't start at origin");
  }

  if (imageRect.right != maskRect.right ||
      imageRect.bottom != maskRect.bottom) {
    BAIL("Image dimensions don't match mask dimensions");
  }

  GUARD_LET_TRY(GWorld, image,
                window.FastGWorld(imageRect.right, imageRect.bottom));
  TRY(DrawInto(imagePicture, imageRect, image));

  GUARD_LET_TRY(GWorld, mask,
                window.FastGWorld(imageRect.right, imageRect.bottom));
  TRY(DrawInto(maskPicture, maskRect, mask));

  return Ok(MaskedImage(std::move(image), std::move(mask), imageRect));
}

Rect MaskedImage::Bounds() noexcept { return rect; }

Result<Unit> MaskedImage::Draw(AtelierEsri::GWorld &destination,
                               const Rect &destinationRect) noexcept {
  GUARD_LET_TRY(GWorldLockPixelsGuard, destinationLockPixelsGuard,
                destination.LockPixels());
  GUARD_LET_TRY(GWorldLockPixelsGuard, imageLockPixelsGuard,
                image.LockPixels());
  GUARD_LET_TRY(GWorldLockPixelsGuard, maskLockPixelsGuard, mask.LockPixels());

  QD_CHECKED(CopyMask(imageLockPixelsGuard.Bits(), maskLockPixelsGuard.Bits(),
                      destinationLockPixelsGuard.Bits(), &rect, &rect,
                      &destinationRect),
             "CopyMask failed");

  return Ok(Unit());
}

MaskedImage::MaskedImage(GWorld &&image, GWorld &&mask, Rect rect) noexcept
    : image(std::move(image)), mask(std::move(mask)), rect(rect) {}

Result<Unit> MaskedImage::DrawInto(Picture &picture, const Rect &rect,
                                   GWorld &gWorld) noexcept {
  GUARD_LET_TRY(GWorldLockPixelsGuard, lockPixelsGuard, gWorld.LockPixels());
  GWorldActiveGuard activeGuard = gWorld.MakeActive();

  picture.Draw(rect);

  return Ok(Unit());
}

} // namespace AtelierEsri
