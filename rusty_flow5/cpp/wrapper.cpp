#include "wrapper.h"
#include <cstdlib>

#ifdef _OPENMP
#include <omp.h>
#endif

extern "C" void openblas_set_num_threads(int);

namespace
{
    struct OpenBLASThreadInitializer
    {
        OpenBLASThreadInitializer()
        {
            // Set environment variables as a fallback for libraries that
            // read them at startup.
            setenv("OPENBLAS_NUM_THREADS", "1", 1);
            setenv("OMP_NUM_THREADS", "1", 1);

            // Prefer direct API calls when available.
            openblas_set_num_threads(1);
#ifdef _OPENMP
            omp_set_num_threads(1);
#endif
        }
    } openblas_thread_initializer;
}

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