// (c) 2022 Snowapril
// This code is licensed under MIT license (see LICENSE.txt for details)

#ifndef VULKAN_RENDERDEVICE_H
#define VULKAN_RENDERDEVICE_H

#include <RHI/RHIRenderDevice.h>

namespace TDR
{
class VulkanRenderDevice : public RHIRenderDevice
{
 public:
    VulkanRenderDevice() = default;
    virtual ~VulkanRenderDevice() = default;
};
}  // namespace TDR

#endif  // VULKAN_RENDERDEVICE_H