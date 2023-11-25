cmake_minimum_required(VERSION 3.26)
project(AtelierEsri)

set(CMAKE_CXX_STANDARD 17)

add_application(AtelierEsri
        # CONSOLE
        main.cpp
        AppResources.r
        Assets.rsrc
        App.cpp
        Alert.cpp
        Debug.cpp
        Drawing.cpp
        Env.cpp
        Error.cpp
        Game.cpp
        GWorld.cpp
        MaskedImage.cpp
        SpriteSheet.cpp
        Strings.cpp
        Window.cpp
)

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
