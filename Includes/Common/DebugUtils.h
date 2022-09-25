// (c) 2022 Snowapril
// This code is licensed under MIT license (see LICENSE.txt for details)

#ifndef DEBUG_UTILS_H
#define DEBUG_UTILS_H

#include <spdlog/spdlog.h>

namespace TDR
{
void PrintCallStack();

#if defined(TDR_DEBUG)

#if defined(_MSC_VER) || defined(__INTEL_COMPILER)
#define TDR_DEBUG_BREAK __debugbreak()
#else
#define TDR_DEBUG_BREAK
#endif

#define TDR_ASSERT(condition, message)                                  \
    if (!(condition))                                                   \
    {                                                                   \
        spdlog::debug("Assertion failed: {}, {}", #condition, message); \
        PrintCallStack();                                               \
        TDR_DEBUG_BREAK;                                                \
    }

#else

#define TDR_DEBUG_BREAK
#define TDR_ASSERT(condition, message)

#endif
}  // namespace TDR

#endif  // DEBUG_UTILS_HPP