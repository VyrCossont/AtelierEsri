#include "MaskedPict.hpp"

#include <Resources.h>

namespace AtelierEsri {

Result<Picture> Picture::Get(ResourceID resourceID) noexcept {
  GUARD_LET_TRY(PICTResource, resource, PICTResource::Get(resourceID));
  return Ok(Picture(std::move(resource)));
}

Picture::Picture(PICTResource &&resource) noexcept
    : resource(std::move(resource)) {}

Result<MaskedImage> MaskedImage::Get(int16_t imageResourceID,
                                     int16_t maskResourceID) noexcept {
  GUARD_LET_TRY(Picture, imagePicture, Picture::Get(imageResourceID));

  // TODO: (Vyr) get an offscreen GWorld with the same alignment, colors, etc.
  //  as the game window, but a size appropriate to the actual image.
  GWorld image = exit(666);

  GUARD_LET_TRY(Picture, maskPicture, Picture::Get(maskResourceID));

  GWorld mask = exit(666);

  return Ok(MaskedImage(std::move(image), std::move(mask)));
}

} // namespace AtelierEsri
