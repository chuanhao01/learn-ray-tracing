cmake_minimum_required(VERSION 3.6.0)
project(learn-ray-tracing VERSION 0.1.0)

set(CMAKE_CXX_STANDARD 14)
set(CMAKE_C_STANDARD 17)
set (CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

set( SOURCE_ONE_WEEKEND
    src/main.cpp
    src/Vec3.cpp
    src/Color.cpp
    src/Ray.cpp
    src/Objects.cpp
)

add_executable(ray-trace-engine ${SOURCE_ONE_WEEKEND})
