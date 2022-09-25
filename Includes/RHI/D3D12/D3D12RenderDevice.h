// (c) 2022 Snowapril
// This code is licensed under MIT license (see LICENSE.txt for details)

#ifndef D3D12_RENDERDEVICE_H
#define D3D12_RENDERDEVICE_H

#include <RHI/RHIRenderDevice.h>

namespace TDR
{
class D3D12RenderDevice : public RHIRenderDevice
{
 public:
    D3D12RenderDevice() = default;
    virtual ~D3D12RenderDevice() = default;
};
}  // namespace TDR

#endif  // D3D12_RENDERDEVICE_H