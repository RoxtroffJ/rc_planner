cmake_minimum_required(VERSION 3.16)

project(PlaneRun LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# -------------------------------
# Find Qt
# -------------------------------
find_package(Qt6 REQUIRED COMPONENTS Core)

# -------------------------------
# Flow 5
# -------------------------------
if (WIN32)
    include_directories(
        D:/dev/flow5/XFoil-lib
        D:/dev/flow5/fl5-lib
        D:/dev/flow5/fl5-lib/api
    )

    link_directories(
        D:/dev/build/flow5/release/fl5-lib
        D:/dev/build/flow5/release/XFoil-lib
    )

    set(CMAKE_CXX_FLAGS /Zc:__cplusplus)
else()
    include_directories(
        /usr/local/include/XFoil
        /usr/local/include/fl5-lib
        /usr/local/include/fl5-lib/api
        /usr/local/include/opencascade
    )

    link_directories(
        /usr/local/lib
        /usr/lib64
    )
endif()

# -------------------------------
# Build library
# -------------------------------
add_library(Lib lib/lib.cpp)

target_include_directories(Lib
    PUBLIC
        lib
)

# -------------------------------
# Build executable
# -------------------------------
add_executable(Main src/main.cpp)

# -------------------------------
# Link libraries
# -------------------------------
target_link_libraries(Main
    PRIVATE
        XFoil
        fl5-lib
        Qt6::Core
        Lib
)

# -------------------------------
# Install
# -------------------------------
# include(GNUInstallDirs)
# install(TARGETS PlaneRun
#     RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR}
# )
