#include <spdlog/spdlog.h>
#include <Common/CommonConstants.hpp>

using namespace TDR;

int main(int argc, char* argv[])
{
    spdlog::info("This is TDRengine v{}.{}.{}", ENGINE_VERSION_MAJOR,
                 ENGINE_VERSION_MINOR, ENGINE_VERSION_PATCH);

    return 0;
}