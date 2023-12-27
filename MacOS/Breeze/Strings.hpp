#pragma once

namespace Breeze {

class Strings {
 public:
  /// Get the last component of a file path.
  /// `FileName(__FILE__)` is equivalent to clang/future GCC's `__FILE_NAME__`.
  static constexpr const char *FileName(const char *path) {
    // Assume we're compiling on a modern Mac or Linux box,
    // because I have no idea how to detect the host system in a cross build.
    constexpr char separator = '/';

    char const *filename = path;
    if (!filename) {
      return filename;
    }

    char const *c = path;
    while (*c) {
      // Take the last part of the path.
      if (*c == separator && *(c + 1)) {
        filename = c + 1;
      }
      c++;
    }

    return filename;
  }
};

}  // namespace Breeze
