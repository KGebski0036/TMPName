#include <downloader.hpp>
#include <string>
#include "gui.hpp"

auto main() -> int {
    std::string url = "http://localhost:8080/master.json";
    std::string outputFile = "master.json";

    downloadFile(url, outputFile);
    //draw();
}
