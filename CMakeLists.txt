cmake_minimum_required(VERSION 3.15)

project(mydemoapp)

add_executable(myapp main.c)

include_directories(${CMAKE_SOURCE_DIR})

target_link_libraries(myapp PRIVATE ${CMAKE_SOURCE_DIR}/target/release/libmylib.dylib)
