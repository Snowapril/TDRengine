# CMake version
cmake_minimum_required(VERSION 3.18.6 FATAL_ERROR)
cmake_policy(SET CMP0063 NEW) # visibility
set(CMAKE_CXX_VISIBILITY_PRESET hidden)
set(CMAKE_VISIBILITY_INLINES_HIDDEN 1)

# Include cmake modules
list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_SOURCE_DIR}/Builds/CMake")

# Declare project
project(TDRengine)

# Set output directories
set(DEFAULT_CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_LIBRARY_OUTPUT_DIRECTORY})
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${PROJECT_BINARY_DIR}/bin)
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${PROJECT_BINARY_DIR}/lib)
set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${PROJECT_BINARY_DIR}/lib)

# Set enable output of compile commands during generation
set(CMAKE_EXPORT_COMPILE_COMMANDS ON CACHE INTERNAL "")

# Compile options
include(Builds/CMake/CompileOptions.cmake)

# Build type - Release by default
message("CMake build type: " ${CMAKE_BUILD_TYPE})
if(NOT CMAKE_BUILD_TYPE)
    set(CMAKE_BUILD_TYPE Release)
elseif(CMAKE_BUILD_TYPE MATCHES "Debug")
    add_definitions(-DTDR_DEBUG)
endif()

option(BUILD_COVERAGE "Build code coverage" OFF)
if (CMAKE_BUILD_TYPE MATCHES Debug AND CMAKE_COMPILER_IS_GNUCXX AND BUILD_COVERAGE)
    include(CodeCoverage)
    setup_target_for_coverage(${PROJECT_NAME}_coverage UnitTests coverage)
endif()

# Overrides
set(CMAKE_MACOSX_RPATH ON)

# Set resource directory
set(RESOURCES_DIR "${CMAKE_CURRENT_SOURCE_DIR}/Resources/")

# Add definitions for third-party dependencies
add_definitions(-DVK_NO_PROTOTYPES)
add_definitions(-DGLM_FORCE_SWIZZLE)
add_definitions(-DGLM_FORCE_RADIANS)
add_definitions(-DGLM_FORCE_CTOR_INIT)
add_definitions(-DGLM_ENABLE_EXPERIMENTAL)

# Includes
include_directories(Dependencies/src)
include_directories(Dependencies/src/vulkan/include)
include_directories(Dependencies/src/vma/src)
include_directories(Dependencies/src/glfw/include)
include_directories(Dependencies/src/glm)
include_directories(Dependencies/src/assimp/include)
include_directories(Dependencies/src/etc2comp/EtcLib/Etc)
include_directories(Dependencies/src/etc2comp/EtcLib/EtcCodec)
include_directories(Dependencies/src/taskflow)
include_directories(Dependencies/src/stb)
include_directories(Dependencies/src/ImGuizmo/src)
include_directories(Dependencies/src/spdlog/include)
include_directories(Dependencies/src/spirv-reflect/include)

# include_directories(Dependencies/src/DirectXShaderCompiler/include)

# CMake Configs for GLFW
set(GLFW_BUILD_EXAMPLES OFF CACHE BOOL "")
set(GLFW_BUILD_TESTS    OFF CACHE BOOL "")
set(GLFW_BUILD_DOCS     OFF CACHE BOOL "")
set(GLFW_INSTALL        OFF CACHE BOOL "")

# CMake Configs for Assimp
set(ASSIMP_NO_EXPORT                        ON  CACHE BOOL "")
set(ASSIMP_BUILD_DRACO                      OFF CACHE BOOL "")
set(ASSIMP_BUILD_ASSIMP_TOOLS               OFF CACHE BOOL "")
set(ASSIMP_BUILD_TESTS                      OFF CACHE BOOL "")
set(ASSIMP_INSTALL_PDB                      OFF CACHE BOOL "")
set(ASSIMP_BUILD_ALL_IMPORTERS_BY_DEFAULT   OFF CACHE BOOL "")
set(ASSIMP_BUILD_OBJ_IMPORTER               ON  CACHE BOOL "")
set(ASSIMP_BUILD_GLTF_IMPORTER              ON  CACHE BOOL "")

# CMake Configs for MeshOptimizer
set(MESHOPT_BUILD_DEMO          OFF CACHE BOOL "")
set(MESHOPT_BUILD_TOOLS         OFF CACHE BOOL "")
set(MESHOPT_BUILD_SHARED_LIBS   OFF CACHE BOOL "")

# CMake Configs for DirectShaderCompiler
# set(ENABLE_SPIRV_CODEGEN     ON CACHE BOOL "")
# set(LLVM_BUILD_EXAMPLES     OFF CACHE BOOL "")
# set(LLVM_INCLUDE_EXAMPLES   OFF CACHE BOOL "")
# set(LLVM_BUILD_TESTS        OFF CACHE BOOL "")
# set(LLVM_INCLUDE_TESTS      OFF CACHE BOOL "")

# CMake Configs for Spirv-Reflect
set(SPIRV_REFLECT_EXECUTABLE    OFF CACHE BOOL "")
set(SPIRV_REFLECT_EXAMPLES      OFF CACHE BOOL "")
set(SPIRV_REFLECT_BUILD_TESTS   OFF CACHE BOOL "")
set(SPIRV_REFLECT_STATIC_LIB    ON CACHE BOOL "")

add_subdirectory(Dependencies/src/glfw)
add_subdirectory(Dependencies/src/assimp)
add_subdirectory(Dependencies/src/meshoptimizer)
add_subdirectory(Dependencies/src/etc2comp)
add_subdirectory(Dependencies/src/spdlog)
add_subdirectory(Dependencies/src/spirv-reflect)

# add_subdirectory(Dependencies/src/DirectXShaderCompiler)

set_property(GLOBAL PROPERTY USE_FOLDERS ON)
set_property(TARGET glfw          PROPERTY FOLDER "ThirdPartyLibraries")
set_property(TARGET assimp        PROPERTY FOLDER "ThirdPartyLibraries")
set_property(TARGET EtcLib        PROPERTY FOLDER "ThirdPartyLibraries")
set_property(TARGET EtcTool       PROPERTY FOLDER "ThirdPartyLibraries")
set_property(TARGET meshoptimizer PROPERTY FOLDER "ThirdPartyLibraries")
set_property(TARGET spirv-reflect PROPERTY FOLDER "ThirdPartyLibraries")
if(WIN32)
    set_property(TARGET UpdateAssimpLibsDebugSymbolsAndDLLs PROPERTY FOLDER "ThirdPartyLibraries")
endif()
set_property(TARGET uninstall     PROPERTY FOLDER "ThirdPartyLibraries")

# Project modules
add_subdirectory(Sources/Common)
add_subdirectory(Sources/RHI)
add_subdirectory(Sources/Editor)
add_subdirectory(Tests/UnitTests)

# Dependencies
add_dependencies(Sources/RHI Sources/Common)
add_dependencies(Sources/Editor Sources/Common Sources/RHI)
add_dependencies(Tests/UnitTests Sources/Editor Sources/Common Sources/RHI)