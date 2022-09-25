// (c) 2022 Snowapril
// This code is licensed under MIT license (see LICENSE.txt for details)

#ifndef DEBUG_UTILS_H
#define DEBUG_UTILS_H

#include <spdlog/spdlog.h>
#include <iostream>

namespace TDR
{
void PrintCallStack();

#if defined(TDR_DEBUG)

#if defined(_MSC_VER) || defined(__INTEL_COMPILER)
#define TDR_DEBUGBREAK                      \
    {                                       \
        if (IsDebuggerPresent() == TRUE)    \
            __debugbreak();                 \
    }
#else
#define TDR_DEBUGBREAK
#endif

#define TDR_ASSERT(condition, message)                                  \
    if ((condition) == false)                                           \
    {                                                                   \
        spdlog::debug("Assertion failed: {}, {}", #condition, message); \
        PrintCallStack();                                               \
        TDR_DEBUGBREAK;                                                 \
    }

#else

#define TDR_DEBUG_BREAK
#define TDR_ASSERT(condition, message)

#endif
}  // namespace TDR

#endif  // DEBUG_UTILS_H