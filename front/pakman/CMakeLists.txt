set(EXEC_NAME "pakman")
set(SOURCE_FILES
        src/main.cpp)
file(GLOB_RECURSE HEADER_FILES ${CMAKE_CURRENT_SOURCE_DIR}/inc/*.hpp)
add_executable(${EXEC_NAME} ${SOURCE_FILES} ${HEADER_FILES})
target_include_directories(${EXEC_NAME} PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/inc)

set(MAIN_FLAGS ${COMPILE_FLAGS})

target_compile_options(${EXEC_NAME} PRIVATE ${MAIN_FLAGS})