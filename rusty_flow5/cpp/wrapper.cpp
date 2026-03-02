#include "wrapper.h"

namespace modified
{
    namespace globals
    {
        std::unique_ptr<std::string> poplog()
        {
            return std::make_unique<std::string>(::globals::poplog());
        }
    }
}