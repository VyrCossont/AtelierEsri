#pragma once

#include <Quickdraw.h>

#include "GWorld.hpp"
#include "Resource.hpp"
#include "Result.hpp"

namespace AtelierEsri {

using PICTResource = Resource<'PICT', PicPtr, PicHandle, GetPicture>;

/// `PICT` resource.
class Picture {
public:
  static Result<Picture> Get(ResourceID resourceID) noexcept;

private:
  explicit Picture(PICTResource &&resource) noexcept;
  PICTResource resource;
};

class MaskedImage {
public:
  static Result<MaskedImage> Get(int16_t imageResourceID,
                                 int16_t maskResourceID) noexcept;

private:
  explicit MaskedImage(GWorld &&image, GWorld &&mask) noexcept;
  GWorld image;
  GWorld mask;
};

} // namespace AtelierEsri
