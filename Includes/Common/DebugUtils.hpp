#ifndef DEBUG_UTILS_HPP
#define DEBUG_UTILS_HPP

#include <iostream>
#include <spdlog/spdlog.h>

#if defined(WIN32) || defined(_WIN32) || defined(__WIN32__) || defined(__NT__)
#define NOMINMAX
#include <DbgHelp.h>
#include <windows.h>
#pragma comment(lib, "Dbghelp")
#define STDCALL __stdcall
#else
#define STDCALL
#endif

#if defined(__linux__)
#include <execinfo.h>
#include <unistd.h>
#endif

namespace TDR
{
#if defined(WIN32) || defined(_WIN32) || defined(__WIN32__) || defined(__NT__)
static void PrintCallStack()
{
    unsigned int i;
    void* stack[100];
    unsigned short frames;
    SYMBOL_INFO* symbol;
    HANDLE process;

    process = GetCurrentProcess();

    SymSetOptions(SYMOPT_LOAD_LINES);

    SymInitialize(process, NULL, TRUE);

    frames = CaptureStackBackTrace(0, 200, stack, NULL);
    symbol = (SYMBOL_INFO*)calloc(sizeof(SYMBOL_INFO) + 256 * sizeof(char), 1);
    symbol->MaxNameLen = 255;
    symbol->SizeOfStruct = sizeof(SYMBOL_INFO);

    spdlog::debug("---------------------Stack Trace---------------------\n");
    for (i = 0; i < frames; i++)
    {
        SymFromAddr(process, (DWORD64)(stack[i]), 0, symbol);
        DWORD dwDisplacement;
        IMAGEHLP_LINE64 line;

        line.SizeOfStruct = sizeof(IMAGEHLP_LINE64);
        if (!strstr(symbol->Name, "VSDebugLib::") &&
            SymGetLineFromAddr64(process, (DWORD64)(stack[i]), &dwDisplacement,
                                 &line))
        {
            spdlog::debug("Function : {} - line : {}", symbol->Name,
                          line.LineNumber);
        }

        if (0 == strcmp(symbol->Name, "main"))
            break;
    }
    spdlog::debug("-----------------------------------------------------\n");
    free(symbol);
}
#elif __linux__
static void PrintCallStack()
{
    constexpr size_t kTraceDepth = 10;
    void* arr[kTraceDepth];
    size_t size;

    size = backtrace(arr, kTraceDepth);

    spdlog::debug("---------------------Stack Trace---------------------\n");
    // TODO(snowapril) : share the spdlog file descriptor with the backtrace
    backtrace_symbols_fd(arr, size, 1);
    spdlog::debug("-----------------------------------------------------\n");
}
#endif

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