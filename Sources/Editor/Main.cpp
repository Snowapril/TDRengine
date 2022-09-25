// (c) 2022 Snowapril
// This code is licensed under MIT license (see LICENSE.txt for details)

#include <spdlog/spdlog.h>
#include <Common/CommonConstants.h>

using namespace TDR;

int main(int argc, char* argv[])
{
    (void)argc;
    (void)argv;

#if defined(TDR_DEBUG)
    spdlog::set_level(spdlog::level::debug);
#else
    spdlog::set_level(spdlog::level::info);
#endif

    spdlog::info("This is TDRengine v{}.{}.{}", ENGINE_VERSION_MAJOR,
                 ENGINE_VERSION_MINOR, ENGINE_VERSION_PATCH);

    return 0;
}