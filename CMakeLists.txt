cmake_minimum_required(VERSION 3.0.0)
project(learn-ray-tracing VERSION 0.1.0)

set(CMAKE_CXX_STANDARD 14)

set( SOURCE_ONE_WEEKEND
    src/main.cpp
    src/vec3.cpp
)

add_executable(ray-trace-engine ${SOURCE_ONE_WEEKEND})
