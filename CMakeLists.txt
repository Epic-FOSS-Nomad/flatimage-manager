cmake_minimum_required(VERSION 3.16)
project(FlatImageManager VERSION 1.0.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Find Qt components
find_package(Qt6 COMPONENTS Core Widgets Gui Concurrent REQUIRED)

# Get Git commit hash
find_package(Git)
if (GIT_FOUND)
    execute_process(
        COMMAND ${GIT_EXECUTABLE} rev-parse --short HEAD
        WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
        OUTPUT_VARIABLE GIT_COMMIT_HASH
        OUTPUT_STRIP_TRAILING_WHITESPACE
    )
else()
    set(GIT_COMMIT_HASH "unknown")
endif()

string(TIMESTAMP BUILD_DATE "%Y-%m-%dT%H:%M:%S%z")

# Core library
add_library(CoreLib STATIC
    core/lib/fsimaging/ImageBuilder.cpp
)

target_include_directories(CoreLib PUBLIC 
    ${CMAKE_CURRENT_SOURCE_DIR}/core/api
)

target_link_libraries(CoreLib PUBLIC
    Qt6::Core
)

if (UNIX AND NOT APPLE)
    target_link_libraries(CoreLib PRIVATE util)
endif()

configure_file(
    ${CMAKE_CURRENT_SOURCE_DIR}/version.h.in
    ${CMAKE_CURRENT_BINARY_DIR}/version.h
)

# GUI executable
add_executable(FlatImageManager
    gui/main.cpp
    gui/MainWindow.cpp
    gui/theming/ThemeEngine.cpp
)

if (WIN32)
    set(CMAKE_RC_COMPILER_INIT windres)
    enable_language(RC)
    set(CMAKE_RC_COMPILE_OBJECT "<CMAKE_RC_COMPILER> <FLAGS> -O coff <DEFINES> -i <SOURCE> -o <OBJECT>")
    
    set(APP_ICON "${CMAKE_CURRENT_SOURCE_DIR}/resources/icons/app-icon.ico")
    add_executable(FlatImageManager WIN32 ${APP_ICON})
endif()

# Add resources
qt_add_resources(appResources "resources"
    PREFIX "/"
    FILES
        resources/icons/app-icon.svg
        resources/styles/modern-dark.qss
        resources/styles/light.qss
        resources/icons/alpine.png
        resources/icons/arch.png
        resources/icons/blueprint.png
        resources/icons/custom.png
)

target_link_libraries(FlatImageManager PRIVATE
    Qt6::Core
    Qt6::Widgets
    Qt6::Gui
    CoreLib
    appResources
)

# Plugins
add_library(CreatorPlugin MODULE core/plugins/creator/CreatorPlugin.cpp)
target_link_libraries(CreatorPlugin PRIVATE CoreLib Qt6::Core Qt6::Widgets)

add_library(ConverterPlugin MODULE core/plugins/converter/PackageConverter.cpp)
target_link_libraries(ConverterPlugin PRIVATE CoreLib Qt6::Core Qt6::Widgets)

# Final optimizations
if (CMAKE_BUILD_TYPE STREQUAL "Release")
    add_compile_options(-O3 -flto)
    add_link_options(-flto)
endif()

# Installation
install(TARGETS FlatImageManager DESTINATION bin)
install(TARGETS CreatorPlugin ConverterPlugin
        DESTINATION lib/flatimage-manager/plugins)
