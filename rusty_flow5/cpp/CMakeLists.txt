
cmake_minimum_required(VERSION 3.16)

project(PlaneRun LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

set(CARGO_TARGET_DIR ${CMAKE_SOURCE_DIR}/../target)

if (WIN32)
include_directories(D:/dev/flow5/XFoil-lib)
include_directories(D:/dev/flow5/fl5-lib)
include_directories(D:/dev/flow5/fl5-lib/api)
include_directories(C:/Qt/6.9.1/msvc2022_64/include)
include_directories(C:/Qt/6.9.1/msvc2022_64/include/QtCore)

link_directories(D:/dev/build/flow5/release/fl5-lib) 
link_directories(D:/dev/build/flow5/release/XFoil-lib)
link_directories(C:/Qt/6.9.1/msvc2022_64/lib)
set(CMAKE_CXX_FLAGS /Zc:__cplusplus)
else ()
include_directories(/usr/local/include/XFoil/)
include_directories(/usr/local/include/fl5-lib/)
include_directories(/usr/local/include/fl5-lib/api/)

find_package(OpenCASCADE REQUIRED)
find_package(Qt6 REQUIRED COMPONENTS Core)

link_directories(/usr/local/lib/)
link_directories(/usr/lib64/)
endif (WIN32)


# Build `wrapper` as a shared library so its link dependencies are preserved at runtime
add_library(wrapper SHARED wrapper.cpp)

# Ensure the `cpp/` directory is on the include path for the `wrapper` target
target_include_directories(wrapper PRIVATE ${CMAKE_SOURCE_DIR})

# if using MKL, link their libraries
target_link_libraries(wrapper XFoil fl5-lib Qt6::Core ${OpenCASCADE_LIBRARIES})

install(TARGETS wrapper
    LIBRARY DESTINATION lib
    RUNTIME DESTINATION bin
)

# -- cxxbridge integration: include generated headers and sources
# The cxx crate writes generated C++ sources/headers under target/cxxbridge
set(CXXBRIDGE_ROOT ${CARGO_TARGET_DIR}/cxxbridge/rusty_flow5/src)
set(CXXBRIDGE_HDR_DIR ${CARGO_TARGET_DIR}/cxxbridge/rust)

# Collect any generated C++ sources under the cxxbridge output tree (recursively).
if(EXISTS ${CXXBRIDGE_ROOT})
    # initialize list
    set(CXXBRIDGE_SOURCES_FULL)
    file(GLOB_RECURSE _cxxbridge_files
        "${CXXBRIDGE_ROOT}/*.cc"
        "${CXXBRIDGE_ROOT}/*.cpp"
        "${CXXBRIDGE_ROOT}/*.c"
    )
    if(_cxxbridge_files)
        foreach(_f IN LISTS _cxxbridge_files)
            list(APPEND CXXBRIDGE_SOURCES_FULL ${_f})
        endforeach()
        target_sources(wrapper PRIVATE ${CXXBRIDGE_SOURCES_FULL})
        target_include_directories(wrapper PRIVATE ${CXXBRIDGE_ROOT} ${CXXBRIDGE_HDR_DIR})
        message(STATUS "Adding cxxbridge sources: ${CXXBRIDGE_SOURCES_FULL}")
    else()
        message(WARNING "No cxxbridge .cc/.cpp/.c sources found under ${CXXBRIDGE_ROOT}")
    endif()
else()
    message(WARNING "cxxbridge root ${CXXBRIDGE_ROOT} does not exist. Run 'cargo build' to generate C++ glue code.")
endif()