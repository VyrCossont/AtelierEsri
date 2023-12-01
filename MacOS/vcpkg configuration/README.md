# vcpkg configuration

Put these custom triplet files into vcpkg's triplet directory (if managed by
CLion, `~/.vcpkg-clion/vcpkg/triplets/community`). Then set each target's CLion CMake toolchain's environment variables
to use
them, for example:

- `-DVCPKG_TARGET_TRIPLET=m68k-apple-macos`
- `-DCMAKE_TOOLCHAIN_FILE=~/.vcpkg-clion/vcpkg/scripts/buildsystems/vcpkg.cmake`
- `-DVCPKG_CHAINLOAD_TOOLCHAIN_FILE=~/Projects/Retro68-build/toolchain/m68k-apple-macos/cmake/retro68.toolchain.cmake`

It's not clear why `VCPKG_CHAINLOAD_TOOLCHAIN_FILE` has to be set both in the vcpkg triplet and in CLion's settings.
Maybe it's a CLion issue.

Host CLion CMake toolchains must likewise use `VCPKG_TARGET_TRIPLET` for some reason,
or `find_path`/`target_include_directories` will not work:

- `-DVCPKG_TARGET_TRIPLET=x64-osx` (or `arm64-osx` in future)
- `-DCMAKE_TOOLCHAIN_FILE=~/.vcpkg-clion/vcpkg/scripts/buildsystems/vcpkg.cmake`
