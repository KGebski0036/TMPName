#include <downloader.hpp>
#include <string>
#include "gui.hpp"

auto main() -> int {
    std::string url = "http://localhost:8080/master.json";  // Replace with the actual file path
    std::string outputFile = "downloaded_file.txt";

    downloadFile(url, outputFile);
    draw();
}
