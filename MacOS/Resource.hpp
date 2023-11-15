#pragma once

#include <memory>

#include <Resources.h>

#include "Result.hpp"

namespace AtelierEsri {

using ResourceID = int16_t;

/// Release the resource without doing anything else.
template <typename ResourceHandleType = Handle>
void ResourceReleaseFn(ResourceHandleType handle) noexcept {
  // TODO: (Vyr) log or panic if this call fails. It can set `ResError()`.
  ReleaseResource(reinterpret_cast<Handle>(handle));
}

/// Custom deleter for a unique pointer equivalent to the resource's handle.
template <typename ResourceHandleType = Handle,
          auto releaseFn = ResourceReleaseFn<ResourceHandleType>>
class ResourceReleaser {
public:
  void operator()(ResourceHandleType resourceHandle) noexcept {
    releaseFn(resourceHandle);
  };
};

/// Get a resource of a given type without doing anything else.
/// May set `ResError()`.
template <ResType resourceType, typename ResourceHandleType = Handle>
ResourceHandleType ResourceGetFn(ResourceID resourceID) noexcept {
  return static_cast<ResourceHandleType>(GetResource(resourceType, resourceID));
}

// Ignore the not-used warning:
// https://youtrack.jetbrains.com/issue/CPP-26277/Incorrect-The-value-is-never-used-with-templated-class
#pragma clang diagnostic push
#pragma ide diagnostic ignored "OCUnusedStructInspection"
/// Managed handle for a non-purgeable resource.
template <ResType resourceType, typename ResourcePointerType = Ptr,
          typename ResourceHandleType = Handle,
          auto getFn = ResourceGetFn<resourceType, ResourceHandleType>,
          auto releaseFn = ResourceReleaseFn<ResourceHandleType>>
class Resource {
public:
  static Result<Resource> Get(ResourceID resourceID) noexcept {
    ResourceHandleType handle =
        RES_CHECKED(getFn(resourceID), "Couldn't load resource");
    REQUIRE_NOT_NULL(handle);
    return Ok(Resource(handle));
  }

  Resource(Resource &&src) noexcept { this->handle = std::move(src.handle); };

  Resource &operator=(Resource &&src) noexcept {
    this->handle = std::move(src.handle);
    return *this;
  };

  Resource(const Resource &src) = delete;

  Resource &operator=(const Resource &src) = delete;

  /// Get an unmanaged handle to the resource.
  /// TODO: (Vyr) support purgeable resources by making this a guard object.
  ResourceHandleType Unmanaged() noexcept { return handle.get(); }

private:
  explicit Resource(ResourceHandleType handle) noexcept : handle(handle) {}

  std::unique_ptr<ResourcePointerType,
                  ResourceReleaser<ResourceHandleType, releaseFn>>
      handle;
};
#pragma clang diagnostic pop

// These `using` declarations confuse CLion but compile and work fine:
using PICTResource = Resource<'PICT', PicPtr, PicHandle, GetPicture>;

} // namespace AtelierEsri
