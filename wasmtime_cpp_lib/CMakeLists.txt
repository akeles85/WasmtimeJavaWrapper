cmake_minimum_required(VERSION 3.0.0)

project(wasmtime_wrapper_lib)

# Set the C++ standard
set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# project source directories to be compiled
set(user_sources
    "multiply.c"
    "WasmtimeWrapper.cpp"
)

set( BUILD_SHARED_LIBS ON)

set( user_compile_options "-fPIC")

set( WASMTIME_PATH "/Users/akeles/workspace/wasm/wasmtime")

# project include directories
set(user_include_dirs
    "${WASMTIME_PATH}/crates/c-api/include"
    "${WASMTIME_PATH}/crates/c-api/wasm-c-api/include"
    "/opt/homebrew/Cellar/openjdk/19.0.1/include"
)

# project link libraries
set(user_link_libraries
    "dl"
    "c"
    "pthread"
    "m"
    "${WASMTIME_PATH}/target/release/libwasmtime.a"
)

# compiler options
add_compile_options(${user_compile_options})

# set include directories
include_directories(${user_include_dirs})

# set link directories, library search paths
link_directories(${user_link_dirs})

link_libraries(${user_link_libraries})

add_library(${PROJECT_NAME} ${user_sources})