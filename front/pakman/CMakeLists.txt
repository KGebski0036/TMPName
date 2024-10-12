set(EXEC_NAME "pakman")
set(SOURCE_FILES
        src/main.cpp
        inc/downloader.hpp
        src/downloader.cpp
        inc/gui.hpp
        src/gui.cpp)
file(GLOB_RECURSE HEADER_FILES ${CMAKE_CURRENT_SOURCE_DIR}/inc/*.hpp)
add_executable(${EXEC_NAME} ${SOURCE_FILES} ${HEADER_FILES})
target_include_directories(${EXEC_NAME} PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/inc)
target_link_libraries(${EXEC_NAME} PRIVATE cpr::cpr PRIVATE ftxui::screen ftxui::dom ftxui::component nlohmann_json::nlohmann_json)

set(MAIN_FLAGS ${COMPILE_FLAGS})

target_compile_options(${EXEC_NAME} PRIVATE ${MAIN_FLAGS})