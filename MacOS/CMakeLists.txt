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
        Breeze/Alchemy.cpp
)

find_path(BETTER_ENUMS_INCLUDE_DIR NAMES better-enums/enum.h)
target_include_directories(Breeze PUBLIC ${BETTER_ENUMS_INCLUDE_DIR})

# Target for running Breeze tests on the host platform.
if (NOT RETRO68)
    add_executable(BreezeTests
            Breeze
            Breeze/AlchemyTest.cpp
    )
    target_link_libraries(BreezeTests PUBLIC Breeze)
    target_include_directories(BreezeTests PUBLIC "${PROJECT_SOURCE_DIR}")

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
            main.cpp
            AppResources.r
            Assets.rsrc
            App.cpp
            Alert.cpp
            Control.cpp
            Debug.cpp
            Drawing.cpp
            Env.cpp
            Exception.cpp
            Game.cpp
            GWorld.cpp
            Inventory/InventoryCell.cpp
            Inventory/InventoryController.cpp
            MaskedImage.cpp
            Material.cpp
            SpriteSheet.cpp
            Strings.cpp
            Synthesis/ElementValueDisplay.cpp
            Synthesis/SynthesisCell.cpp
            Synthesis/SynthesisController.cpp
            Synthesis/SynthesisDashboard.cpp
            Synthesis/SynthesisGameMode.cpp
            Window.cpp
    )
    target_link_libraries(AtelierEsri PUBLIC Breeze)
    target_include_directories(AtelierEsri PUBLIC "${PROJECT_SOURCE_DIR}")
endif ()

# Size reduction disabled because it breaks exceptions on 68k:
# https://github.com/autc04/Retro68/issues/217

# make the result as small as possible
# by removing unused code (gc-sections)
# and by removing macsbug function names on 68K
# (don't do this when debugging...)
#set_target_properties(AtelierEsri PROPERTIES COMPILE_OPTIONS -ffunction-sections)
#if (CMAKE_SYSTEM_NAME MATCHES Retro68)
#    set_target_properties(AtelierEsri PROPERTIES LINK_FLAGS "-Wl,-gc-sections -Wl,--mac-strip-macsbug")
#else ()
#    set_target_properties(AtelierEsri PROPERTIES LINK_FLAGS "-Wl,-gc-sections")
#endif ()
