cmake_minimum_required(VERSION 3.27)
project(AtelierEsri)

set(CMAKE_CXX_STANDARD 17)

# Suppress warning about CMAKE_TOOLCHAIN_FILE: https://stackoverflow.com/a/17085081
MESSAGE(STATUS "Using toolchain file: ${CMAKE_TOOLCHAIN_FILE}")

if ((VCPKG_TARGET_TRIPLET STREQUAL m68k-apple-macos)
        OR (VCPKG_TARGET_TRIPLET STREQUAL powerpc-apple-macos)
        OR (VCPKG_TARGET_TRIPLET STREQUAL powerpc-apple-macos-carbon))
    set(RETRO68 TRUE CACHE BOOL "Is this target Retro68?")
else ()
    set(RETRO68 FALSE CACHE BOOL "Is this target Retro68?")
endif ()

# Portable library containing platform-independent code.
# In particular, should not depend on Apple universal headers.
add_library(Breeze STATIC
        src/Breeze/Alchemy.cpp
        src/Breeze/Cinematics.cpp
)

find_path(BETTER_ENUMS_INCLUDE_DIR NAMES better-enums/enum.h)
target_include_directories(Breeze PUBLIC ${BETTER_ENUMS_INCLUDE_DIR})

# Target for running Breeze tests on the host platform.
if (NOT RETRO68)
    add_executable(BreezeTests
            src/Breeze
            src/Breeze/AlchemyTest.cpp
            src/Breeze/BinIO/BinIOTest.cpp
            src/Breeze/Geometry/IntersectsTest.cpp
    )
    target_link_libraries(BreezeTests PUBLIC Breeze)
    target_include_directories(BreezeTests PUBLIC "${PROJECT_SOURCE_DIR}/src")

    find_package(Catch2 3 REQUIRED)
    target_link_libraries(BreezeTests PRIVATE Catch2::Catch2WithMain)

    include(CTest)
    include(Catch)
    catch_discover_tests(BreezeTests)
endif ()

# The actual game, for the target platforms.
if (RETRO68)
    add_application(AtelierEsri
            # CONSOLE
            src/main.cpp
            src/AppResources.r
            src/Assets.rsrc
            src/App.cpp
            src/Alert.cpp
            src/AtelierInterior/AtelierInteriorGameMode.cpp
            src/Cinematic/AlchemySlightlyExplained.cpp
            src/Cinematic/CinematicGameMode.cpp
            src/Control.cpp
            src/Debug.cpp
            src/Drawing.cpp
            src/Env.cpp
            src/Exception.cpp
            src/Game.cpp
            src/GWorld.cpp
            src/Inventory/InventoryCell.cpp
            src/Inventory/InventoryController.cpp
            src/MaskedImage.cpp
            src/Material.cpp
            src/SpriteSheet.cpp
            src/Strings.cpp
            src/Synthesis/ElementValueDisplay.cpp
            src/Synthesis/SynthesisCell.cpp
            src/Synthesis/SynthesisController.cpp
            src/Synthesis/SynthesisDashboard.cpp
            src/Synthesis/SynthesisGameMode.cpp
            src/Walkaround/Tilemap.cpp
            src/Walkaround/TMXData.cpp
            src/Walkaround/TSXData.cpp
            src/Walkaround/WalkaroundGameMode.cpp
            src/Window.cpp
    )
    target_link_libraries(AtelierEsri PUBLIC Breeze)
    target_include_directories(AtelierEsri PUBLIC "${PROJECT_SOURCE_DIR}/src")

    add_custom_target(Launch
            "${RETRO68_ROOT}/bin/LaunchAPPL"
            --emulator shared
            --shared-directory "${CMAKE_HOME_DIRECTORY}/../../../Desktop/MacEmulation/LaunchAPPLShared"
            "${AtelierEsri_BINARY_DIR}/AtelierEsri.APPL"
    )
    add_dependencies(Launch AtelierEsri_APPL)
endif ()

# Size reduction disabled because it breaks exceptions on 68k:
# https://github.com/autc04/Retro68/issues/217

# make the result as small as possible
# by removing unused code (gc-sections)
# and by removing macsbug function names on 68K
# (don't do this when debugging...)
#set_target_properties(AtelierEsri PROPERTIES COMPILE_OPTIONS -ffunction-sections)
#if (CMAKE_SYSTEM_NAME MATCHES Retro68)
#    set_target_properties(AtelierEsri PROPERTIES LINK_FLAGS " -Wl,-gc-sections -Wl,--mac-strip-macsbug")
#else ()
#    set_target_properties(AtelierEsri PROPERTIES LINK_FLAGS "-Wl,-gc-sections")
#endif ()
