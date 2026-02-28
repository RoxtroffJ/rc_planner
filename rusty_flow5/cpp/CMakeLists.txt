
cmake_minimum_required(VERSION 3.16)

project(PlaneRun LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

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


add_library(wrapper wrapper.cpp)

# if using MKL, link their libraries
target_link_libraries(wrapper XFoil fl5-lib Qt6::Core ${OpenCASCADE_LIBRARIES})

install(TARGETS wrapper
    LIBRARY DESTINATION lib
    RUNTIME DESTINATION bin
)