// (c) 2022 Snowapril
// This code is licensed under MIT license (see LICENSE.txt for details)

#ifndef TDR_ENGINE_H
#define TDR_ENGINE_H

#include <RHI/RHIRenderDevice.h>

namespace TDR
{
class TDRengine
{
 public:
    TDRengine() = default;
    ~TDRengine() = default;

 private:
    RHIRenderDevice* _renderDevice{ nullptr };
};
}  // namespace TDR

#endif  // TDR_ENGINE_H