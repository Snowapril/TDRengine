# Target name
set(target RHI)
set(ROOT_DIR ${PROJECT_SOURCE_DIR})
set(LIBRARY_DIR ${PROJECT_SOURCE_DIR}/Dependencies/src)
set(PUBLIC_HDR_DIR ${PROJECT_SOURCE_DIR}/Includes)
set(SRC_DIR ${CMAKE_CURRENT_SOURCE_DIR})

# Set public headers
set(PUBLIC_HDRS
    ${PUBLIC_HDR_DIR}/D3D12/D3D12RenderDevice.h
    ${PUBLIC_HDR_DIR}/Vulkan/VulkanRenderDevice.h
    ${PUBLIC_HDR_DIR}/RHIRenderDevice.h
    ${PUBLIC_HDR_DIR}/TDRengine.h
)

# Set Sources
set(SRCS
    ${SRC_DIR}/D3D12/D3D12RenderDevice.cpp
    ${SRC_DIR}/Vulkan/VulkanRenderDevice.cpp
    ${SRC_DIR}/RHIRenderDevice.cpp
    ${SRC_DIR}/TDRengine.cpp
)

# Build library
add_library(${target} ${SRCS})

# Project options
set_target_properties(${target}
    PROPERTIES
    ${DEFAULT_PROJECT_OPTIONS}
    PUBLIC_HEADER "${PUBLIC_HDRS}"
)

#Include directories
target_include_directories(${target}
    PUBLIC
    $<BUILD_INTERFACE:${PUBLIC_HDR_DIR}>
    $<INSTALL_INTERFACE:${CMAKE_INSTALL_PREFIX}/include>
    PRIVATE
    ${PUBLIC_HDR_DIR}
)

# Compile options
target_compile_options(${target}
    PUBLIC
    PRIVATE
    ${DEFAULT_COMPILE_OPTIONS}
)

# Link libraries
target_link_libraries(${target}
    PUBLIC
    ${DEFAULT_LINKER_OPTIONS}
	${DEFAULT_LIBRARIES}
    PRIVATE
    Common
)

# Compile definitions
target_compile_definitions(${target}
    PRIVATE
    RESOURCES_DIR="${RESOURCES_DIR}"
    ${DEFAULT_COMPILE_DEFINITIONS}
)

# Install
install(TARGETS  
        ${target} 
        RUNTIME  DESTINATION bin
        LIBRARY  DESTINATION lib
        ARCHIVE  DESTINATION lib
        PUBLIC_HEADER DESTINATION include
)