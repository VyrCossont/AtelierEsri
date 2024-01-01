#pragma once

#include <Resources.h>

#include <memory>

#include "Exception.hpp"

namespace AtelierEsri {

using ResourceID = int16_t;

/// Release the resource without doing anything else.
template <typename ResourceHandleType = Handle>
void ResourceReleaseFn(ResourceHandleType handle) {
  // TODO: (Vyr) log or panic if this call fails. It can set `ResError()`.
  ReleaseResource(reinterpret_cast<Handle>(handle));
}

/// Custom deleter for a unique pointer equivalent to the resource's handle.
template <
    typename ResourceHandleType = Handle,
    auto releaseFn = ResourceReleaseFn<ResourceHandleType>>
class ResourceReleaser {
 public:
  void operator()(ResourceHandleType resourceHandle) {
    releaseFn(resourceHandle);
  }
};

/// Get a resource of a given type without doing anything else.
/// May set `ResError()`.
template <ResType resourceType, typename ResourceHandleType = Handle>
ResourceHandleType ResourceGetFn(const ResourceID resourceID) {
  return static_cast<ResourceHandleType>(GetResource(resourceType, resourceID));
}

// Ignore the not-used warning:
// https://youtrack.jetbrains.com/issue/CPP-26277/Incorrect-The-value-is-never-used-with-templated-class
#pragma clang diagnostic push
#pragma ide diagnostic ignored "OCUnusedStructInspection"
/// Managed handle for a non-purgeable resource.
template <
    ResType resourceType,
    typename ResourcePointerType = Ptr,
    typename ResourceHandleType = Handle,
    auto getFn = ResourceGetFn<resourceType, ResourceHandleType>,
    auto releaseFn = ResourceReleaseFn<ResourceHandleType>>
class Resource {
 public:
  static Resource Get(ResourceID resourceID) {
    ResourceHandleType handle =
        RES_CHECKED(getFn(resourceID), "Couldn't load resource");
    REQUIRE_NOT_NULL(handle);
    return Resource(handle);
  }

  Resource(Resource &&src) noexcept { this->handle = std::move(src.handle); }

  Resource &operator=(Resource &&src) noexcept {
    this->handle = std::move(src.handle);
    return *this;
  }

  Resource(const Resource &src) = delete;

  Resource &operator=(const Resource &src) = delete;

  /// Get an unmanaged handle to the resource.
  /// TODO: (Vyr) support purgeable resources by making this a guard object.
  ResourceHandleType Unmanaged() const { return handle.get(); }

 private:
  explicit Resource(ResourceHandleType handle) : handle(handle) {}

  std::unique_ptr<
      ResourcePointerType,
      ResourceReleaser<ResourceHandleType, releaseFn>>
      handle;
};
#pragma clang diagnostic pop

// These `using` declarations confuse CLion but compile and work fine:
// ReSharper disable CppMultiCharacterLiteral
using PICTResource = Resource<'PICT', PicPtr, PicHandle, GetPicture>;
using RGNResource = Resource<'RGN#'>;
using NinePatchResource = Resource<'9PC#'>;
// ReSharper restore CppMultiCharacterLiteral

}  // namespace AtelierEsri
