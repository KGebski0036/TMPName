#include <downloader.hpp>
#include <string>
#include <fstream>

#include "gui.hpp"
#include "nlohmann/json.hpp"

auto main() -> int {
    std::string url = "http://localhost:8080/get_metadata_all";  // Replace with the actual file path
    std::string outputFile = "master.json";

    downloadFile(url, outputFile);

    std::ifstream t(outputFile);
    std::stringstream buffer;
    buffer << t.rdbuf();
    const nlohmann::json j = nlohmann::json::parse(buffer.str());
    draw(j);
}
